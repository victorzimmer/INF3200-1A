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

// Node represent a node in the cluster
struct Node {
    hostname: String,
    port: u16,
    position: i32,
    range: i32,
}

struct NodeConfig {
    local: Node,
    successor: Option<Node>,
    precessor: Option<Node>,
    finger_table: Vec<Node>,
    storage: Arc<Storage>,
}

// // function that takes in a key (as a string) and returns a int (u64)
// fn hash(key: &str) -> u64 {
//     let mut hasher = Sha256::new(); // create a new Sha256 - 256 byte (32-bytes) hash
//     hasher.update(key); // update the hasher with the key
//     let result = hasher.finalize(); // finalize the hasher and store the result in the variable result
// }

fn shortest_distance_on_circumference(p1: i64, p2: i64) -> i64 {
    let forwards_distance = p2 - p1;
    let backwards_distance = (360 - p2) + p1;

    if (forwards_distance < backwards_distance) {
        return forwards_distance;
    } else {
        return -backwards_distance;
    }
}

// end-point to test if the server is running
#[get("/helloworld")]
fn helloworld(node_config: &State<NodeConfig>) -> String {
    format!("{}:{}", node_config.local.hostname, node_config.local.port)
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
    let hashed_location: i32 = hashed
        .iter()
        .fold(0, |accumulator, x| accumulator + (*x as i32));

    println!("Hashed value: {:?}\n", hashed);
    println!("Hashed location: {:?}\n", hashed_location);
}

// endpoint to store a key-value pair
#[put("/storage/<key>", format = "text", data = "<value>")]
fn put_storage(key: &str, value: &str, node_config: &State<NodeConfig>) -> () {
    // TODO: find out what type it should return. should not be _
    println!("Put storage, key: {}, value: {}", key, value);

    let mut hasher = Sha1::new();

    hasher.update(b"Text");
    let hashed = hasher.finalize();
    let hashed_location: i32 = hashed.iter().fold(0, |i, x| i + (*x as i32));

    println!("Hashed value: {:?}\n", hashed);
    println!("Hashed location: {:?}\n", hashed_location);

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
fn get_precessor(node_config: &State<NodeConfig>) -> String {
    println!("Get precessor");
    // format!(
    //     "{}",
    //     a1_config.node.precessor.as_ref().expect("No precessor")
    // )
    todo!();
}

#[get("/ring/successor")]
fn get_successor(node_config: &State<NodeConfig>) -> String {
    println!("Get successor");
    // format!(
    //     "{}",
    //     a1_config.node.successor.as_ref().expect("No successor")
    // )
    todo!();
}

#[put("/ring/precessor/<new_precessor>")]
fn put_precessor(node_config: &State<NodeConfig>, new_precessor: &str) -> String {
    println!("Put precessor");
    // a1_config.node.precessor = newPrecessor;
    // format!(
    //     "{}",
    //     a1_config.node.precessor.as_ref().expect("No precessor")
    // )
    todo!();
}

#[put("/ring/successor/<new_successor>")]
fn put_successor(node_config: &State<NodeConfig>, new_successor: &str) -> String {
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
    let node_config = NodeConfig {
        local: Node {
            hostname: env::var("A1_HOSTNAME").expect("Hostname not provided!"),
            port: env::var("A1_PORT")
                .expect("Port not provided.")
                .parse()
                .expect("Unable to parse port value."),
            position: 0,
            range: 0,
        },
        successor: None,
        precessor: None,
        finger_table: vec![],
        storage: Arc::new(Storage::new()),
    };

    node_config.storage.store("key", "stored_value");

    println!(
        "Retrieved: {}",
        node_config
            .storage
            .retrieve("key")
            .expect("No value retrieved!")
    );

    rocket::build().manage(node_config).mount(
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
