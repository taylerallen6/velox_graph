#![cfg(test)]

use crate::graph::{VeloxGraphHash, VeloxGraphVec};
use crate::unsigned_int::UnsignedInt;
use crate::ConnectionsBackward;
use crate::ConnectionsForward;
use crate::Graph;
use crate::{HashConnectionsBackward, VecConnectionsBackward};
use crate::{HashConnectionsForward, VecConnectionsForward};

use serde::{Deserialize, Serialize};
use std::usize;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SomeData {
    x: u32,
    y: u32,
}

// INFO: TEST SAVE TO DISK.
#[test]
fn test_save_to_disk_vec_usize() {
    test_save_to_disk::<
        usize,
        VecConnectionsForward<usize, u32>,
        VecConnectionsBackward<usize>,
        VeloxGraphVec<
            usize,    // NodeIdT
            SomeData, // NodeT
            u32,      // ConnectionT
        >,
        usize,
        VecConnectionsForward<usize, u32>,
        VecConnectionsBackward<usize>,
        VeloxGraphVec<
            usize,    // NodeIdT
            SomeData, // NodeT
            u32,      // ConnectionT
        >,
    >();
}

#[test]
fn test_save_to_disk_vec_u16() {
    test_save_to_disk::<
        u16,
        VecConnectionsForward<u16, u32>,
        VecConnectionsBackward<u16>,
        VeloxGraphVec<
            u16,      // NodeIdT
            SomeData, // NodeT
            u32,      // ConnectionT
        >,
        u16,
        VecConnectionsForward<u16, u32>,
        VecConnectionsBackward<u16>,
        VeloxGraphVec<
            u16,      // NodeIdT
            SomeData, // NodeT
            u32,      // ConnectionT
        >,
    >();
}

#[test]
fn test_save_to_disk_vec_change_size() {
    test_save_to_disk::<
        u16,
        VecConnectionsForward<u16, u32>,
        VecConnectionsBackward<u16>,
        VeloxGraphVec<
            u16,      // NodeIdT
            SomeData, // NodeT
            u32,      // ConnectionT
        >,
        u32,
        VecConnectionsForward<u32, u32>,
        VecConnectionsBackward<u32>,
        VeloxGraphVec<
            u32,      // NodeIdT
            SomeData, // NodeT
            u32,      // ConnectionT
        >,
    >();
}

#[test]
fn test_save_to_disk_hash_usize() {
    test_save_to_disk::<
        usize,
        HashConnectionsForward<usize, u32>,
        HashConnectionsBackward<usize>,
        VeloxGraphHash<
            usize,    // NodeIdT
            SomeData, // NodeT
            u32,      // ConnectionT
        >,
        usize,
        HashConnectionsForward<usize, u32>,
        HashConnectionsBackward<usize>,
        VeloxGraphHash<
            usize,    // NodeIdT
            SomeData, // NodeT
            u32,      // ConnectionT
        >,
    >();
}

#[test]
fn test_save_to_disk_hash_u16() {
    test_save_to_disk::<
        u16,
        HashConnectionsForward<u16, u32>,
        HashConnectionsBackward<u16>,
        VeloxGraphHash<
            u16,      // NodeIdT
            SomeData, // NodeT
            u32,      // ConnectionT
        >,
        u16,
        HashConnectionsForward<u16, u32>,
        HashConnectionsBackward<u16>,
        VeloxGraphHash<
            u16,      // NodeIdT
            SomeData, // NodeT
            u32,      // ConnectionT
        >,
    >();
}

#[test]
fn test_save_to_disk_hash_change_size() {
    test_save_to_disk::<
        u16,
        HashConnectionsForward<u16, u32>,
        HashConnectionsBackward<u16>,
        VeloxGraphHash<
            u16,      // NodeIdT
            SomeData, // NodeT
            u32,      // ConnectionT
        >,
        u32,
        HashConnectionsForward<u32, u32>,
        HashConnectionsBackward<u32>,
        VeloxGraphHash<
            u32,      // NodeIdT
            SomeData, // NodeT
            u32,      // ConnectionT
        >,
    >();
}

fn test_save_to_disk<
    NodeIdT1: UnsignedInt,
    ConnForwardT1: ConnectionsForward<NodeIdT1, u32>,
    ConnBackwardT1: ConnectionsBackward<NodeIdT1>,
    GraphT1: Graph<NodeIdT1, ConnForwardT1, ConnBackwardT1, SomeData, u32>,
    NodeIdT2: UnsignedInt,
    ConnForwardT2: ConnectionsForward<NodeIdT2, u32>,
    ConnBackwardT2: ConnectionsBackward<NodeIdT2>,
    GraphT2: Graph<NodeIdT2, ConnForwardT2, ConnBackwardT2, SomeData, u32>,
>() {
    let file_path = "./save_file.vg".to_string();
    let mut graph = GraphT1::new();

    // println!("num_entries: {}", graph.num_entries());
    assert_eq!(graph.num_entries(), 0);
    assert_eq!(graph.empty_slots(), &vec![]);

    // INFO: create nodes.
    let node_id0 = graph.node_create(SomeData { x: 134, y: 351 });
    let node_id1 = graph.node_create(SomeData { x: 4, y: 1 });
    let node_id2 = graph.node_create(SomeData { x: 234, y: 5 });
    let node_id3 = graph.node_create(SomeData { x: 63, y: 42 });
    let node_id4 = graph.node_create(SomeData { x: 35, y: 208 });
    let node_id5 = graph.node_create(SomeData { x: 2643, y: 62 });
    assert_eq!(graph.num_entries(), 6);
    assert_eq!(graph.empty_slots(), &vec![]);

    // INFO: delete one.
    graph.node_delete(node_id3).unwrap();
    assert_eq!(graph.num_entries(), 5);
    assert_eq!(graph.empty_slots(), &vec![3]);

    // INFO: connect some.
    graph.nodes_connection_set(node_id0, node_id2, 545).unwrap();
    graph.nodes_connection_set(node_id0, node_id1, 3).unwrap();
    graph.nodes_connection_set(node_id0, node_id4, 93).unwrap();
    graph.nodes_connection_set(node_id1, node_id0, 355).unwrap();
    graph.nodes_connection_set(node_id1, node_id2, 73).unwrap();
    graph.nodes_connection_set(node_id4, node_id2, 355).unwrap();
    graph.nodes_connection_set(node_id5, node_id0, 73).unwrap();
    graph.nodes_connection_set(node_id5, node_id4, 457).unwrap();

    graph.save(file_path.clone()).unwrap();

    let mut loaded_graph = GraphT2::load(file_path.clone()).unwrap();
    assert_eq!(loaded_graph.num_entries(), 5);
    assert_eq!(loaded_graph.empty_slots(), &vec![3]);

    let node0 = loaded_graph.node_get(node_id0).unwrap();
    assert_eq!(node0.data.x, 134);
    assert_eq!(node0.data.y, 351);
    let node0_forwards = node0.connections_forward().data();
    assert_eq!(node0_forwards.len(), 3);
    // println!("{:?}", node0_forwards);
    assert_eq!(node0_forwards[0].node_id() as usize, node_id2);
    assert_eq!(node0_forwards[0].data, 545);
    assert_eq!(node0_forwards[1].node_id() as usize, node_id1);
    assert_eq!(node0_forwards[1].data, 3);
    assert_eq!(node0_forwards[2].node_id() as usize, node_id4);
    assert_eq!(node0_forwards[2].data, 93);

    let node1 = loaded_graph.node_get(node_id1).unwrap();
    assert_eq!(node1.data.x, 4);
    assert_eq!(node1.data.y, 1);
    let node1_forwards = &node1.connections_forward().data();
    assert_eq!(node1_forwards.len(), 2);
    // println!("{:?}", node1_forwards);
    assert_eq!(node1_forwards[0].node_id() as usize, node_id0);
    assert_eq!(node1_forwards[0].data, 355);
    assert_eq!(node1_forwards[1].node_id() as usize, node_id2);
    assert_eq!(node1_forwards[1].data, 73);

    let node2 = loaded_graph.node_get(node_id2).unwrap();
    assert_eq!(node2.data.x, 234);
    assert_eq!(node2.data.y, 5);
    let node2_forwards = &node2.connections_forward().data();
    assert_eq!(node2_forwards.len(), 0);
    // println!("{:?}", node2_forwards);
}
