#![cfg(test)]

use crate::graph::{VeloxGraphHash, VeloxGraphVec};
use crate::unsigned_int::UnsignedInt;
use crate::ConnectionsBackward;
use crate::ConnectionsForward;
use crate::Graph;
use crate::{HashConnectionsBackward, VecConnectionsBackward};
use crate::{HashConnectionsForward, VecConnectionsForward};

use rand::seq::SliceRandom;
use std::time::Instant;
use std::time::{self, Duration};
use std::{thread, usize};

const NUM_NODES: usize = 10_000;
const NUM_CONNECTIONS_CREATE: usize = 10_000;
const NUM_CONNECTIONS_PER_NODE: usize = 10;

// INFO: TEST SPEED.
#[test]
fn speed_test_vec_usize() {
    speed_test::<
        usize,
        VecConnectionsForward<usize, u32>,
        VecConnectionsBackward<usize>,
        VeloxGraphVec<
            usize, // NodeIdT
            u32,   // NodeT
            u32,   // ConnectionT
        >,
    >();
}

#[test]
fn speed_test_vec_u16() {
    speed_test::<
        u16,
        VecConnectionsForward<u16, u32>,
        VecConnectionsBackward<u16>,
        VeloxGraphVec<
            u16, // NodeIdT
            u32, // NodeT
            u32, // ConnectionT
        >,
    >();
}

#[test]
fn speed_test_hash_usize() {
    speed_test::<
        usize,
        HashConnectionsForward<usize, u32>,
        HashConnectionsBackward<usize>,
        VeloxGraphHash<
            usize, // NodeIdT
            u32,   // NodeT
            u32,   // ConnectionT
        >,
    >();
}

#[test]
fn speed_test_hash_u16() {
    speed_test::<
        u16,
        HashConnectionsForward<u16, u32>,
        HashConnectionsBackward<u16>,
        VeloxGraphHash<
            u16, // NodeIdT
            u32, // NodeT
            u32, // ConnectionT
        >,
    >();
}

fn speed_test<
    NodeIdT: UnsignedInt,
    ConnForwardT: ConnectionsForward<NodeIdT, u32>,
    ConnBackwardT: ConnectionsBackward<NodeIdT>,
    GraphT: Graph<NodeIdT, ConnForwardT, ConnBackwardT, u32, u32>,
>() {
    let file_path = "./speed_test_save_file.vg".to_string();

    let delay_time = time::Duration::from_millis(1000);

    let mut random_nodes: Vec<usize> = (0..NUM_NODES).collect();
    random_nodes.shuffle(&mut rand::rng());

    let mut graph = GraphT::new();
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

    let mut graph = GraphT::load(file_path.clone()).unwrap();

    let time_elapsed = timestamp.elapsed();
    println!("load time: {:.2?}", time_elapsed);

    println!("num_entries: {}", graph.num_entries());

    delete_nodes_test(&mut graph, NUM_NODES);
}

fn create_nodes_test<
    NodeIdT: UnsignedInt,
    ConnForwardT: ConnectionsForward<NodeIdT, u32>,
    ConnBackwardT: ConnectionsBackward<NodeIdT>,
    GraphT: Graph<NodeIdT, ConnForwardT, ConnBackwardT, u32, u32>,
>(
    graph: &mut GraphT,
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

fn create_connections_test<
    NodeIdT: UnsignedInt,
    ConnForwardT: ConnectionsForward<NodeIdT, u32>,
    ConnBackwardT: ConnectionsBackward<NodeIdT>,
    GraphT: Graph<NodeIdT, ConnForwardT, ConnBackwardT, u32, u32>,
>(
    graph: &mut GraphT,
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

fn delete_nodes_test<
    NodeIdT: UnsignedInt,
    ConnForwardT: ConnectionsForward<NodeIdT, u32>,
    ConnBackwardT: ConnectionsBackward<NodeIdT>,
    GraphT: Graph<NodeIdT, ConnForwardT, ConnBackwardT, u32, u32>,
>(
    graph: &mut GraphT,
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
