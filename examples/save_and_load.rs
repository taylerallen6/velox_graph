use velox_graph::graph::VeloxGraphVec;
use velox_graph::ConnectionsForward;
use velox_graph::Graph;

fn main() {
    // INFO: Initialize the graph with data.
    let mut graph: VeloxGraphVec<
        usize, // NodeIdT: Size for each node id.
        u32,   // NodeT
        f64,   // ConnectionT
    > = VeloxGraphVec::new();
    let node_id0 = graph.node_create(634);
    let node_id1 = graph.node_create(43);
    graph
        .nodes_connection_set(node_id0, node_id1, 5.24)
        .unwrap();
    println!("num_entries {}", graph.num_entries());

    // INFO: Save the graph to file of your choice.
    let file_path = "some_file.vg".to_string();
    graph.save(file_path.clone()).unwrap();

    // INFO: Load the graph back from file.
    let mut loaded_graph: VeloxGraphVec<usize, u32, f64> =
        VeloxGraphVec::load(file_path.clone()).unwrap();
    println!("num_entries {}", loaded_graph.num_entries());

    // INFO: Get a mutable reference to that node.
    let node0 = loaded_graph.node_get(node_id0).unwrap();
    println!("node0 data: {:?}", node0.data);
    println!(
        "node0 connections: {:?}",
        node0.connections_forward().data()
    );
}
