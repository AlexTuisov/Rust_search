use crate::problems::problem::Problem;
use crate::search::{action::Action, state::StateTrait, state::Value};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Sled {
    pub location: String,
    pub supplies: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub sleds: HashMap<String, Sled>,
    pub waypoint_supplies: HashMap<String, i32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpeditionProblem {
    goal_locations: HashMap<String, String>,
    waypoint_connections: HashMap<String, Vec<String>>,
    sled_capacity: HashMap<String, i32>
}

impl StateTrait for State {}

impl ExpeditionProblem {
    pub fn possible_move_forwards_action(&self, sled_id: &str, from: &str, to: &str) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("sled".to_string(), Value::Text(sled_id.to_string()));
        parameters.insert("from".to_string(), Value::Text(from.to_string()));
        parameters.insert("to".to_string(), Value::Text(to.to_string()));
        Action::new(format!("move_forwards_{}_{}_{}", sled_id, from, to), 1, parameters)
    }

    pub fn possible_store_supplies_action(&self, sled_id: &str, waypoint: &str) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("sled".to_string(), Value::Text(sled_id.to_string()));
        parameters.insert("waypoint".to_string(), Value::Text(waypoint.to_string()));
        Action::new(format!("store_supplies_{}_{}", sled_id, waypoint), 1, parameters)
    }

    pub fn possible_retrieve_supplies_action(&self, sled_id: &str, waypoint: &str) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("sled".to_string(), Value::Text(sled_id.to_string()));
        parameters.insert("waypoint".to_string(), Value::Text(waypoint.to_string()));
        Action::new(format!("retrieve_supplies_{}_{}", sled_id, waypoint), 1, parameters)
    }

    pub fn apply_move_forwards(&self, state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let sled_id = match action.parameters.get("sled").unwrap() {
            Value::Text(id) => id,
            _ => panic!("Expected string value for sled"),
        };
        let to = match action.parameters.get("to").unwrap() {
            Value::Text(val) => val,
            _ => panic!("Expected string value for destination"),
        };

        if let Some(sled) = new_state.sleds.get_mut(sled_id) {
            sled.location = to.clone();
            sled.supplies -= 1;
        }
        new_state
    }

    pub fn apply_store_supplies(&self, state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let sled_id = match action.parameters.get("sled").unwrap() {
            Value::Text(id) => id,
            _ => panic!("Expected string value for sled"),
        };
        let waypoint = match action.parameters.get("waypoint").unwrap() {
            Value::Text(val) => val,
            _ => panic!("Expected string value for waypoint"),
        };

        if let Some(sled) = new_state.sleds.get_mut(sled_id) {
            if let Some(waypoint_supplies) = new_state.waypoint_supplies.get_mut(waypoint) {
                *waypoint_supplies += 1;
                sled.supplies -= 1;
            }
        }
        new_state
    }

    pub fn apply_retrieve_supplies(&self, state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let sled_id = match action.parameters.get("sled").unwrap() {
            Value::Text(id) => id,
            _ => panic!("Expected string value for sled"),
        };
        let waypoint = match action.parameters.get("waypoint").unwrap() {
            Value::Text(val) => val,
            _ => panic!("Expected string value for waypoint"),
        };

        if let Some(sled) = new_state.sleds.get_mut(sled_id) {
            if let Some(waypoint_supplies) = new_state.waypoint_supplies.get_mut(waypoint) {
                if let Some(&capacity) = self.sled_capacity.get(sled_id) {
                    if sled.supplies < capacity {
                        *waypoint_supplies -= 1;
                        sled.supplies += 1;
                    }
                }
            }
        }
        new_state
    }
}

impl Problem for ExpeditionProblem {
    type State = State;

    fn get_possible_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();

        for (sled_id, sled) in &state.sleds {
            if let Some(&capacity) = self.sled_capacity.get(sled_id) {
                // Move forwards actions
                if sled.supplies >= 1 {
                    if let Some(next_waypoints) = self.waypoint_connections.get(&sled.location) {
                        for next_waypoint in next_waypoints {
                            actions.push(self.possible_move_forwards_action(
                                sled_id,
                                &sled.location,
                                next_waypoint,
                            ));
                        }
                    }
                }

                // Store supplies actions
                if sled.supplies >= 1 {
                    actions.push(self.possible_store_supplies_action(
                        sled_id,
                        &sled.location,
                    ));
                }

                // Retrieve supplies actions
                if let Some(&waypoint_supply) = state.waypoint_supplies.get(&sled.location) {
                    if waypoint_supply >= 1 && capacity > sled.supplies {
                        actions.push(self.possible_retrieve_supplies_action(
                            sled_id,
                            &sled.location,
                        ));
                    }
                }
            }
        }

        actions
    }

    fn apply_action(&self, state: &State, action: &Action) -> State {
        if action.name.starts_with("move_forwards") {
            self.apply_move_forwards(state, action)
        } else if action.name.starts_with("store_supplies") {
            self.apply_store_supplies(state, action)
        } else if action.name.starts_with("retrieve_supplies") {
            self.apply_retrieve_supplies(state, action)
        } else {
            panic!("Unknown action");
        }
    }

    fn is_goal_state(&self, state: &State) -> bool {
        self.goal_locations.iter().all(|(sled_id, target_loc)| {
            state.sleds.get(sled_id).map_or(false, |sled| sled.location == *target_loc)
        })
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
        let problem: ExpeditionProblem = serde_json::from_value(problem_value.clone())
            .expect("Failed to deserialize problem");

        (state, problem)
    }

    fn heuristic(&self, state: &State) -> f64 {
        0.0 // Simple heuristic can be implemented based on distance to goals
    }
}