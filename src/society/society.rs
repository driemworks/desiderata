//! 
//! # Overview
//! 
//! Implementation of a Society
//! 
//! In general, a society is a digraph where each node represents 
//! a member of the society and each edge between nodes
//! represents some social linkage between them.
//! 
//! ## Goal
//! 
//! To build a society of agents (or inquirers) as per the Laputa framework
//!

use petgraph::graph::Graph;

pub struct SimpleSociety {
    pub network: Graph<i32, ()>,
}

pub trait Society {
    fn new(size: i32) -> Self;
}

impl Society for SimpleSociety {

    fn new(_size: i32) -> SimpleSociety {
        // Create an undirected graph with `i32` nodes and edges with `()` associated data.
        let g = Graph::<i32, ()>::from_edges(&[
            (1, 2), 
            (2, 3), 
            (3, 4),
            (1, 4),
        ]);
        SimpleSociety{ network: g }
    }
}

