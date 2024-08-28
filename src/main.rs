#[macro_use]
extern crate rocket;

use rocket::State;
use std::env;

struct A1Config {
    hostname: String,
    port: String,
}

#[get("/helloworld")]
fn helloworld(a1_config: &State<A1Config>) -> String {
    format!("{}:{}", a1_config.hostname, a1_config.port)
}

#[launch]
fn rocket() -> _ {
    let a1_config = A1Config {
        hostname: env::var("A1_HOSTNAME").expect("Hostname not provided!"),
        port: env::var("A1_PORT").expect("Port not provided!"),
    };

    rocket::build()
        .manage(a1_config)
        .mount("/", routes![helloworld])
}
