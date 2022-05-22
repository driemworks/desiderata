use petgraph::dot::{Dot, Config};
mod society;
use crate::society::{Society, SimpleSociety};

/// Execute the simulation over a desired number of iterations
/// 
/// * `society`: A digraph representing the society on which the simulation is run
/// * `num_iterations`: the number of iterations to run the simulation for
/// 
fn run_simulation(_society: SimpleSociety, num_iterations: i32) {
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
    let society: SimpleSociety = Society::new(10);
    println!("{:?}", Dot::with_config(&society.network, &[Config::EdgeNoLabel]));
    run_simulation(society, 100);
}
