use crate::modules::connections_backward::connections_backward_trait::ConnectionsBackward;
use crate::modules::connections_forward::connections_forward_trait::ConnectionsForward;
use crate::modules::error::VeloxGraphError;
use crate::modules::node::Node;
use crate::modules::unsigned_int::UnsignedInt;

use serde::{de::DeserializeOwned, Serialize};

pub(crate) mod graph_private {
    pub trait GraphSealed {}
}

pub trait GraphInternal<NodeIdT, ConnForwardT, ConnBackwardT, NodeDataT, ConnectionDataT>:
    graph_private::GraphSealed + Sized
where
    ConnForwardT: ConnectionsForward<NodeIdT, ConnectionDataT>,
    ConnBackwardT: ConnectionsBackward<NodeIdT>,
    NodeIdT: UnsignedInt,
    NodeDataT: Clone + Serialize + DeserializeOwned,
    ConnectionDataT: Clone + Serialize + DeserializeOwned,
{
    fn nodes_vector(
        &self,
    ) -> &Vec<Option<Node<NodeIdT, ConnForwardT, ConnBackwardT, NodeDataT, ConnectionDataT>>>;
    fn empty_slots(&self) -> &Vec<usize>;
}

pub trait Graph<NodeIdT, ConnForwardT, ConnBackwardT, NodeDataT, ConnectionDataT>:
    GraphInternal<NodeIdT, ConnForwardT, ConnBackwardT, NodeDataT, ConnectionDataT>
where
    ConnForwardT: ConnectionsForward<NodeIdT, ConnectionDataT>,
    ConnBackwardT: ConnectionsBackward<NodeIdT>,
    NodeIdT: UnsignedInt,
    NodeDataT: Clone + Serialize + DeserializeOwned,
    ConnectionDataT: Clone + Serialize + DeserializeOwned,
{
    fn new() -> Self;
    fn num_entries(&self) -> usize;
    fn node_create(&mut self, node_data: NodeDataT) -> usize;
    fn node_get<'a>(
        &'a mut self,
        node_id: usize,
    ) -> Result<
        &'a mut Node<NodeIdT, ConnForwardT, ConnBackwardT, NodeDataT, ConnectionDataT>,
        VeloxGraphError,
    >;
    fn node_delete(&mut self, node_id_to_delete: usize) -> Result<(), VeloxGraphError>;
    fn nodes_connection_set(
        &mut self,
        first_node_id: usize,
        second_node_id: usize,
        connection_data: ConnectionDataT,
    ) -> Result<(), VeloxGraphError>;
    fn nodes_connection_remove(
        &mut self,
        first_node_id: usize,
        second_node_id: usize,
    ) -> Result<(), VeloxGraphError>;
    fn save(&self, file_path: String) -> Result<(), VeloxGraphError>;
    fn load(
        file_path: String,
    ) -> Result<
        impl Graph<NodeIdT, ConnForwardT, ConnBackwardT, NodeDataT, ConnectionDataT>,
        VeloxGraphError,
    >;
}
