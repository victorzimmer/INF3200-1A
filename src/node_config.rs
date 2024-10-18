use crate::{Network, Node, Storage};  

pub struct NodeConfig {
    pub network: Option<Network>,
    pub connected: bool,
    pub local: Node,
    pub successor: Option<Node>,
    pub precessor: Option<Node>,
    pub finger_table: Vec<Node>,
    pub storage: Storage,
    pub crashed: bool,
}

impl NodeConfig {
    // Method to simulate crashing the node
    pub fn crash(&mut self) {
        self.crashed = true;
    }

    // Method to simulate recovering the node
    pub fn recover(&mut self) {
        self.crashed = false;
    }

    // Function to check if the node is crashed
    pub fn is_crashed(&self) -> bool {
        self.crashed
    }
}
