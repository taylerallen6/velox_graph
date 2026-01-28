use crate::modules::unsigned_int::UnsignedInt;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(bound = "
    NodeIdT: UnsignedInt,
    ConnectionDataT: Serialize + DeserializeOwned,
")]
pub struct ForwardConnection<NodeIdT, ConnectionDataT>
where
    NodeIdT: UnsignedInt,
    ConnectionDataT: Clone + Serialize + DeserializeOwned,
{
    pub(crate) node_id: NodeIdT,
    pub data: ConnectionDataT,
}

impl<NodeIdT, ConnectionDataT> ForwardConnection<NodeIdT, ConnectionDataT>
where
    NodeIdT: UnsignedInt,
    ConnectionDataT: Clone + Serialize + DeserializeOwned,
{
    pub(crate) fn new(node_id: NodeIdT, data: ConnectionDataT) -> Self {
        ForwardConnection { node_id, data }
    }

    pub fn node_id(&self) -> usize {
        self.node_id.to_usize()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(bound = "
    NodeIdT: UnsignedInt,
")]
pub struct BackwardConnection<NodeIdT>
where
    NodeIdT: UnsignedInt,
{
    pub(crate) node_id: NodeIdT,
}

impl<NodeIdT> BackwardConnection<NodeIdT>
where
    NodeIdT: UnsignedInt,
{
    pub(crate) fn new(node_id: NodeIdT) -> Self {
        BackwardConnection { node_id }
    }

    pub fn node_id(&self) -> usize {
        self.node_id.to_usize()
    }
}
