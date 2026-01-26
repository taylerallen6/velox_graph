use crate::modules::connections_backward::connections_backward_trait::ConnectionsBackward;
use crate::modules::connections_backward::{
    hash_connections_backward::HashConnectionsBackward,
    vec_connections_backward::VecConnectionsBackward,
};
use crate::modules::connections_forward::connections_forward_trait::ConnectionsForward;
use crate::modules::connections_forward::{
    hash_connections_forward::HashConnectionsForward,
    vec_connections_forward::VecConnectionsForward,
};
use crate::modules::error::VeloxGraphError;
use crate::modules::graph_settings::VeloxGraghSettings;
use crate::modules::node::Node;
use crate::modules::unsigned_int::UnsignedInt;

use postcard;
use serde::{de::DeserializeOwned, Serialize};
use std::fs::File;
use std::io::{LineWriter, Write};
use std::marker::PhantomData;
use std::os::unix::prelude::FileExt;

// Now alias the concrete Container specializations.
pub type VeloxGraphVec<NodeIdT, NodeDataT, ConnectionDataT> = VeloxGraph<
    NodeIdT,
    VecConnectionsForward<NodeIdT, ConnectionDataT>,
    VecConnectionsBackward<NodeIdT>,
    NodeDataT,
    ConnectionDataT,
>;

pub type VeloxGraphHash<NodeIdT, NodeDataT, ConnectionDataT> = VeloxGraph<
    NodeIdT,
    HashConnectionsForward<NodeIdT, ConnectionDataT>,
    HashConnectionsBackward<NodeIdT>,
    NodeDataT,
    ConnectionDataT,
>;

#[allow(private_bounds)]
pub struct VeloxGraph<NodeIdT, ConnForwardT, ConnBackwardT, NodeDataT, ConnectionDataT>
where
    NodeIdT: UnsignedInt,
    ConnForwardT: ConnectionsForward<NodeIdT, ConnectionDataT>,
    ConnBackwardT: ConnectionsBackward<NodeIdT>,
    NodeDataT: Clone + Serialize + DeserializeOwned,
    ConnectionDataT: Clone + Serialize + DeserializeOwned,
{
    #[allow(dead_code)]
    settings: VeloxGraghSettings,

    // INFO: metedata.
    // latest_available_slot: usize,
    num_entries: usize,
    // num_used_slots: usize,
    pub(crate) nodes_vector:
        Vec<Option<Node<NodeIdT, ConnForwardT, ConnBackwardT, NodeDataT, ConnectionDataT>>>,
    pub(crate) empty_slots: Vec<usize>,

    // PhantomData to "use" the other generics.
    _phantom_id: PhantomData<NodeIdT>,
    _phantom_node_data: PhantomData<NodeDataT>,
    _phantom_conn_data: PhantomData<ConnectionDataT>,
}

#[allow(private_bounds)]
impl<NodeIdT, ConnForwardT, ConnBackwardT, NodeDataT, ConnectionDataT>
    VeloxGraph<NodeIdT, ConnForwardT, ConnBackwardT, NodeDataT, ConnectionDataT>
where
    ConnForwardT: ConnectionsForward<NodeIdT, ConnectionDataT>,
    ConnBackwardT: ConnectionsBackward<NodeIdT>,
    NodeIdT: UnsignedInt,
    NodeDataT: Clone + Serialize + DeserializeOwned,
    ConnectionDataT: Clone + Serialize + DeserializeOwned,
{
    /// Initialize the graph.
    ///
    /// # Example
    ///
    /// ```
    /// // INFO: Sample data to store in the nodes. This is CUSTOM DATA defined by you that is stored in each node.
    /// #[derive(Clone, Debug, Serialize, Deserialize)]
    /// struct NodeData {
    ///     x: u32,
    ///     y: u32,
    /// }
    ///
    /// // INFO: Sample data to store in the connections. This is CUSTOM DATA defined by you that is stored in each connection.
    /// #[derive(Clone, Debug, Serialize, Deserialize)]
    /// struct ConnData {
    ///     a: u32,
    ///     b: f64,
    /// }
    ///
    /// // INFO: Initialize the graph with types NodeData, for nodes, and ConnData, for connections.
    /// let mut graph: VeloxGraph<
    ///     usize,      // NodeIdT: Size for each node id.
    ///     NodeData,   // NodeDataT
    ///     ConnData,   // ConnectionDataT
    /// > = VeloxGraph::new();
    /// assert_eq!(graph.num_entries, 0);
    /// ```
    pub fn new() -> Self {
        let settings = VeloxGraghSettings::new();

        Self {
            settings,
            num_entries: 0,
            nodes_vector: Vec::new(),
            empty_slots: Vec::new(),

            _phantom_id: PhantomData,
            _phantom_node_data: PhantomData,
            _phantom_conn_data: PhantomData,
        }
    }

    pub fn num_entries(&self) -> usize {
        self.num_entries
    }

    /// Create nodes.
    ///
    /// # Example
    ///
    /// ```
    /// // INFO: Initialize the graph.
    /// let mut graph: VeloxGraph<
    ///     usize,    // NodeIdT: Size for each node id.
    ///     u32,      // NodeDataT
    ///     f64,      // ConnectionDataT
    /// > = VeloxGraph::new();
    ///
    /// // INFO: Create your first nodes.
    /// let node_id0 = graph.node_create(634);
    /// let node_id1 = graph.node_create(43);
    ///
    /// assert_eq!(graph.num_entries, 2);
    /// ```
    pub fn node_create(&mut self, node_data: NodeDataT) -> usize {
        // let new_node_option = NodeOption {
        //     is_used: SLOT_USED,
        //     node: Node::new(0, node_data),
        // };
        let new_node_option = Some(Node::new(0, node_data));

        let new_node_id = match self.empty_slots.pop() {
            Some(new_node_id_value) => {
                self.nodes_vector[new_node_id_value] = new_node_option;
                new_node_id_value
            }
            None => {
                self.nodes_vector.push(new_node_option);
                self.nodes_vector.len() - 1
            }
        };

        self.num_entries += 1;

        if let Some(node) = &mut self.nodes_vector[new_node_id] {
            let new_node_id_generic = NodeIdT::from_usize(new_node_id);
            node.node_id = new_node_id_generic;
        }

        new_node_id
    }

    /// Get mutable access to a node.
    ///
    /// # Example
    ///
    /// ```
    /// // INFO: Initialize the graph.
    /// let mut graph: VeloxGraph<
    ///     usize,    // NodeIdT: Size for each node id.
    ///     u32,      // NodeDataT
    ///     f64,      // ConnectionDataT
    /// > = VeloxGraph::new();
    ///
    /// // INFO: Create a node.
    /// let node_id = graph.node_create(4);
    ///
    /// // INFO: Get a mutable reference to that node.
    /// let node = graph.node_get(node_id).unwrap();
    ///
    /// assert_eq!(node.data, 4);
    ///
    /// // INFO: Make changes to the node.
    /// node.data += 5
    ///
    /// assert_eq!(node.data, 9);
    /// ```
    pub fn node_get<'a>(
        &'a mut self,
        node_id: usize,
    ) -> Result<
        &'a mut Node<NodeIdT, ConnForwardT, ConnBackwardT, NodeDataT, ConnectionDataT>,
        VeloxGraphError,
    > {
        if node_id >= self.nodes_vector.len() {
            return Err(VeloxGraphError::SlotNotAllocated(node_id));
        }

        let node_option = &mut self.nodes_vector[node_id];

        match node_option {
            Some(node) => Ok(node),
            None => return Err(VeloxGraphError::SlotNotUsed(node_id)),
        }
    }

    /// Delete nodes.
    ///
    /// # Example
    ///
    /// ```
    /// // INFO: Initialize the graph.
    /// let mut graph: VeloxGraph<
    ///     usize,    // NodeIdT: Size for each node id.
    ///     u32,      // NodeDataT
    ///     f64,      // ConnectionDataT
    /// > = VeloxGraph::new();
    ///
    /// // INFO: Create a node.
    /// let node_id = graph.node_create(634);
    /// assert_eq!(graph.num_entries, 1);
    ///
    /// // INFO: Delete the node. Its connections are automatically deleted as well.
    /// graph.node_delete(node_id).unwrap();
    ///
    /// assert_eq!(graph.num_entries, 0);
    /// ```
    pub fn node_delete(&mut self, node_id_to_delete: usize) -> Result<(), VeloxGraphError> {
        let mut node_to_delete = self.node_get(node_id_to_delete)?.clone();

        node_to_delete
            .connections_backward()
            .data()
            // .clone()
            .iter()
            .for_each(|connection_node_id| {
                // let node = &mut self.nodes_vector[*connection_node_id].node;
                let connection_node_id = connection_node_id.node_id.to_usize();
                let node = self.node_get(connection_node_id).unwrap();
                node.connections_forward().remove(node_id_to_delete);
            });

        node_to_delete
            .connections_forward()
            .data()
            // .clone()
            .iter()
            .for_each(|connection| {
                // let node = &mut self.nodes_vector[*connection_node_id].node;
                let connection_node_id = connection.node_id().to_usize();
                let node = self.node_get(connection_node_id).unwrap();
                node.connections_backward.delete(node_id_to_delete);
            });

        match node_id_to_delete == self.nodes_vector.len() - 1 {
            true => {
                self.nodes_vector.pop();
            }
            false => {
                self.empty_slots.push(node_id_to_delete);
                self.nodes_vector[node_id_to_delete] = None;
            }
        }

        self.num_entries -= 1;
        Ok(())
    }

    /// Create node connections.
    ///
    /// # Example
    ///
    /// ```
    /// // INFO: Initialize the graph.
    /// let mut graph: VeloxGraph<
    ///     usize,    // NodeIdT: Size for each node id.
    ///     u32,      // NodeDataT
    ///     f64,      // ConnectionDataT
    /// > = VeloxGraph::new();
    ///
    /// // INFO: Create example nodes.
    /// let node_id0 = graph.node_create(634);
    /// let node_id1 = graph.node_create(43);
    ///
    /// // INFO: Create connection from node0 to node1.
    /// graph.nodes_connection_set(node_id0, node_id1, 5.24).unwrap();
    ///
    /// // INFO: Get a mutable reference to that node.
    /// let node0 = graph.node_get(node_id0).unwrap();
    ///
    /// assert_eq!(node0.connections_forward_get_all().data_vec.len(), 1);
    /// ```
    pub fn nodes_connection_set(
        &mut self,
        first_node_id: usize,
        second_node_id: usize,
        connection_data: ConnectionDataT,
    ) -> Result<(), VeloxGraphError> {
        // INFO: check if both nodes exist, then create connection.
        let _second_node = self.node_get(second_node_id)?;
        let first_node = self.node_get(first_node_id)?;

        first_node
            .connections_forward()
            .set(second_node_id, connection_data);

        let second_node = self.node_get(second_node_id)?;
        second_node.connections_backward.create(first_node_id);

        Ok(())
    }

    /// Delete node connections.
    ///
    /// # Example
    ///
    /// ```
    /// // INFO: Initialize the graph.
    /// let mut graph: VeloxGraph<
    ///     usize,    // NodeIdT: Size for each node id.
    ///     u32,      // NodeDataT
    ///     f64,      // ConnectionDataT
    /// > = VeloxGraph::new();
    ///
    /// // INFO: Create example nodes.
    /// let node_id0 = graph.node_create(634);
    /// let node_id1 = graph.node_create(43);
    ///
    /// // INFO: Create connection from node0 to node1.
    /// graph.nodes_connection_set(node_id0, node_id1, 5.24).unwrap();
    ///
    /// // INFO: Get a mutable reference to that node.
    /// let node0 = graph.node_get(node_id0).unwrap();
    /// assert_eq!(node0.connections_forward_get_all().data_vec.len(), 1);
    ///
    /// // INFO: Delete node connection.
    /// graph.nodes_connection_remove(node_id0, node_id1).unwrap();
    ///
    /// assert_eq!(node0.connections_forward_get_all().data_vec.len(), 0);
    /// ```
    pub fn nodes_connection_remove(
        &mut self,
        first_node_id: usize,
        second_node_id: usize,
    ) -> Result<(), VeloxGraphError> {
        // INFO: check if both nodes exist, then create connection.
        let _second_node = self.node_get(second_node_id)?;
        let first_node = self.node_get(first_node_id)?;

        first_node.connections_forward().remove(second_node_id);

        let second_node = self.node_get(second_node_id)?;
        second_node.connections_backward.delete(first_node_id);

        Ok(())
    }

    /// Save graph to file.
    ///
    /// # Example
    ///
    /// ```
    /// // INFO: Initialize the graph.
    /// let mut graph: VeloxGraph<
    ///     usize,    // NodeIdT: Size for each node id.
    ///     u32,      // NodeDataT
    ///     f64,      // ConnectionDataT
    /// > = VeloxGraph::new();
    ///
    /// // INFO: Create example nodes.
    /// let node_id0 = graph.node_create(634);
    /// let node_id1 = graph.node_create(43);
    ///
    /// // INFO: Create connection from node0 to node1.
    /// graph.nodes_connection_set(node_id0, node_id1, 5.24).unwrap();
    ///
    /// // INFO: Save the graph.
    /// graph.save("some_file.vg".to_string()).unwrap();
    /// ```
    pub fn save(&self, file_path: String) -> Result<(), VeloxGraphError> {
        let file = File::create(file_path)?;
        let mut file = LineWriter::new(file);

        // INFO: store empty_slots.
        let empty_slots_encoded: Vec<u8> = postcard::to_stdvec(&self.empty_slots)?;
        let empty_slots_encoded_len = empty_slots_encoded.len() as u32;
        let len_encoded = empty_slots_encoded_len.to_le_bytes();
        file.write_all(&len_encoded)?;
        file.write_all(&empty_slots_encoded)?;

        // INFO: store number of node_options.
        let num_node_options = self.nodes_vector.len() as u32;
        let num_node_options_encoded = num_node_options.to_le_bytes();
        file.write_all(&num_node_options_encoded)?;

        // INFO: store each node_option.
        for node_option in &self.nodes_vector {
            let encoded_node_option: Vec<u8> = postcard::to_stdvec(node_option)?;

            let encoded_node_option_len = encoded_node_option.len() as u32;
            let len_encoded = encoded_node_option_len.to_le_bytes();
            file.write_all(&len_encoded)?;
            file.write_all(&encoded_node_option)?;
        }

        Ok(())
    }

    /// Load graph from file.
    ///
    /// # Example
    ///
    /// ```
    /// // INFO: Load the graph.
    /// let mut graph: VeloxGraph<
    ///     usize,    // NodeIdT: Size for each node id.
    ///     u32,      // NodeDataT
    ///     f64,      // ConnectionDataT
    /// > = VeloxGraph::load("some_file.vg".to_string()).unwrap();
    /// println!("num_entries {}", graph.num_entries);
    /// ```
    pub fn load(file_path: String) -> Result<Self, VeloxGraphError> {
        let mut new_graph = Self::new();

        let file = File::open(file_path)?;

        let mut start_byte = 0;

        // INFO: load empty_slots.
        let mut raw_data = [0; 4];
        file.read_at(&mut raw_data, start_byte)?;
        let len = u32::from_le_bytes(raw_data) as usize;
        start_byte += 4;

        let raw_len = len * 4;
        let mut raw_data = vec![0; raw_len];
        file.read_at(&mut raw_data, start_byte)?;
        start_byte += len as u64;

        let (empty_slots, _len): (Vec<usize>, usize) = postcard::from_bytes(&raw_data[..])?;
        new_graph.empty_slots = empty_slots;

        // INFO: load number of node_options.
        let mut raw_data = [0; 4];
        file.read_at(&mut raw_data, start_byte)?;
        let num_node_options = u32::from_le_bytes(raw_data) as usize;
        start_byte += 4;

        // INFO: load each node_option.
        for _index in 0..num_node_options {
            let mut raw_data = [0; 4];
            file.read_at(&mut raw_data, start_byte)?;
            let len = u32::from_le_bytes(raw_data) as usize;
            start_byte += 4;

            let raw_len = len * 4;
            let mut raw_data = vec![0; raw_len];
            file.read_at(&mut raw_data, start_byte)?;
            start_byte += len as u64;

            let (node_option, _len): (
                Option<Node<NodeIdT, ConnForwardT, ConnBackwardT, NodeDataT, ConnectionDataT>>,
                usize,
            ) = postcard::from_bytes(&raw_data[..])?;

            if let Some(_node) = &node_option {
                new_graph.num_entries += 1;
            }

            new_graph.nodes_vector.push(node_option);
        }

        Ok(new_graph)
    }
}
