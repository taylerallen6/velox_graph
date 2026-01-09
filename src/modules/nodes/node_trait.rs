use crate::modules::connections_backward::connections_backward_trait::ConnectionsBackward;
use crate::modules::connections_forward::connections_forward_trait::{
    ConnectionsForward, ConnectionsForwardPublic,
};
use crate::modules::unsigned_int::UnsignedInt;

use serde::{de::DeserializeOwned, Serialize};

pub(crate) trait Node<NodeIdT, NodeDataT, ConnectionDataT>:
    Sized + Serialize + DeserializeOwned + Clone
where
    NodeIdT: UnsignedInt,
    NodeDataT: Clone + Serialize + DeserializeOwned,
    ConnectionDataT: Clone + Serialize + DeserializeOwned,
{
    fn new(node_id: usize, node_data: NodeDataT) -> Self;
    fn node_id(&mut self) -> &mut NodeIdT;
    fn connections_forward(&mut self) -> &mut impl ConnectionsForward<NodeIdT, ConnectionDataT>;
    fn connections_backward(&mut self) -> &mut impl ConnectionsBackward<NodeIdT>;

    fn connection_forward_set(&mut self, node_id_value: usize, connection_data: ConnectionDataT);
    fn connection_forward_remove(&mut self, node_id_value: usize);
    fn connection_backward_create(&mut self, node_id_value: usize);
    fn connection_backward_delete(&mut self, node_id_value: usize);
}

pub trait NodePublic<NodeIdT, NodeDataT, ConnectionDataT>:
    Sized + Serialize + DeserializeOwned + Clone
where
    NodeIdT: UnsignedInt,
    NodeDataT: Clone + Serialize + DeserializeOwned,
    ConnectionDataT: Clone + Serialize + DeserializeOwned,
{
    fn node_id(&self) -> usize;
    fn data(&mut self) -> &mut NodeDataT;

    fn connections_forward_get_all<'a>(
        &'a mut self,
    ) -> &'a mut impl ConnectionsForwardPublic<NodeIdT, ConnectionDataT>;
    fn connections_backward_get_all<'a>(&'a self) -> &'a Vec<NodeIdT>;
}
