use crate::problems::problem::Problem;
use crate::search::{action::Action, state::StateTrait, state::Value};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
// use crate::problems::taxi_problem::taxi_problem::TaxiProblem;

include!("refined_heuristic.in");

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub x_values: BTreeMap<String, i32>, // Maps farms to their x values
    pub adjacencies: BTreeMap<String, Vec<String>>, // Maps farms to their neighbors
    pub goal_thresholds: BTreeMap<String, i32>, // Maps farms to their goal thresholds
    pub weighted_sum_goal: WeightedSumGoal, // Stores weighted sum goal weights and threshold
    pub cost: i32,                       // Total cost accumulated
}

impl State {
    pub fn new() -> Self {
        State {
            x_values: BTreeMap::new(),
            adjacencies: BTreeMap::new(),
            goal_thresholds: BTreeMap::new(),
            weighted_sum_goal: WeightedSumGoal {
                weights: BTreeMap::new(),
                threshold: 0,
            },
            cost: 0,
        }
    }
}

impl State {}

impl StateTrait for State {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeightedSumGoal {
    pub weights: BTreeMap<String, i64>, // Scaled weights for hashing
    pub threshold: i64,                 // Scaled threshold for hashing
}

impl WeightedSumGoal {
    const SCALE_FACTOR: i64 = 1_000_000; // Scale factor to preserve precision

    pub fn new(weights: BTreeMap<String, f64>, threshold: f64) -> Self {
        let scaled_weights = weights
            .into_iter()
            .map(|(key, value)| (key, (value * Self::SCALE_FACTOR as f64).round() as i64))
            .collect();

        let scaled_threshold = (threshold * Self::SCALE_FACTOR as f64).round() as i64;

        WeightedSumGoal {
            weights: scaled_weights,
            threshold: scaled_threshold,
        }
    }

    pub fn get_original_weights(&self) -> BTreeMap<String, f64> {
        self.weights
            .iter()
            .map(|(key, value)| (key.clone(), *value as f64 / Self::SCALE_FACTOR as f64))
            .collect()
    }
}

pub struct FarmProblem {
    pub farms: Vec<String>,                         // List of farms
    pub x_values: BTreeMap<String, i32>,            // X values for each farm
    pub adjacencies: BTreeMap<String, Vec<String>>, // Adjacencies between farms     // Initial cost
    pub goal_thresholds: BTreeMap<String, i32>,     // Goal thresholds for each farm
    pub weighted_sum_goal: WeightedSumGoal,         // Weighted sum goal
}
impl FarmProblem {
    // Constructor that takes a path to a JSON file and loads the data
    pub fn new_from_json(path: &str) -> Self {
        // Load the data from the JSON file
        let file = std::fs::File::open(path).expect("File not found");
        let reader = std::io::BufReader::new(file);
        let json: serde_json::Value = serde_json::from_reader(reader).expect("Error reading JSON");

        // Parse farms
        let farms = json["farms"]
            .as_array()
            .unwrap()
            .iter()
            .map(|f| f.as_str().unwrap().to_string())
            .collect::<Vec<_>>();

        // Parse adjacencies
        let adjacencies = json["initial_state"]["adjacencies"]
            .as_object()
            .unwrap()
            .iter()
            .map(|(farm, neighbors)| {
                let neighbors_vec = neighbors
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|n| n.as_str().unwrap().to_string())
                    .collect::<Vec<_>>();
                (farm.clone(), neighbors_vec)
            })
            .collect::<BTreeMap<_, _>>();

        // Parse goal thresholds
        let goal_thresholds = json["goal"]["x_thresholds"]
            .as_object()
            .unwrap()
            .iter()
            .map(|(farm, threshold)| (farm.clone(), threshold.as_i64().unwrap() as i32))
            .collect::<BTreeMap<_, _>>();

        // Parse weighted sum goal with scaled values
        let weights = json["goal"]["weighted_sum_goal"]["weights"]
            .as_object()
            .unwrap()
            .iter()
            .map(|(farm, weight)| (farm.clone(), weight.as_f64().unwrap()))
            .collect::<BTreeMap<_, _>>();
        let weighted_sum_goal = WeightedSumGoal::new(
            weights,
            json["goal"]["weighted_sum_goal"]["threshold"]
                .as_f64()
                .unwrap(),
        );

        // Parse and initialize the State
        let initial_state = State {
            x_values: json["initial_state"]["x_values"]
                .as_object()
                .unwrap()
                .iter()
                .map(|(farm, x_val)| (farm.clone(), x_val.as_i64().unwrap() as i32))
                .collect(),
            adjacencies: json["initial_state"]["adjacencies"]
                .as_object()
                .unwrap()
                .iter()
                .map(|(farm, neighbors)| {
                    let neighbors_vec = neighbors
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|n| n.as_str().unwrap().to_string())
                        .collect::<Vec<_>>();
                    (farm.clone(), neighbors_vec)
                })
                .collect(),
            goal_thresholds: goal_thresholds.clone(),
            weighted_sum_goal: weighted_sum_goal.clone(),
            cost: json["initial_state"]["cost"].as_i64().unwrap() as i32,
        };

        // Construct and return the FarmProblem instance
        FarmProblem {
            farms,
            adjacencies,
            goal_thresholds,
            weighted_sum_goal,
            x_values: initial_state.x_values.clone(),
        }
    }

    pub fn create_initial_state(&self) -> State {
        State {
            x_values: self.x_values.clone(),
            adjacencies: self.adjacencies.clone(),
            goal_thresholds: self.goal_thresholds.clone(),
            weighted_sum_goal: self.weighted_sum_goal.clone(),
            cost: 0, // Assuming initial cost is 0; adjust if needed
        }
    }
}

impl Problem for FarmProblem {
    type State = State;
    fn get_possible_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();

        // Retrieve adjacencies directly from the state's adjacencies field
        for (farm, neighbors) in &state.adjacencies {
            // Retrieve x_value directly from the state's x_values field
            if let Some(&x_value) = state.x_values.get(farm) {
                // Check neighbors for possible actions
                for neighbor in neighbors {
                    if x_value >= 4 {
                        let mut params = HashMap::new();
                        params.insert("farm0".to_string(), Value::Text(farm.clone()));
                        params.insert("farm1".to_string(), Value::Text(neighbor.clone()));

                        actions.push(Action::new("move-fast".to_string(), 1, params));
                    }

                    if x_value >= 1 {
                        let mut params = HashMap::new();
                        params.insert("farm0".to_string(), Value::Text(farm.clone()));
                        params.insert("farm1".to_string(), Value::Text(neighbor.clone()));

                        actions.push(Action::new("move-slow".to_string(), 1, params));
                    }
                }
            }
        }

        actions
    }

    fn apply_action(&self, state: &State, action: &Action) -> State {
        let mut new_state = state.clone();

        // Retrieve the parameters from the action
        let farm0 = match action.parameters.get("farm0") {
            Some(Value::Text(farm)) => farm.clone(),
            _ => return new_state, // If farm0 is not present or not a string, return unchanged state
        };

        let farm1 = match action.parameters.get("farm1") {
            Some(Value::Text(farm)) => farm.clone(),
            _ => return new_state, // If farm1 is not present or not a string, return unchanged state
        };

        // Retrieve x_values for both farms directly from the state's fields
        let x_farm0 = match state.x_values.get(&farm0) {
            Some(&x) => x,
            _ => return new_state, // If x value for farm0 is missing, return unchanged state
        };

        let x_farm1 = match state.x_values.get(&farm1) {
            Some(&x) => x,
            _ => return new_state, // If x value for farm1 is missing, return unchanged state
        };

        // Apply effects based on action type
        if action.name == "move-fast" && x_farm0 >= 4 {
            new_state.x_values.insert(farm0.clone(), x_farm0 - 4);
            new_state.x_values.insert(farm1.clone(), x_farm1 + 2);
            new_state.cost += 1; // Update the cost in the new state
        } else if action.name == "move-slow" && x_farm0 >= 1 {
            new_state.x_values.insert(farm0.clone(), x_farm0 - 1);
            new_state.x_values.insert(farm1.clone(), x_farm1 + 1);
        }

        new_state
    }

    fn is_goal_state(&self, state: &State) -> bool {
        // Check if all x_values meet or exceed their respective thresholds
        for (farm, threshold) in &self.goal_thresholds {
            let x_value = match state.x_values.get(farm) {
                Some(&value) => value,
                _ => panic!("There was a problem: missing x_value for farm {}", farm),
            };
            if x_value < *threshold {
                return false; // If any farm's x_value is below the threshold, it's not a goal state
            }
        }

        // Calculate the total weighted sum for the goal
        let mut total_weighted_sum = 0.0;
        for (farm, &weight) in &self.weighted_sum_goal.weights {
            let x_value = match state.x_values.get(farm) {
                Some(&value) => value as f64,
                _ => panic!("There was a problem: missing x_value for farm {}", farm),
            };
            total_weighted_sum += weight as f64 * x_value;
        }

        // Check if the total weighted sum meets or exceeds the goal threshold
        total_weighted_sum >= self.weighted_sum_goal.threshold as f64
    }

    fn heuristic(&self, state: &State) -> f64 {
        // heuristic is imported during build time from include!("refined_heuristic.in")
        heuristic(self, state)
    }

    fn load_state_from_json(json_path: &str) -> (State, Self) {
        let problem = Self::new_from_json(json_path);
        let initial_state = Self::create_initial_state(&problem);
        (initial_state, problem)
    }
}
