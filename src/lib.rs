//! # ![](../assets/velox_graph_logo.png)
//!
//! # VeloxGraph
//!
//! [![Crates.io](https://img.shields.io/crates/v/velox_graph.svg)](https://crates.io/crates/velox_graph)
//! [![Apache2.0 licensed](https://img.shields.io/badge/license-Apache2.0-blue.svg)](https://github.com/taylerallen6/velox_graph/blob/main/LICENSE)
//! [![Documentation](https://docs.rs/velox_graph/badge.svg)](https://docs.rs/velox_graph)
//!
//! VeloxGraph is an extremely fast, efficient, low-level, in-memory, minimal graph database (wow, that is a mouth full). It is not revolutionary in its design but has a few key features that make it vital to the development of a new type of neural network architecture that I am working on, and THAT is what I consider revolutionary.
//!
//! ### Basic Code Example
//! ```rust
//! use velox_graph::VeloxGraph;
//!
//! fn main() {
//!     // INFO: Initialize the graph.
//!     let mut graph: VeloxGraph<u32, f64> = VeloxGraph::new();
//!
//!     // INFO: Create your first nodes.
//!     let node_id0 = graph.node_create(634);
//!     let node_id1 = graph.node_create(43);
//!
//!     // INFO: Create connection from node0 to node1.
//!     graph.nodes_connection_set(node_id0, node_id1, 5.24).unwrap();
//!
//!     // INFO: Get a mutable reference to that node.
//!     let node0 = graph.node_get(node_id0).unwrap();
//!
//!     println!("node0 data: {:?}", node0.data);
//!     println!("node0 connections: {:?}", &node0.connections_forward_get_all().data_vec);
//! }
//! ```
//!
//! ### More Complex Code Example
//! ```rust
//! use velox_graph::VeloxGraph;
//!
//! // INFO: Sample data to store in the nodes.
//! #[derive(Clone, Debug)]
//! struct NodeData {
//!     x: u32,
//!     y: u32,
//! }
//!
//! // INFO: Sample data to store in the connections.
//! #[derive(Clone, Debug)]
//! struct ConnData {
//!     a: u32,
//!     b: f64,
//! }
//!
//! fn main() {
//!     // INFO: Initialize the graph.
//!     let mut graph: VeloxGraph<NodeData, ConnData> = VeloxGraph::new();
//!
//!     // INFO: Create your first node.
//!     let node_id0 = graph.node_create(NodeData { x: 134, y: 351 });
//!     println!("num_entries: {}", graph.num_entries);
//!
//!     // INFO: Get a mutable reference to that node.
//!     let node = graph.node_get(node_id0).unwrap();
//!     println!("node data: {:?}", node.data);
//!
//!     // INFO: You can then edit that node in place. Remember this a mutable reference, no need to save.
//!     node.data.x += 4;
//!     node.data.y = 2431;
//!
//!     // INFO: You can get the node again if you want to verify that it was edited.
//!     let node = graph.node_get(node_id0).unwrap();
//!     println!("node data: {:?}", node.data);
//!
//!     // INFO: Create 2 more nodes.
//!     let node_id1 = graph.node_create(NodeData { x: 234, y: 5 });
//!     let node_id2 = graph.node_create(NodeData { x: 63, y: 42 });
//!     println!("num_entries: {}", graph.num_entries);
//!
//!     // INFO: Create connections some connections between nodes.
//!     graph
//!         .nodes_connection_set(node_id0, node_id1, ConnData { a: 243, b: 54.5 })
//!         .unwrap();
//!     graph
//!         .nodes_connection_set(node_id0, node_id2, ConnData { a: 63, b: 9.413 })
//!         .unwrap();
//!     graph
//!         .nodes_connection_set(node_id1, node_id2, ConnData { a: 2834, b: 5.24 })
//!         .unwrap();
//!     graph
//!         .nodes_connection_set(node_id2, node_id0, ConnData { a: 7, b: 463.62 })
//!         .unwrap();
//!
//!     // INFO: Loop through each connection that this node connects forward to (forward connections). You can NOT edit the connections.
//!     let node = graph.node_get(node_id0).unwrap();
//!     for connection in &node.connections_forward_get_all().data_vec {
//!         println!("forward_connection: {:?}", connection);
//!     }
//!
//!     // INFO: You can also see the what nodes the TO this node (backward connections). You can NOT edit the connections.
//!     let node2 = graph.node_get(node_id2).unwrap();
//!     for connection in node2.connections_backward_get_all() {
//!         println!("backward_connection: {:?}", connection);
//!     }
//!
//!     // INFO: Delete node connections.
//!     graph.nodes_connection_remove(node_id0, node_id1).unwrap();
//!     graph.nodes_connection_remove(node_id0, node_id2).unwrap();
//!
//!     // INFO: Delete nodes. Their connections are automatically deleted as well.
//!     graph.node_delete(0).unwrap();
//!     graph.node_delete(1).unwrap();
//!     graph.node_delete(2).unwrap();
//!     println!("num_entries: {}", graph.num_entries);
//! }
//! ```

pub mod tests;

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::io;
use std::result::Result::Err;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VeloxGraphError {
    #[error("Failed to create/load database file: {0}")]
    FileFail(String),
    #[error("io Error")]
    IoError(#[from] io::Error),
    #[error("toml deserialize Error")]
    TomlDeserializeError(#[from] toml::de::Error),
    #[error("toml serialize Error")]
    TomlSerializeError(#[from] toml::ser::Error),

    #[error("database: Empty_slots vector is empty")]
    EmptySlotsVectorIsEmpty,
    #[error("database: Slot {0} is not allocated. No data here. This means that slot >= nodes_vector.len(). You cannot access, update, or remove a slot that is not allocated.")]
    SlotNotAllocated(usize),
    #[error("database: Slot {0} is not used. No data here. Slot should already be in empty_slots vector. You cannot access, update, or remove a slot that is not in use.")]
    SlotNotUsed(usize),
    #[error("database: Connection {0} is not set. No data here.")]
    ConnectionNotSet(usize),

    #[error("unknown database error")]
    Unknown,
}

const SLOT_USED: bool = true;
const SLOT_NOT_USED: bool = false;

// const SETTINGS_FILE_NAME: &str = "vdb_settings.toml";

// INFO: TOML serialized
#[derive(Serialize, Deserialize, Debug)]
pub struct VeloxGraghSettings {
    version: String,
}

impl VeloxGraghSettings {
    fn new() -> VeloxGraghSettings {
        VeloxGraghSettings {
            version: String::from("2.0"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Connection<ConnectionT: Clone> {
    pub node_id: usize,
    pub data: ConnectionT,
}

#[derive(Clone, Debug)]
pub struct ConnectionsForward<ConnectionT: Clone> {
    pub lookup_hash: HashMap<usize, usize>,
    pub data_vec: Vec<Connection<ConnectionT>>,
}

impl<ConnectionT: Clone> ConnectionsForward<ConnectionT> {
    fn new() -> ConnectionsForward<ConnectionT> {
        ConnectionsForward {
            lookup_hash: HashMap::new(),
            data_vec: Vec::new(),
        }
    }

    /// Get immutable access to a ONE FORWARD connection
    ///
    /// # Example
    ///
    /// ```
    /// // INFO: Initialize the graph.
    /// let mut graph: VeloxGraph<u32, f64> = VeloxGraph::new();
    ///
    /// // INFO: Create example nodes.
    /// let node_id0 = graph.node_create(634);
    /// let node_id1 = graph.node_create(43);
    ///
    /// // INFO: Create connection from node0 to node1.
    /// graph.nodes_connection_set(node_id0, node_id1, 5.24).unwrap();
    ///
    /// // INFO: Get a mutable reference to that node.
    /// let node0 = graph.node_get(node_id0).unwrap();
    ///
    /// // INFO: Get a immutable reference to that node's forward connections.
    /// let forward_connections = node0.connections_forward_get_all();
    ///
    /// // INFO: Get a immutable reference to one connection.
    /// let connection = forward_connections.get(node_id1).unwrap();
    ///
    /// assert_eq!(connection.data, 5.24);
    /// ```
    pub fn get<'a>(
        &'a self,
        node_id: usize,
    ) -> Result<&'a Connection<ConnectionT>, VeloxGraphError> {
        match self.lookup_hash.get(&node_id) {
            Some(&connection_index) => Ok(&self.data_vec[connection_index]),
            None => Err(VeloxGraphError::ConnectionNotSet(node_id)),
        }
    }
}

#[derive(Clone)]
pub struct Node<NodeT: Clone, ConnectionT: Clone> {
    node_id: usize,
    pub data: NodeT,

    connections_forward: ConnectionsForward<ConnectionT>,
    connections_backward: HashSet<usize>,
}

#[derive(Clone)]
struct NodeOption<NodeT: Clone, ConnectionT: Clone> {
    is_used: bool,
    node: Node<NodeT, ConnectionT>,
}

impl<NodeT: Clone, ConnectionT: Clone> Node<NodeT, ConnectionT> {
    fn new(node_id: usize, node_data: NodeT) -> Node<NodeT, ConnectionT> {
        Node {
            node_id,
            data: node_data,
            connections_forward: ConnectionsForward::new(),
            connections_backward: HashSet::new(),
        }
    }

    /// Get immutable access to a node's FORWARD connections.
    ///
    /// # Example
    ///
    /// ```
    /// // INFO: Initialize the graph.
    /// let mut graph: VeloxGraph<u32, f64> = VeloxGraph::new();
    ///
    /// // INFO: Create example nodes.
    /// let node_id0 = graph.node_create(634);
    /// let node_id1 = graph.node_create(43);
    ///
    /// // INFO: Create connection from node0 to node1.
    /// graph.nodes_connection_set(node_id0, node_id1, 5.24).unwrap();
    ///
    /// // INFO: Get a mutable reference to that node.
    /// let node0 = graph.node_get(node_id0).unwrap();
    ///
    /// // INFO: Get a immutable reference to that node's forward connections.
    /// let forward_connections = node0.connections_forward_get_all();
    /// let connection = forward_connections.get(node_id1).unwrap();
    ///
    /// assert_eq!(connection.data, 5.24);
    /// ```
    pub fn connections_forward_get_all<'a>(&'a self) -> &'a ConnectionsForward<ConnectionT> {
        &self.connections_forward
    }

    /// Get immutable access to a node's FORWARD connections.
    ///
    /// # Example
    ///
    /// ```
    /// // INFO: Initialize the graph.
    /// let mut graph: VeloxGraph<u32, f64> = VeloxGraph::new();
    ///
    /// // INFO: Create example nodes.
    /// let node_id0 = graph.node_create(634);
    /// let node_id1 = graph.node_create(43);
    ///
    /// // INFO: Create connection from node0 to node1.
    /// graph.nodes_connection_set(node_id0, node_id1, 5.24).unwrap();
    ///
    /// // INFO: Get a mutable reference to that node.
    /// let node1 = graph.node_get(node_id1).unwrap();
    ///
    /// // INFO: Get a immutable reference to that node's backward connections.
    /// let backward_connections = node1.connections_backward_get_all();
    /// let does_connection_exist = backward_connections.contains(node_id0);
    ///
    /// assert_eq!(does_connection_exist, true);
    /// ```
    pub fn connections_backward_get_all<'a>(&'a self) -> &'a HashSet<usize> {
        &self.connections_backward
    }

    fn connection_forward_set(&mut self, node_id_value: usize, connection_data: ConnectionT) {
        match self.connections_forward.lookup_hash.get(&node_id_value) {
            Some(&connection_index) => {
                let connection = &mut self.connections_forward.data_vec[connection_index];
                connection.data = connection_data;
            }
            None => {
                let new_connection = Connection {
                    node_id: node_id_value,
                    data: connection_data,
                };

                self.connections_forward.data_vec.push(new_connection);

                let new_connection_index = self.connections_forward.data_vec.len() - 1;
                self.connections_forward
                    .lookup_hash
                    .insert(node_id_value, new_connection_index);
            }
        }
    }

    fn connection_forward_remove(&mut self, node_id_value: usize) {
        //self.connections_forward.remove(&node_id_value);
        let data_vec_len = self.connections_forward.data_vec.len();

        if data_vec_len == 1 {
            if let Some(&connection_index) =
                self.connections_forward.lookup_hash.get(&node_id_value)
            {
                self.connections_forward.data_vec.remove(connection_index);
                self.connections_forward.lookup_hash.remove(&node_id_value);
            }

            return;
        }

        if let Some(&connection_index) = self.connections_forward.lookup_hash.get(&node_id_value) {
            if data_vec_len == 1 || connection_index == data_vec_len - 1 {
                if let Some(&connection_index) =
                    self.connections_forward.lookup_hash.get(&node_id_value)
                {
                    self.connections_forward.data_vec.remove(connection_index);
                    self.connections_forward.lookup_hash.remove(&node_id_value);
                }

                return;
            }

            // INFO: ELSE, DO THIS
            self.connections_forward
                .data_vec
                .swap_remove(connection_index);

            let connection = &self.connections_forward.data_vec[connection_index];

            self.connections_forward
                .lookup_hash
                .insert(connection.node_id, connection_index);

            self.connections_forward.lookup_hash.remove(&node_id_value);
        }
    }

    fn connection_backward_create(&mut self, node_id_value: usize) {
        self.connections_backward.insert(node_id_value);
    }

    fn connection_backward_delete(&mut self, node_id_value: usize) {
        //self.connections_backward.remove(&node_id_value);

        match self.connections_backward.get(&node_id_value) {
            Some(_connection_val) => {
                self.connections_backward.remove(&node_id_value);
            }
            //None => println!("backward connection already not there."),
            None => (),
        };
    }
}

pub struct VeloxGraph<NodeT: Clone, ConnectionT: Clone> {
    #[allow(dead_code)]
    settings: VeloxGraghSettings,

    // INFO: metedata.
    // latest_available_slot: usize,
    pub num_entries: usize,
    // num_used_slots: usize,
    nodes_vector: Vec<NodeOption<NodeT, ConnectionT>>,
    empty_slots: Vec<usize>,
}

impl<NodeT: Clone, ConnectionT: Clone> VeloxGraph<NodeT, ConnectionT> {
    /// Initialize the graph.
    ///
    /// # Example
    ///
    /// ```
    /// // INFO: Sample data to store in the nodes. This is CUSTOM DATA defined by you that is stored in each node.
    /// #[derive(Clone, Debug)]
    /// struct NodeData {
    ///     x: u32,
    ///     y: u32,
    /// }
    ///
    /// // INFO: Sample data to store in the connections. This is CUSTOM DATA defined by you that is stored in each connection.
    /// #[derive(Clone, Debug)]
    /// struct ConnData {
    ///     a: u32,
    ///     b: f64,
    /// }
    ///
    /// // INFO: Initialize the graph with types NodeData, for nodes, and ConnData, for connections.
    /// let mut graph: VeloxGraph<NodeData, ConnData> = VeloxGraph::new();
    /// assert_eq!(graph.num_entries, 0);
    /// ```
    pub fn new() -> VeloxGraph<NodeT, ConnectionT> {
        let settings = VeloxGraghSettings::new();

        VeloxGraph {
            settings,
            num_entries: 0,
            nodes_vector: Vec::new(),
            empty_slots: Vec::new(),
        }
    }

    /// Create nodes.
    ///
    /// # Example
    ///
    /// ```
    /// // INFO: Initialize the graph.
    /// let mut graph: VeloxGraph<u32, f64> = VeloxGraph::new();
    ///
    /// // INFO: Create your first nodes.
    /// let node_id0 = graph.node_create(634);
    /// let node_id1 = graph.node_create(43);
    ///
    /// assert_eq!(graph.num_entries, 2);
    /// ```
    pub fn node_create(&mut self, node_data: NodeT) -> usize {
        let new_node_option = NodeOption {
            is_used: SLOT_USED,
            node: Node::new(0, node_data),
        };

        let new_node_id = match self.empty_slots.pop() {
            Some(new_node_id_value) => {
                self.nodes_vector[new_node_id_value] = new_node_option;
                new_node_id_value
            }
            None => {
                self.nodes_vector.push(new_node_option);
                self.nodes_vector.len() - 1
            }
        };

        self.num_entries += 1;

        self.nodes_vector[new_node_id].node.node_id = new_node_id;

        new_node_id
    }

    /// Get mutable access to a node.
    ///
    /// # Example
    ///
    /// ```
    /// // INFO: Initialize the graph.
    /// let mut graph: VeloxGraph<u32, f64> = VeloxGraph::new();
    ///
    /// // INFO: Create a node.
    /// let node_id = graph.node_create(4);
    ///
    /// // INFO: Get a mutable reference to that node.
    /// let node = graph.node_get(node_id).unwrap();
    ///
    /// assert_eq!(node.data, 4);
    ///
    /// // INFO: Make changes to the node.
    /// node.data += 5
    ///
    /// assert_eq!(node.data, 9);
    /// ```
    pub fn node_get<'a>(
        &'a mut self,
        node_id: usize,
    ) -> Result<&'a mut Node<NodeT, ConnectionT>, VeloxGraphError> {
        if node_id >= self.nodes_vector.len() {
            return Err(VeloxGraphError::SlotNotAllocated(node_id));
        }

        let node_option = &mut self.nodes_vector[node_id];

        if node_option.is_used == SLOT_NOT_USED {
            return Err(VeloxGraphError::SlotNotUsed(node_id));
        }

        Ok(&mut node_option.node)
    }

    /// Delete nodes.
    ///
    /// # Example
    ///
    /// ```
    /// // INFO: Initialize the graph.
    /// let mut graph: VeloxGraph<u32, f64> = VeloxGraph::new();
    ///
    /// // INFO: Create a node.
    /// let node_id = graph.node_create(634);
    /// assert_eq!(graph.num_entries, 1);
    ///
    /// // INFO: Delete the node. Its connections are automatically deleted as well.
    /// graph.node_delete(node_id).unwrap();
    ///
    /// assert_eq!(graph.num_entries, 0);
    /// ```
    pub fn node_delete(&mut self, node_id_to_delete: usize) -> Result<(), VeloxGraphError> {
        let node_to_delete = self.node_get(node_id_to_delete)?.clone();

        node_to_delete
            .connections_backward
            // .clone()
            .iter()
            .for_each(|&connection_node_id| {
                // let node = &mut self.nodes_vector[*connection_node_id].node;
                let node = self.node_get(connection_node_id).unwrap();
                node.connection_forward_remove(node_id_to_delete);
            });

        node_to_delete
            .connections_forward
            .data_vec
            // .clone()
            .iter()
            .for_each(|connection| {
                // let node = &mut self.nodes_vector[*connection_node_id].node;
                let node = self.node_get(connection.node_id).unwrap();
                node.connection_backward_delete(node_id_to_delete);
            });

        match node_id_to_delete == self.nodes_vector.len() - 1 {
            true => {
                self.nodes_vector.pop();
            }
            false => {
                self.empty_slots.push(node_id_to_delete);
                self.nodes_vector[node_id_to_delete].is_used = SLOT_NOT_USED;
            }
        }

        self.num_entries -= 1;
        Ok(())
    }

    /// Create node connections.
    ///
    /// # Example
    ///
    /// ```
    /// // INFO: Initialize the graph.
    /// let mut graph: VeloxGraph<u32, f64> = VeloxGraph::new();
    ///
    /// // INFO: Create example nodes.
    /// let node_id0 = graph.node_create(634);
    /// let node_id1 = graph.node_create(43);
    ///
    /// // INFO: Create connection from node0 to node1.
    /// graph.nodes_connection_set(node_id0, node_id1, 5.24).unwrap();
    ///
    /// // INFO: Get a mutable reference to that node.
    /// let node0 = graph.node_get(node_id0).unwrap();
    ///
    /// assert_eq!(node0.connections_forward_get_all().data_vec.len(), 1);
    /// ```
    pub fn nodes_connection_set(
        &mut self,
        first_node_id: usize,
        second_node_id: usize,
        connection_data: ConnectionT,
    ) -> Result<(), VeloxGraphError> {
        // INFO: check if both nodes exist, then create connection.
        let _second_node = self.node_get(second_node_id)?;
        let first_node = self.node_get(first_node_id)?;

        first_node.connection_forward_set(second_node_id, connection_data);

        let second_node = self.node_get(second_node_id)?;
        second_node.connection_backward_create(first_node_id);

        Ok(())
    }

    /// Delete node connections.
    ///
    /// # Example
    ///
    /// ```
    /// // INFO: Initialize the graph.
    /// let mut graph: VeloxGraph<u32, f64> = VeloxGraph::new();
    ///
    /// // INFO: Create example nodes.
    /// let node_id0 = graph.node_create(634);
    /// let node_id1 = graph.node_create(43);
    ///
    /// // INFO: Create connection from node0 to node1.
    /// graph.nodes_connection_set(node_id0, node_id1, 5.24).unwrap();
    ///
    /// // INFO: Get a mutable reference to that node.
    /// let node0 = graph.node_get(node_id0).unwrap();
    /// assert_eq!(node0.connections_forward_get_all().data_vec.len(), 1);
    ///
    /// // INFO: Delete node connection.
    /// graph.nodes_connection_remove(node_id0, node_id1).unwrap();
    ///
    /// assert_eq!(node0.connections_forward_get_all().data_vec.len(), 0);
    /// ```
    pub fn nodes_connection_remove(
        &mut self,
        first_node_id: usize,
        second_node_id: usize,
    ) -> Result<(), VeloxGraphError> {
        // INFO: check if both nodes exist, then create connection.
        let _second_node = self.node_get(second_node_id)?;
        let first_node = self.node_get(first_node_id)?;

        first_node.connection_forward_remove(second_node_id);

        let second_node = self.node_get(second_node_id)?;
        second_node.connection_backward_delete(first_node_id);

        Ok(())
    }
}
