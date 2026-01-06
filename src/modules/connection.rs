use crate::modules::error::VeloxGraphError;
use crate::modules::unsigned_int::UnsignedInt;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(bound(
    serialize = "NodeIdT: UnsignedInt, ConnectionT: Serialize",
    deserialize = "NodeIdT: UnsignedInt, ConnectionT: DeserializeOwned"
))]
pub struct Connection<NodeIdT, ConnectionT>
where
    NodeIdT: UnsignedInt,
    ConnectionT: Clone + Serialize + DeserializeOwned,
{
    pub node_id: NodeIdT,
    pub data: ConnectionT,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(bound(
    serialize = "NodeIdT: UnsignedInt, ConnectionT: Serialize",
    deserialize = "NodeIdT: UnsignedInt, ConnectionT: DeserializeOwned"
))]
pub struct ConnectionsForward<NodeIdT, ConnectionT>
where
    NodeIdT: UnsignedInt,
    ConnectionT: Clone + Serialize + DeserializeOwned,
{
    pub(crate) lookup_hash: HashMap<NodeIdT, NodeIdT>,
    pub data_vec: Vec<Connection<NodeIdT, ConnectionT>>,
}

impl<NodeIdT, ConnectionT> ConnectionsForward<NodeIdT, ConnectionT>
where
    NodeIdT: UnsignedInt,
    ConnectionT: Clone + Serialize + DeserializeOwned,
{
    pub(crate) fn new() -> ConnectionsForward<NodeIdT, ConnectionT> {
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
    /// let mut graph: VeloxGraph<
    ///     usize,    // NodeIdT: Size for each node id.
    ///     u32,      // NodeT
    ///     f64,      // ConnectionT
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
    pub fn get<'a>(
        &'a self,
        node_id: usize,
    ) -> Result<&'a Connection<NodeIdT, ConnectionT>, VeloxGraphError> {
        let node_id_generic = NodeIdT::from_usize(node_id);
        match self.lookup_hash.get(&node_id_generic) {
            Some(&connection_index) => {
                let connection_index: usize = connection_index.to_usize();
                Ok(&self.data_vec[connection_index])
            }
            None => {
                let node_id: usize = node_id.to_usize();
                Err(VeloxGraphError::ConnectionNotSet(node_id))
            }
        }
    }
}
