use crate::modules::connection::ForwardConnection;
use crate::modules::connections_forward::connections_forward_trait::{
    private::Sealed, ConnectionsForward, ConnectionsForwardInternal,
};
use crate::modules::error::VeloxGraphError;
use crate::modules::unsigned_int::UnsignedInt;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

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
    data: Vec<ForwardConnection<NodeIdT, ConnectionDataT>>,
}

impl<NodeIdT, ConnectionDataT> Sealed for VecConnectionsForward<NodeIdT, ConnectionDataT>
where
    NodeIdT: UnsignedInt,
    ConnectionDataT: Clone + Serialize + DeserializeOwned,
{
}

impl<NodeIdT, ConnectionDataT> ConnectionsForwardInternal<NodeIdT, ConnectionDataT>
    for VecConnectionsForward<NodeIdT, ConnectionDataT>
where
    NodeIdT: UnsignedInt,
    ConnectionDataT: Clone + Serialize + DeserializeOwned,
{
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn set(&mut self, node_id_value: usize, connection_data: ConnectionDataT) {
        let node_id_value = NodeIdT::from_usize(node_id_value);
        match self
            .data
            .iter()
            .position(|item| item.node_id == node_id_value)
        {
            Some(index) => {
                let connection = &mut self.data[index];
                connection.data = connection_data;
            }
            None => {
                let new_connection = ForwardConnection::new(node_id_value, connection_data);
                self.data.push(new_connection)
            }
        }
    }

    fn remove(&mut self, node_id_value: usize) {
        let node_id_value = NodeIdT::from_usize(node_id_value);
        //self.remove(&node_id_value);

        if let Some(index) = self
            .data
            .iter()
            .position(|item| item.node_id == node_id_value)
        {
            self.data.swap_remove(index);
        };
    }
}

impl<NodeIdT, ConnectionDataT> ConnectionsForward<NodeIdT, ConnectionDataT>
    for VecConnectionsForward<NodeIdT, ConnectionDataT>
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
    /// let mut graph: VeloxGraphVec<
    ///     usize,    // NodeIdT: Size for each node id.
    ///     u32,      // NodeDataT
    ///     f64,      // ConnectionDataT
    /// > = VeloxGraphVec::new();
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
        let node_id = NodeIdT::from_usize(node_id);
        match self.data.iter().position(|item| item.node_id == node_id) {
            Some(connection_index) => Ok(&mut self.data[connection_index]),
            None => {
                let node_id: usize = node_id.to_usize();
                Err(VeloxGraphError::ConnectionNotSet(node_id))
            }
        }
    }
}
