use crate::modules::unsigned_int::UnsignedInt;

use serde::{de::DeserializeOwned, Serialize};

pub(crate) trait ConnectionsBackward<NodeIdT>:
    Sized + Serialize + DeserializeOwned + Clone
where
    NodeIdT: UnsignedInt,
{
    fn new() -> Self;
    fn data(&mut self) -> &mut Vec<NodeIdT>;
}
