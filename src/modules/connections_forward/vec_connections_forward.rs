use crate::modules::connection::Connection;
use crate::modules::connections_forward::connections_forward_trait::{
    ConnectionsForward, ConnectionsForwardPublic,
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
pub struct VecConnectionsForward<NodeIdT, ConnectionDataT>
where
    NodeIdT: UnsignedInt,
    ConnectionDataT: Clone + Serialize + DeserializeOwned,
{
    pub(crate) lookup_hash: HashMap<NodeIdT, NodeIdT>,
    pub(crate) data: Vec<Connection<NodeIdT, ConnectionDataT>>,
}

impl<NodeIdT, ConnectionDataT> ConnectionsForward<NodeIdT, ConnectionDataT>
    for VecConnectionsForward<NodeIdT, ConnectionDataT>
where
    NodeIdT: UnsignedInt,
    ConnectionDataT: Clone + Serialize + DeserializeOwned,
{
    fn new() -> Self {
        Self {
            lookup_hash: HashMap::new(),
            data: Vec::new(),
        }
    }

    fn data(&self) -> &Vec<Connection<NodeIdT, ConnectionDataT>> {
        &self.data
    }
}

impl<NodeIdT, ConnectionDataT> ConnectionsForwardPublic<NodeIdT, ConnectionDataT>
    for VecConnectionsForward<NodeIdT, ConnectionDataT>
where
    NodeIdT: UnsignedInt,
    ConnectionDataT: Clone + Serialize + DeserializeOwned,
{
    fn data(&self) -> &Vec<Connection<NodeIdT, ConnectionDataT>> {
        &self.data
    }

    /// Get immutable access to a ONE FORWARD connection
    ///
    /// # Example
    ///
    /// ```
    /// // INFO: Initialize the graph.
    /// let mut graph: VeloxGraph<
    ///     usize,    // NodeIdT: Size for each node id.
    ///     u32,      // NodeDataT
    ///     f64,      // ConnectionDataT
    /// > = VeloxGraph::new();
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
    fn get<'a>(
        &'a mut self,
        node_id: usize,
    ) -> Result<&'a mut Connection<NodeIdT, ConnectionDataT>, VeloxGraphError> {
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
