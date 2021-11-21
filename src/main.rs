//! src/main.rs
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    
    // Panic if we cannot read configuration
    let configuration = get_configuration().expect("Failed to read configurtion.");
    
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db()); 


    // We have removed the hard-coded `8000` - it's now coming from our setting
    let address = format!("{}:{}", 
        configuration.application.host, configuration.application.port);
    let listener = TcpListener::bind(address)?;

    println!("Server is now listening on {}:{}", configuration.application.host, configuration.application.port);
    
    run(listener, connection_pool)?.await 
}