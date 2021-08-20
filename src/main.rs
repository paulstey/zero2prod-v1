//! src/main.rs
use std::net::TcpListener;
use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Panic if we cannot read configuration
    let configuration = get_configuration().expect("Failed to read configurtion.");
    
    // We have removed the hard-coded `8000` - it's now coming from our setting
    let address = format!("127.0.0.1:{}", configuration.application_port);

    let listener = TcpListener::bind(address)
        .expect("Failed to bind random port.");

    println!("Server is now listening on http://127.0.0.1:{}", configuration.application_port);
    
    run(listener)?.await 
}