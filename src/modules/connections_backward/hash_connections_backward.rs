use crate::modules::{
    connections_backward::connections_backward_trait::ConnectionsBackward,
    unsigned_int::UnsignedInt,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(bound(
    serialize = "NodeIdT: UnsignedInt",
    deserialize = "NodeIdT: UnsignedInt"
))]
pub struct HashConnectionsBackward<NodeIdT>
where
    NodeIdT: UnsignedInt,
{
    pub data: Vec<NodeIdT>,
}

impl<NodeIdT> ConnectionsBackward<NodeIdT> for HashConnectionsBackward<NodeIdT>
where
    NodeIdT: UnsignedInt,
{
    fn new() -> Self {
        Self { data: Vec::new() }
    }
    fn data(&mut self) -> &mut Vec<NodeIdT> {
        &mut self.data
    }
}
