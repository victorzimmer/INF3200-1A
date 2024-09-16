#[macro_use]
extern crate rocket;

use rocket::State;
use std::env;

struct Node {
    id: String,
    address: String,
    successor: Option<String>,
    predecessor: Option<String>,
    finger_table: Vec<String>,
    data: Arc<Mutex<HashMap<String, String>>,       // datastruktur som inneholder en HashMap hvor både nøklene og verdiene er String
}

struct A1Config {
    hostname: String,
    port: String,
    node: Arc<Mutex<Node>>,         // Mutex to ensure that only one thread can modify it at a time, Arc to allow multiple threads to have access to this protected Node
}

fn hash(key: &str) -> String {
    // hash function
}

#[get("/helloworld")]
fn helloworld(a1_config: &State<A1Config>) -> String {
    format!("{}:{}", a1_config.hostname, a1_config.port)
}

#[get("/storage/<key>")]
fn get_storage(key: &str)-> _ {
    println!("Get storage, key: {}", key);
}

#[put("/storage/<key>", format="json", data = "<data>")]
fn put_storage(key: &str)-> _ {
    println!("Put storage, key: {}", key);
}

#[get("/network")]
fn get_network()-> _ {
    println!("Get network");
}

#[get("/ring/precessor")]
fn get_precessor()-> _ {
    println!("Get precessor");
}

#[get("/ring/successor")]
fn get_successor()-> _ {
    println!("Get successor");
}

#[put("/ring/precessor")]
fn put_precessor()-> _ {
    println!("Get precessor");
}

#[put("/ring/successor")]
fn put_successor()-> _ {
    println!("Get successor");
}

#[get("/ring/finger_table")]
fn get_finger_table()-> _ {
    println!("Get finger table");
}

#[get("/ring/calculate_fingertable")]
fn calculate_finger_table()-> _ {
    println!("Calculate finger table");
}


#[launch]
fn rocket() -> _ {
    let a1_config = A1Config {
        hostname: env::var("A1_HOSTNAME").expect("Hostname not provided!"),
        port: env::var("A1_PORT").expect("Port not provided!"),
    };

    rocket::build()
        .manage(a1_config)
        .mount("/", routes![helloworld, get_storage, put_storage, get_network, 
                            get_precessor, get_successor, put_precessor, 
                            put_successor, get_finger_table, calculate_finger_table])
}
