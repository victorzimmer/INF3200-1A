#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use rocket::State;
use std::collections::HashMap;
use std::env;
use std::env;
use std::sync::{Arc, Mutex};

// use sha2::{Digest, Sha256};

// Node represent a node in the Chord
struct Node {
    id: String,
    address: String,
    successor: Option<String>,
    precessor: Option<String>,
    finger_table: Vec<String>,
    data: Arc<Mutex<HashMap<String, String>>>, //Key-value pairs stored on this node
}

struct A1Config {
    hostname: String,
    port: String,
    node: Node, // Mutex to ensure that only one thread can modify it at a time, Arc to allow multiple threads to have access to this protected Node
}

// function that takes in a key (as a string) and returns a int (u64)
fn hash(key: &str) -> u64 {
    let mut hasher = Sha256::new(); // create a new Sha256 - 256 byte (32-bytes) hash
    hasher.update(key); // update the hasher with the key
    let result = hasher.finalize(); // finalize the hasher and store the result in the variable result
}

// end-point to test if the server is running
#[get("/helloworld")]
fn helloworld(a1_config: &State<A1Config>) -> String {
    format!("{}:{}", a1_config.hostname, a1_config.port);
}

// endpoint to retrive a value for a given
#[get("/storage/<key>")]
fn get_storage(key: &str) -> _ {
    println!("Get storage, key: {}", key);
    let node = a1_config.node.lock().unwrap();
    node.data.lock().unwrap().get(key).clonned();
    //  TODO: check if it is responsible for the given key, if not forward the request to the correct node
}

// endpoint to store a key-value pair
#[put("/storage/<key>", format = "json", data = "<value>")]
fn put_storage(key: &str, value: Json<String>, a1_config: &State<A1Config>) -> _ {
    // TODO: find out what type it should return. should not be _
    println!("Put storage, key: {}", key);
    let node = a1_config.node.lock().unwrap();
    node.data
        .lock()
        .unwrap()
        .insert(key.to_string(), value.into_inner());
    //  TODO: check if it is responsible for the given key, if not forward the request to the correct node
}

// Endpoint to get information about the network
#[get("/network")]
fn get_network() -> _ {
    println!("Get network");
}

#[get("/ring/precessor")]
fn get_precessor(a1_config: &State<A1Config>) -> Json<Vec<String>> {
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
fn get_finger_table() -> _ {
    println!("Get finger table");
}

#[get("/ring/calculate_fingertable")]
fn calculate_finger_table() -> _ {
    println!("Calculate finger table");
}

#[launch]
fn rocket() -> _ {
    let node = Node {
        id: env::var("ID").expect("id not provided"),
        address: env::var("address").expect("address not provided"),
        successor: _,
        precessor: _,
        finger_table: vec![],
        data: vec![],
    };

    let a1_config = A1Config {
        hostname: env::var("A1_HOSTNAME").expect("Hostname not provided!"),
        port: env::var("A1_PORT").expect("Port not provided!"),
        node: node,
    };

    rocket::build().manage(a1_config).mount(
        "/",
        routes![
            helloworld,
            get_storage,
            put_storage,
            get_network,
            get_precessor,
            get_successor,
            put_precessor,
            put_successor,
            get_finger_table,
            calculate_finger_table
        ],
    )
}
