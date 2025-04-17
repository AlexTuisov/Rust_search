use crate::problems::problem::Problem;
use crate::search::{action::Action, state::StateTrait, state::Value};
use serde::{Deserialize, Serialize};
use serde_json::from_reader;
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub current_time: i32,
    pub funds: i32,
    pub stored_units: i32,
    pub stored_capacity: i32,
    pub demand: HashMap<i32, i32>
}

impl StateTrait for State {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HydropowerProblem {
    pub goal_funds: i32,
    pub before: HashMap<i32, i32>, // Maps "from" time to "to" time
}

impl HydropowerProblem {
    pub fn possible_advance_time_actions(&self, state: &State) -> Option<Action> {
        if let Some(&next_time) = self.before.get(&state.current_time) {
            let mut parameters = HashMap::new();
            parameters.insert("from".to_string(), Value::Text(state.current_time.to_string()));
            parameters.insert("to".to_string(), Value::Text(next_time.to_string()));
            Some(Action::new(
                format!("advance_time_{}_{}", state.current_time, next_time),
                1,
                parameters,
            ))
        } else {
            None // No valid next time step
        }
    }

    pub fn possible_pump_water_actions(state: &State) -> Option<Action> {
        if let Some(current_demand) = state.demand.get(&state.current_time) {
            let mut parameters = HashMap::new();
            parameters.insert(
                "time".to_string(),
                Value::Text(state.current_time.to_string()),
            );
            parameters.insert("demand".to_string(), Value::Int(*current_demand));
            Some(Action::new(
                format!("pump_water_up_{}", state.current_time),
                1,
                parameters,
            ))
        } else {
            None // No action if no demand for the current time
        }
    }
    
    pub fn possible_generate_actions(state: &State) -> Option<Action> {
        if let Some(current_demand) = state.demand.get(&state.current_time) {
            let mut parameters = HashMap::new();
            parameters.insert(
                "time".to_string(),
                Value::Text(state.current_time.to_string()),
            );
            parameters.insert("demand".to_string(), Value::Int(*current_demand));
            Some(Action::new(
                format!("generate_{}", state.current_time),
                1,
                parameters,
            ))
        } else {
            None // No action if no demand for the current time
        }
    }
    pub fn apply_advance_time_action(state: &State, _action: &Action) -> State {
        let mut new_state: State = state.clone();
        new_state.current_time += 1;
        new_state
    }

    pub fn apply_pump_water_action(state: &State, _action: &Action) -> State {
        let mut new_state = state.clone();
        let current_demand = state.demand.get(&state.current_time).unwrap();
        new_state.stored_units += 1;
        new_state.stored_capacity -= 1;
        new_state.funds -= current_demand * 105 / 100;
        new_state
    }

    pub fn apply_generate_action(state: &State, _action: &Action) -> State {
        let mut new_state = state.clone();
        let current_demand = state.demand.get(&state.current_time).unwrap();
        new_state.stored_units -= 1;
        new_state.stored_capacity += 1;
        new_state.funds += current_demand;
        new_state
    }
}

impl Problem for HydropowerProblem {
    type State = State;

    fn get_possible_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();

        // Only allow advancing time if there is a valid transition
        if let Some(action) = self.possible_advance_time_actions(state) {
            actions.push(action);
        }

        if state.stored_capacity > 0 {
            if let Some(action) = Self::possible_pump_water_actions(state) {
                actions.push(action);
            }
        }

        if state.stored_units > 0 {
            if let Some(action) = Self::possible_generate_actions(state) {
                actions.push(action);
            }
        }

        actions
    }

    fn apply_action(&self, state: &State, action: &Action) -> State {
        if action.name.starts_with("advance_time") {
            Self::apply_advance_time_action(state, action)
        } else if action.name.starts_with("pump_water_up") {
            Self::apply_pump_water_action(state, action)
        } else if action.name.starts_with("generate") {
            Self::apply_generate_action(state, action)
        } else {
            panic!("Unknown action");
        }
    }

    fn is_goal_state(&self, state: &State) -> bool {
        state.funds >= self.goal_funds
    }

    fn load_state_from_json(json_path: &str) -> (State, Self) {
        let file = File::open(json_path).expect("Failed to open JSON file");
        let reader = BufReader::new(file);
        let json_data: JsonValue = from_reader(reader).expect("Failed to parse JSON");

        let state_data = json_data.get("state").expect("Missing 'state' field in JSON");
        let problem_data = json_data.get("problem").expect("Missing 'problem' field in JSON");

        let state = State {
            current_time: 0,
            funds: state_data.get("funds").unwrap_or(&JsonValue::from(0)).as_i64().unwrap() as i32,
            stored_units: 0,
            stored_capacity: state_data.get("stored_capacity").unwrap_or(&JsonValue::from(0)).as_i64().unwrap() as i32,
            demand: problem_data.get("demands")
                .and_then(|demands| demands.as_array())
                .unwrap_or(&vec![])
                .iter()
                .map(|demand| {
                    let time = demand.get("time")
                        .and_then(|t| t.as_str())
                        .unwrap_or("t0000")
                        .trim_start_matches('t')
                        .parse::<i32>()
                        .unwrap_or(0);
                    let value = demand.get("value").unwrap_or(&JsonValue::from(0)).as_i64().unwrap() as i32;
                    (time, value)
                })
                .collect(),
        };

        let goal_funds = problem_data.get("goal_funds").unwrap_or(&JsonValue::from(0)).as_i64().unwrap() as i32;

        // Parse the "before" field into a HashMap of tuples
        let before = problem_data.get("before")
            .and_then(|before| before.as_array())
            .unwrap_or(&vec![])
            .iter()
            .map(|entry| {
                let from = entry.get("from")
                    .and_then(|f| f.as_str())
                    .unwrap_or("t0000")
                    .trim_start_matches('t')
                    .parse::<i32>()
                    .unwrap_or(0);
                let to = entry.get("to")
                    .and_then(|t| t.as_str())
                    .unwrap_or("t0000")
                    .trim_start_matches('t')
                    .parse::<i32>()
                    .unwrap_or(0);
                (from, to)
            })
            .collect();

        let problem = HydropowerProblem { goal_funds, before };

        (state, problem)
    }

    fn heuristic(&self, _state: &State) -> f64 {
        // heuristic is imported during build time from include!("refined_heuristic.in")
        0.0
    }
}