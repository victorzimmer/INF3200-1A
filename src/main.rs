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
        config.port,
    )
}

#[launch]
fn rocket() -> Rocket<Build> {
    let address = "127.0.0.1".to_string(); // Du kan endre til en annen adresse hvis ønskelig
    let port = 50900; // Definer porten her

    let config = rocket::Config {
        address: address.clone(),
        port,
        ..rocket::Config::default()
    };

    let server_config = ServerConfig {
        address,
        port,
    };

    rocket::custom(config)
        .manage(server_config)
        .mount("/", routes![hello])
}
