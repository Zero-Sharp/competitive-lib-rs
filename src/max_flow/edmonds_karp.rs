use num::Zero;
use std::ops::{Add,AddAssign,SubAssign,Sub};
use std::collections::VecDeque;
use crate::graph::AccGraph;

pub fn solve<A,U>(
    graph: &A,
    s: usize,
    t: usize,
) -> A
where A: AccGraph<Value = U>,
    U: Copy + Ord + Add + Sub<Output = U> + Zero + AddAssign + SubAssign,
{
    let size = graph.size();
    let mut flow: A = graph.clone();
    for from in 0..size {
        for (to, _) in graph.neighbors(from) {
            flow.add_edge(from,to,U::zero());
        }
    }
    let mut res: A= graph.clone();
    while let Some((pass, min)) = bfs(size, &res, s, t) {
        augment_along_pass(&pass, &mut flow, &mut res, min, graph);
    }
    flow
}

fn bfs<A,U>(
    size: usize,
    res: &A,
    s: usize,
    t: usize,
) -> Option<(Vec<(usize, usize)>, U)>
where A: AccGraph<Value = U>,
      U: Copy + Ord + Add + Sub<Output = U> + Zero + AddAssign + SubAssign,
{
    let mut que = VecDeque::new();
    let mut arrived = vec![None; size];
    que.push_back(s);
    while let Some(from) = que.pop_front() {
        for (to, _) in res.neighbors(from) {
            if arrived[to].is_none() {
                arrived[to] = Some(from);
                que.push_back(to);
            }
        }
        if let Some(mut from) = arrived[t] {
            let mut ret = Vec::new();
            let mut to = t;
            let mut min = res.get(from,to).unwrap();
            loop {
                ret.push((from, to));
                if min > res.get(from,to).unwrap() {
                    min = res.get(from,to).unwrap();
                }
                to = from;
                from = arrived[from].unwrap();
                if from == s {
                    ret.push((from,to));
                    break;
                }
            }
            ret.reverse();
            return Some((ret, min));
        }
    }
    None
}

fn augment_along_pass<A,U>(
    pass: &Vec<(usize, usize)>,
    flow: &mut A,
    g_f: &mut A,
    g: U,
    graph: &A,
)
where A: AccGraph<Value = U>,
      U: Copy + Ord + Add + Sub<Output = U> + Zero + AddAssign + SubAssign,
{
        let plus = |x| {
            match x {
                Some(val) => Some(val+g),
                None => Some(g)
            }
        };
        let minus = |x| {
            match x {
                Some(val) => {
                    if val <= g {
                        None
                    } else {
                        Some(val-g)
                    }
                },
                None => unreachable!()
            }
        };
        for &(from, to) in pass {
            if graph.is_edge(from,to) {
                flow.modify(from,to,|x| x.map(|x| x+g));
                g_f.modify(from,to,minus);
                g_f.modify(to,from,plus);
            } else {
                flow.modify(to,from,|x| x.map(|x| x-g));
                g_f.modify(to,from,plus);
                g_f.modify(from,to,minus);
            }
        }
}
