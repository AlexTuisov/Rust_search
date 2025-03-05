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
    pub sled_locations: BTreeMap<i32, i32>,        // sled_id -> waypoint_id
    pub sled_supplies: BTreeMap<i32, i32>,         // sled_id -> supplies
    pub sled_capacity: BTreeMap<i32, i32>,         // sled_id -> capacity
    pub waypoint_supplies: BTreeMap<i32, i32>,     // waypoint_id -> supplies
    pub waypoint_connections: BTreeMap<i32, Vec<i32>>, // waypoint_id -> next_waypoint_ids
}

impl StateTrait for State {}

pub struct ExpeditionProblem {
    goal_locations: BTreeMap<i32,i32>, // target locations for sleds
}

impl ExpeditionProblem {
    pub fn possible_move_forwards_action(sled: i32, from: i32, to: i32) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("sled".to_string(), Value::Int(sled));
        parameters.insert("from".to_string(), Value::Int(from));
        parameters.insert("to".to_string(), Value::Int(to));
        Action::new(format!("move_forwards_{}_{}_{}", sled, from, to), 1, parameters)
    }

    pub fn possible_move_backwards_action(sled: i32, from: i32, to: i32) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("sled".to_string(), Value::Int(sled));
        parameters.insert("from".to_string(), Value::Int(from));
        parameters.insert("to".to_string(), Value::Int(to));
        Action::new(format!("move_backwards_{}_{}_{}", sled, from, to), 1, parameters)
    }

    pub fn possible_store_supplies_action(sled: i32, waypoint: i32) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("sled".to_string(), Value::Int(sled));
        parameters.insert("waypoint".to_string(), Value::Int(waypoint));
        Action::new(format!("store_supplies_{}_{}", sled, waypoint), 1, parameters)
    }

    pub fn possible_retrieve_supplies_action(sled: i32, waypoint: i32) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("sled".to_string(), Value::Int(sled));
        parameters.insert("waypoint".to_string(), Value::Int(waypoint));
        Action::new(format!("retrieve_supplies_{}_{}", sled, waypoint), 1, parameters)
    }

    pub fn apply_move_forwards_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let sled = match action.parameters.get("sled").unwrap() {
        Value::Int(val) => *val,
        _ => panic!("Expected integer value for sled")
        };
        let to = match action.parameters.get("to").unwrap() {
        Value::Int(val) => *val,
        _ => panic!("Expected integer value for destination")
        };
        new_state.sled_locations.insert(sled, to);
        *new_state.sled_supplies.get_mut(&sled).unwrap() -= 1;
        new_state
    }

    pub fn apply_move_backwards_action(state: &State, action: &Action) -> State {
    let mut new_state = state.clone();
    let sled = match action.parameters.get("sled").unwrap() {
        Value::Int(val) => *val,
        _ => panic!("Expected integer value for sled")
    };
    let to = match action.parameters.get("to").unwrap() {
        Value::Int(val) => *val,
        _ => panic!("Expected integer value for destination")
    };
    
    new_state.sled_locations.insert(sled, to);
    *new_state.sled_supplies.get_mut(&sled).unwrap() -= 1;
    new_state
}

pub fn apply_store_supplies_action(state: &State, action: &Action) -> State {
    let mut new_state = state.clone();
    let sled = match action.parameters.get("sled").unwrap() {
        Value::Int(val) => *val,
        _ => panic!("Expected integer value for sled")
    };
    let waypoint = match action.parameters.get("waypoint").unwrap() {
        Value::Int(val) => *val,
        _ => panic!("Expected integer value for waypoint")
    };
    
    *new_state.waypoint_supplies.get_mut(&waypoint).unwrap() += 1;
    *new_state.sled_supplies.get_mut(&sled).unwrap() -= 1;
    new_state
}

pub fn apply_retrieve_supplies_action(state: &State, action: &Action) -> State {
    let mut new_state = state.clone();
    let sled = match action.parameters.get("sled").unwrap() {
        Value::Int(val) => *val,
        _ => panic!("Expected integer value for sled")
    };
    let waypoint = match action.parameters.get("waypoint").unwrap() {
        Value::Int(val) => *val,
        _ => panic!("Expected integer value for waypoint")
    };
    
    *new_state.waypoint_supplies.get_mut(&waypoint).unwrap() -= 1;
    *new_state.sled_supplies.get_mut(&sled).unwrap() += 1;
    new_state
    }

}


impl Problem for ExpeditionProblem {
    type State = State;

    fn get_possible_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        
        for (&sled, &current_loc) in &state.sled_locations {
            // Check if we have all required data for this sled
            if let (Some(&supplies), Some(&capacity)) = (
                state.sled_supplies.get(&sled),
                state.sled_capacity.get(&sled)
            ) {
                if supplies >= 1 {
                    // Check if we have connections for current location
                    if let Some(next_waypoints) = state.waypoint_connections.get(&current_loc) {
                        for &next_waypoint in next_waypoints {
                            actions.push(Self::possible_move_forwards_action(sled, current_loc, next_waypoint));
                        }
                    }
                    
                    // Check waypoint supplies exists before store/retrieve actions
                    if let Some(&waypoint_supply) = state.waypoint_supplies.get(&current_loc) {
                        if supplies >= 1 {
                            actions.push(Self::possible_store_supplies_action(sled, current_loc));
                        }
                        
                        if waypoint_supply >= 1 && capacity > supplies {
                            actions.push(Self::possible_retrieve_supplies_action(sled, current_loc));
                        }
                    }
                }
            }
        }
        
        actions
    }


    fn apply_action(&self, state: &State, action: &Action) -> State {
        if action.name.starts_with("move_forwards") {
            Self::apply_move_forwards_action(state, action)
        } else if action.name.starts_with("move_backwards") {
            Self::apply_move_backwards_action(state, action)
        } else if action.name.starts_with("store_supplies") {
            Self::apply_store_supplies_action(state, action)
        } else if action.name.starts_with("retrieve_supplies") {
            Self::apply_retrieve_supplies_action(state, action)
        } else {
            panic!("Unknown action");
        }
    }

    fn load_state_from_json(json_path: &str) -> (State, Self) {
        let file = File::open(json_path).expect("Failed to open JSON file");
        let reader = BufReader::new(file);
        let json_data: JsonValue = from_reader(reader).expect("Failed to parse JSON");

        let state = State {
            sled_locations: json_data["sled_locations"]
                .as_object()
                .unwrap()
                .iter()
                .map(|(k, v)| (k.parse::<i32>().unwrap(), v.as_i64().unwrap() as i32))
                .collect(),
            sled_supplies: json_data["sled_supplies"]
                .as_object()
                .unwrap()
                .iter()
                .map(|(k, v)| (k.parse::<i32>().unwrap(), v.as_i64().unwrap() as i32))
                .collect(),
            sled_capacity: json_data["sled_capacity"]
                .as_object()
                .unwrap()
                .iter()
                .map(|(k, v)| (k.parse::<i32>().unwrap(), v.as_i64().unwrap() as i32))
                .collect(),
            waypoint_supplies: json_data["waypoint_supplies"]
                .as_object()
                .unwrap()
                .iter()
                .map(|(k, v)| (k.parse::<i32>().unwrap(), v.as_i64().unwrap() as i32))
                .collect(),
            waypoint_connections: json_data["waypoint_connections"]
                .as_object()
                .unwrap()
                .iter()
                .map(|(k, v)| {
                    (k.parse::<i32>().unwrap(),
                    v.as_array().unwrap().iter().map(|x| x.as_i64().unwrap() as i32).collect())
                })
                .collect(),
        };

    let goal_locations: BTreeMap<i32, i32> = json_data["goal_locations"]
        .as_object()
        .unwrap()
        .iter()
        .map(|(k, v)| (k.parse::<i32>().unwrap(), v.as_i64().unwrap() as i32))
        .collect();

    (state, Self { goal_locations })
}



    fn is_goal_state(&self, state: &State) -> bool {
    self.goal_locations.iter().all(|(&sled, &target_loc)| {
        state.sled_locations.get(&sled) == Some(&target_loc)
        })
    }


    fn heuristic(&self, state: &State) -> f64 {
        0.0 // Simple heuristic can be implemented based on distance to goals
    }
}
