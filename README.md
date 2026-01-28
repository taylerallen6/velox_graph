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
velox_graph = "5.0.0"
```

### Basic Code Example
```rust
use velox_graph::VeloxGraph;

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
```

### Save and Load Example
```rust
use velox_graph::VeloxGraph;

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
```

### More Examples

There are a few more in-depth examples [here](https://github.com/taylerallen6/velox_graph/tree/main/examples) that I encourage you to look through.

## Status

> ⚠️ This project is in early development.  
> Expect frequent changes as components mature.

## License

Licensed under:

- [Apache License, Version 2.0](LICENSE)

## Contact

Created by **Tayler Allen**  
For questions or collaboration, open an issue or discussion on GitHub.
