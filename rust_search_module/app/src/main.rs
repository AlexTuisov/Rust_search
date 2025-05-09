use search_core::problems::red_car_problem::red_car_problem::RedCarProblem;
// use search_core::problems::block_grouping_problem::block_grouping_problem::BlockGroupingProblem;
// use search_core::problems::compute_number_problem::compute_number_problem::ComputeNumberProblem;
// use search_core::problems::counters_expression_problem::counters_expression_problem::CountersExpressionProblem;
// use search_core::problems::counters_problem::counters_problem::CountersProblem;
// use search_core::problems::ext_plant_watering_problem::ext_plant_watering_problem::ExtPlantWateringProblem;
// use search_core::problems::farm_problem::farm_problem::FarmProblem;
// use search_core::problems::fo_counters_expression_problem::fo_counters_expression_problem::FoCountersExpressionProblem;
// use search_core::problems::fo_counters_problem::fo_counters_problem::FoCountersProblem;
//use search_core::problems::fo_sailing_problem::fo_sailing_problem::FooSailingProblem;
// use search_core::problems::market_trader_problem::market_trader_problem::MarketTraderProblem;
// use search_core::problems::path_ways_metric_problem::path_ways_metric_problem::PathWaysMetricProblem;
//use search_core::problems::red_car_problem::red_car_problem::RedCarProblem;
//use search_core::problems::zenotravel_problem::zenotravel_problem::ZenoTravelProblem;
use search_core::problems::tpp_problem::tpp_problem::TppProblem;
use search_core::problems::zenotravel_fuel_problem::zenotravel_fuel_problem::ZenoTravelFuelProblem;
use search_core::problems::zenotravel_problem::zenotravel_problem::ZenoTravelProblem;
use search_core::problems::zenotravel_time_problem::zenotravel_time_problem::ZenoTravelTimeProblem;
use search_core::search::solve::solve_problem;
use std::time;
use time::Instant;

fn main() {
    let start_time = Instant::now();
    solve_problem::<RedCarProblem>(
        "search_core/src/inputs/red_car_problem/problems_json/pfile4.json",
        "A*",
    );

    let elapsed_time = start_time.elapsed(); // Calculate elapsed time
    println!("Execution time: {:?}", elapsed_time);
}
