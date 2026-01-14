use crate::modules::connection::BackwardConnection;
use crate::modules::unsigned_int::UnsignedInt;

use serde::{de::DeserializeOwned, Serialize};

pub(crate) trait ConnectionsBackwardInternal<NodeIdT>:
    Sized + Serialize + DeserializeOwned + Clone
where
    NodeIdT: UnsignedInt,
{
    fn new() -> Self;
    fn create(&mut self, node_id_value: usize);
    fn delete(&mut self, node_id_value: usize);
}

pub trait ConnectionsBackward<NodeIdT>: Sized + Serialize + DeserializeOwned + Clone
where
    NodeIdT: UnsignedInt,
{
    fn data(&self) -> &Vec<BackwardConnection<NodeIdT>>;
}
