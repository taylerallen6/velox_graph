#![cfg(test)]

use super::graph::{VeloxGraphHash, VeloxGraphVec};
use super::unsigned_int::UnsignedInt;
use super::ConnectionsBackward;
use super::ConnectionsForward;

use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use std::time::{self, Duration};
use std::{thread, usize};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SomeData {
    x: u32,
    y: u32,
}

// INFO: TEST SAVE TO DISK.
#[test]
fn test_save_to_disk() {
    let file_path = "./save_file.vg".to_string();
    let mut graph: VeloxGraphVec<
        usize,    // NodeIdT
        SomeData, // NodeT
        u32,      // ConnectionT
    > = VeloxGraphVec::new();

    // println!("num_entries: {}", graph.num_entries());
    assert_eq!(graph.num_entries(), 0);
    assert_eq!(graph.empty_slots, vec![]);

    // INFO: create nodes.
    let node_id0 = graph.node_create(SomeData { x: 134, y: 351 });
    let node_id1 = graph.node_create(SomeData { x: 4, y: 1 });
    let node_id2 = graph.node_create(SomeData { x: 234, y: 5 });
    let node_id3 = graph.node_create(SomeData { x: 63, y: 42 });
    let node_id4 = graph.node_create(SomeData { x: 35, y: 208 });
    let node_id5 = graph.node_create(SomeData { x: 2643, y: 62 });
    assert_eq!(graph.num_entries(), 6);
    assert_eq!(graph.empty_slots, vec![]);

    // INFO: delete one.
    graph.node_delete(node_id3).unwrap();
    assert_eq!(graph.num_entries(), 5);
    assert_eq!(graph.empty_slots, vec![3]);

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

    let mut loaded_graph: VeloxGraphVec<
        usize,    // NodeIdT
        SomeData, // NodeT
        u32,      // ConnectionT
    > = VeloxGraphVec::load(file_path.clone()).unwrap();
    assert_eq!(loaded_graph.num_entries(), 5);
    assert_eq!(loaded_graph.empty_slots, vec![3]);

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

// INFO: TEST SAVE TO DISK WITH SIZE u16.
#[test]
fn test_save_to_disk_u16() {
    let file_path = "./save_file.vg".to_string();
    let mut graph: VeloxGraphVec<
        u16,      // NodeIdT
        SomeData, // NodeT
        u32,      // ConnectionT
    > = VeloxGraphVec::new();

    // println!("num_entries: {}", graph.num_entries());
    assert_eq!(graph.num_entries(), 0);
    assert_eq!(graph.empty_slots, vec![]);

    // INFO: create nodes.
    let node_id0 = graph.node_create(SomeData { x: 134, y: 351 });
    let node_id1 = graph.node_create(SomeData { x: 4, y: 1 });
    let node_id2 = graph.node_create(SomeData { x: 234, y: 5 });
    let node_id3 = graph.node_create(SomeData { x: 63, y: 42 });
    let node_id4 = graph.node_create(SomeData { x: 35, y: 208 });
    let node_id5 = graph.node_create(SomeData { x: 2643, y: 62 });
    assert_eq!(graph.num_entries(), 6);
    assert_eq!(graph.empty_slots, vec![]);

    // INFO: delete one.
    graph.node_delete(node_id3).unwrap();
    assert_eq!(graph.num_entries(), 5);
    assert_eq!(graph.empty_slots, vec![3]);

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

    let mut loaded_graph: VeloxGraphVec<
        u16,      // NodeIdT
        SomeData, // NodeT
        u32,      // ConnectionT
    > = VeloxGraphVec::load(file_path.clone()).unwrap();
    assert_eq!(loaded_graph.num_entries(), 5);
    assert_eq!(loaded_graph.empty_slots, vec![3]);

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
    let node1_forwards = node1.connections_forward().data();
    assert_eq!(node1_forwards.len(), 2);
    // println!("{:?}", node1_forwards);
    assert_eq!(node1_forwards[0].node_id() as usize, node_id0);
    assert_eq!(node1_forwards[0].data, 355);
    assert_eq!(node1_forwards[1].node_id() as usize, node_id2);
    assert_eq!(node1_forwards[1].data, 73);

    let node2 = loaded_graph.node_get(node_id2).unwrap();
    assert_eq!(node2.data.x, 234);
    assert_eq!(node2.data.y, 5);
    let node2_forwards = node2.connections_forward().data();
    assert_eq!(node2_forwards.len(), 0);
    // println!("{:?}", node2_forwards);
}

// INFO: TEST SAVE TO DISK WITH CHANGING SIZES.
#[test]
fn test_save_to_disk_change_size() {
    let file_path = "./save_file.vg".to_string();
    let mut graph: VeloxGraphVec<
        u16,      // NodeIdT
        SomeData, // NodeT
        u32,      // ConnectionT
    > = VeloxGraphVec::new();

    // println!("num_entries: {}", graph.num_entries());
    assert_eq!(graph.num_entries(), 0);
    assert_eq!(graph.empty_slots, vec![]);

    // INFO: create nodes.
    let node_id0 = graph.node_create(SomeData { x: 134, y: 351 });
    let node_id1 = graph.node_create(SomeData { x: 4, y: 1 });
    let node_id2 = graph.node_create(SomeData { x: 234, y: 5 });
    let node_id3 = graph.node_create(SomeData { x: 63, y: 42 });
    let node_id4 = graph.node_create(SomeData { x: 35, y: 208 });
    let node_id5 = graph.node_create(SomeData { x: 2643, y: 62 });
    assert_eq!(graph.num_entries(), 6);
    assert_eq!(graph.empty_slots, vec![]);

    // INFO: delete one.
    graph.node_delete(node_id3).unwrap();
    assert_eq!(graph.num_entries(), 5);
    assert_eq!(graph.empty_slots, vec![3]);

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

    let mut loaded_graph: VeloxGraphVec<
        u32,      // NodeIdT
        SomeData, // NodeT
        u32,      // ConnectionT
    > = VeloxGraphVec::load(file_path.clone()).unwrap();
    assert_eq!(loaded_graph.num_entries(), 5);
    assert_eq!(loaded_graph.empty_slots, vec![3]);

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
    let node1_forwards = node1.connections_forward().data();
    assert_eq!(node1_forwards.len(), 2);
    // println!("{:?}", node1_forwards);
    assert_eq!(node1_forwards[0].node_id() as usize, node_id0);
    assert_eq!(node1_forwards[0].data, 355);
    assert_eq!(node1_forwards[1].node_id() as usize, node_id2);
    assert_eq!(node1_forwards[1].data, 73);

    let node2 = loaded_graph.node_get(node_id2).unwrap();
    assert_eq!(node2.data.x, 234);
    assert_eq!(node2.data.y, 5);
    let node2_forwards = node2.connections_forward().data();
    assert_eq!(node2_forwards.len(), 0);
    // println!("{:?}", node2_forwards);
}

// INFO: TEST BASIC FUNCTIONALITY.
#[test]
fn test_basic_functions() {
    let mut graph: VeloxGraphVec<
        usize,    // NodeIdT
        SomeData, // NodeT
        u32,      // ConnectionT
    > = VeloxGraphVec::new();

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
    assert_eq!(backwards.data()[0].node_id, 0);

    let node3 = graph.node_get(3).unwrap();
    let backwards = node3.connections_backward();
    // println!("forwards: {:?}", forwards);
    assert_eq!(backwards.data().len(), 1);
    assert_eq!(backwards.data()[0].node_id, 0);

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

// INFO: TEST BASIC FUNCTIONALITY.
#[test]
fn test_basic_functions_u16() {
    let mut graph: VeloxGraphVec<
        u16,      // NodeIdT
        SomeData, // NodeT
        u32,      // ConnectionT
    > = VeloxGraphVec::new();

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
    assert_eq!(backwards.data()[0].node_id, 0);

    let node3 = graph.node_get(3).unwrap();
    let backwards = node3.connections_backward();
    // println!("forwards: {:?}", forwards);
    assert_eq!(backwards.data().len(), 1);
    assert_eq!(backwards.data()[0].node_id, 0);

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

const NUM_NODES: usize = 10_000;
const NUM_CONNECTIONS_CREATE: usize = 10_000;
const NUM_CONNECTIONS_PER_NODE: usize = 10;

#[test]
fn speed_test() {
    let file_path = "./speed_test_save_file.vg".to_string();

    let delay_time = time::Duration::from_millis(1000);

    let mut random_nodes: Vec<usize> = (0..NUM_NODES).collect();
    random_nodes.shuffle(&mut rand::rng());

    let mut graph: VeloxGraphHash<
        usize, // NodeIdT
        u32,   // NodeT
        u32,   // ConnectionT
    > = VeloxGraphHash::new();
    thread::sleep(delay_time);

    create_nodes_test(&mut graph, NUM_NODES);

    thread::sleep(delay_time);

    let random_nodes_to_connect: Vec<usize> = (0..NUM_NODES).collect();

    create_connections_test(
        &mut graph,
        NUM_CONNECTIONS_CREATE,
        random_nodes.clone(),
        random_nodes_to_connect,
    );

    thread::sleep(delay_time);

    let timestamp = Instant::now();

    graph.save(file_path.clone()).unwrap();

    let time_elapsed = timestamp.elapsed();
    println!("save time: {:.2?}", time_elapsed);
    let timestamp = Instant::now();

    let mut graph: VeloxGraphHash<
        usize, // NodeIdT
        u32,   // NodeT
        u32,   // ConnectionT
    > = VeloxGraphHash::load(file_path.clone()).unwrap();

    let time_elapsed = timestamp.elapsed();
    println!("load time: {:.2?}", time_elapsed);

    println!("num_entries: {}", graph.num_entries());

    delete_nodes_test(&mut graph, NUM_NODES);
}

#[test]
fn speed_test_u16() {
    let file_path = "./speed_test_save_file.vg".to_string();

    let delay_time = time::Duration::from_millis(1000);

    let mut random_nodes: Vec<usize> = (0..NUM_NODES).collect();
    random_nodes.shuffle(&mut rand::rng());

    let mut graph: VeloxGraphHash<
        u16, // NodeIdT
        u32, // NodeT
        u32, // ConnectionT
    > = VeloxGraphHash::new();
    thread::sleep(delay_time);

    create_nodes_test(&mut graph, NUM_NODES);

    thread::sleep(delay_time);

    let random_nodes_to_connect: Vec<usize> = (0..NUM_NODES).collect();

    create_connections_test(
        &mut graph,
        NUM_CONNECTIONS_CREATE,
        random_nodes.clone(),
        random_nodes_to_connect,
    );

    thread::sleep(delay_time);

    let timestamp = Instant::now();

    graph.save(file_path.clone()).unwrap();

    let time_elapsed = timestamp.elapsed();
    println!("save time: {:.2?}", time_elapsed);
    let timestamp = Instant::now();

    let mut graph: VeloxGraphHash<
        u16, // NodeIdT
        u32, // NodeT
        u32, // ConnectionT
    > = VeloxGraphHash::load(file_path.clone()).unwrap();

    let time_elapsed = timestamp.elapsed();
    println!("load time: {:.2?}", time_elapsed);

    println!("num_entries: {}", graph.num_entries());

    delete_nodes_test(&mut graph, NUM_NODES);
}

fn create_nodes_test<NodeIdT: UnsignedInt>(
    graph: &mut VeloxGraphHash<
        NodeIdT, // NodeIdT
        u32,     // NodeT
        u32,     // ConnectionT
    >,
    num_nodes: usize,
) {
    let mut create_times = Vec::new();
    let mut timestamp: Instant;
    let mut time_elapsed: Duration;

    for i in 0..num_nodes {
        timestamp = Instant::now();
        graph.node_create(i as u32);
        time_elapsed = timestamp.elapsed();
        create_times.push(time_elapsed);
    }

    // INFO: end time test and display time elapsed.
    let mean_time = create_times.iter().sum::<Duration>() / create_times.len() as u32;
    println!("create: mean time per db operation: {:.2?}", mean_time);
    let max_time = create_times.iter().max().unwrap();
    println!("create: max time per db operation: {:.2?}", max_time);
    let min_time = create_times.iter().min().unwrap();
    println!("create: min time per db operation: {:.2?}", min_time);
    println!();
}

fn create_connections_test<NodeIdT: UnsignedInt>(
    graph: &mut VeloxGraphHash<
        NodeIdT, // NodeIdT
        u32,     // NodeT
        u32,     // ConnectionT
    >,
    num_connections_to_create: usize,
    random_nodes: Vec<usize>,
    mut random_nodes_to_connect: Vec<usize>,
) {
    let random_nodes = &random_nodes[..num_connections_to_create];

    let mut create_connections_times = Vec::new();
    let mut timestamp: Instant;
    let mut time_elapsed: Duration;

    for (i, &entry_index) in random_nodes.iter().enumerate() {
        random_nodes_to_connect.shuffle(&mut rand::rng());
        let random_nodes_to_connect = &random_nodes_to_connect[..NUM_CONNECTIONS_PER_NODE];

        timestamp = Instant::now();

        for &second_node in random_nodes_to_connect {
            graph
                .nodes_connection_set(entry_index, second_node, i as u32)
                .unwrap();
        }

        time_elapsed = timestamp.elapsed();
        create_connections_times.push(time_elapsed);
    }

    // INFO: end time test and display time elapsed.
    let time_per_operation =
        create_connections_times.iter().sum::<Duration>() / create_connections_times.len() as u32;
    println!(
        "create_connections: mean time per db operation: {:.2?}",
        time_per_operation
    );
    let max_time = create_connections_times.iter().max().unwrap();
    println!(
        "create_connections: max time per db operation: {:.2?}",
        max_time
    );
    let min_time = create_connections_times.iter().min().unwrap();
    println!(
        "create_connections: min time per db operation: {:.2?}",
        min_time
    );
    println!();
}

fn delete_nodes_test<NodeIdT: UnsignedInt>(
    graph: &mut VeloxGraphHash<
        NodeIdT, // NodeIdT
        u32,     // NodeT
        u32,     // ConnectionT
    >,
    num_nodes: usize,
) {
    let mut delete_times = Vec::new();
    let mut timestamp: Instant;
    let mut time_elapsed: Duration;

    for i in 0..num_nodes {
        timestamp = Instant::now();
        graph.node_delete(i).unwrap();
        time_elapsed = timestamp.elapsed();
        delete_times.push(time_elapsed);
    }

    // INFO: end time test and display time elapsed.
    let mean_time = delete_times.iter().sum::<Duration>() / delete_times.len() as u32;
    println!("delete: mean time per db operation: {:.2?}", mean_time);
    let max_time = delete_times.iter().max().unwrap();
    println!("delete: max time per db operation: {:.2?}", max_time);
    let min_time = delete_times.iter().min().unwrap();
    println!("delete: min time per db operation: {:.2?}", min_time);
    println!();
}
