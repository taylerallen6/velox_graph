#![cfg(test)]

use crate::graph::{VeloxGraph, VeloxGraphHash, VeloxGraphVec};
use crate::unsigned_int::UnsignedInt;
use crate::ConnectionsBackward;
use crate::ConnectionsForward;

use serde::{Deserialize, Serialize};
use std::usize;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SomeData {
    x: u32,
    y: u32,
}

// INFO: TEST BASIC FUNCTIONALITY.
#[test]
fn test_basic_functions_usize_vec() {
    let graph: VeloxGraphVec<
        usize,    // NodeIdT
        SomeData, // NodeT
        u32,      // ConnectionT
    > = VeloxGraphVec::new();

    test_basic_functions(graph);
}

#[test]
fn test_basic_functions_u16_vec() {
    let graph: VeloxGraphVec<
        u16,      // NodeIdT
        SomeData, // NodeT
        u32,      // ConnectionT
    > = VeloxGraphVec::new();

    test_basic_functions(graph);
}

#[test]
fn test_basic_functions_usize_hash() {
    let graph: VeloxGraphHash<
        usize,    // NodeIdT
        SomeData, // NodeT
        u32,      // ConnectionT
    > = VeloxGraphHash::new();

    test_basic_functions(graph);
}

#[test]
fn test_basic_functions_u16_hash() {
    let graph: VeloxGraphHash<
        u16,      // NodeIdT
        SomeData, // NodeT
        u32,      // ConnectionT
    > = VeloxGraphHash::new();

    test_basic_functions(graph);
}

fn test_basic_functions<
    NodeIdT: UnsignedInt,
    ConnForwardT: ConnectionsForward<NodeIdT, u32>,
    ConnBackwardT: ConnectionsBackward<NodeIdT>,
>(
    mut graph: VeloxGraph<NodeIdT, ConnForwardT, ConnBackwardT, SomeData, u32>,
) {
    // println!("num_entries: {}", graph.num_entries());
    assert_eq!(graph.num_entries(), 0);
    assert_eq!(graph.empty_slots, vec![]);

    let node_id = graph.node_create(SomeData { x: 134, y: 351 });
    assert_eq!(node_id, 0);
    // println!("num_entries: {}", graph.num_entries());
    assert_eq!(graph.num_entries(), 1);
    assert_eq!(graph.empty_slots, vec![]);

    let node = graph.node_get(node_id).unwrap();
    // println!("node data: {:?}", node.data);
    assert_eq!(node.data.x, 134);
    assert_eq!(node.data.y, 351);

    node.data.x += 4;
    node.data.y = 2431;

    let node = graph.node_get(node_id).unwrap();
    // println!("node data: {:?}", node.data());
    assert_eq!(node.data.x, 138);
    assert_eq!(node.data.y, 2431);

    let node_id2 = graph.node_create(SomeData { x: 234, y: 5 });
    assert_eq!(node_id2, 1);
    let node_id3 = graph.node_create(SomeData { x: 63, y: 42 });
    assert_eq!(node_id3, 2);
    let node_id4 = graph.node_create(SomeData { x: 2, y: 51 });
    assert_eq!(node_id4, 3);
    let node_id4 = graph.node_create(SomeData { x: 35, y: 208 });
    assert_eq!(node_id4, 4);

    assert_eq!(graph.num_entries(), 5);
    assert_eq!(graph.nodes_vector.len(), 5);
    assert_eq!(graph.empty_slots.len(), 0);

    let node = graph.node_get(node_id).unwrap();
    let forwards = node.connections_forward();
    // println!("forwards: {:?}", forwards);
    assert_eq!(forwards.data().len(), 0);

    graph.nodes_connection_set(node_id, 2, 53245).unwrap();
    graph.nodes_connection_set(node_id, 3, 24323).unwrap();

    let node = graph.node_get(node_id).unwrap();
    let forwards = node.connections_forward();
    assert_eq!(forwards.data().len(), 2);
    let conn_forward2 = forwards.get(2).unwrap();
    assert_eq!(conn_forward2.node_id(), 2);
    assert_eq!(*conn_forward2.data(), 53245);
    let conn_forward3 = forwards.get(3).unwrap();
    assert_eq!(conn_forward3.node_id(), 3);
    assert_eq!(*conn_forward3.data(), 24323);

    // INFO: START: test setting connection twice
    let temp_node_id = node.node_id().clone();
    graph
        .nodes_connection_set(temp_node_id as usize, 3, 6666)
        .unwrap();

    let node = graph.node_get(node_id).unwrap();
    let forwards = node.connections_forward();
    assert_eq!(forwards.data().len(), 2);
    let conn_forward3 = forwards.get(3).unwrap();
    assert_eq!(conn_forward3.node_id(), 3);
    assert_eq!(*conn_forward3.data(), 6666);
    // INFO: END: test setting connection twice

    let node2 = graph.node_get(2).unwrap();
    let backwards = node2.connections_backward();
    // println!("forwards: {:?}", forwards);
    assert_eq!(backwards.data().len(), 1);
    assert_eq!(backwards.data()[0].node_id, UnsignedInt::from_usize(0));

    let node3 = graph.node_get(3).unwrap();
    let backwards = node3.connections_backward();
    // println!("forwards: {:?}", forwards);
    assert_eq!(backwards.data().len(), 1);
    assert_eq!(backwards.data()[0].node_id, UnsignedInt::from_usize(0));

    graph.nodes_connection_remove(0, 4).unwrap();
    let node0 = graph.node_get(0).unwrap();
    let forwards = node0.connections_forward();
    // println!("forwards: {:?}", forwards);
    assert_eq!(forwards.data().len(), 2);

    graph.nodes_connection_remove(0, 2).unwrap();
    let node0 = graph.node_get(0).unwrap();
    let forwards = node0.connections_forward();
    // println!("forwards: {:?}", forwards);
    assert_eq!(forwards.data().len(), 1);

    graph.nodes_connection_set(2, 0, 352).unwrap();
    let node2 = graph.node_get(2).unwrap();
    let forwards = node2.connections_forward();
    assert_eq!(forwards.data().len(), 1);
    assert_eq!(*forwards.get(0).unwrap().data(), 352);

    graph.nodes_connection_remove(2, 0).unwrap();
    let node2 = graph.node_get(2).unwrap();
    let forwards = node2.connections_forward();
    assert_eq!(forwards.data().len(), 0);

    graph.node_delete(4).unwrap();
    graph.node_delete(2).unwrap();
    assert_eq!(graph.num_entries(), 3);
    assert_eq!(graph.nodes_vector.len(), 4);
    assert_eq!(graph.empty_slots.len(), 1);
    assert_eq!(graph.empty_slots[0], 2);

    let node = graph.node_get(node_id).unwrap();
    let forwards = node.connections_forward();
    // println!("forwards: {:?}", forwards);
    assert_eq!(forwards.data().len(), 1);
    assert_eq!(forwards.get(3).unwrap().node_id(), 3);
    assert_eq!(*forwards.get(3).unwrap().data(), 6666);

    graph.node_delete(0).unwrap();
    assert_eq!(graph.num_entries(), 2);
    assert_eq!(graph.nodes_vector.len(), 4);
    assert_eq!(graph.empty_slots.len(), 2);
    assert_eq!(graph.empty_slots[1], 0);

    let node3 = graph.node_get(3).unwrap();
    let backwards = node3.connections_backward();
    // println!("forwards: {:?}", forwards);
    assert_eq!(backwards.data().len(), 0);

    let node_id5 = graph.node_create(SomeData { x: 63452, y: 846 });
    assert_eq!(node_id5, 0);

    assert_eq!(graph.num_entries(), 3);
    assert_eq!(graph.nodes_vector.len(), 4);
    assert_eq!(graph.empty_slots.len(), 1);
}
