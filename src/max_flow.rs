use num::Zero;
use std::ops::{Add,AddAssign,SubAssign,Sub};
use crate::graph::Graph;

mod edmonds_karp;

pub struct MaxFlow<A>(A);

impl<A,B,U> MaxFlow<A>
    where A: Graph<Item = U, Iterator = B>,
          B: Iterator<Item = (usize,U)>,
          U: Copy + Ord + Add + Sub<Output = U> + Zero + AddAssign + SubAssign
{
    pub fn new(graph: A,) -> Self {
        MaxFlow(graph)
    }

    pub fn edmonds_karp(self, s: usize, t: usize) -> (U,A) {
        let flow = edmonds_karp::solve(&self.0,s,t);
        (flow.iter(s).fold(Zero::zero(), |x, (_, y)| x + y), flow)
    }
}

#[test]
fn test1() {
    use crate::graph::GraphVec;
    let mut graph = GraphVec::new(6);
    graph.add_edge(0,1,1);
    graph.add_edge(0,2,12);
    graph.add_edge(1,3,2);
    graph.add_edge(2,1,6);
    graph.add_edge(2,3,5);
    graph.add_edge(2,4,7);
    graph.add_edge(3,4,10);
    graph.add_edge(3,5,3);
    graph.add_edge(4,5,12);
    let problem = MaxFlow::new(graph).edmonds_karp(0,5);
    assert_eq!(problem.0,13)
}

#[test]
fn test2() {
    use crate::graph::GraphVec;
    let mut graph = GraphVec::new(5);
    graph.add_edge(0,1,10);
    graph.add_edge(0,2,2);
    graph.add_edge(1,2,6);
    graph.add_edge(1,3,6);
    graph.add_edge(2,4,5);
    graph.add_edge(3,2,4);
    graph.add_edge(3,4,8);
    let problem = MaxFlow::new(graph).edmonds_karp(0,4);
    assert_eq!(problem.0,11)
}