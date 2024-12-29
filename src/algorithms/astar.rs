use std::collections::BinaryHeap;
use std::cmp::Reverse;
use crate::algorithms::priority_queue::PriorityQueue;

pub struct AStarQueue {
    heap: BinaryHeap<Reverse<(i32, usize)>>, // (priority, node_index)
}

impl AStarQueue {
    pub fn new() -> Self {
        AStarQueue {
            heap: BinaryHeap::new(),
        }
    }
}

impl PriorityQueue for AStarQueue {
    fn insert(&mut self, node_index: usize, cost: i32, heuristic: f64) {
        let priority = (cost as f64 + heuristic) as i32;
        self.heap.push(Reverse((priority, node_index)));
    }

    fn pop(&mut self) -> Option<usize> {
        self.heap.pop().map(|Reverse((_, index))| index)
    }
}
