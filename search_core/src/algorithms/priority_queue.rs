use std::cmp::Ordering;

// Define a trait for the priority queue to be used in the generic search
pub trait PriorityQueue {
    fn insert(&mut self, node_index: usize, cost: i32, heuristic_value: f64); // Insert a node with its cost or priority
    fn pop(&mut self) -> Option<usize>; // Pop the next node based on the queue’s ordering
}
