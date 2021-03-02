use num::Zero;
use std::ops::{Add,AddAssign,SubAssign,Sub};
use crate::graph::AccGraph;
use std::mem::swap;
use std::cmp::min;

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
    while let Some(blocking) = bfs(size, &res, s, t) {
        // dbg!(&blocking);
        augment_along_pass(blocking, &mut flow, &mut res, graph);
    }
    flow
}

fn bfs<A,U>(
    size: usize,
    res: &A,
    s: usize,
    t: usize,
) -> Option<A>
where A: AccGraph<Value = U>,
      U: Copy + Ord + Add + Sub<Output = U> + Zero + AddAssign + SubAssign,
{
    let mut ret = A::new(res.size());
    let mut arrived = vec![None; size];
    let mut now = vec![s];
    let mut next = Vec::new();
    let mut end = false;
    loop {
        while let Some(from) = now.pop() {
            for (to, _) in res.neighbors(from) {
                if to == t {
                    end = true;
                    let mut vv = Vec::new();
                    let mut to = t;
                    let mut from = from;
                    let mut g = res.get(from,to).unwrap();
                    loop {
                        vv.push((from,to));
                        g = min(g,res.get(from,to).unwrap());
                        if from == s {
                            break;
                        }
                        to = from;
                        from = arrived[from].unwrap();
                    }
                    for (from,to) in vv {
                        ret.modify(from,to,|x| Some(x.unwrap_or(U::zero())+g));
                    }
                } else if arrived[to].is_none() {
                    arrived[to] = Some(from);
                    next.push(to);
                }
            }
        }
        if end {
            return Some(ret)
        }
        if !next.is_empty() {
            swap(&mut now, &mut next);
            continue;
        } else {
            return None
        }
        
    }
}

fn augment_along_pass<A,U>(
    blocking: A,
    flow: &mut A,
    g_f: &mut A,
    graph: &A,
)
where A: AccGraph<Value = U>,
      U: Copy + Ord + Add + Sub<Output = U> + Zero + AddAssign + SubAssign,
{
        for from in 0..blocking.size() {
            for (to,g) in blocking.neighbors(from) {
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
}
