use crate::algorithms::astar::AStarQueue;
use crate::algorithms::bfs::BfsQueue;
use crate::algorithms::dfs::DfsQueue;
use crate::algorithms::gbfs::GBFSQueue;
use crate::algorithms::priority_queue::PriorityQueue;

pub enum SearchQueue {
    AStar(AStarQueue),
    GBFS(GBFSQueue),
    BFS(BfsQueue),
    DFS(DfsQueue),
}

impl PriorityQueue for SearchQueue {
    fn insert(&mut self, node_index: usize, cost: i32, heuristic_value: f64) {
        match self {
            SearchQueue::AStar(queue) => queue.insert(node_index, cost, heuristic_value),
            SearchQueue::GBFS(queue) => queue.insert(node_index, cost, heuristic_value),
            SearchQueue::BFS(queue) => queue.insert(node_index, cost, heuristic_value),
            SearchQueue::DFS(queue) => queue.insert(node_index, cost, heuristic_value),
        }
    }

    fn pop(&mut self) -> Option<usize> {
        match self {
            SearchQueue::AStar(queue) => queue.pop(),
            SearchQueue::GBFS(queue) => queue.pop(),
            SearchQueue::BFS(queue) => queue.pop(),
            SearchQueue::DFS(queue) => queue.pop(),
        }
    }
}
