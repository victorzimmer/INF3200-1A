#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::response::status::{self, BadRequest, Conflict, Created, Custom, NoContent};
use rocket::serde::Deserialize;
use rocket::serde::{json::Json, Serialize};
use rocket::{Shutdown, State};
use sha1::{Digest, Sha1};
use std::env;
use std::sync::{Arc, RwLock};

mod storage;
use storage::Storage;

const RING_SIZE: u16 = u16::MAX; // Maximum size of the ring, and thereby maximum number of nodes supported

// Node represent a node in the cluster
#[derive(Serialize, Deserialize, Clone, Debug)]
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
    longest_range: LongestRangeResponse,
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
    holder: Node,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
struct FingerTableInformation {
    size: u16,
}

fn shortest_distance_on_circumference(p1: u16, p2: u16) -> i32 {
    let forwards_distance = i32::from(p2) - i32::from(p1);
    let backwards_distance = (i32::from(RING_SIZE) - i32::from(p2)) + i32::from(p1);

    if forwards_distance < backwards_distance {
        return forwards_distance.into();
    } else {
        return -i32::from(backwards_distance);
    }
}

// end-point to test if the server is running
#[get("/helloworld")]
fn helloworld(node_config: &State<Arc<RwLock<NodeConfig>>>) -> String {
    format!(
        "{}:{}",
        node_config.read().unwrap().local.hostname,
        node_config.read().unwrap().local.port
    )
}

#[get("/shutdown")]
fn shutdown(shutdown: Shutdown) -> String {
    shutdown.notify();
    String::from("Bye!")
}

// endpoint to retrive a value for a given
#[get("/storage/<key>")]
fn get_storage(
    node_config: &State<Arc<RwLock<NodeConfig>>>,
    key: &str,
) -> Result<String, Custom<String>> {
    let config = node_config.read().expect("RWLock is poisoned");

    // We use the hasher to hash the given key
    let mut hasher = Sha1::new();
    hasher.update(key.as_bytes());
    let hashed = hasher.finalize();

    // For our RING_SIZE = 2^16 = 65 536 that means reading the first two bytes of the hash and interpreting them as a u16.
    let hash_slice: [u8; 2] = [hashed[0], hashed[1]];
    let hashed_location: u16 = u16::from_be_bytes(hash_slice);

    // Special case for range wrapping circle
    if RING_SIZE - config.local.position < config.local.range {
        if hashed_location >= config.local.position
            || hashed_location < config.local.range - (RING_SIZE - config.local.position)
        {
            match config.storage.retrieve(key) {
                Some(value) => return Ok(value),
                None => {
                    return Err(status::Custom(
                        Status::NotFound,
                        String::from("Key not found"),
                    ))
                }
            };
        }
    } else {
        if hashed_location >= config.local.position
            && hashed_location < config.local.position + config.local.range
        {
            match config.storage.retrieve(key) {
                Some(value) => return Ok(value),
                None => {
                    return Err(status::Custom(
                        Status::NotFound,
                        String::from("Key not found"),
                    ))
                }
            };
        }
    }

    // Early returns for cases where key is under over jurisdiction, so if we get here we need to forward the request
    println!("Forwarding request!");
    let mut forward_node_distance =
        shortest_distance_on_circumference(config.local.position, hashed_location).abs();

    let mut forward_node =
        if shortest_distance_on_circumference(config.local.position, hashed_location) < 0 {
            config
                .precessor
                .as_ref()
                .expect("Could not forward, node has no successor")
        } else {
            config
                .successor
                .as_ref()
                .expect("Could not forward, node has no successor")
        };

    // See if the key is closer to any node in the finger table
    if config.finger_table.len() > 0 {
        for node in config.finger_table.iter() {
            if shortest_distance_on_circumference(node.position, hashed_location).abs()
                < forward_node_distance
            {
                forward_node = node;
                forward_node_distance =
                    shortest_distance_on_circumference(node.position, hashed_location).abs();
            }
        }
    }

    let forward_request_uri = format!(
        "http://{}:{}/storage/{}",
        forward_node.hostname, forward_node.port, key
    );

    let forward_request_response = match minreq::get(forward_request_uri).send() {
        Err(_err) => {
            let error_message = String::from("Could not connect to successor to forward request.");
            println!("{}", &error_message);
            return Err(status::Custom(Status::FailedDependency, error_message));
        }
        Ok(response) => response,
    };

    if forward_request_response.status_code == 404 {
        return Err(status::Custom(
            Status::NotFound,
            String::from("Key not found"),
        ));
    } else if forward_request_response.status_code != 200 {
        let error_message = format!(
            "Forward node denied request. Node responded: [{} - {}] {}",
            forward_request_response.status_code,
            forward_request_response.reason_phrase,
            forward_request_response
                .as_str()
                .unwrap_or("Unparseable content")
        );
        println!("{}", &error_message);
        return Err(status::Custom(Status::FailedDependency, error_message));
    }

    return Ok(String::from(
        forward_request_response.as_str().expect("No body found"),
    ));
}

// endpoint to store a key-value pair
#[put("/storage/<key>", format = "text", data = "<value>")]
fn put_storage(
    node_config: &State<Arc<RwLock<NodeConfig>>>,
    key: &str,
    value: &str,
) -> Result<String, Custom<String>> {
    let config = node_config.read().expect("RWLock is poisoned");

    // We use the hasher to hash the given key
    let mut hasher = Sha1::new();
    hasher.update(key.as_bytes());
    let hashed = hasher.finalize();

    // For our RING_SIZE = 2^16 = 65 536 that means reading the first two bytes of the hash and interpreting them as a u16.
    let hash_slice: [u8; 2] = [hashed[0], hashed[1]];
    let hashed_location: u16 = u16::from_be_bytes(hash_slice);

    // Special case for range wrapping circle
    if RING_SIZE - config.local.position < config.local.range {
        if hashed_location >= config.local.position
            || hashed_location < config.local.range - (RING_SIZE - config.local.position)
        {
            config.storage.store(key, value);
            return Ok(String::from(value));
        }
    } else {
        if hashed_location >= config.local.position
            && hashed_location < config.local.position + config.local.range
        {
            config.storage.store(key, value);
            return Ok(String::from(value));
        }
    }

    // Early returns for cases where key is under over jurisdiction, so if we get here we need to forward the request
    println!("Forwarding request!");

    let mut forward_node_distance =
        shortest_distance_on_circumference(config.local.position, hashed_location).abs();

    let mut forward_node =
        if shortest_distance_on_circumference(config.local.position, hashed_location) < 0 {
            config
                .precessor
                .as_ref()
                .expect("Could not forward, node has no successor")
        } else {
            config
                .successor
                .as_ref()
                .expect("Could not forward, node has no successor")
        };

    // See if the key is closer to any node in the finger table
    if config.finger_table.len() > 0 {
        for node in config.finger_table.iter() {
            if shortest_distance_on_circumference(node.position, hashed_location).abs()
                < forward_node_distance
            {
                forward_node = node;
                forward_node_distance =
                    shortest_distance_on_circumference(node.position, hashed_location).abs();
            }
        }
    }

    let forward_request_uri = format!(
        "http://{}:{}/storage/{}",
        forward_node.hostname, forward_node.port, key
    );

    let forward_request_response = match minreq::put(forward_request_uri)
        .with_body(value)
        .with_header("Content-Type", "text/plain")
        .send()
    {
        Err(_err) => {
            let error_message = String::from("Could not connect to successor to forward request.");
            println!("{}", &error_message);
            return Err(status::Custom(Status::FailedDependency, error_message));
        }
        Ok(response) => response,
    };

    if forward_request_response.status_code != 200 {
        let error_message = format!(
            "Forward node denied request. Node responded: [{} - {}] {}",
            forward_request_response.status_code,
            forward_request_response.reason_phrase,
            forward_request_response
                .as_str()
                .unwrap_or("Unparseable content")
        );
        println!("{}", &error_message);
        return Err(status::Custom(Status::FailedDependency, error_message));
    }

    return Ok(String::from(
        forward_request_response.as_str().expect("No body found"),
    ));
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

#[get("/ring/local")]
fn get_local(node_config: &State<Arc<RwLock<NodeConfig>>>) -> Json<Node> {
    let config = node_config.read().expect("RWLock is poisoned");
    return Json(config.local.clone());
}

#[put("/ring/precessor", data = "<new_precessor>")]
fn put_precessor(node_config: &State<Arc<RwLock<NodeConfig>>>, new_precessor: Json<Node>) -> () {
    let mut config = node_config.write().expect("RWLock is poisoned");
    config.precessor = Some(new_precessor.0);
}

#[put("/ring/successor", data = "<new_successor>")]
fn put_successor(node_config: &State<Arc<RwLock<NodeConfig>>>, new_successor: Json<Node>) -> () {
    let mut config = node_config.write().expect("RWLock is poisoned");
    config.successor = Some(new_successor.0.clone());
    config.local.range = new_successor.0.position - config.local.position;
}

#[put("/ring/local", data = "<new_local>")]
fn put_local(node_config: &State<Arc<RwLock<NodeConfig>>>, new_local: Json<Node>) -> () {
    let mut config = node_config.write().expect("RWLock is poisoned");
    config.local = new_local.0;
}

#[get("/ring/finger_table")]
fn get_finger_table(node_config: &State<Arc<RwLock<NodeConfig>>>) -> Json<Vec<Node>> {
    return Json(node_config.read().unwrap().finger_table.clone());
}

#[put("/ring/calculate_finger_table", data = "<finger_table_info>")]
fn calculate_finger_table(
    node_config: &State<Arc<RwLock<NodeConfig>>>,
    finger_table_info: Json<FingerTableInformation>,
) -> Result<String, Custom<String>> {
    let mut config = node_config.write().expect("RWLock is poisoned");
    println!("Calculate finger table");

    config.finger_table.clear();

    // Add local node to finger table, and all other nodes in the network
    let mut complete_node_list = vec![config.local.clone()];
    if !config.connected {
        let error_message = String::from("Node is not connected to a network");
        return Err(status::Custom(Status::FailedDependency, error_message));
    }
    let mut current_node = config.successor.clone().expect("No successor");

    while current_node.hostname != config.local.hostname || current_node.port != config.local.port {
        complete_node_list.push(current_node.clone());

        let get_successor_uri = format!(
            "http://{}:{}/ring/successor",
            current_node.hostname, current_node.port
        );

        let get_successor_response = match minreq::get(get_successor_uri).send() {
            Err(_err) => {
                let error_message = String::from("Could not connect to node to get successor.");
                println!("{}", &error_message);
                return Err(status::Custom(Status::FailedDependency, error_message));
            }
            Ok(response) => response,
        };

        if get_successor_response.status_code != 200 {
            let error_message = format!(
                "Node denied request for successor. Node responded: [{} - {}] {}",
                get_successor_response.status_code,
                get_successor_response.reason_phrase,
                get_successor_response
                    .as_str()
                    .unwrap_or("Unparseable content")
            );
            println!("{}", &error_message);
            return Err(status::Custom(Status::FailedDependency, error_message));
        }

        current_node = match get_successor_response.json::<Node>() {
            Err(_err) => {
                let error_message =
                    String::from("Unable to parse received network information from JSON.");
                println!("{}", &error_message);
                return Err(status::Custom(Status::FailedDependency, error_message));
            }
            Ok(parsed) => parsed,
        };
    }

    if complete_node_list.len() < usize::from(finger_table_info.size) {
        let error_message = String::from("Not enough nodes in network to calculate finger table.");
        println!("{}", &error_message);
        return Err(status::Custom(Status::BadRequest, error_message));
    } else if complete_node_list.len() == usize::from(finger_table_info.size) {
        println!("Creating fully connected finger table, with all nodes in network.");
    }

    println!("Node list length: {}", complete_node_list.len());

    let size = usize::from(finger_table_info.size);
    if size == 0 || size > complete_node_list.len() {
        panic!("Finger table size cannot be zero or greater than the number of nodes.");
    }

    let step = complete_node_list.len() / size;
    for i in 0..size {
        let index = (i * step) % complete_node_list.len();
        println!("Adding node: {}", index);
        config.finger_table.push(complete_node_list[index].clone());
    }

    return Ok(String::from("Finger table calculated"));
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
            holder: config.local.clone(),
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

        if longest_range_upstream.holder.range >= config.local.range {
            return Ok(Json(longest_range_upstream));
        } else {
            let longest_range_response = LongestRangeResponse {
                holder: config.local.clone(),
            };
            return Ok(Json(longest_range_response));
        }
    }
}

#[get("/network/request_join_network_information")]
fn get_network_request_join(
    node_config: &State<Arc<RwLock<NodeConfig>>>,
) -> Result<Json<JoinNetworkInformation>, Custom<String>> {
    let config = node_config.read().expect("RWLock is poisoned");

    if !config.connected {
        let error_message = String::from(
            "Node is not in a network and therefore can't provide information to join.",
        );
        println!("{}", &error_message);
        return Err(status::Custom(Status::FailedDependency, error_message));
    }

    let longest_range: LongestRangeResponse = match get_network_longest_range(node_config) {
        Ok(range) => range.0,
        Err(err) => {
            let error_message = format!("Could not get longest range in network. Error: {}", err.1);
            println!("{}", &error_message);
            return Err(status::Custom(Status::FailedDependency, error_message));
        }
    };

    let join_network_information = JoinNetworkInformation {
        network: config
            .network
            .as_ref()
            .expect("Node was connected, but had no network")
            .clone(),
        longest_range: longest_range,
    };

    return Ok(Json(join_network_information));
}

#[put("/network/join", data = "<existing_node>")]
fn put_network_join(
    node_config: &State<Arc<RwLock<NodeConfig>>>,
    existing_node: Json<SuppliedNode>,
) -> Result<String, Custom<String>> {
    let mut config = node_config.write().expect("RWLock is poisoned");

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

    if received_network_information.longest_range.holder.range < 2 {
        let error_message = String::from("Unable to join as network is already full.");
        println!("{}", &error_message);
        return Err(status::Custom(Status::FailedDependency, error_message));
    }

    // Get successor from holder of longest range
    let get_successor_uri = format!(
        "http://{}:{}/ring/successor",
        received_network_information.longest_range.holder.hostname,
        received_network_information.longest_range.holder.port
    );

    let get_successor_response = match minreq::get(get_successor_uri).send() {
        Err(_err) => {
            let error_message = String::from("Could not connect to node holding longest range.");
            println!("{}", &error_message);
            return Err(status::Custom(Status::FailedDependency, error_message));
        }
        Ok(response) => response,
    };

    if get_successor_response.status_code != 200 {
        let error_message =
            format!(
            "Holder of longest range denied request for successor. Node responded: [{} - {}] {}",
            get_successor_response.status_code,
            get_successor_response.reason_phrase,
            get_successor_response.as_str().unwrap_or("Unparseable content")
        );
        println!("{}", &error_message);
        return Err(status::Custom(Status::FailedDependency, error_message));
    }

    let recieved_successor = match get_successor_response.json::<Node>() {
        Err(_err) => {
            let error_message =
                String::from("Unable to parse received network information from JSON.");
            println!("{}", &error_message);
            return Err(status::Custom(Status::FailedDependency, error_message));
        }
        Ok(parsed) => parsed,
    };

    config.connected = true;
    config.network = Some(received_network_information.network.clone());

    config.local.position = received_network_information.longest_range.holder.position
        + received_network_information.longest_range.holder.range / 2;

    if recieved_successor.position < config.local.position {
        config.local.range = (RING_SIZE - config.local.position) + (recieved_successor.position);
    } else {
        config.local.range = recieved_successor.position - config.local.position;
    }
    println!(
        "Successor position: {}, local position: {}",
        recieved_successor.position, config.local.position
    );

    config.successor = Some(recieved_successor.clone());
    config.precessor = Some(received_network_information.longest_range.holder.clone());

    // Insert local node into ring by setting itself as successor of precessor and itself as precessor of successor
    // Set itself as successor of precessor
    let put_successor_uri = format!(
        "http://{}:{}/ring/successor",
        config
            .precessor
            .as_ref()
            .expect("Precessor was just set, but does not exist")
            .hostname,
        config
            .precessor
            .as_ref()
            .expect("Precessor was just set, but does not exist")
            .port
    );

    let put_successor_response = match minreq::put(put_successor_uri)
        .with_json(&config.local)
        .expect("Could not serialize local node")
        .send()
    {
        Err(_err) => {
            let error_message = String::from("Could not connect to precessor to set successor.");
            println!("{}", &error_message);
            return Err(status::Custom(Status::FailedDependency, error_message));
        }
        Ok(response) => response,
    };

    if put_successor_response.status_code != 200 {
        let error_message = format!(
            "Precessor denied setting successor. Node responded: [{} - {}] {}",
            put_successor_response.status_code,
            put_successor_response.reason_phrase,
            put_successor_response
                .as_str()
                .unwrap_or("Unparseable content")
        );
        println!("{}", &error_message);
        return Err(status::Custom(Status::FailedDependency, error_message));
    }

    // Set itself as precessor of successor
    let put_precessor_uri = format!(
        "http://{}:{}/ring/precessor",
        config
            .successor
            .as_ref()
            .expect("Successor was just set, but does not exist")
            .hostname,
        config
            .successor
            .as_ref()
            .expect("Successor was just set, but does not exist")
            .port
    );

    let put_precessor_response = match minreq::put(put_precessor_uri)
        .with_json(&config.local)
        .expect("Could not serialize local node")
        .send()
    {
        Err(_err) => {
            let error_message = String::from("Could not connect to precessor to set successor.");
            println!("{}", &error_message);
            return Err(status::Custom(Status::FailedDependency, error_message));
        }
        Ok(response) => response,
    };

    if put_precessor_response.status_code != 200 {
        let error_message = format!(
            "Successor denied setting precessor. Node responded: [{} - {}] {}",
            put_precessor_response.status_code,
            put_precessor_response.reason_phrase,
            put_precessor_response
                .as_str()
                .unwrap_or("Unparseable content")
        );
        println!("{}", &error_message);
        return Err(status::Custom(Status::FailedDependency, error_message));
    }

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
            shutdown,
            get_storage,
            put_storage,
            get_network,
            get_precessor,
            get_successor,
            get_local,
            put_precessor,
            put_successor,
            put_local,
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
