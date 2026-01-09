use crate::modules::connection::Connection;
use crate::modules::error::VeloxGraphError;
use crate::modules::unsigned_int::UnsignedInt;

use serde::{de::DeserializeOwned, Serialize};

pub(crate) trait ConnectionsForward<NodeIdT, ConnectionDataT>:
    Sized + Serialize + DeserializeOwned + Clone
where
    NodeIdT: UnsignedInt,
    ConnectionDataT: Clone + Serialize + DeserializeOwned,
{
    fn new() -> Self;
    fn data(&self) -> &Vec<Connection<NodeIdT, ConnectionDataT>>;
}

pub trait ConnectionsForwardPublic<NodeIdT, ConnectionDataT>:
    Sized + Serialize + DeserializeOwned + Clone
where
    NodeIdT: UnsignedInt,
    ConnectionDataT: Clone + Serialize + DeserializeOwned,
{
    fn data(&self) -> &Vec<Connection<NodeIdT, ConnectionDataT>>;
    fn get<'a>(
        &'a mut self,
        node_id: usize,
    ) -> Result<&'a mut Connection<NodeIdT, ConnectionDataT>, VeloxGraphError>;
}
