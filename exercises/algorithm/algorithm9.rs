/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

#[derive(Debug)]
pub struct Heap<T>
where
    T: Default + std::fmt::Debug
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + std::fmt::Debug
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            //items: vec![T::default()],
            items: vec![],
            comparator
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.items.push(value);
        self.count += 1;
        self.up(self.count);
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn children_present(&self, idx: usize) -> bool {
        idx <= self.count
    }

    /*
    fn up(&mut self, idx: usize) {
        let mut ci = idx;
        loop {
            let pi = self.parent_idx(ci);
            if pi == 0 {break}
            if !(self.comparator)(&self.items[ci - 1], &self.items[pi - 1]) {break}
            self.items.swap(ci - 1, pi - 1);
            ci = pi;
        }
    }
    */
    fn up(&mut self, idx: usize) {
        let ci = idx;
        let pi = self.parent_idx(ci);
        if pi == 0 {return;}
        if (self.comparator)(&self.items[pi - 1], &self.items[ci - 1]) {return}
        self.items.swap(ci - 1, pi - 1);
        self.up(pi);
    }

    fn down(&mut self, idx: usize) {
        let li = self.left_child_idx(idx);
        let ri = self.right_child_idx(idx);
        let lp = self.children_present(li);
        let rp = self.children_present(ri);
        if !lp && !rp {return}
        let si = if rp && (self.comparator)(&self.items[ri - 1], &self.items[li - 1]) {ri} else {li};
        //dbg!(&si);
        //dbg!(&idx);
        //dbg!(&self);
        if (self.comparator)(&self.items[idx - 1], &self.items[si - 1]) {return}
        //dbg!(&self);
        //self.items.swap(idx, si);
        self.items.swap(idx - 1, si - 1);
        self.down(si);
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Copy + std::fmt::Debug
{
    type Item = T;

    /*
    fn next(&mut self) -> Option<T> {
        //dbg!(&self.iter_status);
        //dbg!(&self.len());
        //return None;
        //if self.iter_status >= self.count() {self.iter_status = 0; return None;}
        if self.iter_status >= self.len() {self.iter_status = 0; return None;}
        let c = self.iter_status;
        self.iter_status += 1;
        Some(self.items[c])
    }
    */
    fn next(&mut self) -> Option<T> {
        if self.len() == 0 {return None}
        let result = Some(self.items[0]);
        // TODO
        self.count -= 1;
        //self.items[0] = self.items.remove(self.count);
        self.items[0] = self.items[self.count];
        self.items.remove(self.count);
        self.down(1);
        result
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + std::fmt::Debug
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + std::fmt::Debug
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        //dbg!(&heap.next());
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
