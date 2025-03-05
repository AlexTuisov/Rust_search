use crate::problems::problem::Problem;
use crate::search::{action::Action, state::StateTrait, state::Value};
use serde::{Deserialize, Serialize};
use serde_json::from_reader;
use serde_json::Value as JsonValue;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub battery_level: i32,
    pub visited: BTreeMap<i32, bool>,
    pub locations: BTreeMap<i32, (i32, i32, i32)>,
    pub bounds: ((i32, i32), (i32, i32), (i32, i32)),
}

impl StateTrait for State {}

pub struct DroneProblem {
    battery_capacity: i32,
}

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

    pub fn possible_visit_action(state: &State, loc_id: i32) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("location".to_string(), Value::Int(loc_id));
        Action::new(format!("visit_{}", loc_id), 1, parameters)
    }

    pub fn possible_recharge_action(state: &State) -> Action {
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
        if let Some(Value::Int(loc_id)) = action.parameters.get("location") {
            new_state.visited.insert(*loc_id, true);
            new_state.battery_level -= 1;
        }
        new_state
    }

    pub fn apply_recharge_action(state: &State, action: &Action, battery_capacity: i32) -> State {
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
            if state.x <= state.bounds.0.1 - 1 {
                actions.push(Self::possible_increase_x_action(state));
            }
            if state.x >= state.bounds.0.0 + 1 {
                actions.push(Self::possible_decrease_x_action(state));
            }
            if state.y <= state.bounds.1.1 - 1 {
                actions.push(Self::possible_increase_y_action(state));
            }
            if state.y >= state.bounds.1.0 + 1 {
                actions.push(Self::possible_decrease_y_action(state));
            }
            if state.z <= state.bounds.2.1 - 1 {
                actions.push(Self::possible_increase_z_action(state));
            }
            if state.z >= state.bounds.2.0 + 1 {
                actions.push(Self::possible_decrease_z_action(state));
            }
            
            for (loc_id, &(loc_x, loc_y, loc_z)) in &state.locations {
                if state.x == loc_x && state.y == loc_y && state.z == loc_z {
                    actions.push(Self::possible_visit_action(state, *loc_id));
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
        state.visited.values().all(|&v| v) && 
        state.x == 0 && state.y == 0 && state.z == 0
    }

    fn load_state_from_json(json_path: &str) -> (State, Self) {
        let file = File::open(json_path).expect("Failed to open JSON file");
        let reader = BufReader::new(file);
        let json_data: JsonValue = from_reader(reader).expect("Failed to parse JSON");

        let state = State {
            x: 0,
            y: 0,
            z: 0,
            battery_level: json_data["battery_level"].as_i64().unwrap() as i32,
            visited: json_data["visited"]
                .as_object()
                .unwrap()
                .iter()
                .map(|(k, v)| (k.parse::<i32>().unwrap(), v.as_bool().unwrap()))
                .collect(),
            locations: json_data["locations"]
                .as_object()
                .unwrap()
                .iter()
                .map(|(k, v)| {
                    let coords = v.as_array().unwrap();
                    (k.parse::<i32>().unwrap(), 
                    (coords[0].as_i64().unwrap() as i32,
                    coords[1].as_i64().unwrap() as i32,
                    coords[2].as_i64().unwrap() as i32))
                })
                .collect(),
            bounds: (
                (json_data["min_x"].as_i64().unwrap() as i32, json_data["max_x"].as_i64().unwrap() as i32),
                (json_data["min_y"].as_i64().unwrap() as i32, json_data["max_y"].as_i64().unwrap() as i32),
                (json_data["min_z"].as_i64().unwrap() as i32, json_data["max_z"].as_i64().unwrap() as i32)
            ),
        };

        let battery_capacity = json_data["battery_capacity"].as_i64().unwrap() as i32;
        (state, Self { battery_capacity })
    }


    fn heuristic(&self, state: &State) -> f64 {
        // let mut min_distance = f64::MAX;
        
        // for (loc_id, &(loc_x, loc_y, loc_z)) in &state.locations {
        //     if !state.visited.get(loc_id).unwrap_or(&false) {
        //         let distance = ((state.x - loc_x).abs() + 
        //                       (state.y - loc_y).abs() + 
        //                       (state.z - loc_z).abs()) as f64;
        //         min_distance = min_distance.min(distance);
        //     }
        // }
        
        // if min_distance == f64::MAX || state.battery_level <= min_distance as i32 {
        //     min_distance = (state.x.abs() + state.y.abs() + state.z.abs()) as f64;
        // }
        
        // min_distance
        0.0
    }
}
