use crate::shortest_path::{bellman_ford, dijkstra};
use crate::graph::*;
use num::{Signed, Zero};
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Sub, SubAssign};

pub fn solve_st<A,C,U>(
    graph: &A,
    source: usize,
    sink: usize,
    amount: U,
) -> Option<C>
    where A: AccGraph<Value = (U,U)>,
          C: AccGraph<Value = U>,
          U: Debug + PartialOrd + Copy + Zero + Add + AddAssign + Sub + SubAssign + Signed,
{
    let size = graph.size();
    let mut flow: C = Graph::new(size);
    let mut g_f_pot: C = Graph::new(size);
    let mut g_f_cap: C = Graph::new(size);
    for from in 0..size {
        for (to, (cap, cost)) in graph.neighbors(from) {
            flow.add_edge(from,to, U::zero());
            g_f_pot.add_edge(from,to,cost);
            g_f_cap.add_edge(from,to, cap);
        }
    }
    let mut potential = {
        match bellman_ford::solve(&g_f_pot, source) {
            bellman_ford::Either::Shortest(vec) => {
                let mut ret = vec![U::zero(); size];
                for i in 0..size {
                    ret[i] = vec[i].unwrap().0;
                }
                ret
            }
            _ => unreachable!(),
        }
    };
    for from in 0..size {
        for (to, (_, cost)) in graph.neighbors(from) {
            g_f_pot.add_edge(from,to, cost + potential[from] - potential[to]);
        }
    }
    let mut now = amount;
    while now > U::zero() {
        let vec = dijkstra::solve(&g_f_pot, source);
        if vec[sink].is_none() {
            return None
        }
        let vec = vec
            .iter()
            .map(|x| x.unwrap())
            .collect::<Vec<(U, usize)>>();
        let pass = {
            let mut pass = Vec::new();
            let mut to = sink;
            let mut from = vec[to].1;
            while to != source {
                pass.push((from, to));
                to = from;
                from = vec[from].1;
            }
            pass.reverse();
            pass
        };
        let gamma = {
            let mut gamma = now;
            for &(from, to) in &pass {
                if gamma > g_f_cap.get(from,to).unwrap() {
                    gamma = g_f_cap.get(from,to).unwrap();
                }
            }
            gamma
        };
        now -= gamma;
        augment_along_pass(graph, &pass, &mut flow, &mut g_f_cap, gamma);
        update_pot_and_g_f_pot(graph, &vec, &mut potential, &mut g_f_pot, &g_f_cap);
    }
    Some(flow)
}


pub fn solve<A,C,U>(
    graph: &A,
    b: &Vec<U>,
) -> Option<C> 
where A: AccGraph<Value = (U,U)>,
C: AccGraph<Value = U>,
U: Debug + PartialOrd + Copy + Zero + Add + AddAssign + Sub + SubAssign + Signed,
{
    let size = graph.size();
    let b_sum = b
        .iter()
        .filter(|&&x| x > U::zero())
        .fold(U::zero(), |x, &y| x + y);
    let mut b = b.clone();
    let mut graph = graph.clone();
    graph.extend();
    for i in 0..size {
        if b[i] > U::zero() {
            graph.add_edge(i,size,(b[i],U::zero()));
        }
    }
    for x in b.iter_mut() {
        if *x > U::zero() {
            *x = U::zero();
        }
    }
    b.push(b_sum);    
    let mut flow: C = Graph::new(size+1);
    let mut g_f_pot: C = Graph::new(size+1);
    let mut g_f_cap: C = Graph::new(size+1);
    for from in 0..size + 1 {
        for (to, (cap, cost)) in graph.neighbors(from) {
            flow.add_edge(from,to, U::zero());
            g_f_pot.add_edge(from,to,cost);
            g_f_cap.add_edge(from,to, cap);
        }
    }
    let mut potential = {
        match bellman_ford::solve(&g_f_pot, size) {
            bellman_ford::Either::Shortest(vec) => {
                let mut ret = vec![U::zero(); size + 1];
                for i in 0..size + 1 {
                    ret[i] = vec[i].unwrap().0;
                }
                ret
            }
            _ => unreachable!(),
        }
    };
    for from in 0..size + 1 {
        for (to, (_, cost)) in graph.neighbors(from) {
            g_f_pot.add_edge(from,to, cost + potential[from] - potential[to]);
        }
    }
    while b[size] > U::zero() {
        match find(&g_f_pot, size, &b) {
            None => return None,
            Some(t) => {
                let vec = dijkstra::solve(&g_f_pot, size)
                    .iter()
                    .map(|x| x.unwrap())
                    .collect::<Vec<(U, usize)>>();
                let pass = {
                    let mut pass = Vec::new();
                    let mut to = t;
                    let mut from = vec[to].1;
                    while to != size {
                        pass.push((from, to));
                        to = from;
                        from = vec[from].1;
                    }
                    pass.reverse();
                    pass
                };
                let gamma = {
                    let mut gamma = if b[size] > -b[t] { -b[t] } else { b[size] };
                    for &(from, to) in &pass {
                        if gamma > g_f_cap.get(from,to).unwrap() {
                            gamma = g_f_cap.get(from,to).unwrap();
                        }
                    }
                    gamma
                };
                b[size] -= gamma;
                b[t] += gamma;
                augment_along_pass(&graph, &pass, &mut flow, &mut g_f_cap, gamma);
                update_pot_and_g_f_pot(&graph, &vec, &mut potential, &mut g_f_pot, &g_f_cap);
            }
        };
    }
    flow.cut();
    Some(flow)
}

fn find<C,U>(
    g_f_pot: &C,
    s: usize,
    b: &Vec<U>,
) -> Option<usize> 
where C: AccGraph<Value = U>,
U: Debug + PartialOrd + Copy + Zero + Add + AddAssign + Sub + SubAssign + Signed,
{
    let mut scanned = vec![false; g_f_pot.size()];
    scanned[s] = true;
    let mut stack = vec![s];
    while let Some(from) = stack.pop() {
        for (to, _) in g_f_pot.neighbors(from) {
            if !scanned[to] {
                if b[to] < U::zero() {
                    return Some(to);
                }
                scanned[to] = true;
                stack.push(to);
            }
        }
    }
    None
}


fn augment_along_pass<A,C,U>(
    graph: &A,
    pass: &Vec<(usize, usize)>,
    flow: &mut C,
    g_f_cap: &mut C,
    g: U,
)
where A: AccGraph<Value = (U,U)>,
C: AccGraph<Value = U>,
U: Debug + PartialOrd + Copy + Zero + Add + AddAssign + Sub + SubAssign + Signed,
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
            g_f_cap.modify(from,to,minus);
            g_f_cap.modify(to,from,plus);
        } else {
            flow.modify(to,from,|x| x.map(|x| x-g));
            g_f_cap.modify(to,from,plus);
            g_f_cap.modify(from,to,minus);
        }
    }
}

fn update_pot_and_g_f_pot<A,C,U>(
    graph: &A,
    vec: &Vec<(U, usize)>,
    pot: &mut Vec<U>,
    g_f_pot: &mut C,
    g_f_cap: &C,
)
where A: AccGraph<Value = (U,U)>,
C: AccGraph<Value = U>,
U: Debug + PartialOrd + Copy + Zero + Add + AddAssign + Sub + SubAssign + Signed,
{
    for i in 0..graph.size() {
        pot[i] += vec[i].0;
    }
    let mut ret: C = Graph::new(graph.size());
    for from in 0..graph.size() {
        for (to, _) in g_f_cap.neighbors(from) {
            if graph.is_edge(from,to) {
                ret.add_edge(from,to,graph.get(from,to).unwrap().1+ pot[from] - pot[to]);
            } else {
                ret.add_edge(from,to,-graph.get(to,from).unwrap().1+ pot[from] - pot[to]);
            }
        }
    }
    std::mem::swap(&mut ret, g_f_pot);
}
