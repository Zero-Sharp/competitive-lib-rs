mod edmonds_karp;

#[test]
fn test1() {
    use crate::graph::*;
    let mut graph = MatGraph::new(6);
    graph.add_edge(0,1,1);
    graph.add_edge(0,2,12);
    graph.add_edge(1,3,2);
    graph.add_edge(2,1,6);
    graph.add_edge(2,3,5);
    graph.add_edge(2,4,7);
    graph.add_edge(3,4,10);
    graph.add_edge(3,5,3);
    graph.add_edge(4,5,12);
    let problem = edmonds_karp::solve(&graph,0,5);
    assert_eq!(problem.neighbors(0).fold(0, |x, (_, y)| x + y),13)
}


#[test]
fn test2() {
    use crate::graph::*;
    use std::collections::BTreeSet;
    let mut graph = MatGraph::new(5);
    graph.add_edge(0,1,10);
    graph.add_edge(0,2,2);
    graph.add_edge(1,2,6);
    graph.add_edge(1,3,6);
    graph.add_edge(2,4,5);
    graph.add_edge(3,2,4);
    graph.add_edge(3,4,8);
    let problem = edmonds_karp::solve(&graph,0,4);
    let xx: BTreeSet<(usize,i64)> = problem.neighbors(0).collect();
    assert_eq!(xx.into_iter().fold(0, |x, (_, y)| x + y),11)
}
