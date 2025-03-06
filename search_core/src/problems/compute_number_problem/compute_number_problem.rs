use crate::problems::problem::Problem;
use crate::search::{action::Action, state::StateTrait, state::Value};
use serde::{Deserialize, Serialize};
use serde_json::from_reader;

use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

include!("refined_heuristic.in");

fn create_add_action(
    index_a: usize,
    index_b: usize,
    buffer_index: usize,
    combined_values: &[i32],
) -> Action {
    let a = combined_values[index_a];
    let b = combined_values[index_b];
    let name = format!("add {} + {} -> buffer[{}]", a, b, buffer_index);
    let mut parameters = HashMap::new();
    parameters.insert("a".to_string(), Value::Int(a));
    parameters.insert("b".to_string(), Value::Int(b));
    parameters.insert("buffer".to_string(), Value::Int(buffer_index as i32));
    Action::new(name, 1, parameters)
}

fn create_subtract_action(
    index_a: usize,
    index_b: usize,
    buffer_index: usize,
    combined_values: &[i32],
) -> Action {
    let a = combined_values[index_a];
    let b = combined_values[index_b];
    let name = format!("subtract {} - {} -> buffer[{}]", a, b, buffer_index);
    let mut parameters = HashMap::new();
    parameters.insert("a".to_string(), Value::Int(a));
    parameters.insert("b".to_string(), Value::Int(b));
    parameters.insert("buffer".to_string(), Value::Int(buffer_index as i32));
    Action::new(name, 1, parameters)
}

fn create_multiply_action(
    index_a: usize,
    index_b: usize,
    buffer_index: usize,
    combined_values: &[i32],
) -> Option<Action> {
    let a = combined_values[index_a];
    let b = combined_values[index_b];

    // Check for overflow
    if let Some(_result) = a.checked_mul(b) {
        let name = format!("multiply {} * {} -> buffer[{}]", a, b, buffer_index);
        let mut parameters = HashMap::new();
        parameters.insert("a".to_string(), Value::Int(a));
        parameters.insert("b".to_string(), Value::Int(b));
        parameters.insert("buffer".to_string(), Value::Int(buffer_index as i32));
        Some(Action::new(name, 1, parameters))
    } else {
        None // Skip action if multiplication would overflow
    }
}

fn create_divide_action(
    index_a: usize,
    index_b: usize,
    buffer_index: usize,
    combined_values: &[i32],
) -> Option<Action> {
    let a = combined_values[index_a];
    let b = combined_values[index_b];
    if b == 0 {
        return None; // Skip action if dividing by zero
    }

    let name = format!("divide {} / {} -> buffer[{}]", a, b, buffer_index);
    let mut parameters = HashMap::new();
    parameters.insert("a".to_string(), Value::Int(a));
    parameters.insert("b".to_string(), Value::Int(b));
    parameters.insert("buffer".to_string(), Value::Int(buffer_index as i32));

    Some(Action::new(name, 1, parameters))
}

fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    // Check from 5 upwards, skipping even numbers and multiples of 3
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn is_twin_prime(n: i32) -> bool {
    is_prime(n) && (is_prime(n - 2) || is_prime(n + 2))
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub initial_set: Vec<i32>, // A fixed initial set of integers
    pub buffers: Vec<i32>,     // A vector of buffers for intermediate results
    pub goal_threshold: i32,   // The threshold value for the goal
}

impl State {
    fn new() -> Self {
        State {
            initial_set: Vec::new(),
            buffers: Vec::new(),
            goal_threshold: 0,
        }
    }
}

impl StateTrait for State {}

pub struct ComputeNumberProblem {}

impl ComputeNumberProblem {}

impl Problem for ComputeNumberProblem {
    type State = State;
    fn load_state_from_json(json_path: &str) -> (State, Self) {
        let file = File::open(json_path).expect("Failed to open JSON file");
        let reader = BufReader::new(file);
        let json_data: JsonValue = serde_json::from_reader(reader).expect("Failed to parse JSON");

        // Parse initial set
        let initial_set = json_data["initial_set"]
            .as_array()
            .expect("Expected initial_set to be an array")
            .iter()
            .map(|v| v.as_i64().unwrap() as i32)
            .collect();

        // Parse goal threshold
        let goal_threshold = json_data["goal_threshold"]
            .as_i64()
            .expect("Expected goal_threshold to be an integer") as i32;

        // Initialize buffers
        let num_buffers = json_data["num_buffers"]
            .as_i64()
            .expect("Expected num_buffers to be an integer") as usize;
        let buffers = vec![0; num_buffers];

        let state = State {
            initial_set,
            buffers,
            goal_threshold,
        };

        (state, Self {})
    }

    fn get_possible_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();

        let combined_values: Vec<i32> = state
            .initial_set
            .iter()
            .copied()
            .chain(state.buffers.iter().copied())
            .collect();
        let num_values = combined_values.len();
        let num_buffers = state.buffers.len();

        for i in 0..num_values {
            for j in 0..num_values {
                if i == j {
                    continue; // Skip if indices are the same
                }
                for buffer_index in 0..num_buffers {
                    // Generate all possible actions
                    actions.push(create_add_action(i, j, buffer_index, &combined_values));
                    actions.push(create_subtract_action(i, j, buffer_index, &combined_values));
                    if let Some(multiply_action) =
                        create_multiply_action(i, j, buffer_index, &combined_values)
                    {
                        actions.push(multiply_action);
                    }
                    if let Some(divide_action) =
                        create_divide_action(i, j, buffer_index, &combined_values)
                    {
                        actions.push(divide_action);
                    }
                }
            }
        }

        actions
    }

    fn apply_action(&self, state: &State, action: &Action) -> State {
        let mut new_state = state.clone();

        let combined_values: Vec<i32> = state
            .initial_set
            .iter()
            .copied()
            .chain(state.buffers.iter().copied())
            .collect();

        let a = match action.parameters.get("a") {
            Some(Value::Int(value)) => *value,
            _ => panic!("Expected 'a' to be an integer in action parameters"),
        };

        let b = match action.parameters.get("b") {
            Some(Value::Int(value)) => *value,
            _ => panic!("Expected 'b' to be an integer in action parameters"),
        };

        let buffer_index = match action.parameters.get("buffer") {
            Some(Value::Int(index)) => *index as usize,
            _ => panic!("Expected 'buffer' to be an integer in action parameters"),
        };

        let result = match action.name.split_whitespace().next() {
            Some("add") => a + b,
            Some("subtract") => a - b,
            Some("multiply") => a * b,
            Some("divide") => a / b,
            Some("exponentiate") => a.wrapping_pow(b as u32),
            Some(action) => panic!("Unexpected action name: {}", action),
            None => panic!("Action name is empty"),
        };

        new_state.buffers[buffer_index] = result;

        new_state
    }

    fn is_goal_state(&self, state: &State) -> bool {
        state
            .buffers
            .iter()
            .any(|&value| value > state.goal_threshold && is_twin_prime(value))
    }

    fn heuristic(&self, state: &State) -> f64 {
        // heuristic is imported during build time from include!("refined_heuristic.in")
        heuristic(self, state)
    }
}
