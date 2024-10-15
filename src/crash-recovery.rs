use std::sync::{Arc, RwLock};
use warp::reply;
use warp::http::StatusCode;

use crate::NodeConfig;

// Function to handle simulating a crash
pub fn handle_sim_crash(node: Arc<RwLock<NodeConfig>>) -> reply::WithStatus<&'static str> {
    let mut config = node.write().unwrap();
    config.crash();
    reply::with_status("Node crashed", StatusCode::OK)
}

// Function to handle simulating a recovery
pub fn handle_sim_recover(node: Arc<RwLock<NodeConfig>>) -> reply::WithStatus<&'static str> {
    let mut config = node.write().unwrap();
    config.recover();
    reply::with_status("Node recovered", StatusCode::OK)
}

// Function to return an error when node is crashed
pub fn handle_crashed_node() -> reply::WithStatus<&'static str> {
    reply::with_status("Node is not responding", StatusCode::SERVICE_UNAVAILABLE)
}
