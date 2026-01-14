use crate::modules::connection::BackwardConnection;
use crate::modules::connections_backward::connections_backward_trait::{
    ConnectionsBackward, ConnectionsBackwardInternal,
};
use crate::modules::unsigned_int::UnsignedInt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(bound(
    serialize = "NodeIdT: UnsignedInt",
    deserialize = "NodeIdT: UnsignedInt"
))]
pub struct VecConnectionsBackward<NodeIdT>
where
    NodeIdT: UnsignedInt,
{
    pub(crate) data: Vec<BackwardConnection<NodeIdT>>,
}

impl<NodeIdT> ConnectionsBackwardInternal<NodeIdT> for VecConnectionsBackward<NodeIdT>
where
    NodeIdT: UnsignedInt,
{
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    // fn data(&self) -> &Vec<BackwardConnection<NodeIdT>> {
    //     &self.data
    // }

    fn create(&mut self, node_id_value: usize) {
        let node_id_value = NodeIdT::from_usize(node_id_value);
        match self
            .data
            .iter()
            .position(|item| item.node_id == node_id_value)
        {
            Some(index) => {
                let connection = &mut self.data[index];
                connection.node_id = node_id_value;
            }
            None => {
                let new_connection = BackwardConnection::new(node_id_value);
                self.data.push(new_connection)
            }
        }
    }

    fn delete(&mut self, node_id_value: usize) {
        let node_id_value = NodeIdT::from_usize(node_id_value);
        //self.remove(&node_id_value);

        if let Some(index) = self
            .data
            .iter()
            .position(|item| item.node_id == node_id_value)
        {
            self.data.swap_remove(index);
        };
    }
}

impl<NodeIdT> ConnectionsBackward<NodeIdT> for VecConnectionsBackward<NodeIdT>
where
    NodeIdT: UnsignedInt,
{
    fn data(&self) -> &Vec<BackwardConnection<NodeIdT>> {
        &self.data
    }
}
