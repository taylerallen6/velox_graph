# VeloxGraph

[![Crates.io](https://img.shields.io/crates/v/velox_graph.svg)](https://crates.io/crates/velox_graph)
[![Apache2.0 licensed](https://img.shields.io/badge/license-Apache2.0-blue.svg)](https://github.com/taylerallen6/velox_graph/blob/main/LICENSE)
[![Documentation](https://docs.rs/velox_graph/badge.svg)](https://docs.rs/velox_graph)

VeloxGraph is an extremely fast, efficient, low-level, in-memory, minimal graph database (wow, that is a mouth full). It is not revolutionary in its design but has a few key features that make it vital to the development of a new type of neural network architecture that I am working on.

## VeloxGraph vs traditional matrix representations:

Much of the neural networks are designed around parallel processing. This is great for certain tasks, but for tasks that handle very sparse data, this can be a waste of processing power. Particularly, I am referring to sparsely connected networks, in which nodes may only have 10s or 100s of connections. For these calculations, even using a gpu will just slow it down. 

I primarily wrote this code for use in a new type of neural network that I am working on that heavily relies on these types of connections. I focused on allowing very fast traversal of immediate connections, both forward (for triggering the next nodes) and backward (for looking up previous connections when deleting a node). These are some of the primary features that make this so useful to me.

Matrix representations, on the other hand, are highly efficient (and the preferred option) when handling densely connected networks. The first two layers in my neural network are still very densely connected and, therefore, use matrices to represent the nodes and connections. But every layer after that use this VeloxGraph database for representing the mostly sparse connections.

## Getting Started

### Install

Add this to your Cargo.toml file in your rust project:
```toml
[dependencies]
velox_graph = "4.0.0"
```

### Basic Code Example
```rust
use velox_graph::VeloxGraph;

fn main() {
    // INFO: Initialize the graph.
    let mut graph: VeloxGraph<
        usize,    // NodeIdT: Size for each node id. Small saves memory. Larger allows more nodes.
        u32,      // NodeT
        f64,      // ConnectionT
    > = VeloxGraph::new();

    // INFO: Create your first nodes.
    let node_id0 = graph.node_create(634);
    let node_id1 = graph.node_create(43);

    // INFO: Create connection from node0 to node1.
    graph.nodes_connection_set(node_id0, node_id1, 5.24).unwrap();

    // INFO: Get a mutable reference to that node.
    let node0 = graph.node_get(node_id0).unwrap();

    println!("node0 data: {:?}", node0.data);
    println!("node0 connections: {:?}", &node0.connections_forward_get_all().data_vec);
}
```

### Save and Load Example
```rust
use velox_graph::VeloxGraph;

fn main() {
    // INFO: Initialize the graph with data.
    let mut graph: VeloxGraph<
        usize,    // NodeIdT: Size for each node id.
        u32,      // NodeT
        f64,      // ConnectionT
    > = VeloxGraph::new();
    let node_id0 = graph.node_create(634);
    let node_id1 = graph.node_create(43);
    graph.nodes_connection_set(node_id0, node_id1, 5.24).unwrap();
    println!("num_entries {}", graph.num_entries);

    // INFO: Save the graph to file of your choice.
    let file_path = "some_file.vg".to_string();
    graph.save(file_path).unwrap();
    
    // INFO: Load the graph back from file.
    let mut loaded_graph: VeloxGraph<usize, u32, f64> = VeloxGraph::load(file_path).unwrap();
    println!("num_entries {}", loaded_graph.num_entries);

    // INFO: Get a mutable reference to that node.
    let node0 = loaded_graph.node_get(node_id0).unwrap();
    println!("node0 data: {:?}", node0.data);
    println!("node0 connections: {:?}", &node0.connections_forward_get_all().data_vec);
}
```

### More Complex Code Example
```rust
use velox_graph::VeloxGraph;
use serde::{Deserialize, Serialize};

// INFO: Sample data to store in the nodes.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct NodeData {
    x: u32,
    y: u32,
}

// INFO: Sample data to store in the connections.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct ConnData {
    a: u32,
    b: f64,
}

fn main() {
    // INFO: Initialize the graph.
    let mut graph: VeloxGraph<
        usize,      // NodeIdT: Size for each node id.
        NodeData,   // NodeT
        ConnData,   // ConnectionT
    > = VeloxGraph::new();

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
        .nodes_connection_set(node_id0, node_id1, ConnData { a: 243, b: 54.5 })
        .unwrap();
    graph
        .nodes_connection_set(node_id0, node_id2, ConnData { a: 63, b: 9.413 })
        .unwrap();
    graph
        .nodes_connection_set(node_id1, node_id2, ConnData { a: 2834, b: 5.24 })
        .unwrap();
    graph
        .nodes_connection_set(node_id2, node_id0, ConnData { a: 7, b: 463.62 })
        .unwrap();

    // INFO: Loop through each connection that this node connects forward to (forward connections). You can NOT edit the connections.
    let node = graph.node_get(node_id0).unwrap();
    for connection in &node.connections_forward_get_all().data_vec {
        println!("forward_connection: {:?}", connection);
    }

    // INFO: You can also see the what nodes the TO this node (backward connections). You can NOT edit the connections.
    let node2 = graph.node_get(node_id2).unwrap();
    for connection in node2.connections_backward_get_all() {
        println!("backward_connection: {:?}", connection);
    }

    // INFO: Delete node connections.
    graph.nodes_connection_remove(node_id0, node_id1).unwrap();
    graph.nodes_connection_remove(node_id0, node_id2).unwrap();

    // INFO: Delete nodes. Their connections are automatically deleted as well.
    graph.node_delete(0).unwrap();
    graph.node_delete(1).unwrap();
    graph.node_delete(2).unwrap();
    println!("num_entries: {}", graph.num_entries);
}
```
