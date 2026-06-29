use crate::email::error::EmailError;
use crate::email::ports::email::EmailI;
use crate::model::{Email, EmailStatus, Sender};
use crate::utils::path_utils::get_current_exec_path;
use chrono::DateTime;
use google_gmail1::api::{Message, ModifyMessageRequest};
use google_gmail1::hyper_rustls::HttpsConnector;
use google_gmail1::hyper_util::client::legacy::connect::HttpConnector;
use google_gmail1::{hyper_rustls, hyper_util, yup_oauth2, Gmail};
use log::warn;
use regex::Regex;
use std::str::FromStr;

pub struct GmailAdapter {
    client: Gmail<HttpsConnector<HttpConnector>>,
    scope: String,
}

impl EmailI for GmailAdapter {
    async fn new() -> Result<Self, EmailError> {
        let exec_path = get_current_exec_path()?;

        let mut json_path = exec_path.clone();
        json_path.push("client_secret.json");

        let secret: yup_oauth2::ApplicationSecret =
            yup_oauth2::read_application_secret(json_path).await?;

        let connector = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()?
            .https_only()
            .enable_http2()
            .build();

        let executor = hyper_util::rt::TokioExecutor::new();

        let mut token_path = exec_path;
        token_path.push("token.json");
        let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
            secret,
            yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
            yup_oauth2::client::CustomHyperClientBuilder::from(
                hyper_util::client::legacy::Client::builder(executor).build(connector),
            ),
        )
        .persist_tokens_to_disk(token_path)
        .build()
        .await?;

        let scopes = "https://www.googleapis.com/auth/gmail.modify";
        auth.token(&[scopes]).await?;

        let client =
            hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
                .build(
                    hyper_rustls::HttpsConnectorBuilder::new()
                        .with_native_roots()?
                        .https_or_http()
                        .enable_http2()
                        .build(),
                );

        Ok(GmailAdapter {
            client: Gmail::new(client, auth),
            scope: String::from(scopes),
        })
    }

    async fn get_unread(&self) -> Result<Vec<Email>, EmailError> {
        const SENDER: &str = "dan@tldrnewsletter.com";

        let mut query = "is:unread from:".to_string();
        query.push_str(SENDER);

        // Get alls unread mails ids
        let (_, list) = self
            .client
            .users()
            .messages_list("me")
            .q(query.as_str())
            .add_scope(self.scope.as_str())
            .doit()
            .await?;

        let mut emails = Vec::new();

        if let Some(messages) = list.messages {
            for m in messages {
                let id = m.id.ok_or(EmailError {
                    message: "Message ID is missing".to_string(),
                })?;

                let (_, msg) = self
                    .client
                    .users()
                    .messages_get("me", &id)
                    .format("full") // Get the entire message
                    .add_scope(self.scope.as_str())
                    .doit()
                    .await?;

                // Extract sender
                let re = Regex::new(r"\s*<.*?>")?;
                let sender: Sender = Self::extract_from_header(&msg, "From".to_string())
                    .as_deref()
                    .map(|data| {
                        let cleaned = re.replace_all(data, "");
                        Sender::from_str(&cleaned).unwrap_or(Sender::ToSort)
                    })
                    .unwrap_or_else(|| {
                        warn!("Fail to parse sender");
                        Sender::ToSort
                    });

                // Extract content
                let content: String = msg
                    .payload
                    .as_ref()
                    .and_then(|payload| {
                        let direct_data = payload.body.as_ref().and_then(|b| b.data.as_ref());

                        if let Some(data) = direct_data {
                            Some(String::from_utf8_lossy(data).into_owned())
                        } else {
                            payload.parts.as_ref()?.iter().find_map(|part| {
                                part.body
                                    .as_ref()
                                    .and_then(|b| b.data.as_ref())
                                    .map(|data| String::from_utf8_lossy(data).into_owned())
                            })
                        }
                    })
                    .unwrap_or_else(|| {
                        warn!("Email {} has no content", id);
                        "No content".to_string()
                    });

                // Date
                let receive_date = DateTime::from_timestamp_millis(msg.internal_date.unwrap_or(0))
                    .ok_or(EmailError {
                        message: "Unable to parse date".to_string(),
                    })?;

                emails.push(Email {
                    id,
                    status: EmailStatus::UNREAD,
                    content,
                    sender,
                    receive_date,
                });
            }
        }
        Ok(emails)
    }

    async fn update_mail_status(
        &self,
        email: &Email,
        status: EmailStatus,
    ) -> Result<(), EmailError> {
        let req = ModifyMessageRequest {
            add_label_ids: None,
            remove_label_ids: Some(vec![status.to_string()]),
        };

        self.client
            .users()
            .messages_modify(req, "me", &*email.id)
            .add_scope(self.scope.as_str())
            .doit()
            .await?;

        Ok(())
    }
}

impl GmailAdapter {
    fn extract_from_header(message: &Message, key: String) -> Option<String> {
        message.payload.clone()?.headers.as_ref().and_then(|h| {
            h.iter()
                .find(|h| h.name == Some(key.clone()))
                .and_then(|h| h.value.clone())
        })
    }
}
