#[macro_use]
extern crate rocket;

use rocket::response::status::{BadRequest, NoContent};
use rocket::serde::Deserialize;
use rocket::serde::{json::Json, Serialize};
use rocket::State;
use sha1::{Digest, Sha1};
use std::env;
use std::sync::{Arc, RwLock};

mod storage;
use storage::Storage;

const CHORD_RING_SIZE: u32 = 255 ^ 20; // Currently set to maximum sum of hash function output (20 bytes)

// Node represent a node in the cluster
#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
struct Node {
    hostname: String,
    port: u16,
    position: u32,
    range: u32,
}

struct NodeConfig {
    local: Node,
    successor: Option<Node>,
    precessor: Option<Node>,
    finger_table: Vec<Node>,
    storage: Storage,
}

// // function that takes in a key (as a string) and returns a int (u64)
// fn hash(key: &str) -> u64 {
//     let mut hasher = Sha256::new(); // create a new Sha256 - 256 byte (32-bytes) hash
//     hasher.update(key); // update the hasher with the key
//     let result = hasher.finalize(); // finalize the hasher and store the result in the variable result
// }

// fn shortest_distance_on_circumference(p1: u32, p2: u32) -> i32 {
//     let forwards_distance = p2 - p1;
//     let backwards_distance = (CHORD_RING_SIZE - p2) + p1;

//     if (forwards_distance < backwards_distance) {
//         return forwards_distance.into();
//     } else {
//         return -backwards_distance;
//     }
// }

// end-point to test if the server is running
#[get("/helloworld")]
fn helloworld(node_config: &State<Arc<RwLock<NodeConfig>>>) -> String {
    format!(
        "{}:{}",
        node_config.read().unwrap().local.hostname,
        node_config.read().unwrap().local.port
    )
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
fn put_storage(key: &str, value: &str, node_config: &State<Arc<RwLock<NodeConfig>>>) -> () {
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
fn get_network(node_config: &State<Arc<RwLock<NodeConfig>>>) -> Json<Vec<String>> {
    let mut known_nodes: Vec<String> = Vec::new();

    match node_config.read().unwrap().precessor.clone() {
        None => {}
        Some(node) => {
            let mut hostname_port = String::new();
            hostname_port.push_str(&node.hostname);
            hostname_port.push_str(":");
            hostname_port.push_str(&node.port.to_string());
            known_nodes.push(hostname_port);
        }
    }

    match node_config.read().unwrap().successor.clone() {
        None => {}
        Some(node) => {
            let mut hostname_port = String::new();
            hostname_port.push_str(&node.hostname);
            hostname_port.push_str(":");
            hostname_port.push_str(&node.port.to_string());
            known_nodes.push(hostname_port);
        }
    }

    return Json(known_nodes);
    // println!("Get network");
    // todo!();
}

#[get("/ring/precessor")]
fn get_precessor(node_config: &State<Arc<RwLock<NodeConfig>>>) -> Result<Json<Node>, NoContent> {
    match node_config.read().unwrap().precessor.clone() {
        None => Err(NoContent),
        Some(precessor) => return Ok(Json(precessor)),
    }
}

#[get("/ring/successor")]
fn get_successor(node_config: &State<Arc<RwLock<NodeConfig>>>) -> Result<Json<Node>, NoContent> {
    match node_config.read().unwrap().successor.clone() {
        None => Err(NoContent),
        Some(successor) => return Ok(Json(successor)),
    }
}

#[put("/ring/precessor", data = "<new_precessor>")]
fn put_precessor(node_config: &State<Arc<RwLock<NodeConfig>>>, new_precessor: Json<Node>) -> () {
    node_config.write().unwrap().precessor = Some(new_precessor.0);
}

#[put("/ring/successor", data = "<new_successor>")]
fn put_successor(node_config: &State<Arc<RwLock<NodeConfig>>>, new_successor: Json<Node>) -> () {
    node_config.write().unwrap().successor = Some(new_successor.0);
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
    let node_config = Arc::new(RwLock::new(NodeConfig {
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
        storage: Storage::new(),
    }));

    node_config
        .write()
        .unwrap()
        .storage
        .store("key", "stored_value");

    println!(
        "Retrieved: {}",
        node_config
            .read()
            .unwrap()
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
