use crate::algorithms::priority_queue::PriorityQueue;

pub struct DfsQueue {
    stack: Vec<usize>,
}

impl DfsQueue {
    pub fn new() -> Self {
        DfsQueue {
            stack: Vec::new(),
        }
    }
}

impl PriorityQueue for DfsQueue {
    fn insert(&mut self, node_index: usize, cost: i32, heuristic_value: f64) {
        self.stack.push(node_index);
    }

    fn pop(&mut self) -> Option<usize> {
        self.stack.pop()
    }
}
