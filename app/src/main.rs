use std::time;
use search_core::search::solve::solve_problem;
use search_core::problems::farm_problem::farm_problem::FarmProblem;
use search_core::problems::market_trader_problem::market_trader_problem::MarketTraderProblem;
use search_core::problems::compute_number_problem::compute_number_problem::ComputeNumberProblem;
use search_core::problems::compute_number_problem::compute_number_problem::RedCarProblem;
use time::Instant;

fn main() {
    let start_time = Instant::now();
    solve_problem::<MarketTraderProblem>(
        "search_core/src/inputs/example_inputs/market_trader_problem/input_1.json",
        "GBFS",
    );

    let elapsed_time = start_time.elapsed(); // Calculate elapsed time
    println!("Execution time: {:?}", elapsed_time);
}