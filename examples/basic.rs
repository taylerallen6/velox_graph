use velox_graph::graph::VeloxGraphVec;
use velox_graph::ConnectionsForward;
use velox_graph::Graph;

fn main() {
    // INFO: Initialize the graph.
    let mut graph: VeloxGraphVec<
        usize, // NodeIdT: Size for each node id. Small saves memory. Larger allows more nodes. If you are unsure, stick with usize.
        u32,   // NodeT
        f64,   // ConnectionT
    > = VeloxGraphVec::new();

    // INFO: Create your first nodes.
    let node_id0 = graph.node_create(634);
    let node_id1 = graph.node_create(43);

    // INFO: Create connection from node0 to node1.
    graph
        .nodes_connection_set(node_id0, node_id1, 5.24)
        .unwrap();

    // INFO: Get a mutable reference to that node.
    let node0 = graph.node_get(node_id0).unwrap();

    println!("node0 data: {:?}", node0.data);
    println!(
        "node0 connections: {:?}",
        node0.connections_forward().data()
    );
}
