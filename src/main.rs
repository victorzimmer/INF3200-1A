#[macro_use]
extern crate rocket;

use hex_literal::hex;
// use rocket::serde:
use rocket::State;
use sha1::{Digest, Sha1};
use std::env;
use std::sync::Arc;

mod storage;
use storage::Storage;

// Node represent a node in the Chord
struct Node {
    id: String,
    address: String,
    successor: Option<String>,
    precessor: Option<String>,
    finger_table: Vec<String>,
    storage: Arc<Storage>,
}

struct A1Config {
    hostname: String,
    port: String,
    node: Node, // Mutex to ensure that only one thread can modify it at a time, Arc to allow multiple threads to have access to this protected Node
}

// // function that takes in a key (as a string) and returns a int (u64)
// fn hash(key: &str) -> u64 {
//     let mut hasher = Sha256::new(); // create a new Sha256 - 256 byte (32-bytes) hash
//     hasher.update(key); // update the hasher with the key
//     let result = hasher.finalize(); // finalize the hasher and store the result in the variable result
// }

fn shortest_distance_on_circumference(p1: i64, p2: i64) -> i64 {
    let forwardsDistance = p2 - p1;
    let backwardsDistance = (360 - p2) + p1;

    if (forwardsDistance < backwardsDistance) {
        return forwardsDistance;
    } else {
        return -backwardsDistance;
    }
}

// end-point to test if the server is running
#[get("/helloworld")]
fn helloworld(a1_config: &State<A1Config>) -> String {
    format!("{}:{}", a1_config.hostname, a1_config.port)
}

// endpoint to retrive a value for a given
#[get("/storage/<key>")]
fn get_storage(key: &str) -> () {
    println!("Get storage, key: {}", key);
    // let node = a1_config.node.lock().unwrap();
    // node.data.lock().unwrap().get(key).clonned();
    //  TODO: check if it is responsible for the given key, if not forward the request to the correct node
    let mut hasher = Sha1::new();

    hasher.update(b"Text");
    let hashed = hasher.finalize();

    println!("Hashed value: {:?}\n", hashed);
}

// endpoint to store a key-value pair
#[put("/storage/<key>", format = "json", data = "<value>")]
fn put_storage(key: &str, value: &str, a1_config: &State<A1Config>) -> () {
    // TODO: find out what type it should return. should not be _
    println!("Put storage, key: {}", key);

    let mut hasher = Sha1::new();

    hasher.update(b"Text");
    let hashed = hasher.finalize();

    println!("Hashed value: {:?}\n", hashed);

    // a1_config.node.data.insert(key.to_string(), value);
    // let node = a1_config.node.lock().unwrap();
    // node.data
    //     .lock()
    //     .unwrap()
    //     .insert(key.to_string(), value.into_inner());
    //  TODO: check if it is responsible for the given key, if not forward the request to the correct node
}

// Endpoint to get information about the network
#[get("/network")]
fn get_network() -> () {
    println!("Get network");
    todo!();
}

#[get("/ring/precessor")]
fn get_precessor(a1_config: &State<A1Config>) -> String {
    println!("Get precessor");
    // format!(
    //     "{}",
    //     a1_config.node.precessor.as_ref().expect("No precessor")
    // )
    todo!();
}

#[get("/ring/successor")]
fn get_successor(a1_config: &State<A1Config>) -> String {
    println!("Get successor");
    // format!(
    //     "{}",
    //     a1_config.node.successor.as_ref().expect("No successor")
    // )
    todo!();
}

#[put("/ring/precessor/<new_precessor>")]
fn put_precessor(a1_config: &State<A1Config>, new_precessor: &str) -> String {
    println!("Put precessor");
    // a1_config.node.precessor = newPrecessor;
    // format!(
    //     "{}",
    //     a1_config.node.precessor.as_ref().expect("No precessor")
    // )
    todo!();
}

#[put("/ring/successor/<new_successor>")]
fn put_successor(a1_config: &State<A1Config>, new_successor: &str) -> String {
    println!("Put successor");
    // format!(
    //     "{}",
    //     a1_config.node.successor.as_ref().expect("No successor")
    // )
    todo!();
}

#[get("/ring/finger_table")]
fn get_finger_table() -> () {
    println!("Get finger table");
    todo!();
}

#[get("/ring/calculate_fingertable")]
fn calculate_finger_table() -> () {
    println!("Calculate finger table");
    todo!();
}

#[launch]
fn rocket() -> _ {
    let node = Node {
        id: env::var("ID").expect("id not provided"),
        address: env::var("A1_HOSTNAME").expect("address not provided"),
        successor: None,
        precessor: None,
        finger_table: vec![],
        storage: Arc::new(Storage::new()),
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
