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
use std::collections::HashMap;
// use crate::agent;

pub trait Society<A> {
    fn new(size: i32, initial_nodes: Vec<A>) -> Self;
}

pub struct SimpleSociety<A> {
    pub network: Graph<A, ()>,
}

impl<A> Society<A> for SimpleSociety<A> {

    fn new(_size: i32, initial_nodes: Vec<A>) -> SimpleSociety<A> {
        // Create an undirected graph with `A` (agent type) nodes and edges with `()` associated data.
        let mut g = Graph::<A, ()>::new();
        for node in initial_nodes {
            g.add_node(node);
        }
        SimpleSociety{ network: g }
    }
}

