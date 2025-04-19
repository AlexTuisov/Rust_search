use crate::problems::problem::Problem;
use crate::search::{action::Action, state::StateTrait, state::Value};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub x: i32, // x coordinate of the drone
    pub y: i32, // y coordinate of the drone
    pub z: i32, // z coordinate of the drone
    pub battery_level: i32, // battery level of the drone
    pub visited: HashMap<String, bool>, // point => visited or not
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DroneProblem {
    pub battery_capacity: i32, // battery capacity of the drone
    pub bounds: ((i32, i32), (i32, i32), (i32, i32)), // x bounds tuple, y bounds tuple, z bounds tuple
    pub locations: HashMap<String, (i32, i32, i32)>, // point => (x, y, z)
}

impl StateTrait for State {}


impl DroneProblem {
    pub fn possible_increase_x_action(state: &State) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("x".to_string(), Value::Int(state.x));
        Action::new(format!("increase_x_{}", state.x), 1, parameters)
    }

    pub fn possible_decrease_x_action(state: &State) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("x".to_string(), Value::Int(state.x));
        Action::new(format!("decrease_x_{}", state.x), 1, parameters)
    }

    pub fn possible_increase_y_action(state: &State) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("y".to_string(), Value::Int(state.y));
        Action::new(format!("increase_y_{}", state.y), 1, parameters)
    }

    pub fn possible_decrease_y_action(state: &State) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("y".to_string(), Value::Int(state.y));
        Action::new(format!("decrease_y_{}", state.y), 1, parameters)
    }

    pub fn possible_increase_z_action(state: &State) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("z".to_string(), Value::Int(state.z));
        Action::new(format!("increase_z_{}", state.z), 1, parameters)
    }

    pub fn possible_decrease_z_action(state: &State) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("z".to_string(), Value::Int(state.z));
        Action::new(format!("decrease_z_{}", state.z), 1, parameters)
    }

    pub fn possible_visit_action(_state: &State, loc_id: String) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("location".to_string(), Value::Text(loc_id.clone()));
        Action::new(format!("visit_{}", loc_id), 1, parameters)
    }

    pub fn possible_recharge_action(_state: &State) -> Action {
        Action::new("recharge".to_string(), 0, HashMap::new())
    }

    pub fn apply_increase_x_action(state: &State) -> State {
        let mut new_state = state.clone();
        new_state.x += 1;
        new_state.battery_level -= 1;
        new_state
    }

    pub fn apply_decrease_x_action(state: &State) -> State {
        let mut new_state = state.clone();
        new_state.x -= 1;
        new_state.battery_level -= 1;
        new_state
    }

    pub fn apply_increase_y_action(state: &State) -> State {
        let mut new_state = state.clone();
        new_state.y += 1;
        new_state.battery_level -= 1;
        new_state
    }

    pub fn apply_decrease_y_action(state: &State) -> State {
        let mut new_state = state.clone();
        new_state.y -= 1;
        new_state.battery_level -= 1;
        new_state
    }

    pub fn apply_increase_z_action(state: &State) -> State {
        let mut new_state = state.clone();
        new_state.z += 1;
        new_state.battery_level -= 1;
        new_state
    }

    pub fn apply_decrease_z_action(state: &State) -> State {
        let mut new_state = state.clone();
        new_state.z -= 1;
        new_state.battery_level -= 1;
        new_state
    }

    pub fn apply_visit_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        if let Some(Value::Text(loc_id)) = action.parameters.get("location") {
            new_state.visited.insert(loc_id.clone(), true);
            new_state.battery_level -= 1;
        }
        new_state
    }

    pub fn apply_recharge_action(state: &State, _action: &Action, battery_capacity: i32) -> State {
        let mut new_state = state.clone();
        new_state.battery_level = battery_capacity;
        new_state
    }
}

impl Problem for DroneProblem {
    type State = State;

    fn get_possible_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();

        if state.battery_level >= 1 {
            if state.x <= self.bounds.0 .1 - 1 {
                actions.push(Self::possible_increase_x_action(state));
            }
            if state.x >= self.bounds.0 .0 + 1 {
                actions.push(Self::possible_decrease_x_action(state));
            }
            if state.y <= self.bounds.1 .1 - 1 {
                actions.push(Self::possible_increase_y_action(state));
            }
            if state.y >= self.bounds.1 .0 + 1 {
                actions.push(Self::possible_decrease_y_action(state));
            }
            if state.z <= self.bounds.2 .1 - 1 {
                actions.push(Self::possible_increase_z_action(state));
            }
            if state.z >= self.bounds.2 .0 + 1 {
                actions.push(Self::possible_decrease_z_action(state));
            }

            for (loc_id, &(loc_x, loc_y, loc_z)) in &self.locations {
                if state.x == loc_x && state.y == loc_y && state.z == loc_z {
                    actions.push(Self::possible_visit_action(state, loc_id.clone()));
                }
            }
        }

        if state.x == 0 && state.y == 0 && state.z == 0 {
            actions.push(Self::possible_recharge_action(state));
        }

        actions
    }

    fn apply_action(&self, state: &State, action: &Action) -> State {
        if action.name.starts_with("increase_x") {
            Self::apply_increase_x_action(state)
        } else if action.name.starts_with("decrease_x") {
            Self::apply_decrease_x_action(state)
        } else if action.name.starts_with("increase_y") {
            Self::apply_increase_y_action(state)
        } else if action.name.starts_with("decrease_y") {
            Self::apply_decrease_y_action(state)
        } else if action.name.starts_with("increase_z") {
            Self::apply_increase_z_action(state)
        } else if action.name.starts_with("decrease_z") {
            Self::apply_decrease_z_action(state)
        } else if action.name.starts_with("visit") {
            Self::apply_visit_action(state, action)
        } else if action.name.starts_with("recharge") {
            Self::apply_recharge_action(state, action, self.battery_capacity)
        } else {
            panic!("Unknown action");
        }
    }

    fn is_goal_state(&self, state: &State) -> bool {
        state.visited.values().all(|&v| v) && state.x == 0 && state.y == 0 && state.z == 0
    }

    fn load_state_from_json(json_path: &str) -> (State, Self) {
        let json_str = fs::read_to_string(json_path).expect("Failed to read JSON file");
        let json_value: JsonValue = serde_json::from_str(&json_str).expect("Failed to parse JSON");

        let state_value = json_value
            .get("state")
            .expect("Missing 'state' field in JSON");
        let problem_value = json_value
            .get("problem")
            .expect("Missing 'problem' field in JSON");

        let state: State = serde_json::from_value(state_value.clone())
            .expect("Failed to deserialize state");
        let problem: DroneProblem = serde_json::from_value(problem_value.clone())
            .expect("Failed to deserialize problem");

        (state, problem)
    }

    fn heuristic(&self, _state: &State) -> f64 {
        0.0
    }
}