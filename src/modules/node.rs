use crate::modules::connections_backward::connections_backward_trait::ConnectionsBackwardInternal;
use crate::modules::connections_forward::connections_forward_trait::{
    ConnectionsForward, ConnectionsForwardInternal,
};
use crate::modules::unsigned_int::UnsignedInt;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Clone, Serialize, Deserialize)]
#[serde(bound = "
    NodeIdT: UnsignedInt,
    NodeDataT: Serialize + DeserializeOwned,
    ConnectionDataT: Serialize + DeserializeOwned,
    ConnForwardT: ConnectionsForwardInternal<NodeIdT, ConnectionDataT>
        + ConnectionsForward<NodeIdT, ConnectionDataT>,
    ConnBackwardT: ConnectionsBackwardInternal<NodeIdT>,
")]
#[allow(private_bounds)]
pub struct Node<NodeIdT, ConnForwardT, ConnBackwardT, NodeDataT, ConnectionDataT>
where
    NodeIdT: UnsignedInt,
    ConnForwardT: ConnectionsForwardInternal<NodeIdT, ConnectionDataT>
        + ConnectionsForward<NodeIdT, ConnectionDataT>,
    ConnBackwardT: ConnectionsBackwardInternal<NodeIdT>,
    NodeDataT: Clone + Serialize + DeserializeOwned,
    ConnectionDataT: Clone + Serialize + DeserializeOwned,
{
    pub(crate) node_id: NodeIdT,
    pub data: NodeDataT,

    connections_forward: ConnForwardT,
    pub(crate) connections_backward: ConnBackwardT,

    _phantom_conn_data: PhantomData<ConnectionDataT>,
}

#[allow(private_bounds)]
impl<NodeIdT, ConnForwardT, ConnBackwardT, NodeDataT, ConnectionDataT>
    Node<NodeIdT, ConnForwardT, ConnBackwardT, NodeDataT, ConnectionDataT>
where
    NodeIdT: UnsignedInt,
    ConnForwardT: ConnectionsForwardInternal<NodeIdT, ConnectionDataT>
        + ConnectionsForward<NodeIdT, ConnectionDataT>,
    ConnBackwardT: ConnectionsBackwardInternal<NodeIdT>,
    NodeDataT: Clone + Serialize + DeserializeOwned,
    ConnectionDataT: Clone + Serialize + DeserializeOwned,
{
    pub(crate) fn new(node_id: usize, node_data: NodeDataT) -> Self {
        let node_id = NodeIdT::from_usize(node_id);

        Self {
            node_id,
            data: node_data,
            connections_forward: ConnForwardT::new(),
            connections_backward: ConnBackwardT::new(),

            _phantom_conn_data: PhantomData,
        }
    }

    pub fn node_id(&self) -> usize {
        self.node_id.to_usize()
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
    /// let forward_connections = node0.connections_forward();
    /// let connection = forward_connections.get(node_id1).unwrap();
    ///
    /// assert_eq!(connection.data, 5.24);
    /// ```
    pub fn connections_forward<'a>(&'a mut self) -> &'a mut ConnForwardT {
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
    /// let backward_connections = node1.connections_backward();
    /// let does_connection_exist = backward_connections.contains(node_id0);
    ///
    /// assert_eq!(does_connection_exist, true);
    /// ```
    pub fn connections_backward<'a>(&'a self) -> &'a ConnBackwardT {
        &self.connections_backward
    }
}
