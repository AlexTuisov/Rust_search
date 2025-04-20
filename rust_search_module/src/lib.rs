use pyo3::prelude::*;
use search_core::problems::compute_number_problem::compute_number_problem::ComputeNumberProblem;
use search_core::problems::farm_problem::farm_problem::FarmProblem;
use search_core::problems::market_trader_problem::market_trader_problem::MarketTraderProblem;
use search_core::search::solve::solve_problem;

#[pyfunction]
fn solve_problem_py(json_path: &str, search_strategy: &str, problem_type: &str) -> PyResult<()> {
    match problem_type {
        "MarketTraderProblem" => {
            solve_problem::<MarketTraderProblem>(json_path, search_strategy);
        }
        "FarmProblem" => {
            solve_problem::<FarmProblem>(json_path, search_strategy);
        }
        "ComputeNumberProblem" => {
            solve_problem::<ComputeNumberProblem>(json_path, search_strategy);
        }
        // Add more matches for other problem types
        _ => {
            // Return a Python exception if an unknown problem type is provided
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Unknown problem type: {}",
                problem_type
            )));
        }
    }
    Ok(())
}

// #[pymodule]
// fn rust_search_module(_py: Python, m: &PyModule) -> PyResult<()> {
//     // Expose the `solve_problem_py` function under the name `solve_problem` in Python
//     m.add_function(wrap_pyfunction!(solve_problem_py, m)?)?;
//     Ok(())
// }
