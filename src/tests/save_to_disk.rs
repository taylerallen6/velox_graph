#![cfg(test)]

use crate::graph::{VeloxGraphHash, VeloxGraphVec};
use crate::ConnectionsForward;

use serde::{Deserialize, Serialize};
use std::usize;

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
