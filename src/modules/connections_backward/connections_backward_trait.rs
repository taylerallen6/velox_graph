use crate::modules::connection::BackwardConnection;
use crate::modules::unsigned_int::UnsignedInt;

use serde::{de::DeserializeOwned, Serialize};

pub mod private {
    pub trait Sealed {}
}

pub trait ConnectionsBackwardInternal<NodeIdT>:
    private::Sealed + Sized + Serialize + DeserializeOwned + Clone
where
    NodeIdT: UnsignedInt,
{
    fn new() -> Self;
    fn create(&mut self, node_id_value: usize);
    fn delete(&mut self, node_id_value: usize);
}

pub trait ConnectionsBackward<NodeIdT>:
    ConnectionsBackwardInternal<NodeIdT> + Sized + Serialize + DeserializeOwned + Clone
where
    NodeIdT: UnsignedInt,
{
    fn data(&self) -> &Vec<BackwardConnection<NodeIdT>>;
}
