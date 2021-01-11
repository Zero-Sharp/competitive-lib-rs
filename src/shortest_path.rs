use crate::graph::Graph;
use std::ops::Add;
use num::Zero;

pub mod dijkstra;
pub mod bellman_ford;

pub struct ShortestPath<A> {
    graph: A,
    start: usize
}


impl<A,B,U> ShortestPath<A>
    where A: Graph<Item = U, Iterator = B>,
          B: Iterator<Item = (usize,U)>,
          U: Add + Zero + PartialOrd + Copy + PartialEq,
{
    pub fn new(graph: A, start: usize) -> Self {
        ShortestPath {
            graph: graph,
            start: start
        }
    }

    pub fn dijkstra(&self) -> Vec<Option<(U, usize)>> {
        dijkstra::solve(&self.graph,self.start)
    }
}

#[test]
fn test() {
    use crate::graph::GraphVec;
    let mut graph = GraphVec::new(6);
    graph.add_edge(0,1,5);
    graph.add_edge(0,2,4);
    graph.add_edge(0,3,2);
    graph.add_edge(1,2,2);
    graph.add_edge(1,5,6);
    graph.add_edge(2,1,2);
    graph.add_edge(2,3,3);
    graph.add_edge(2,4,2);
    graph.add_edge(3,4,6);
    graph.add_edge(4,5,4);

    graph.add_edge(1,0,5);
    graph.add_edge(2,0,4);
    graph.add_edge(3,0,2);
    graph.add_edge(2,1,2);
    graph.add_edge(5,1,6);
    graph.add_edge(1,2,2);
    graph.add_edge(3,2,3);
    graph.add_edge(4,2,2);
    graph.add_edge(4,3,6);
    graph.add_edge(5,4,4);

    let problem = ShortestPath::new(graph,0);
    let ans: Vec<(usize,usize)> = problem.dijkstra().iter().map(|x| x.unwrap()).collect();

    assert_eq!(ans[0].0,0);
    assert_eq!(ans[1].0,5);
    assert_eq!(ans[2].0,4);
    assert_eq!(ans[3].0,2);
    assert_eq!(ans[4].0,6);
    assert_eq!(ans[5].0,10)
}
