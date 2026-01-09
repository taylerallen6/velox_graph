use crate::modules::connection::Connection;
use crate::modules::connections_backward::connections_backward_trait::ConnectionsBackward;
use crate::modules::connections_backward::hash_connections_backward::HashConnectionsBackward;
use crate::modules::connections_forward::connections_forward_trait::ConnectionsForward;
use crate::modules::connections_forward::hash_connections_forward::HashConnectionsForward;
use crate::modules::nodes::node_trait::Node;
use crate::modules::unsigned_int::UnsignedInt;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Serialize, Deserialize)]
#[serde(bound(
    serialize = "NodeIdT: UnsignedInt, NodeDataT: Serialize, ConnectionDataT: Serialize",
    deserialize = "NodeIdT: UnsignedInt, NodeDataT: DeserializeOwned, ConnectionDataT: DeserializeOwned"
))]
pub struct HashNode<NodeIdT, NodeDataT, ConnectionDataT>
where
    NodeIdT: UnsignedInt,
    NodeDataT: Clone + Serialize + DeserializeOwned,
    ConnectionDataT: Clone + Serialize + DeserializeOwned,
{
    node_id: NodeIdT,
    data: NodeDataT,

    connections_forward: HashConnectionsForward<NodeIdT, ConnectionDataT>,
    connections_backward: HashConnectionsBackward<NodeIdT>,
}

impl<NodeIdT, NodeDataT, ConnectionDataT> Node<NodeIdT, NodeDataT, ConnectionDataT>
    for HashNode<NodeIdT, NodeDataT, ConnectionDataT>
where
    NodeIdT: UnsignedInt,
    NodeDataT: Clone + Serialize + DeserializeOwned,
    ConnectionDataT: Clone + Serialize + DeserializeOwned,
{
    fn new(node_id: usize, node_data: NodeDataT) -> Self {
        let node_id = NodeIdT::from_usize(node_id);

        Self {
            node_id,
            data: node_data,
            connections_forward: HashConnectionsForward::new(),
            connections_backward: HashConnectionsBackward::new(),
        }
    }

    fn connections_forward(&mut self) -> &mut impl ConnectionsForward<NodeIdT, ConnectionDataT> {
        &mut self.connections_forward
    }
    fn connections_backward(&mut self) -> &mut impl ConnectionsBackward<NodeIdT> {
        &mut self.connections_backward
    }

    fn connection_forward_set(&mut self, node_id_value: usize, connection_data: ConnectionDataT) {
        let node_id_value = NodeIdT::from_usize(node_id_value);

        match self.connections_forward.lookup_hash.get(&node_id_value) {
            Some(&connection_index) => {
                let connection_index: usize = connection_index.to_usize();
                let connection = &mut self.connections_forward.data[connection_index];
                connection.data = connection_data;
            }
            None => {
                let new_connection = Connection {
                    node_id: node_id_value,
                    data: connection_data,
                };

                self.connections_forward.data.push(new_connection);

                let new_connection_index = self.connections_forward.data.len() - 1;
                let new_connection_index = NodeIdT::from_usize(new_connection_index);
                self.connections_forward
                    .lookup_hash
                    .insert(node_id_value, new_connection_index);
            }
        }
    }

    fn connection_forward_remove(&mut self, node_id_value: usize) {
        let node_id_value = NodeIdT::from_usize(node_id_value);

        //self.connections_forward.remove(&node_id_value);
        let data_vec_len = self.connections_forward.data.len();

        if data_vec_len == 1 {
            if let Some(&connection_index) =
                self.connections_forward.lookup_hash.get(&node_id_value)
            {
                let connection_index: usize = connection_index.to_usize();
                self.connections_forward.data.remove(connection_index);
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
                    self.connections_forward.data.remove(connection_index);
                    self.connections_forward.lookup_hash.remove(&node_id_value);
                }

                return;
            }

            // INFO: ELSE, DO THIS
            self.connections_forward.data.swap_remove(connection_index);

            let connection = &self.connections_forward.data[connection_index];

            let connection_index = NodeIdT::from_usize(connection_index);
            self.connections_forward
                .lookup_hash
                .insert(connection.node_id, connection_index);

            self.connections_forward.lookup_hash.remove(&node_id_value);
        }
    }

    fn connection_backward_create(&mut self, node_id_value: usize) {
        let node_id_value = NodeIdT::from_usize(node_id_value);
        self.connections_backward.insert(node_id_value);
    }

    fn connection_backward_delete(&mut self, node_id_value: usize) {
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

impl<NodeIdT, NodeDataT, ConnectionDataT> HashNode<NodeIdT, NodeDataT, ConnectionDataT>
where
    NodeIdT: UnsignedInt,
    NodeDataT: Clone + Serialize + DeserializeOwned,
    ConnectionDataT: Clone + Serialize + DeserializeOwned,
{
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
    ) -> &'a HashConnectionsForward<NodeIdT, ConnectionDataT> {
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
}
