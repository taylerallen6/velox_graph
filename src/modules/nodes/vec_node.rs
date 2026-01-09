use crate::modules::connection::Connection;
use crate::modules::connections_backward::connections_backward_trait::ConnectionsBackward;
use crate::modules::connections_backward::vec_connections_backward::VecConnectionsBackward;
use crate::modules::connections_forward::connections_forward_trait::{
    ConnectionsForward, ConnectionsForwardPublic,
};
use crate::modules::connections_forward::vec_connections_forward::VecConnectionsForward;
use crate::modules::nodes::node_trait::{Node, NodePublic};
use crate::modules::unsigned_int::UnsignedInt;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(bound(
    serialize = "NodeIdT: UnsignedInt, NodeDataT: Serialize, ConnectionDataT: Serialize",
    deserialize = "NodeIdT: UnsignedInt, NodeDataT: DeserializeOwned, ConnectionDataT: DeserializeOwned"
))]
pub struct VecNode<NodeIdT, NodeDataT, ConnectionDataT>
where
    NodeIdT: UnsignedInt,
    NodeDataT: Clone + Serialize + DeserializeOwned,
    ConnectionDataT: Clone + Serialize + DeserializeOwned,
{
    node_id: NodeIdT,
    data: NodeDataT,

    connections_forward: VecConnectionsForward<NodeIdT, ConnectionDataT>,
    connections_backward: VecConnectionsBackward<NodeIdT>,
}

impl<NodeIdT, NodeDataT, ConnectionDataT> Node<NodeIdT, NodeDataT, ConnectionDataT>
    for VecNode<NodeIdT, NodeDataT, ConnectionDataT>
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
            connections_forward: VecConnectionsForward::new(),
            connections_backward: VecConnectionsBackward::new(),
        }
    }

    fn node_id(&mut self) -> &mut NodeIdT {
        &mut self.node_id
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
                let new_connection = Connection::new(node_id_value, connection_data);
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
            let connection_node_id = NodeIdT::from_usize(connection.node_id());

            let connection_index = NodeIdT::from_usize(connection_index);
            self.connections_forward
                .lookup_hash
                .insert(connection_node_id, connection_index);

            self.connections_forward.lookup_hash.remove(&node_id_value);
        }
    }

    fn connection_backward_create(&mut self, node_id_value: usize) {
        let node_id_value = NodeIdT::from_usize(node_id_value);
        match self
            .connections_backward
            .data
            .iter()
            .position(|&item| item == node_id_value)
        {
            Some(index) => self.connections_backward.data[index] = node_id_value,
            None => self.connections_backward.data.push(node_id_value),
        }
    }

    fn connection_backward_delete(&mut self, node_id_value: usize) {
        let node_id_value = NodeIdT::from_usize(node_id_value);
        //self.connections_backward.remove(&node_id_value);

        if let Some(index) = self
            .connections_backward
            .data
            .iter()
            .position(|&item| item == node_id_value)
        {
            self.connections_backward.data.swap_remove(index);
        };
    }
}

impl<NodeIdT, NodeDataT, ConnectionDataT> NodePublic<NodeIdT, NodeDataT, ConnectionDataT>
    for VecNode<NodeIdT, NodeDataT, ConnectionDataT>
where
    NodeIdT: UnsignedInt,
    NodeDataT: Clone + Serialize + DeserializeOwned,
    ConnectionDataT: Clone + Serialize + DeserializeOwned,
{
    fn node_id(&self) -> usize {
        self.node_id.to_usize()
    }
    fn data(&mut self) -> &mut NodeDataT {
        &mut self.data
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
    fn connections_forward_get_all<'a>(
        &'a mut self,
    ) -> &'a mut impl ConnectionsForwardPublic<NodeIdT, ConnectionDataT> {
        &mut self.connections_forward
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
    fn connections_backward_get_all<'a>(&'a self) -> &'a Vec<NodeIdT> {
        &self.connections_backward.data
    }
}
