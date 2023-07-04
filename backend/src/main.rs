#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let settings = rust_auth_backend::settings::get_settings().expect("Failed to read settings.");

    let subscriber = rust_auth_backend::telemetry::get_subscriber(settings.clone().debug);
    rust_auth_backend::telemetry::init_subscriber(subscriber);

    let application = rust_auth_backend::startup::Application::build(settings).await?;

    tracing::event!(target: "backend", tracing::Level::INFO, "Listening on http://127.0.0.1:{}/", application.port());

    application.run_until_stopped().await?;
    Ok(())
}