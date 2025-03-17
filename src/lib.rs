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

#[derive(Clone)]
pub struct Node<NodeT: Clone, ConnectionT: Clone> {
    node_id: usize,
    pub data: NodeT,

    connections_forward: HashMap<usize, Connection<ConnectionT>>,
    connections_backward: HashSet<usize>,
}

#[derive(Clone)]
pub struct NodeOption<NodeT: Clone, ConnectionT: Clone> {
    is_used: bool,
    node: Node<NodeT, ConnectionT>,
}

impl<NodeT: Clone, ConnectionT: Clone> Node<NodeT, ConnectionT> {
    pub fn new(node_id: usize, node_data: NodeT) -> Node<NodeT, ConnectionT> {
        Node {
            node_id,
            data: node_data,
            connections_forward: HashMap::new(),
            connections_backward: HashSet::new(),
        }
    }

    pub fn connections_forward_get<'a>(&'a self) -> &'a HashMap<usize, Connection<ConnectionT>> {
        &self.connections_forward
    }

    pub fn connections_backward_get<'a>(&'a self) -> &'a HashSet<usize> {
        &self.connections_backward
    }

    fn connection_forward_create(&mut self, node_id_value: usize, connection_data: ConnectionT) {
        let new_connection = Connection {
            node_id: node_id_value,
            data: connection_data,
        };

        self.connections_forward
            .insert(node_id_value, new_connection);
    }

    fn connection_forward_delete(&mut self, node_id_value: usize) {
        //self.connections_forward.remove(&node_id_value);

        match self.connections_forward.get(&node_id_value) {
            Some(_connection_val) => {
                self.connections_forward.remove(&node_id_value);
            }
            //None => println!("forward connection already not there."),
            None => (),
        };
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
    pub fn new() -> VeloxGraph<NodeT, ConnectionT> {
        let settings = VeloxGraghSettings::new();

        VeloxGraph {
            settings,
            num_entries: 0,
            nodes_vector: Vec::new(),
            empty_slots: Vec::new(),
        }
    }

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

    pub fn node_delete(&mut self, node_id_to_delete: usize) -> Result<(), VeloxGraphError> {
        let node_to_delete = self.node_get(node_id_to_delete)?.clone();

        node_to_delete
            .connections_backward
            // .clone()
            .iter()
            .for_each(|&connection_node_id| {
                // let node = &mut self.nodes_vector[*connection_node_id].node;
                let node = self.node_get(connection_node_id).unwrap();
                node.connection_forward_delete(node_id_to_delete);
            });

        node_to_delete
            .connections_forward
            // .clone()
            .keys()
            .for_each(|&connection_node_id| {
                // let node = &mut self.nodes_vector[*connection_node_id].node;
                let node = self.node_get(connection_node_id).unwrap();
                node.connection_backward_delete(node_id_to_delete);
            });

        self.empty_slots.push(node_id_to_delete);
        self.num_entries -= 1;
        Ok(())
    }

    pub fn nodes_connection_create(
        &mut self,
        first_node_id: usize,
        second_node_id: usize,
        connection_data: ConnectionT,
    ) -> Result<(), VeloxGraphError> {
        // INFO: check if both nodes exist, then create connection.
        let _second_node = self.node_get(second_node_id)?;
        let first_node = self.node_get(first_node_id)?;

        first_node.connection_forward_create(second_node_id, connection_data);

        let second_node = self.node_get(second_node_id)?;
        second_node.connection_backward_create(first_node_id);

        Ok(())
    }

    pub fn nodes_connection_delete(
        &mut self,
        first_node_id: usize,
        second_node_id: usize,
    ) -> Result<(), VeloxGraphError> {
        // INFO: check if both nodes exist, then create connection.
        let _second_node = self.node_get(second_node_id)?;
        let first_node = self.node_get(first_node_id)?;

        first_node.connection_forward_delete(second_node_id);

        let second_node = self.node_get(second_node_id)?;
        second_node.connection_backward_delete(first_node_id);

        Ok(())
    }
}
