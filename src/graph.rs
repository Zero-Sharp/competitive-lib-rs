use std::fmt::Debug;
use std::collections::{BTreeSet,BTreeMap};
//use std::marker::PhantomData;
//use std::ptr::NonNull;
use std::rc::Rc;
use std::cell::RefCell;
use std::ptr;

pub trait Graph {
    type Item;
    type Iterator;

    fn new(size: usize) -> Self;
    fn extend(&mut self);
    fn cut(&mut self);
    fn len(&self) -> usize;
    fn is_edge(&self, from: usize, to: usize) -> bool;
    fn get(&self, from: usize, to: usize) -> Option<Self::Item>;
    fn add_edge(&mut self, u: usize, v: usize, w: Self::Item);
    fn remove(&mut self, u: usize, v: usize);
    fn modify(&mut self, u: usize, v: usize, update: impl Fn(Option<Self::Item>) -> Option<Self::Item>);
    fn iter(&self, entry: usize) -> Self::Iterator;
}

#[derive(Debug,Clone)]
pub struct GraphVec<A>(Vec<Vec<Option<A>>>);
 
impl<A:Copy + Clone> Graph for GraphVec<A> {
    type Item = A;
    type Iterator = GraphVecIter<A>;
    fn new(size: usize) -> Self {
        GraphVec(vec![vec![None;size];size])
    }
    fn extend(&mut self) {
        self.0.push(Vec::new());
    }
    fn cut(&mut self) {
        self.0.pop();
    }
    fn len(&self) -> usize {
        self.0.len()
    }
    fn add_edge(&mut self, u: usize, v: usize, w: Self::Item)  {
        self.0[u][v] = Some(w);
    }
    fn remove(&mut self, u: usize, v: usize) {
        self.0[u][v] = None;
    }
    fn modify(&mut self, u: usize, v: usize, update: impl Fn(Option<Self::Item>) -> Option<Self::Item>) {
        self.0[u][v] = update(self.0[u][v]);
    }
    fn is_edge(&self, from: usize, to: usize) -> bool {
        !self.0[from][to].is_none()
    }
    fn get(&self, from: usize, to: usize) -> Option<A> {
        self.0[from][to]
    }
    fn iter(&self, entry: usize) -> Self::Iterator {
        let ptr = self.0[entry].as_ptr();
        GraphVecIter {
            ptr: ptr,
            end: unsafe {ptr.offset(self.0[entry].len() as isize) },
            count: 0,
        }
    }
}
 
pub struct GraphVecIter<A> {
    // buf: NonNull<Option<A>>,
    // phantom: PhantomData<A>,
    // cap: usize,
    ptr: *const Option<A>,
    end: *const Option<A>,
    count: usize,
}
 
impl<A: Copy> Iterator for GraphVecIter<A> {
    type Item = (usize,A);
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.ptr == self.end {
                return None
            } else {
                let old = self.ptr;
                self.ptr = unsafe { self.ptr.offset(1) };
                let old_count = self.count;
                self.count += 1;
                match unsafe {ptr::read(old)} {
                    None => continue,
                    Some(x) => return Some((old_count,x))
                } 
            }
        }
    }
}


#[derive(Debug,Clone)]
pub struct GraphSet<A> {
    mat: Vec<Rc<RefCell<Vec<Option<A>>>>>,
    entries: Vec<BTreeSet<usize>>
}

pub struct GraphSetIter<A> {
    vec: Rc<RefCell<Vec<Option<A>>>>,
    iter: std::collections::btree_set::IntoIter<usize>
}

impl<A: Copy + Debug> Iterator for GraphSetIter<A> {
    type Item = (usize,A);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(entry) => {
                Some((entry,self.vec.borrow()[entry].unwrap()))
            },
            None => None
        }
    }
}

impl<A: Copy + Clone> Graph for GraphSet<A> {
    type Item = A;
    type Iterator = GraphSetIter<A>;
    fn new(size: usize) -> Self {
        GraphSet {
            mat: vec![vec![None;size];size].into_iter().map(|v| Rc::new(RefCell::new(v))).collect(),
            entries: vec![BTreeSet::new();size]
        }
    }
    fn add_edge(&mut self, u: usize, v: usize, w: A)  {
        self.mat[u].borrow_mut()[v] = Some(w);
        self.entries[u].insert(v);
    }
    fn extend(&mut self) {
        self.mat.push(Rc::new(RefCell::new(Vec::new())));
        self.entries.push(BTreeSet::new());
    }
    fn cut(&mut self) {
        self.mat.pop();
        self.entries.pop();
    }
    fn len(&self) -> usize {
        self.mat.len()
    }
    fn remove(&mut self, u: usize, v: usize) {
        self.mat[u].borrow_mut()[v] = None;
        self.entries[u].remove(&v);
    }
    fn modify(&mut self, u: usize, v: usize, update: impl Fn(Option<A>) -> Option<A>) {
        let xx = self.mat[u].borrow()[v];
        let ret = update(xx);
        self.mat[u].borrow_mut()[v] = ret;
        if ret.is_none() {
            self.entries[u].remove(&v);
        } else {
            self.entries[u].insert(v);
        }
    }
    fn is_edge(&self, u: usize, v: usize) -> bool {
        !self.mat[u].borrow()[v].is_none()
    }
    fn get(&self, u: usize, v: usize) -> Option<A> {
        self.mat[u].borrow()[v]
    }
    fn iter(&self, entry: usize) -> Self::Iterator {
        GraphSetIter {
            vec: self.mat[entry].clone(),
            iter: self.entries[entry].clone().into_iter()
        }
    }
}

#[derive(Debug,Clone)]
pub struct GraphMap<A>(Vec<BTreeMap<usize,Option<A>>>);

impl<A: Copy> Graph for GraphMap<A> {
    type Item = A;
    type Iterator = GraphMapIter<A>;
    fn new(size: usize) -> Self {
        GraphMap(vec![BTreeMap::new();size])
    }
    fn extend(&mut self) {
        self.0.push(BTreeMap::new())
    }
    fn cut(&mut self) {
        self.0.pop();
    }
    fn len(&self) -> usize {
        self.0.len()
    }
    fn add_edge(&mut self, u: usize, v: usize, w: A)  {
        self.0[u].insert(v,Some(w));
    }
    fn remove(&mut self, u: usize, v: usize) {
        self.0[u].remove(&v);
    }
    fn modify(&mut self, u: usize, v: usize, update: impl Fn(Option<A>) -> Option<A>) {
        match self.0[u].get(&v).map(|&x| x) {
            None => {
                let ret = update(None);
                self.0[u].insert(v,ret);
            },
            Some(x) => {
                *self.0[u].entry(v).or_insert(None) = update(x);
            }
        }
    }
    fn is_edge(&self, u: usize, v: usize) -> bool {
        let xx = self.0[u].get(&v).map(|&x| x);
        if xx.is_none() {
            return false
        }
        !xx.unwrap().is_none()
    }
    fn get(&self, u: usize, v: usize) -> Option<A> {
        let xx = self.0[u].get(&v).map(|&x| x);
        if xx.is_none() {
            return None
        }
        xx.unwrap()
    }
    fn iter(&self, entry: usize) -> Self::Iterator {
        GraphMapIter(self.0[entry].clone().into_iter(),0)
    }
}

pub struct GraphMapIter<A>(std::collections::btree_map::IntoIter<usize,Option<A>>,usize);

impl<A: Copy> Iterator for GraphMapIter<A> {
    type Item = (usize,A);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.next() {
                None => return None,
                Some((_,None)) => {},
                Some((x,Some(y))) => return Some((x,y))
            }
        }
    }
}