use crate::modules::connection::{Connection, ConnectionsForward};
use crate::modules::unsigned_int::UnsignedInt;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Serialize, Deserialize)]
#[serde(bound(
    serialize = "NodeIdT: UnsignedInt, NodeT: Serialize, ConnectionT: Serialize",
    deserialize = "NodeIdT: UnsignedInt, NodeT: DeserializeOwned, ConnectionT: DeserializeOwned"
))]
pub struct Node<NodeIdT, NodeT, ConnectionT>
where
    NodeIdT: UnsignedInt,
    NodeT: Clone + Serialize + DeserializeOwned,
    ConnectionT: Clone + Serialize + DeserializeOwned,
{
    pub(crate) node_id: NodeIdT,
    pub data: NodeT,

    pub(crate) connections_forward: ConnectionsForward<NodeIdT, ConnectionT>,
    pub(crate) connections_backward: HashSet<NodeIdT>,
}

impl<NodeIdT, NodeT, ConnectionT> Node<NodeIdT, NodeT, ConnectionT>
where
    NodeIdT: UnsignedInt,
    NodeT: Clone + Serialize + DeserializeOwned,
    ConnectionT: Clone + Serialize + DeserializeOwned,
{
    pub(crate) fn new(node_id: usize, node_data: NodeT) -> Node<NodeIdT, NodeT, ConnectionT> {
        let node_id = NodeIdT::from_usize(node_id);

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
    /// let mut graph: VeloxGraph<usize, usize, u32, f64> = VeloxGraph::new();
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
    pub fn connections_forward_get_all<'a>(
        &'a self,
    ) -> &'a ConnectionsForward<NodeIdT, ConnectionT> {
        &self.connections_forward
    }

    /// Get immutable access to a node's FORWARD connections.
    ///
    /// # Example
    ///
    /// ```
    /// // INFO: Initialize the graph.
    /// let mut graph: VeloxGraph<usize, usize, u32, f64> = VeloxGraph::new();
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
    pub fn connections_backward_get_all<'a>(&'a self) -> &'a HashSet<NodeIdT> {
        &self.connections_backward
    }

    pub(crate) fn connection_forward_set(
        &mut self,
        node_id_value: usize,
        connection_data: ConnectionT,
    ) {
        let node_id_value = NodeIdT::from_usize(node_id_value);

        match self.connections_forward.lookup_hash.get(&node_id_value) {
            Some(&connection_index) => {
                let connection_index: usize = connection_index.to_usize();
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
                let new_connection_index = NodeIdT::from_usize(new_connection_index);
                self.connections_forward
                    .lookup_hash
                    .insert(node_id_value, new_connection_index);
            }
        }
    }

    pub(crate) fn connection_forward_remove(&mut self, node_id_value: usize) {
        let node_id_value = NodeIdT::from_usize(node_id_value);

        //self.connections_forward.remove(&node_id_value);
        let data_vec_len = self.connections_forward.data_vec.len();

        if data_vec_len == 1 {
            if let Some(&connection_index) =
                self.connections_forward.lookup_hash.get(&node_id_value)
            {
                let connection_index: usize = connection_index.to_usize();
                self.connections_forward.data_vec.remove(connection_index);
                self.connections_forward.lookup_hash.remove(&node_id_value);
            }

            return;
        }

        if let Some(&connection_index) = self.connections_forward.lookup_hash.get(&node_id_value) {
            let connection_index: usize = connection_index.to_usize();
            if data_vec_len == 1 || connection_index == data_vec_len - 1 {
                if let Some(&connection_index) =
                    self.connections_forward.lookup_hash.get(&node_id_value)
                {
                    let connection_index: usize = connection_index.to_usize();
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

            let connection_index = NodeIdT::from_usize(connection_index);
            self.connections_forward
                .lookup_hash
                .insert(connection.node_id, connection_index);

            self.connections_forward.lookup_hash.remove(&node_id_value);
        }
    }

    pub(crate) fn connection_backward_create(&mut self, node_id_value: usize) {
        let node_id_value = NodeIdT::from_usize(node_id_value);
        self.connections_backward.insert(node_id_value);
    }

    pub(crate) fn connection_backward_delete(&mut self, node_id_value: usize) {
        let node_id_value = NodeIdT::from_usize(node_id_value);
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
