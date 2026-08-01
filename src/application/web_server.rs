use crate::domain::application::model::ApplicationConfiguration;
use crate::infrastructure::web::router::get_router;
use log::info;

pub async fn run_web_server(application_configuration: &ApplicationConfiguration) {

    let router = get_router(&application_configuration);
    let port: &String = &application_configuration.config.server.port;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    info!("Web server is up successfully, listen on 0.0.0.0:{}",port);
    axum::serve(listener, router).await.unwrap();

}
