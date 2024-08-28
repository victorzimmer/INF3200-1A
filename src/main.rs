#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket, State};
use std::env;

struct ServerConfig {
    address: String, 
    hostname: String,
    port: u16,
}

#[get("/helloworld")]
fn hello(config: &State<ServerConfig>) -> String {
    format!(
        "Hello, world! The server is running on address: {}, hostname: {}, and port: {}", 
        config.address,
        config.hostname,
        config.port, 
    )
}

#[launch]
fn rocket() -> Rocket<Build> {
    let hostname = env::var("HOST").expect("hostname not set");
    let config = rocket::Config::figment().extract::<rocket::Config>().unwrap();
    let server_config = ServerConfig { 
        address: config.address.to_string(), 
        port: config.port, 
        hostname: hostname,
    };

    rocket::build()
        .manage(server_config)
        .mount("/", routes![hello])
}
