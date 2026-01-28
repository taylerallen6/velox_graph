use crate::modules::connection::ForwardConnection;
use crate::modules::connections_forward::connections_forward_trait::{
    private::Sealed, ConnectionsForward, ConnectionsForwardInternal,
};
use crate::modules::error::VeloxGraphError;
use crate::modules::unsigned_int::UnsignedInt;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(bound(
    serialize = "NodeIdT: UnsignedInt, ConnectionDataT: Serialize",
    deserialize = "NodeIdT: UnsignedInt, ConnectionDataT: DeserializeOwned"
))]
pub struct HashConnectionsForward<NodeIdT, ConnectionDataT>
where
    NodeIdT: UnsignedInt,
    ConnectionDataT: Clone + Serialize + DeserializeOwned,
{
    pub(crate) lookup_hash: HashMap<NodeIdT, NodeIdT>,
    data: Vec<ForwardConnection<NodeIdT, ConnectionDataT>>,
}

impl<NodeIdT, ConnectionDataT> Sealed for HashConnectionsForward<NodeIdT, ConnectionDataT>
where
    NodeIdT: UnsignedInt,
    ConnectionDataT: Clone + Serialize + DeserializeOwned,
{
}

impl<NodeIdT, ConnectionDataT> ConnectionsForwardInternal<NodeIdT, ConnectionDataT>
    for HashConnectionsForward<NodeIdT, ConnectionDataT>
where
    NodeIdT: UnsignedInt,
    ConnectionDataT: Clone + Serialize + DeserializeOwned,
{
    // fn data(&self) -> &Vec<ForwardConnection<NodeIdT, ConnectionDataT>> {
    //     &self.data
    // }

    fn new() -> Self {
        Self {
            lookup_hash: HashMap::new(),
            data: Vec::new(),
        }
    }

    fn set(&mut self, node_id_value: usize, connection_data: ConnectionDataT) {
        let node_id_value = NodeIdT::from_usize(node_id_value);

        match self.lookup_hash.get(&node_id_value) {
            Some(&connection_index) => {
                let connection_index: usize = connection_index.to_usize();
                let connection = &mut self.data[connection_index];
                connection.data = connection_data;
            }
            None => {
                let new_connection = ForwardConnection::new(node_id_value, connection_data);
                self.data.push(new_connection);

                let new_connection_index = self.data.len() - 1;
                let new_connection_index = NodeIdT::from_usize(new_connection_index);
                self.lookup_hash.insert(node_id_value, new_connection_index);
            }
        }
    }

    fn remove(&mut self, node_id_value: usize) {
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

impl<NodeIdT, ConnectionDataT> ConnectionsForward<NodeIdT, ConnectionDataT>
    for HashConnectionsForward<NodeIdT, ConnectionDataT>
where
    NodeIdT: UnsignedInt,
    ConnectionDataT: Clone + Serialize + DeserializeOwned,
{
    fn data(&self) -> &Vec<ForwardConnection<NodeIdT, ConnectionDataT>> {
        &self.data
    }

    /// Get immutable access to a ONE FORWARD connection
    ///
    /// # Example
    ///
    /// ```
    /// // INFO: Initialize the graph.
    /// let mut graph: VeloxGraphHash<
    ///     usize,    // NodeIdT: Size for each node id.
    ///     u32,      // NodeDataT
    ///     f64,      // ConnectionDataT
    /// > = VeloxGraphHash::new();
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
    /// let forward_connections = node0.connections_forward();
    ///
    /// // INFO: Get a immutable reference to one connection.
    /// let connection = forward_connections.get(node_id1).unwrap();
    ///
    /// assert_eq!(connection.data, 5.24);
    /// ```
    fn get<'a>(
        &'a mut self,
        node_id: usize,
    ) -> Result<&'a mut ForwardConnection<NodeIdT, ConnectionDataT>, VeloxGraphError> {
        let node_id_generic = NodeIdT::from_usize(node_id);
        match self.lookup_hash.get(&node_id_generic) {
            Some(&connection_index) => {
                let connection_index: usize = connection_index.to_usize();
                Ok(&mut self.data[connection_index])
            }
            None => {
                let node_id: usize = node_id.to_usize();
                Err(VeloxGraphError::ConnectionNotSet(node_id))
            }
        }
    }
}
