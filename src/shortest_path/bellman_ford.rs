use std::ops::Add;
use num::Zero;
use crate::graph::Graph;

#[derive(Debug,Clone,PartialEq,Eq)]
pub enum Which<U> {
    Shortest(U,Vec<(usize,usize)>),
    Inaccseible,
    NegInf
}

pub fn solve_with_goal<A,B,U>(graph: &impl Graph<Item = U, Iterator = B>, start: usize, goal: usize) -> Which<U>
    where U: Add + Zero + PartialOrd + Copy,
          B: Iterator<Item = (usize,U)>
{
    use Which::*; 
    let size = graph.len();
    let mut ret = vec![None; size];
    ret[start] = Some((U::zero(), 0));
    for _ in 0..size {
        let mut flag = false;
        for from in 0..size {
            for (to, weight) in graph.iter(from) {
                if !ret[from].is_none()
                    && (ret[to].is_none()
                        || ret[to].map(|(x, _)| x) > ret[from].map(|(x, _)| x + weight))
                {
                    ret[to] = Some((ret[from].unwrap().0 + weight, from));
                    flag = true;
                }
            }
        }
        if !flag {
            if let Some((val,from)) = ret[goal] {
                let mut from = from;
                let mut to = goal;
                let mut pass = Vec::new();
                while to != start {
                    pass.push((from,to));
                    to = from;
                    from = ret[from].unwrap().1;
                }
                return Shortest(val,pass)
            } else {
                return Inaccseible
            }
        }
    }
    let mut neg = vec![false; size];
    for _ in 0..size {
        for from in 0..size {
            for (to, weight) in graph.iter(from) {
                if !ret[from].is_none()
                    && (ret[to].is_none()
                        || ret[to].map(|(x, _)| x) > ret[from].map(|(x, _)| x + weight))
                {
                    ret[to] = Some((ret[from].unwrap().0 + weight, from));
                    neg[to] = true;
                }
                if neg[from] {
                    neg[to] = true;
                }
                if neg[to] {
                    return NegInf
                }
            }
        }
    }
    if let Some((val,from)) = ret[goal] {
        let mut from = from;
        let mut to = goal;
        let mut pass = Vec::new();
        while to != start {
            pass.push((from,to));
            to = from;
            from = ret[from].unwrap().1;
        }
        return Shortest(val,pass)
    }
    unreachable!()
}

#[derive(Debug,Clone,PartialEq,Eq)]
pub enum Either<U> {
    Shortest(Vec<Option<(U, usize)>>),
    NegCircuit(Vec<(usize, usize)>),
}

pub fn solve<U: Add + Zero + PartialOrd + Copy, B: Iterator<Item = (usize,U)>>(
    graph: &impl Graph<Item = U, Iterator = B>, 
    start: usize
) -> Either<U> {
    use Either::*;
    let size = graph.len();
    let mut ret = vec![None; size];
    ret[start] = Some((U::zero(), 0));
    for _ in 0..size {
        let mut flag = false;
        for from in 0..size {
            for (to, weight) in graph.iter(from) {
                if !ret[from].is_none()
                    && (ret[to].is_none()
                        || ret[to].map(|(x, _)| x) > ret[from].map(|(x, _)| x + weight))
                {
                    ret[to] = Some((ret[from].unwrap().0 + weight, from));
                    flag = true;
                }
            }
        }
        if !flag {
            return Shortest(ret);
        }
    }
    let mut neg = Vec::new();
    for from in 0..size {
        for (to, weight) in graph.iter(from) {
            if !ret[from].is_none()
                && !ret[to].is_none()
                && ret[to].map(|(x, _)| x) > ret[from].map(|(x, _)| x + weight)
            {
                let mut from = from;
                let mut to = to;
                for _ in 0..size * size {
                    neg.push((from, to));
                    to = from;
                    from = ret[from].unwrap().1;
                    if neg[0] == (from, to) {
                        neg.reverse();
                        return NegCircuit(neg);
                    }
                }
            }
        }
    }
    unreachable!()
}
