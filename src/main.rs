//! src/main.rs
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use tracing_log::LogTracer;





#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Redirect all `log`'s events to our new subscriber
    LogTracer::init().expect("Failed to set logger");
  
    // We are falling back to printing all logs at info-level or above
    // if the RUST_LOG environment variable has not been set.
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    let formatting_layer = BunyanFormattingLayer::new(
        "zero2prod".into(),
        // Output the formatted spans to STDOUt
        std::io::stdout
    );

    // The `with`method is provided by `SubscriberExt`, an extension
    // trait for `Subscriber`exposed by `tracing_subscriber
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    // `set_global_default`can be used by applications to specify
    // what subscriber should be used to process spans.
    set_global_default(subscriber).expect("Failed to set subscriber");

    
    // Panic if we cannot read configuration
    let configuration = get_configuration().expect("Failed to read configurtion.");
    
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await 
        .expect("Failed to connect to Postgres.");

    // We have removed the hard-coded `8000` - it's now coming from our setting
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    println!("Server is now listening on http://127.0.0.1:{}", configuration.application_port);
    
    run(listener, connection_pool)?.await 
}