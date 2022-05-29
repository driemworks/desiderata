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
// use crate::agent;

pub trait Society {
    fn new(size: i32) -> Self;
}

pub struct SimpleSociety<A> {
    pub network: Option<Graph<A, ()>>,
}

impl<A> Society for SimpleSociety<A> {

    fn new(_size: i32) -> SimpleSociety<A> {
        // Create an undirected graph with `A` (agent type) nodes and edges with `()` associated data.
        // let g = Graph::<A, ()>::from_edges(&[
        //     // (1, 2), 
        //     // (2, 3), 
        //     // (3, 4),
        //     // (1, 4),
        // ]);
        SimpleSociety{ network: None }
    }
}

