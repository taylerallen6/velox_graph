use crate::modules::connection::ForwardConnection;
use crate::modules::error::VeloxGraphError;
use crate::modules::unsigned_int::UnsignedInt;

use serde::{de::DeserializeOwned, Serialize};

pub(crate) mod private {
    pub trait Sealed {}
}

pub trait ConnectionsForwardInternal<NodeIdT, ConnectionDataT>:
    private::Sealed + Sized + Serialize + DeserializeOwned + Clone
where
    NodeIdT: UnsignedInt,
    ConnectionDataT: Clone + Serialize + DeserializeOwned,
{
    fn new() -> Self;
    fn set(&mut self, node_id_value: usize, connection_data: ConnectionDataT);
    fn remove(&mut self, node_id_value: usize);
}

pub trait ConnectionsForward<NodeIdT, ConnectionDataT>:
    ConnectionsForwardInternal<NodeIdT, ConnectionDataT> + Sized + Serialize + DeserializeOwned + Clone
where
    NodeIdT: UnsignedInt,
    ConnectionDataT: Clone + Serialize + DeserializeOwned,
{
    fn data(&self) -> &Vec<ForwardConnection<NodeIdT, ConnectionDataT>>;
    fn get<'a>(
        &'a mut self,
        node_id: usize,
    ) -> Result<&'a mut ForwardConnection<NodeIdT, ConnectionDataT>, VeloxGraphError>;
}
