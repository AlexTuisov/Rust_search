use std::collections::BinaryHeap;
use std::cmp::Reverse;
use crate::algorithms::priority_queue::PriorityQueue;

pub struct GBFSQueue {
    heap: BinaryHeap<Reverse<(i32, usize)>>, // (priority, node_index)
}

impl crate::algorithms::gbfs::GBFSQueue {
    pub fn new() -> Self {
        crate::algorithms::gbfs::GBFSQueue {
            heap: BinaryHeap::new(),
        }
    }
}

impl PriorityQueue for crate::algorithms::gbfs::GBFSQueue {
    fn insert(&mut self, node_index: usize, cost: i32, heuristic: f64) {
        let priority = heuristic as i32;
        self.heap.push(Reverse((priority, node_index)));
    }

    fn pop(&mut self) -> Option<usize> {
        self.heap.pop().map(|Reverse((_, index))| index)
    }
}