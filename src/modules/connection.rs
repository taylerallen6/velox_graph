use crate::modules::unsigned_int::UnsignedInt;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(bound(
    serialize = "NodeIdT: UnsignedInt, ConnectionDataT: Serialize",
    deserialize = "NodeIdT: UnsignedInt, ConnectionDataT: DeserializeOwned"
))]
pub struct Connection<NodeIdT, ConnectionDataT>
where
    NodeIdT: UnsignedInt,
    ConnectionDataT: Clone + Serialize + DeserializeOwned,
{
    node_id: NodeIdT,
    pub(crate) data: ConnectionDataT,
}

impl<NodeIdT, ConnectionDataT> Connection<NodeIdT, ConnectionDataT>
where
    NodeIdT: UnsignedInt,
    ConnectionDataT: Clone + Serialize + DeserializeOwned,
{
    pub(crate) fn new(node_id: NodeIdT, data: ConnectionDataT) -> Self {
        Connection { node_id, data }
    }

    pub fn node_id(&self) -> usize {
        self.node_id.to_usize()
    }
    pub fn data(&mut self) -> &ConnectionDataT {
        &mut self.data
    }
}
