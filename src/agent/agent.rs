/// each agent in a society has some:
///     - credence in a proposition at time t
///     - relibability in her neighbors in the society (i.e. connection)
pub trait Agent {

    fn new(id: i32) -> Self;

    /// The inquirer's credence in a proposition at some time
    /// This function must meet the standard axioms of probability measure. 
    /// The output should be in the range [0, 1]
    /// 
    /// Axioms of Probability:
    ///
    /// Axiom 1: For any event $A$, $P(A) \geq 0$.
    ///
    /// Axiom 2: Probability of the sample space $S$ is $P(S)=1$.
    /// 
    /// Axiom 3: If $A_1, A_2, A_3, \cdots$ are disjoint events, then $P(A_1 \cup A_2 \cup A_3 \cdots)=P(A_1)+P(A_2)+P(A_3)+\cdots$
    /// 
    fn credence(proposition: u32, time: u64) -> u64;
    
    fn inquire();

    fn gossip();
}

pub struct SimpleAgent {
    pub id: i32,
}

impl Agent for SimpleAgent {
    fn new(id: i32) -> SimpleAgent {
        SimpleAgent{ id: id }
    }

    fn credence(p: u32, t: u64) -> u64 {
        0
    }

    fn inquire() {

    }

    fn gossip() {
        
    }
}