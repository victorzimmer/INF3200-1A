#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket, State};
use std::env;
use gethostname::gethostname;

struct ServerConfig {
    address: String, 
    hostname: String,
    port: u16,
}

#[get("/helloworld")]
fn hello(config: &State<ServerConfig>) -> String {
    format!(
        "{}:{}", 
        config.hostname,
        config.port, 
    )
}

#[launch]
fn rocket() -> Rocket<Build> {
    let hostname = gethostname().to_string_lossy().into_owned();
    let config = rocket::Config::figment().extract::<rocket::Config>().unwrap();
    let server_config = ServerConfig { 
        address: config.address.to_string(), 
        port: config.port, 
        hostname: hostname,
    };

    println!("Server started at {}:{}", server_config.hostname, server_config.port);
    
    rocket::build()
        .manage(server_config)
        .mount("/", routes![hello])
}
