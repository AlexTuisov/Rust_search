use search_core::problems::compute_number_problem::compute_number_problem::ComputeNumberProblem;
use search_core::problems::farm_problem::farm_problem::FarmProblem;
use search_core::problems::market_trader_problem::market_trader_problem::MarketTraderProblem;
use search_core::problems::red_car_problem::red_car_problem::RedCarProblem;
use search_core::problems::foo_sailing_problem::foo_sailing_problem::FooSailingProblem;
use search_core::search::solve::solve_problem;
use std::time;
use time::Instant;

fn main() {
    let start_time = Instant::now();
    solve_problem::<FooSailingProblem>(
        "search_core/src/inputs/example_inputs/foo_sailing_problem/pfile1.json",
        "A*",
    );

    let elapsed_time = start_time.elapsed(); // Calculate elapsed time
    println!("Execution time: {:?}", elapsed_time);
}
