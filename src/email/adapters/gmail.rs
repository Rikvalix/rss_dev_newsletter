extern crate google_gmail1 as gmail1;

use crate::email::ports::email::EmailI;
use crate::model::{Email, EmailStatus};
use chrono::DateTime;
use gmail1::{hyper_rustls, hyper_util, yup_oauth2, Gmail};
use google_gmail1::api::ModifyMessageRequest;
use google_gmail1::hyper_rustls::HttpsConnector;
use google_gmail1::hyper_util::client::legacy::connect::HttpConnector;

pub struct GmailAdapter {
    client: Gmail<HttpsConnector<HttpConnector>>,
    scope: String,
}

impl EmailI for GmailAdapter {
    async fn get_unread(&self) -> Vec<Email> {
        // Get alls unread mails ids
        let (_, list) = self
            .client
            .users()
            .messages_list("me")
            .q("is:unread")
            .add_scope(self.scope.as_str())
            .doit()
            .await
            .expect("Error during unread mails listing");

        let mut emails = Vec::new();

        if let Some(messages) = list.messages {
            for m in messages {
                let id = m.id.unwrap();

                let (_, msg) = self
                    .client
                    .users()
                    .messages_get("me", &id)
                    .add_scope(self.scope.as_str())
                    .doit()
                    .await
                    .unwrap();

                // Extract subject
                let subject = msg
                    .payload
                    .as_ref()
                    .and_then(|p| p.headers.as_ref())
                    .and_then(|headers| {
                        headers
                            .iter()
                            .find(|h| h.name == Some("Subject".to_string()))
                            .and_then(|h| h.value.clone())
                    })
                    .unwrap_or_else(|| "No subject".to_string());

                // Date
                let receive_date = DateTime::from_timestamp_millis(msg.internal_date.unwrap_or(0))
                    .expect("Could not parse receive date");

                emails.push(Email {
                    id,
                    status: EmailStatus::UNREAD,
                    content: msg.snippet.unwrap_or_default(),
                    subject,
                    receive_date,
                });
            }
        }
        emails
    }

    async fn update_mail_status(&self, email: &Email, status: EmailStatus) {
        let req = ModifyMessageRequest {
            add_label_ids: None,
            remove_label_ids: Some(vec![status.to_string()]),
        };

        let result = self
            .client
            .users()
            .messages_modify(req, "me", &*email.id)
            .add_scope(self.scope.as_str())
            .doit()
            .await
            .expect("Update status failed for mail");

        println!("Résultat: {}", result.0.status());
    }
}

impl GmailAdapter {
    pub async fn new() -> Self {
        let secret: yup_oauth2::ApplicationSecret =
            yup_oauth2::read_application_secret("client_secret.json")
                .await
                .expect("Fail to read application secret file: client_secret.json");

        let connector = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .unwrap()
            .https_only()
            .enable_http2()
            .build();

        let executor = hyper_util::rt::TokioExecutor::new();
        let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
            secret,
            yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
            yup_oauth2::client::CustomHyperClientBuilder::from(
                hyper_util::client::legacy::Client::builder(executor).build(connector),
            ),
        )
        .persist_tokens_to_disk("token.json")
        .build()
        .await
        .unwrap();

        let scopes = "https://www.googleapis.com/auth/gmail.modify";
        auth.token(&[scopes])
            .await
            .expect("Échec de l'obtention initiale du token");

        let client =
            hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
                .build(
                    hyper_rustls::HttpsConnectorBuilder::new()
                        .with_native_roots()
                        .unwrap()
                        .https_or_http()
                        .enable_http2()
                        .build(),
                );

        GmailAdapter {
            client: Gmail::new(client, auth),
            scope: String::from(scopes),
        }
    }
}
