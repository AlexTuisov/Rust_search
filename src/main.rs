#![allow(unused_variables, dead_code, unused_imports)]
mod search;
mod problems;
mod algorithms;

use crate::search::search_tree;
use crate::search::state::{StateTrait, Value};
use crate::search::action::Action;
// use crate::problems::example_problem::SimpleProblem;
use crate::algorithms::bfs::BfsQueue;
use crate::algorithms::dfs::DfsQueue;
use crate::algorithms::astar::AStarQueue;
use crate::algorithms::gbfs::GBFSQueue;
use crate::algorithms::search_queue::SearchQueue;
use crate::problems::problem::Problem;
use crate::search::search_tree::SearchTree;
use crate::search::search::generic_search;
use crate::algorithms::priority_queue::PriorityQueue;
use std::time::Instant;
use crate::problems::farm_problem::farm_problem::FarmProblem;
use crate::problems::market_trader_problem::market_trader_problem::MarketTraderProblem;
use crate::problems::compute_number_problem::compute_number_problem::ComputeNumberProblem;
use crate::search::node::Node;

pub fn solve_problem<P>(json_path: &str, search_strategy: &str)
where
    P: Problem,
{
    let (initial_state, problem) = P::load_state_from_json(json_path);

    let initial_node = Node::new_empty(initial_state.clone());
    let mut tree = SearchTree::new(initial_node.state.clone());

    let queue = match search_strategy {
        "A*" => SearchQueue::AStar(AStarQueue::new()),
        "GBFS" => SearchQueue::GBFS(GBFSQueue::new()),
        "BFS" => SearchQueue::BFS(BfsQueue::new()),
        "DFS" => SearchQueue::DFS(DfsQueue::new()),
        _ => panic!("Unknown search strategy: {}", search_strategy),
    };

    let result = generic_search(
        &mut tree,
        |state| problem.get_possible_actions(state),
        |state, action| problem.apply_action(state, action),
        |state| problem.is_goal_state(state),
        queue,
        |state| problem.heuristic(state),
    );

    match result {
        Ok(actions) => {
            let total_cost: f64 = actions.iter().map(|action| action.cost as f64).sum();
            let action_names: Vec<_> = actions.iter().map(|action| &action.name).collect();
            println!("Solution found with actions: {:?}", action_names);
            println!("Total cost of actions: {}", total_cost);
            println!("Total length of the solution: {}", action_names.len());
        }
        Err(msg) => {
            println!("Search failed: {}", msg);
        }
    }
}

fn main() {
    let start_time = Instant::now();
    solve_problem::<MarketTraderProblem>(
        "inputs/example_inputs/market_trader_problem/input_1.json",
        "GBFS",
    );

    let elapsed_time = start_time.elapsed(); // Calculate elapsed time
    println!("Execution time: {:?}", elapsed_time);
}