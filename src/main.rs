#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket, State};

struct ServerConfig {
    address: String, 
    port: u16,
}

#[get("/helloworld")]
fn hello(config: &State<ServerConfig>) -> String {
    format!(
        "Hello, world! The server is running on address: {}, and port: {}", 
        config.address,
        config.port
    )
}

#[launch]
fn rocket() -> Rocket<Build> {
    let config = rocket::Config::figment().extract::<rocket::Config>().unwrap();
    let server_config = ServerConfig { 
        address: config.address.to_string(), 
        port: config.port };

    rocket::build()
        .manage(server_config)
        .mount("/", routes![hello])
}
