use crate::graph::Graph;
use std::fmt::Debug;
use std::ops::{Add,AddAssign,Sub,SubAssign};
use num::{Signed,Zero};

mod successive_shortest_path;

pub struct MinCostFlow<A,U> {
    graph: A,
    b: Vec<U>
}

impl<A,B,U> MinCostFlow<A,U>
    where  A: Graph<Item = (U,U), Iterator = B> + Clone,
           B: Iterator<Item = (usize,(U,U))>,
           U: Debug + PartialOrd + Copy + Zero + Add + AddAssign + Sub + SubAssign + Signed,
{
    pub fn new(graph: A, b: Vec<U>) -> Self {
        MinCostFlow {
            graph: graph,
            b: b,
        }
    }

    pub fn min_cost_flow<C,D>(&self) -> Option<(U,C)>
        where C: Graph<Item = U, Iterator = D>,
              D: Iterator<Item = (usize,U)>,
    {
        let xx: Option<C> = successive_shortest_path::solve(&self.graph,&self.b);
        match xx {
            None => None,
            Some(flow) => {
                let mut ans = U::zero();
                for from in 0..self.graph.len() {
                    for (to,(_,cost)) in self.graph.iter(from) {
                        ans += cost*flow.get(from,to).unwrap();
                    }
                }
                Some((ans,flow))
            }
        }
    }
}

pub struct MinCostFlowST<A,U> {
    graph: A,
    s: usize,
    t: usize,
    f: U
}

impl<A,B,U> MinCostFlowST<A,U>
    where  A: Graph<Item = (U,U), Iterator = B>,
           B: Iterator<Item = (usize,(U,U))>,
           U: Debug + PartialOrd + Copy + Zero + Add + AddAssign + Sub + SubAssign + Signed,
{
    pub fn new(graph: A, source: usize, sink: usize, amount: U) -> Self {
        MinCostFlowST {
            graph: graph,
            s: source,
            t: sink,
            f: amount
        }
    }

    pub fn min_cost_flow<C,D>(&self) -> Option<(U,C)>
        where C: Graph<Item = U, Iterator = D>,
              D: Iterator<Item = (usize,U)>,
    {
        let xx: Option<C> = successive_shortest_path::solve_st(&self.graph,self.s,self.t,self.f);
        match xx {
            None => None,
            Some(flow) => {
                let mut ans = U::zero();
                for from in 0..self.graph.len() {
                    for (to,(_,cost)) in self.graph.iter(from) {
                        ans += cost*flow.get(from,to).unwrap();
                    }
                }
                Some((ans,flow))
            }
        }
    }
}


#[test]
fn test1() {
    use crate::graph::GraphVec;
    let mut graph = GraphVec::new(5);
    graph.add_edge(0,1,(10,2));
    graph.add_edge(0,2,(2,4));
    graph.add_edge(1,2,(6,6));
    graph.add_edge(1,3,(6,2));
    graph.add_edge(3,2,(3,3));
    graph.add_edge(2,4,(5,2));
    graph.add_edge(3,4,(8,6));
    let problem: Option<(i64,GraphVec<i64>)> = MinCostFlowST::new(graph,0,4,9).min_cost_flow();
    match problem {
        None => unreachable!(),
        Some(xx) => {
            assert_eq!(80,xx.0);
        }
    }
}

#[test]
fn test2() {
    use crate::graph::GraphVec;
    let mut graph = GraphVec::new(6);
    graph.add_edge(0,1,(3,2));
    graph.add_edge(0,2,(2,1));
    graph.add_edge(1,2,(2,2));
    graph.add_edge(1,3,(3,4));
    graph.add_edge(2,3,(5,1));
    graph.add_edge(2,4,(6,2));
    graph.add_edge(3,4,(2,2));
    graph.add_edge(3,5,(6,3));
    graph.add_edge(4,5,(10,2));
    let problem: Option<(i64,GraphVec<i64>)> = MinCostFlowST::new(graph,0,5,6).min_cost_flow();
    assert!(problem.is_none())
}

#[test]
fn test3() {
    use crate::graph::GraphVec;
    let mut graph = GraphVec::new(5);
    graph.add_edge(0,1,(10,2));
    graph.add_edge(0,2,(2,4));
    graph.add_edge(1,2,(6,6));
    graph.add_edge(1,3,(6,2));
    graph.add_edge(3,2,(3,3));
    graph.add_edge(2,4,(5,2));
    graph.add_edge(3,4,(8,6));
    let b = vec![9,0,0,0,-9];
    let problem: Option<(i64,GraphVec<i64>)> = MinCostFlow::new(graph,b).min_cost_flow();
    match problem {
        None => unreachable!(),
        Some(xx) => {
            assert_eq!(80,xx.0);
        }
    }
}

/*
#[test]
fn test3() {
    use crate::graph::GraphVec;
    let mut graph = GraphVec::new(10);
    graph.add_edge(0,3,(9,12));
    graph.add_edge(0,4,(15,0));
    graph.add_edge(0,6,(9,12));
    graph.add_edge(0,8,(12,16));
    graph.add_edge(0,9,(19,5));
    graph.add_edge(3,1,(10,16));
    graph.add_edge(3,5,(8,13));
    graph.add_edge(4,5,(9,10));
    graph.add_edge(4,9,(11,7));
    graph.add_edge(5,1,(10,4));
    graph.add_edge(5,9,(7,19));
    graph.add_edge(6,1,(9,17));
    graph.add_edge(6,2,(12,13));
    graph.add_edge(6,9,(11,11));
    graph.add_edge(7,6,(3,14));
    let problem: Option<(i64,GraphVec<i64>)> = MinCostFlowST::new(graph,0,9,20).min_cost_flow();
    match problem {
        None => unreachable!(),
        Some(xx) => {
            assert_eq!(102,xx.0);
        }
    }
}
*/