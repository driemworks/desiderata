//!
//! Accept input, configure society, and execute the simulation

use petgraph::dot::{Dot, Config};
use std::collections::HashMap;

mod society;
mod agent;

use crate::society::{Society, SimpleSociety};
use crate::agent::{Agent, SimpleAgent};

/// Execute the simulation over a desired number of iterations
/// 
/// * `society`: A digraph representing the society on which the simulation is run
/// * `num_iterations`: the number of iterations to run the simulation for
/// 
fn run_simulation(_society: SimpleSociety<SimpleAgent>, num_iterations: i32) {
    for _iteration in 0..num_iterations - 1 {
        // println!("START: iteration {:?}", iteration);
        // during each iteration, each node updates her belief
        // through both inquiry and communication
        // println!("END: iteration {:?}", iteration);
    }
}

fn main() {
    println!("--------------");
    println!("* DESIDERATA *");
    println!("--------------");
    // build "society" with initial conditions
    let initial_node_config: Vec<SimpleAgent> = Vec::new();
    let society: SimpleSociety<SimpleAgent> = Society::<SimpleAgent>::new(10, initial_node_config);
    // println!("{:?}", Dot::with_config(&society.network, &[Config::EdgeNoLabel]));
    run_simulation(society, 100);
}
