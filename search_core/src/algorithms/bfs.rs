use std::collections::VecDeque;
use crate::algorithms::priority_queue::PriorityQueue;

// A simple FIFO queue for BFS
pub struct BfsQueue {
    queue: VecDeque<usize>,
}

impl BfsQueue {
    pub fn new() -> Self {
        BfsQueue {
            queue: VecDeque::new(),
        }
    }
}

impl PriorityQueue for BfsQueue {
    fn insert(&mut self, node_index: usize, cost: i32, heuristic_value: f64) {
        self.queue.push_back(node_index);
    }

    fn pop(&mut self) -> Option<usize> {
        self.queue.pop_front()
    }
}
