//! src/main.rs
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Panic if we cannot read configuration
    let configuration = get_configuration().expect("Failed to read configurtion.");
    
    let connection = PgPool::connect(&configuration.database.connection_string())
        .await 
        .expect("Failed to connect to Postgres.");

    // We have removed the hard-coded `8000` - it's now coming from our setting
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    println!("Server is now listening on http://127.0.0.1:{}", configuration.application_port);
    
    run(listener, connection)?.await 
}