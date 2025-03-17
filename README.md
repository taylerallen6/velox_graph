# velox_graph

Simple overview of use/purpose.

## Description

VeloxGraph is a high-performance embedded in-memory graph database designed for a new type of AI models and custom neural networks. Optimized for real-time graph traversal and relationship mapping, it enables ultra-fast data access for AI inference, knowledge representation, and dynamic learning models. With a lightweight footprint and seamless integration, VeloxGraph accelerates graph-based computations in edge devices, robotics, and next-gen AI systems.

## Getting Started

### Example
```rust
use velox_graph::VeloxGraph;

// INFO: Sample data to store in the nodes.
#[derive(Clone, Debug)]
struct NodeData {
    x: u32,
    y: u32,
}

// INFO: Sample data to store in the connections.
#[derive(Clone, Debug)]
struct ConnData {
    a: u32,
    b: f64,
}

fn main() {
    // INFO: Initialize the graph.
    let mut graph: VeloxGraph<NodeData, ConnData> = VeloxGraph::new();

    // INFO: Create your first node.
    let node_id0 = graph.node_create(NodeData { x: 134, y: 351 });
    println!("num_entries: {}", graph.num_entries);

    // INFO: Get a mutable reference to that node.
    let node = graph.node_get(node_id0).unwrap();
    println!("node data: {:?}", node.data);

    // INFO: You can then edit that node in place. Remember this a mutable reference, no need to save.
    node.data.x += 4;
    node.data.y = 2431;

    // INFO: You can get the node again if you want to verify that it was edited.
    let node = graph.node_get(node_id0).unwrap();
    println!("node data: {:?}", node.data);

    // INFO: Create 2 more nodes.
    let node_id1 = graph.node_create(NodeData { x: 234, y: 5 });
    let node_id2 = graph.node_create(NodeData { x: 63, y: 42 });
    println!("num_entries: {}", graph.num_entries);

    // INFO: Create connections some connections between nodes.
    graph
        .nodes_connection_create(node_id0, node_id1, ConnData { a: 243, b: 54.5 })
        .unwrap();
    graph
        .nodes_connection_create(node_id0, node_id2, ConnData { a: 84, b: 9.413 })
        .unwrap();
    graph
        .nodes_connection_create(node_id1, node_id2, ConnData { a: 84, b: 9.413 })
        .unwrap();
    graph
        .nodes_connection_create(node_id2, node_id0, ConnData { a: 84, b: 9.413 })
        .unwrap();

    // INFO: Loop through each connection that this node connects forward to (forward connections). You can NOT edit the connections.
    let node = graph.node_get(node_id0).unwrap();
    for connection in node.connections_forward_get().values() {
        println!("forward_connection: {:?}", connection);
    }

    // INFO: You can also see the what nodes the TO this node (backward connections). You can NOT edit the connections.
    let node2 = graph.node_get(node_id2).unwrap();
    for connection in node2.connections_backward_get() {
        println!("backward_connection: {:?}", connection);
    }

    // INFO: Delete node connections.
    graph.nodes_connection_delete(node_id0, node_id1).unwrap();
    graph.nodes_connection_delete(node_id0, node_id2).unwrap();

    // INFO: Delete nodes. Their connections are automatically deleted as well.
    graph.node_delete(0).unwrap();
    graph.node_delete(1).unwrap();
    graph.node_delete(2).unwrap();
    println!("num_entries: {}", graph.num_entries);
}
```
