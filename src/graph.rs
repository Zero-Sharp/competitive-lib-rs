use std::ptr;
use std::collections::BTreeMap;

pub trait Graph: Clone {
    type Value: Copy;
    type Iter: Iterator<Item = (usize, Self::Value)>;

    fn new(size: usize) -> Self;
    fn size(&self) -> usize;
    fn add_edge(&mut self, u: usize, v: usize, w: Self::Value);
    fn neighbors(&self, from: usize) -> Self::Iter;
}

#[derive(Clone,Debug)]
pub struct ListGraph<U>(Vec<Vec<(usize, U)>>);

pub struct ListGraphIter<U> {
    ptr: *const (usize, U),
    end: *const (usize, U),
}

impl<U: Copy> Iterator for ListGraphIter<U> {
    type Item = (usize, U);
    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr == self.end {
            None
        } else {
            let old = self.ptr;
            self.ptr = unsafe { self.ptr.offset(1) };
            Some(unsafe { ptr::read(old) })
        }
    }
}

impl<U: Copy> Graph for ListGraph<U> {
    type Value = U;
    type Iter = ListGraphIter<U>;

    fn new(size: usize) -> Self {
        ListGraph(vec![Vec::new();size])
    }
    fn size(&self) -> usize {
        self.0.len()
    }
    fn add_edge(&mut self, u: usize, v: usize, w: U) {
        self.0[u].push((v,w));
    }
    fn neighbors(&self, from: usize) -> Self::Iter {
        let ptr = self.0[from].as_ptr();
        ListGraphIter {
            ptr: ptr,
            end: unsafe {ptr.offset(self.0[from].len() as isize) },
        }
    }
}

#[derive(Clone,Debug)]
pub struct MatGraph<U> {
    mat: Vec<Vec<Option<U>>>,
    list: Vec<Vec<usize>>,
}

pub struct MatGraphIter<U> {
    ptr: *const usize,
    end: *const usize,
    vec_ptr: *const Option<U>,
}

impl<U: Copy> Iterator for MatGraphIter<U> {
    type Item = (usize,U);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.ptr == self.end {
                return None;
            } else {
                let old = self.ptr;
                let index = unsafe {ptr::read(old)};
                self.ptr = unsafe { self.ptr.offset(1) };
                match unsafe { ptr::read(self.vec_ptr.offset(index as isize)) } {
                    None => continue,
                    Some(w) => return Some((index, w)),
                }
            }
        }
    }
}

impl<U: Copy> Graph for MatGraph<U> {
    type Value = U;
    type Iter = MatGraphIter<U>;

    fn new(size: usize) -> Self {
        MatGraph {
            mat: vec![vec![None;size];size],
            list: vec![Vec::new();size],
        }
    }
    fn size(&self) -> usize {
        self.mat.len()
    }
    fn add_edge(&mut self, u: usize, v: usize, w: U) {
        self.mat[u][v] = Some(w);
        self.list[u].push(v);
    }
    fn neighbors(&self, from: usize) -> Self::Iter {
        let ptr = self.list[from].as_ptr();
        MatGraphIter {
            ptr: ptr,
            end: unsafe {ptr.offset(self.list[from].len() as isize) },
            vec_ptr: self.mat[from].as_ptr(),
        }
    }
}

#[derive(Clone,Debug)]
pub struct MapGraph<U>(Vec<BTreeMap<usize,U>>);

impl<U: Copy> Graph for MapGraph<U> {
    type Value = U;
    type Iter = std::collections::btree_map::IntoIter<usize,U>;

    fn new(size: usize) -> Self {
        MapGraph(vec![BTreeMap::new();size])
    }
    fn size(&self) -> usize {
        self.0.len()
    }
    fn add_edge(&mut self, u: usize, v: usize, w: U) {
        self.0[u].insert(v,w);
    }
    fn neighbors(&self, from: usize) -> Self::Iter {
        self.0[from].clone().into_iter()
    }
}

pub trait AccGraph: Graph {
    fn get(&self, from: usize, to: usize) -> Option<Self::Value>;
    fn is_edge(&self, from: usize, to: usize) -> bool;
    fn remove(&mut self, u: usize, v: usize);
    fn modify(&mut self, u: usize, v: usize, update: impl Fn(Option<Self::Value>) -> Option<Self::Value>);
    fn extend(&mut self);
    fn cut(&mut self);
}

impl<U: Copy> AccGraph for MatGraph<U> {
    fn get(&self, from: usize, to: usize) -> Option<Self::Value> {
        self.mat[from][to]
    }
    fn is_edge(&self, from: usize, to: usize) -> bool {
        self.get(from,to).is_some()
    }
    fn remove(&mut self, from: usize, to: usize) {
        self.mat[from][to] = None;
    }
    fn modify(&mut self, from: usize, to: usize, update: impl Fn(Option<Self::Value>) -> Option<Self::Value>) {
        let old = self.get(from,to);
        let new = update(old);
        match old {
            None => match new {
                None => {},
                Some(y) => self.add_edge(from,to,y),    
            }
            Some(_) => match new {
                None => self.remove(from,to),
                Some(y) => self.mat[from][to] = Some(y)
            }
        }
    }
    fn extend(&mut self) {
        let old_size = self.size();
        for i in 0..old_size {
            self.mat[i].push(None);
        }
        self.mat.push(vec![None;old_size+1]);
        self.list.push(Vec::new());
    }
    fn cut(&mut self) {
        let old_size = self.size();
        for i in 0..old_size {
            self.mat[i].pop();
        }
        self.mat.pop();
        self.list.pop();
    }
}

impl<U: Copy> AccGraph for MapGraph<U> {
    fn get(&self, from: usize, to: usize) -> Option<Self::Value> {
        self.0[from].get(&to).map(|&x| x)
    }
    fn is_edge(&self, from: usize, to: usize) -> bool {
        self.get(from,to).is_some()
    }
    fn remove(&mut self, from: usize, to: usize) {
        self.0[from].remove(&to);
    }
    fn modify(&mut self, from: usize, to: usize, update: impl Fn(Option<Self::Value>) -> Option<Self::Value>) {
        let old = self.get(from,to);
        let new = update(old);
        match old {
            None => match new {
                None => {},
                Some(y) => self.add_edge(from,to,y),    
            }
            Some(_) => match new {
                None => self.remove(from,to),
                Some(y) => self.add_edge(from,to,y),   
            }
        }
    }
    fn extend(&mut self) {
        self.0.push(BTreeMap::new());
    }
    fn cut(&mut self) {
        self.0.pop();
    }
}

/*
pub type Index = usize;
pub struct Graph<U> {
    body: Vec<Vec<Index>>,
    edges: Vec<Option<(usize,usize,U)>>,
}

impl<U: Copy> Graph<U> {
    pub fn new(size: usize) -> Self {
        Graph {
            body: vec![Vec::new(); size],
            edges: Vec::new(),
        }
    }
    pub fn len(&self) -> usize {
        self.body.len()
    }
    pub fn add_edge(&mut self, u: usize, v: usize, w: U) {
        let new_index = self.edges.len();
        self.edges.push(Some((u,v,w)));
        self.body[u].push(new_index);
    }
    pub fn remove(&mut self, index: Index) {
        self.edges[index] = None;
    }
    pub fn neighbors(&self, from: usize) -> GraphIter<U> {
        let ptr = self.body[from].as_ptr();
        GraphIter {
            ptr: ptr,
            end: unsafe {ptr.offset(self.body[from].len() as isize) },
            edges_as_ptr: self.edges.as_ptr(),
        }
    }
    pub fn get(&self, index: Index) -> Option<(usize,usize,U)> {
        self.edges[index]
    }
    pub fn is_edge_slow(&self, from: usize, to: usize) -> bool {
        for &index in &self.body[from] {
            match self.edges[index] {
                None => continue,
                Some((_,too,_)) => {
                    if too == to {
                        return true;
                    }
                }
            }
        }
        false
    }
    pub fn modify(&mut self, index: Index, update: impl Fn(Option<U>) -> Option<U>) {
        let old = self.edges[index];
        let new = update(old.map(|x| x.2));
        match old {
            None => unreachable!(),
            Some(x) => {
                match new {
                    None => self.edges[index] = None,
                    Some(y) => self.edges[index] = Some((x.0,x.1,y)),
                }
            }
        }
    }
    pub fn modify_slow(&mut self, from: usize, to: usize, update: impl Fn(Option<U>) -> Option<U>) {
        match self.find_index(from,to) {
            None => {
                match update(None) {
                    None => {},
                    Some(w) => {
                        self.add_edge(from,to,w)
                    }
                }
            }
            Some(index) => self.modify(index, update)
        }
    }
    pub fn find_index(&self, from: usize, to: usize) -> Option<Index> {
        for &index in &self.body[from] {
            if let Some((from,too,val)) = self.get(index) {
                if to == too {
                    return Some(index)
                }
            }
        }
        None
    }
    /*
    fn extend(&mut self);
    fn cut(&mut self);
    fn is_edge(&self, from: usize, to: usize) -> bool;
    fn modify(&mut self, u: usize, v: usize, update: impl Fn(Option<Self::Item>) -> Option<Self::Item>);
    */
}

pub struct GraphIter<U> {
    ptr: *const usize,
    end: *const usize,
    edges_as_ptr: *const Option<(usize, usize,U)>,
}

impl<U: Copy> Iterator for GraphIter<U> {
    type Item = (usize,usize,Index, U); // from,to,index,val
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.ptr == self.end {
                return None;
            } else {
                let old = self.ptr;
                let index = unsafe {ptr::read(old)};
                self.ptr = unsafe { self.ptr.offset(1) };
                match unsafe { ptr::read(self.edges_as_ptr.offset(index as isize)) } {
                    None => continue,
                    Some((from,to, w)) => return Some((from,to,index, w)),
                }
            }
        }
    }
}
*/

pub struct Residual<A> {
    forward: A,
    back: A,
}

#[derive(Copy,Clone)]
pub enum Either {
    Forward,
    Back,
}

#[derive(Clone,Copy)]
pub enum EitherV<A> {
    Forward(A),
    Back(A),
}

impl<A> EitherV<A> {
    pub fn unwrap(self) -> A {
        match self {
            EitherV::Forward(x) => x,
            EitherV::Back(x) => x,
        }
    }
    pub fn label(self) -> Either {
        match self {
            EitherV::Forward(_x) => Either::Forward,
            EitherV::Back(_x) => Either::Back,
        }
    }
}

impl<A: AccGraph> Residual<A> {
    pub fn new(n: usize) -> Self {
        Residual {
            forward: A::new(n),
            back: A::new(n),
        }
    }
    pub fn new_from_graph(graph: &A) -> Self {
        Residual {
            forward: graph.clone(),
            back: A::new(graph.size()),
        }
    }
    pub fn size(&self) -> usize {
        self.forward.size()
    }
    pub fn get(&self, which: Either, u: usize, v: usize) -> Option<A::Value> {
        match which {
            Either::Forward => self.forward.get(u,v),
            Either::Back => self.back.get(u,v),
        }
    }
    pub fn neighbors(&self, u: usize) -> impl Iterator<Item = (usize,EitherV<A::Value>)> {
        let x = self.forward.neighbors(u);
        let y = self.back.neighbors(u);
        x.map(|(i,x)| (i,EitherV::Forward(x))).chain(y.map(|(i,x)| (i,EitherV::Back(x))))
    }
    pub fn add_edge(&mut self, u: usize, v: usize, w: EitherV<A::Value>) {
        match w {
            EitherV::Forward(x) => self.forward.add_edge(u,v,x),
            EitherV::Back(x) => self.back.add_edge(u,v,x),
        }
    }
    pub fn modify(&mut self, which: Either, u: usize, v: usize, update: impl Fn(Option<A::Value>) -> Option<A::Value>) {
        match which {
            Either::Forward => self.forward.modify(u,v,update),
            Either::Back => self.back.modify(u,v,update),
        }
    }
}
