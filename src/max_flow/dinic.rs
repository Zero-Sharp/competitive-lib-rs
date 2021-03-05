use num::Zero;
use std::ops::{Add,AddAssign,SubAssign,Sub};
use crate::graph::*;
use std::mem::swap;
use std::cmp::min;
use std::collections::BTreeSet;

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
    let mut res = Residual::new_from_graph(graph);
    while let Some(blocking) = find_blocking(&res, s, t) {
        // dbg!(&blocking);
        augment_along_pass(blocking, &mut flow, &mut res);
    }
    flow
}

fn find_blocking<A,U>(
    res: &Residual<A>,
    s: usize,
    t: usize,
) -> Option<Residual<A>>
where A: AccGraph<Value = U>,
      U: Copy + Ord + Add + Sub<Output = U> + Zero + AddAssign + SubAssign,
{
    let mut lvl: Residual<A> = Residual::new(res.size());
    let mut now = vec![s];
    let mut next = Vec::new();
    let mut end = false;
    let mut arrived = vec![false; res.size()];
    arrived[s] = true;
    loop {
        let mut now_arrived = BTreeSet::new();
        while let Some(from) = now.pop() {
            for (to, val) in res.neighbors(from) {
                if to == t {
                    end = true;
                }
                if !arrived[to] {
                    lvl.add_edge(to,from,val);
                    now_arrived.insert(to);
                    next.push(to);
                }
            }
        }
        if end {
            break;
        } else if !next.is_empty() {
            swap(&mut now, &mut next);
            for n in now_arrived {
                arrived[n] = true;
            }
            continue;
        } else {
            return None
        }
    }
    use crate::graph::EitherV::*;
    let mut ret = Residual::new(res.size());
    let mut stack = vec![t];
    let mut prev = vec![(t,Forward(U::zero()));res.size()];
    while let Some(from) = stack.pop() {
        for (to,val) in lvl.neighbors(from) {
            if to == s {
                let mut path = Vec::new();
                let mut to = to;
                let mut from = from;
                let mut val = val;
                let mut g = val.unwrap();
                loop {
                    path.push((from,to,val));
                    g = min(g,val.unwrap());
                    if from == t {
                        break;
                    }
                    to = from;
                    val = prev[from].1;
                    from = prev[from].0;
                }
                for (from,to,val) in path {
                    ret.modify(val.label(),to,from, |x| Some(x.unwrap_or(U::zero())+g));
                }
            } else {
                prev[to] = (from,val);
                stack.push(to);
            }
        }
    }
    Some(ret)
}

fn augment_along_pass<A,U>(
    blocking: Residual<A>,
    flow: &mut A,
    g_f: &mut Residual<A>,
)
where A: AccGraph<Value = U>,
      U: Copy + Ord + Add + Sub<Output = U> + Zero + AddAssign + SubAssign,
{
    use crate::graph::Either::*;
        for from in 0..blocking.size() {
            for (to,val) in blocking.neighbors(from) {
                let g = val.unwrap();
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
                match val {
                    EitherV::Forward(g) => {
                        flow.modify(from,to,|x| x.map(|x| x+g));
                        g_f.modify(Forward,from,to,minus);
                        g_f.modify(Back,to,from,plus);
                    },
                    EitherV::Back(g) => {
                        flow.modify(to,from,|x| x.map(|x| x-g));
                        g_f.modify(Forward,to,from,plus);
                        g_f.modify(Back,from,to,minus);
                    }
                }
            }
        }
}

/*
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
    while let Some(blocking) = bfs(&res, s, t) {
        // dbg!(&blocking);
        augment_along_pass(blocking, &mut flow, &mut res, graph);
    }
    flow
}

fn bfs<A,U>(
    res: &A,
    s: usize,
    t: usize,
) -> Option<A>
where A: AccGraph<Value = U>,
      U: Copy + Ord + Add + Sub<Output = U> + Zero + AddAssign + SubAssign,
{
    let mut lvl = A::new(res.size());
    let mut now = vec![s];
    let mut next = Vec::new();
    let mut end = false;
    let mut arrived = vec![false; res.size()];
    arrived[s] = true;
    loop {
        let mut now_arrived = BTreeSet::new();
        while let Some(from) = now.pop() {
            for (to, val) in res.neighbors(from) {
                if to == t {
                    end = true;
                }
                if !arrived[to] {
                    lvl.add_edge(to,from,val);
                    now_arrived.insert(to);
                    next.push(to);
                }
            }
        }
        if end {
            break;
        }
        if !next.is_empty() {
            swap(&mut now, &mut next);
            for n in now_arrived {
                arrived[n] = true;
            }
            continue;
        } else {
            return None
        }
    }
    let mut ret = A::new(res.size());
    let mut stack = vec![t];
    let mut prev = vec![t;res.size()];
    while let Some(from) = stack.pop() {
        for (to,_) in lvl.neighbors(from) {
            if to == s {
                let mut path = Vec::new();
                let mut to = to;
                let mut from = from;
                let mut g = lvl.get(from,to).unwrap();
                loop {
                    path.push((from,to));
                    g = min(g,lvl.get(from,to).unwrap());
                    if from == t {
                        break;
                    }
                    to = from;
                    from = prev[from];
                }
                for (from,to) in path {
                    ret.modify(to,from, |x| Some(x.unwrap_or(U::zero())+g));
                }
            } else {
                prev[to] = from;
                stack.push(to);
            }
        }
    }
    Some(ret)
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
*/