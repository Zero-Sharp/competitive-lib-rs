use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::ops::Add;
use num::Zero;
use crate::graph::Graph;

#[derive(Clone, Copy)]
pub struct Pair<U> {
    label: usize,
    value: U,
}

impl<U> Pair<U> {
    fn new(l: usize, v: U) -> Pair<U> {
        Pair { label: l, value: v }
    }
}

impl<U: PartialOrd + Copy + PartialEq> Ord for Pair<U> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.partial_cmp(&other.value).unwrap()
    }
}

impl<U: PartialOrd + Copy> PartialOrd for Pair<U> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<U: PartialEq + Copy> PartialEq for Pair<U> {
    fn eq(&self, other: &Self) -> bool {
        (self.label, self.value) == (other.label, other.value)
    }
}

impl<U:PartialEq + Copy> Eq for Pair<U> {}

pub fn solve<U: Add + Zero + PartialOrd + Copy + PartialEq, A: Graph<Value = U>>(
    graph: &A, 
    start: usize
) -> Vec<Option<(U, usize)>> {
    let size = graph.size();
    let mut potential = vec![None; size];
    potential[start] = Some((U::zero(), 0));
    let mut heap = BinaryHeap::new();
    heap.push(Reverse(Pair::new(start, U::zero())));
    let mut set = vec![false; size];
    let mut count = 0;
    while let Some(Reverse(pair)) = heap.pop() {
        if !set[pair.label] {
            set[pair.label] = true;
            count += 1;
            for (v,weight) in graph.neighbors(pair.label) {
                if !set[v] && (potential[v].is_none() || potential[v].map(|x| x.0) > potential[pair.label].map(|x| x.0+weight)) {
                    potential[v] = Some((potential[pair.label].unwrap().0 + weight, pair.label));
                    heap.push(Reverse(Pair::new(v, potential[v].unwrap().0)));
                }
            }
        }
        if count == size {
            break;
        }
    }
    potential
}


pub fn solve_with_goal<U: Add + Zero + PartialOrd + Copy + PartialEq, A: Graph<Value = U>> (
    graph: &A, 
    start: usize, 
    goal: usize
) -> Option<(U,Vec<(usize,usize)>)> {
    let size = graph.size();
    let mut potential = vec![None; size];
    potential[start] = Some((U::zero(), 0));
    let mut heap = BinaryHeap::new();
    heap.push(Reverse(Pair::new(start, U::zero())));
    let mut set = vec![false; size];
    while let Some(Reverse(pair)) = heap.pop() {
        if !set[pair.label] {
            set[pair.label] = true;
            for (v, weight) in graph.neighbors(pair.label) {
                if !set[v] && (potential[v].is_none() || potential[v].map(|x| x.0) > potential[pair.label].map(|x| x.0+weight)) {
                    potential[v] = Some((potential[pair.label].unwrap().0 + weight, pair.label));
                    heap.push(Reverse(Pair::new(v, potential[v].unwrap().0)));
                }
            }
        }
        if set[goal] {
            let (val,from) = potential[goal].unwrap();
            let mut from = from;
            let mut to = goal;
            let mut pass = Vec::new();
            while to != start {
                pass.push((from,to));
                to = from;
                from = potential[from].unwrap().1;
            }
            return Some((val,pass))
        }
    }
    None
}
