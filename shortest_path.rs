pub mod dijkstra;
pub mod bellman_ford;


#[test]
fn test() {
    use crate::graph::*;
    let mut graph = ListGraph::new(6);
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

    let ans: Vec<(usize,usize)> = dijkstra::solve(&graph,0).iter().map(|x| x.unwrap()).collect();

    assert_eq!(ans[0].0,0);
    assert_eq!(ans[1].0,5);
    assert_eq!(ans[2].0,4);
    assert_eq!(ans[3].0,2);
    assert_eq!(ans[4].0,6);
    assert_eq!(ans[5].0,10)
}