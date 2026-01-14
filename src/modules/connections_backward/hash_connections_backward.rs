use crate::modules::connection::BackwardConnection;
use crate::modules::connections_backward::connections_backward_trait::{
    ConnectionsBackward, ConnectionsBackwardInternal,
};
use crate::modules::unsigned_int::UnsignedInt;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(bound(
    serialize = "NodeIdT: UnsignedInt",
    deserialize = "NodeIdT: UnsignedInt"
))]
pub struct HashConnectionsBackward<NodeIdT>
where
    NodeIdT: UnsignedInt,
{
    pub(crate) lookup_hash: HashMap<NodeIdT, NodeIdT>,
    data: Vec<BackwardConnection<NodeIdT>>,
}

impl<NodeIdT> ConnectionsBackwardInternal<NodeIdT> for HashConnectionsBackward<NodeIdT>
where
    NodeIdT: UnsignedInt,
{
    fn new() -> Self {
        Self {
            lookup_hash: HashMap::new(),
            data: Vec::new(),
        }
    }

    // fn data(&self) -> &Vec<BackwardConnection<NodeIdT>> {
    //     &self.data
    // }

    fn create(&mut self, node_id_value: usize) {
        let node_id_value = NodeIdT::from_usize(node_id_value);

        match self.lookup_hash.get(&node_id_value) {
            Some(&connection_index) => {
                let connection_index: usize = connection_index.to_usize();
                let connection = &mut self.data[connection_index];
                connection.node_id = node_id_value;
            }
            None => {
                let new_connection = BackwardConnection::new(node_id_value);
                self.data.push(new_connection);

                let new_connection_index = self.data.len() - 1;
                let new_connection_index = NodeIdT::from_usize(new_connection_index);
                self.lookup_hash.insert(node_id_value, new_connection_index);
            }
        }

        // let node_id_value = NodeIdT::from_usize(node_id_value);
        // self.data.insert(node_id_value);
    }

    fn delete(&mut self, node_id_value: usize) {
        let node_id_value = NodeIdT::from_usize(node_id_value);

        //self.remove(&node_id_value);
        let data_vec_len = self.data.len();

        if data_vec_len == 1 {
            if let Some(&connection_index) = self.lookup_hash.get(&node_id_value) {
                let connection_index: usize = connection_index.to_usize();
                self.data.remove(connection_index);
                self.lookup_hash.remove(&node_id_value);
            }

            return;
        }

        if let Some(&connection_index) = self.lookup_hash.get(&node_id_value) {
            let connection_index: usize = connection_index.to_usize();
            if data_vec_len == 1 || connection_index == data_vec_len - 1 {
                if let Some(&connection_index) = self.lookup_hash.get(&node_id_value) {
                    let connection_index: usize = connection_index.to_usize();
                    self.data.remove(connection_index);
                    self.lookup_hash.remove(&node_id_value);
                }

                return;
            }

            // INFO: ELSE, DO THIS
            self.data.swap_remove(connection_index);

            let connection = &self.data[connection_index];
            let connection_node_id = NodeIdT::from_usize(connection.node_id());

            let connection_index = NodeIdT::from_usize(connection_index);
            self.lookup_hash
                .insert(connection_node_id, connection_index);

            self.lookup_hash.remove(&node_id_value);
        }
    }
}

impl<NodeIdT> ConnectionsBackward<NodeIdT> for HashConnectionsBackward<NodeIdT>
where
    NodeIdT: UnsignedInt,
{
    fn data(&self) -> &Vec<BackwardConnection<NodeIdT>> {
        &self.data
    }
}
