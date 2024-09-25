#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::response::status::{self, BadRequest, Conflict, Created, Custom, NoContent};
use rocket::serde::Deserialize;
use rocket::serde::{json::Json, Serialize};
use rocket::State;
use sha1::{Digest, Sha1};
use std::env;
use std::sync::{Arc, RwLock};

mod storage;
use storage::Storage;

const RING_SIZE: u16 = u16::MAX; // Maximum size of the ring, and thereby maximum number of nodes supported

// Node represent a node in the cluster
#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
struct Node {
    hostname: String,
    port: u16,
    position: u16,
    range: u16,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
struct Network {
    network_id: String,
}

struct NodeConfig {
    network: Option<Network>,
    connected: bool,
    local: Node,
    successor: Option<Node>,
    precessor: Option<Node>,
    finger_table: Vec<Node>,
    storage: Storage,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
struct JoinNetworkInformation {
    network: Network,
    position: u16,
    range: u16,
    successor: Node,
    precessor: Node,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
struct SuppliedNetworkInformation {
    network_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
struct SuppliedNode {
    hostname: String,
    port: u16,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
struct LongestRangeRequest {
    started_by: Node,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
struct LongestRangeResponse {
    longest_range: Node,
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
fn get_finger_table(node_config: &State<Arc<RwLock<NodeConfig>>>) -> Json<Vec<Node>> {
    return Json(node_config.read().unwrap().finger_table.clone());
}

#[get("/ring/calculate_fingertable")]
fn calculate_finger_table() -> () {
    println!("Calculate finger table");
    todo!();
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
}

#[put("/network/initialize", data = "<network_information>")]
fn put_network_initialize(
    node_config: &State<Arc<RwLock<NodeConfig>>>,
    network_information: Json<SuppliedNetworkInformation>,
) -> Result<String, Conflict<&str>> {
    let mut config = node_config.write().expect("RWLock is poisoned");

    if config.connected {
        return Err(status::Conflict("Node is already connected to network"));
    }

    config.connected = true;
    config.network = Some(Network {
        network_id: network_information.network_id.clone(),
    });

    config.local.position = 0;
    config.local.range = RING_SIZE;
    config.precessor = Some(config.local.clone());
    config.successor = Some(config.local.clone());

    return Ok(format!(
        "Initialized network with network_id: {}",
        config
            .network
            .clone()
            .map_or(String::from("default"), |network| network.network_id)
    ));
}

#[get("/network/longest_range")]
fn get_network_longest_range(
    node_config: &State<Arc<RwLock<NodeConfig>>>,
) -> Result<Json<LongestRangeResponse>, Custom<String>> {
    let config = node_config.read().expect("RWLock is poisoned");
    let longest_range_request = LongestRangeRequest {
        started_by: config.local.clone(),
    };

    if !config.connected {
        let error_message = String::from(
            "Node is not in a network and therefore can't provide information on longest range.",
        );
        println!("{}", &error_message);
        return Err(status::Custom(Status::FailedDependency, error_message));
    }

    let successor = config.successor.as_ref().expect("No successor");

    let longest_range_uri = format!(
        "http://{}:{}/network/longest_range",
        successor.hostname, successor.port
    );
    let upstream_response = match minreq::post(longest_range_uri)
        .with_json(&longest_range_request)
        .expect("Unable to serialize request")
        .send()
    {
        Err(_err) => {
            let error_message = format!(
                "Could not connect to successor node: {}:{}.",
                successor.hostname, successor.port
            );
            println!("{}", &error_message);
            return Err(status::Custom(Status::FailedDependency, error_message));
        }
        Ok(upstream_response) => upstream_response,
    };

    if upstream_response.status_code != 200 {
        let error_message = format!(
            "Successor node did not provide longest range. Node responded: [{} - {}] {}",
            upstream_response.status_code,
            upstream_response.reason_phrase,
            upstream_response.as_str().unwrap_or("Unparseable content")
        );
        println!("{}", &error_message);
        return Err(status::Custom(Status::FailedDependency, error_message));
    }

    let longest_range_upstream = match upstream_response.json::<LongestRangeResponse>() {
        Err(_err) => {
            let error_message =
                String::from("Unable to parse received longest range information from JSON.");
            println!("{}", &error_message);
            return Err(status::Custom(Status::FailedDependency, error_message));
        }
        Ok(longest_range_upstream) => longest_range_upstream,
    };

    return Ok(Json(longest_range_upstream));
}

#[post("/network/longest_range", data = "<longest_range_request>")]
fn post_network_longest_range(
    node_config: &State<Arc<RwLock<NodeConfig>>>,
    longest_range_request: Json<LongestRangeRequest>,
) -> Result<Json<LongestRangeResponse>, Custom<String>> {
    let config = node_config.read().expect("RWLock is poisoned");

    if !config.connected {
        let error_message = String::from(
            "Node is not in a network and therefore can't provide information on longest range.",
        );
        println!("{}", &error_message);
        return Err(status::Custom(Status::FailedDependency, error_message));
    }

    if longest_range_request.0.started_by.hostname == config.local.hostname
        && longest_range_request.0.started_by.port == config.local.port
    {
        let longest_range_response = LongestRangeResponse {
            longest_range: config.local.clone(),
        };
        return Ok(Json(longest_range_response));
    } else {
        let successor = config.successor.as_ref().expect("No successor");

        let longest_range_uri = format!(
            "http://{}:{}/network/longest_range",
            successor.hostname, successor.port
        );
        let upstream_response = match minreq::post(longest_range_uri)
            .with_json(&longest_range_request.0)
            .expect("Unable to serialize request")
            .send()
        {
            Err(_err) => {
                let error_message = String::from("Could not connect to successor node.");
                println!("{}", &error_message);
                return Err(status::Custom(Status::FailedDependency, error_message));
            }
            Ok(upstream_response) => upstream_response,
        };

        if upstream_response.status_code != 200 {
            let error_message = format!(
                "Successor node did not provide longest range. Node responded: [{} - {}] {}",
                upstream_response.status_code,
                upstream_response.reason_phrase,
                upstream_response.as_str().unwrap_or("Unparseable content")
            );
            println!("{}", &error_message);
            return Err(status::Custom(Status::FailedDependency, error_message));
        }

        let longest_range_upstream = match upstream_response.json::<LongestRangeResponse>() {
            Err(_err) => {
                let error_message =
                    String::from("Unable to parse received longest range information from JSON.");
                println!("{}", &error_message);
                return Err(status::Custom(Status::FailedDependency, error_message));
            }
            Ok(longest_range_upstream) => longest_range_upstream,
        };

        if longest_range_upstream.longest_range.range >= config.local.range {
            return Ok(Json(longest_range_upstream));
        } else {
            let longest_range_response = LongestRangeResponse {
                longest_range: config.local.clone(),
            };
            return Ok(Json(longest_range_response));
        }
    }
}

#[get("/network/request_join_network_information")]
fn get_network_request_join(
    node_config: &State<Arc<RwLock<NodeConfig>>>,
) -> Result<Json<JoinNetworkInformation>, Custom<&str>> {
    if !node_config.read().expect("RWLock is poisoned").connected {
        println!("Node is not in a network and therefore can't provide information to join.");
        return Err(status::Custom(
            Status::FailedDependency,
            "Node is not in a network and therefore can't provide information to join.",
        ));
    }

    // let join_network_information = JoinNetworkInformation {
    //     network: node_config
    //         .read()
    //         .expect("RWLock is poisoned")
    //         .network
    //         .expect("Node was connected, but had no network")
    //         .clone(),
    //     position: 0,
    //     range: 0,
    //     successor: None,
    //     precessor: None,
    // };

    todo!();
}

#[put("/network/join", data = "<existing_node>")]
fn put_network_join(
    node_config: &State<Arc<RwLock<NodeConfig>>>,
    existing_node: Json<SuppliedNode>,
) -> Result<String, Custom<String>> {
    let join_uri = format!(
        "http://{}:{}/network/request_join_network_information",
        existing_node.0.hostname, existing_node.0.port
    );

    let join_response = match minreq::get(join_uri).send() {
        Err(_err) => {
            let error_message = String::from("Could not connect to existing node.");
            println!("{}", &error_message);
            return Err(status::Custom(Status::FailedDependency, error_message));
        }
        Ok(join_reponse) => join_reponse,
    };

    if join_response.status_code != 200 {
        let error_message = format!(
            "Existing node denied join request. Node responded: [{} - {}] {}",
            join_response.status_code,
            join_response.reason_phrase,
            join_response.as_str().unwrap_or("Unparseable content")
        );
        println!("{}", &error_message);
        return Err(status::Custom(Status::FailedDependency, error_message));
    }

    let received_network_information = match join_response.json::<JoinNetworkInformation>() {
        Err(_err) => {
            let error_message =
                String::from("Unable to parse received network information from JSON.");
            println!("{}", &error_message);
            return Err(status::Custom(Status::FailedDependency, error_message));
        }
        Ok(received_network_information) => received_network_information,
    };

    let mut config = node_config.write().expect("RWLock is poisoned");

    config.connected = true;
    config.network = Some(Network {
        network_id: received_network_information.network.network_id,
    });
    config.local.position = received_network_information.position;
    config.local.range = received_network_information.range;
    config.successor = Some(received_network_information.successor);
    config.precessor = Some(received_network_information.precessor);

    return Ok(format!(
        "Joined network with ID: {}",
        config
            .network
            .clone()
            .map_or(String::from("Unknown"), |network| network
                .network_id
                .clone())
    ));
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
        network: None,
        connected: false,
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
            calculate_finger_table,
            get_network_request_join,
            get_network_longest_range,
            post_network_longest_range,
            put_network_initialize,
            put_network_join
        ],
    )
}
