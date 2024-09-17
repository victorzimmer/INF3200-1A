#[macro_use]
extern crate rocket;

use rocket::State;
use std::env;
use std::sync::{Arc, Mutex};
// use sha2::{Digest, Sha256};


struct Node {
    id: String,
    address: String,
    successor: Option<String>,
    precessor: Option<String>,
    finger_table: Vec<String>,
    data: Arc<Mutex<HashMap<String, String>>      // datastruktur som inneholder en HashMap hvor både nøklene og verdiene er String
}

struct A1Config {
    hostname: String,
    port: String,
    node: Node    // Mutex to ensure that only one thread can modify it at a time, Arc to allow multiple threads to have access to this protected Node
}

// function that takes in a key (as a string) and returns a int (u64)
fn hash(key: &str) -> _ {
    let mut hasher = Sha256::new();          // create a new Sha256 - 256 byte (32-bytes) hash
    hasher.update(key);                      // update the hasher with the key
    let result = hasher.finalize();          // finalize the hasher and store the result in the variable result
}

#[get("/helloworld")]
fn helloworld(a1_config: &State<A1Config>) -> String {
    format!("{}:{}", a1_config.hostname, a1_config.port);
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
fn get_precessor(a1_config: &State<A1Config>) -> String {
    println!("Get precessor");
    format!("{}", a1_config.node.precessor);
}

#[get("/ring/successor")]
fn get_successor(a1_config: &State<A1Config>) -> String {
    println!("Get successor");
    format!("{}", a1_config.node.successor);
}

#[put("/ring/precessor")]
fn put_precessor(a1_config: &State<A1Config>, newPrecessor: &str) -> String {
    println!("Put precessor");
    a1_config.node.precessor = newPrecessor;
}

#[put("/ring/successor")]
fn put_successor(a1_config: &State<A1Config>, newSuccessor: &str) -> String {
    println!("Put successor");
    a1_config.node.successor = newSuccessor;
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
    let node = Node {
        id: env::var("ID").expect("id not provided"),
        address: env::var("address").expect("address not provided"),
        successor: _,
        precessor: _,
        finger_table: vec!(),
        data: vec!(),
    };

    let a1_config = A1Config {
        hostname: env::var("A1_HOSTNAME").expect("Hostname not provided!"),
        port: env::var("A1_PORT").expect("Port not provided!"),
        node: node,
    };



    rocket::build()
        .manage(a1_config)
        .mount("/", routes![helloworld, get_storage, put_storage, get_network,
                            get_precessor, get_successor, put_precessor,
                            put_successor, get_finger_table, calculate_finger_table])
}
