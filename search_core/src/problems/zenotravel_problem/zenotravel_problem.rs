use crate::problems::problem::Problem;
use crate::search::{action::Action, state::StateTrait, state::Value};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::fs;

// State holds the current status of all airplanes and persons
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct State {
    pub airplanes: Vec<Airplane>, // list of all airplanes and their attributes
    pub persons: Vec<Person>,     // list of all persons and their current status
}

impl State {}
impl StateTrait for State {}

// Represents an airplane with fuel, capacity, speed, and passenger limits
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Airplane {
    pub index: i32,      // unique identifier for the airplane
    pub slow_burn: i32,  // fuel consumed per distance unit when flying slow
    pub slow_speed: i32, // speed (distance per time unit) when flying slow
    pub fast_burn: i32,  // fuel consumed per distance unit when flying fast
    pub fast_speed: i32, // speed when flying fast
    pub capacity: i32,   // maximum fuel capacity
    pub fuel: i32,       // current fuel level
    pub location: i32,   // current city index
    pub zoom_limit: i32, // max passengers allowed for fast flight
    pub onboard: i32,    // number of persons currently onboard
}

// Represents a person, either on the ground or in an airplane
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Person {
    pub location: i32,    // city index if on ground, -1 if onboard
    pub on_airplane: i32, // airplane index if onboard, -1 if on ground
}

// Goal specifies desired final locations for airplanes and persons
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Goal {
    pub airplanes: Vec<(i32, i32)>, // (airplane_index, goal_city_index)
    pub persons: Vec<(i32, i32)>,   // (person_index, goal_city_index)
}

// Minimization weights for fuel and time in cost calculation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MinimizeCondition {
    pub fuel: i32, // weight for total fuel consumed
    pub time: i32, // weight for total travel time
}

// Problem definition including goals, map distances, and cost weights
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZenoTravelProblem {
    pub goal: Goal,                                  // goal state requirements
    pub num_cities: i32,                             // total number of cities
    pub distances: HashMap<String, Vec<(i32, i32)>>, // map: location -> [(city_index, distance)]
    pub minimize: MinimizeCondition,                 // cost weights for fuel and time
}

impl ZenoTravelProblem {
    /// Create refuel actions for every airplane (refill fuel to capacity)
    pub fn get_refuel_actions(state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for airplane in &state.airplanes {
            let mut parameters = HashMap::new();
            let action_name = format!("refuel_airplane{}", airplane.index);
            parameters.insert("airplane".to_string(), Value::Int(airplane.index));
            // fixed cost of 1 for refueling
            actions.push(Action::new(action_name, 1, parameters));
        }
        actions
    }

    /// Generate boarding actions for persons on the ground
    pub fn get_possible_board_persons(state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for (j, person) in state.persons.iter().enumerate() {
            if person.on_airplane == -1 {
                // only if on ground
                for airplane in &state.airplanes {
                    if airplane.location == person.location {
                        let mut parameters = HashMap::new();
                        let action_name =
                            format!("board_person{}_to_airplane{}", j, airplane.index);
                        parameters.insert("person".to_string(), Value::Int(j as i32));
                        parameters.insert("airplane".to_string(), Value::Int(airplane.index));
                        // fixed cost of 1 for boarding
                        actions.push(Action::new(action_name, 1, parameters));
                    }
                }
            }
        }
        actions
    }

    /// Generate debark actions for persons onboard
    pub fn get_possible_debark_persons(state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for (j, person) in state.persons.iter().enumerate() {
            if person.on_airplane != -1 {
                // only if onboard
                for airplane in &state.airplanes {
                    if airplane.index == person.on_airplane {
                        let mut parameters = HashMap::new();
                        let action_name =
                            format!("debark_person{}_from_airplane{}", j, airplane.index);
                        parameters.insert("person".to_string(), Value::Int(j as i32));
                        parameters.insert("airplane".to_string(), Value::Int(airplane.index));
                        parameters.insert(
                            "airplane_location".to_string(),
                            Value::Int(airplane.location),
                        );
                        // fixed cost of 1 for debarking
                        actions.push(Action::new(action_name, 1, parameters));
                    }
                }
            }
        }
        actions
    }

    /// Generate flight actions (slow and fast) with cost = fuel_weight*fuel + time_weight*time
    pub fn get_possible_airplane_flys(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for airplane in &state.airplanes {
            if let Some(pairs) = self.distances.get(&airplane.location.to_string()) {
                for (city, distance) in pairs {
                    // slow flight option
                    if airplane.fuel >= airplane.slow_burn * distance {
                        let mut parameters = HashMap::new();
                        let action_name = format!(
                            "fly_slow_airplane{}_from{}_to_city{}",
                            airplane.index, airplane.location, city
                        );
                        parameters.insert("airplane".to_string(), Value::Int(airplane.index));
                        parameters.insert("to".to_string(), Value::Int(*city));
                        parameters.insert("distance".to_string(), Value::Int(*distance));
                        let total_fuel = airplane.slow_burn * distance;
                        let total_time = distance / airplane.slow_speed;
                        let cost =
                            self.minimize.fuel * total_fuel + self.minimize.time * total_time;
                        actions.push(Action::new(action_name, cost, parameters));
                    }
                    // fast flight option
                    if airplane.fuel >= airplane.fast_burn * distance
                        && airplane.onboard <= airplane.zoom_limit
                    {
                        let mut parameters = HashMap::new();
                        let action_name = format!(
                            "fly_fast_airplane{}_from{}_to_city{}",
                            airplane.index, airplane.location, city
                        );
                        parameters.insert("airplane".to_string(), Value::Int(airplane.index));
                        parameters.insert("to".to_string(), Value::Int(*city));
                        parameters.insert("distance".to_string(), Value::Int(*distance));
                        let total_fuel = airplane.fast_burn * distance;
                        let total_time = distance / airplane.fast_speed;
                        let cost =
                            self.minimize.fuel * total_fuel + self.minimize.time * total_time;
                        actions.push(Action::new(action_name, cost, parameters));
                    }
                }
            }
        }
        actions
    }

    /// Apply refuel: refill airplane fuel to capacity
    pub fn apply_refuel_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let idx = match action.parameters.get("airplane") {
            Some(Value::Int(i)) => *i as usize,
            _ => panic!("Invalid airplane index parameter."),
        };
        new_state.airplanes[idx].fuel = new_state.airplanes[idx].capacity; // refill
        new_state
    }

    /// Apply boarding: update person's on_airplane and airplane onboard count
    pub fn apply_board_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let plane_idx = match action.parameters.get("airplane") {
            Some(Value::Int(i)) => *i as usize,
            _ => panic!("Invalid airplane index parameter."),
        };
        let person_idx = match action.parameters.get("person") {
            Some(Value::Int(i)) => *i as usize,
            _ => panic!("Invalid person index parameter."),
        };
        new_state.persons[person_idx].on_airplane = plane_idx as i32;
        new_state.persons[person_idx].location = -1; // now in-air
        new_state.airplanes[plane_idx].onboard += 1; // increment onboard
        new_state
    }

    /// Apply debark: update person's location and airplane onboard count
    pub fn apply_debark_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let plane_idx = match action.parameters.get("airplane") {
            Some(Value::Int(i)) => *i as usize,
            _ => panic!("Invalid airplane index parameter."),
        };
        let person_idx = match action.parameters.get("person") {
            Some(Value::Int(i)) => *i as usize,
            _ => panic!("Invalid person index parameter."),
        };
        let loc = match action.parameters.get("airplane_location") {
            Some(Value::Int(i)) => *i,
            _ => panic!("Invalid location parameter."),
        };
        new_state.persons[person_idx].on_airplane = -1; // now on-ground
        new_state.persons[person_idx].location = loc; // set city
        new_state.airplanes[plane_idx].onboard -= 1; // decrement onboard
        new_state
    }

    /// Apply fast flight: update location and deduct fast fuel
    pub fn apply_fast_fly_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let dist = match action.parameters.get("distance") {
            Some(Value::Int(i)) => *i,
            _ => panic!("Invalid distance parameter."),
        };
        let idx = match action.parameters.get("airplane") {
            Some(Value::Int(i)) => *i as usize,
            _ => panic!("Invalid airplane parameter."),
        };
        let to = match action.parameters.get("to") {
            Some(Value::Int(i)) => *i,
            _ => panic!("Invalid destination parameter."),
        };
        new_state.airplanes[idx].location = to;
        new_state.airplanes[idx].fuel -= new_state.airplanes[idx].fast_burn * dist;
        new_state
    }

    /// Apply slow flight: update location and deduct slow fuel
    pub fn apply_slow_fly_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let dist = match action.parameters.get("distance") {
            Some(Value::Int(i)) => *i,
            _ => panic!("Invalid distance parameter."),
        };
        let idx = match action.parameters.get("airplane") {
            Some(Value::Int(i)) => *i as usize,
            _ => panic!("Invalid airplane parameter."),
        };
        let to = match action.parameters.get("to") {
            Some(Value::Int(i)) => *i,
            _ => panic!("Invalid destination parameter."),
        };
        new_state.airplanes[idx].location = to;
        new_state.airplanes[idx].fuel -= new_state.airplanes[idx].slow_burn * dist;
        new_state
    }
}

impl Problem for ZenoTravelProblem {
    type State = State;

    /// Collect all possible actions in the current state
    fn get_possible_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        actions.extend(Self::get_refuel_actions(state));
        actions.extend(self.get_possible_airplane_flys(state));
        actions.extend(Self::get_possible_board_persons(state));
        actions.extend(Self::get_possible_debark_persons(state));
        actions
    }

    /// Dispatch apply_* based on action name prefix
    fn apply_action(&self, state: &State, action: &Action) -> State {
        if action.name.starts_with("refuel_") {
            Self::apply_refuel_action(state, action)
        } else if action.name.starts_with("board_") {
            Self::apply_board_action(state, action)
        } else if action.name.starts_with("debark_") {
            Self::apply_debark_action(state, action)
        } else if action.name.starts_with("fly_fast_") {
            Self::apply_fast_fly_action(state, action)
        } else if action.name.starts_with("fly_slow_") {
            Self::apply_slow_fly_action(state, action)
        } else {
            panic!("Unknown action type: {}", action.name)
        }
    }

    /// Check if all airplanes and persons meet goal positions
    fn is_goal_state(&self, state: &State) -> bool {
        for &(idx, city) in &self.goal.airplanes {
            if state.airplanes[idx as usize].location != city {
                return false;
            }
        }
        for &(idx, city) in &self.goal.persons {
            let p = &state.persons[idx as usize];
            if p.on_airplane != -1 || p.location != city {
                return false;
            }
        }
        true
    }

    fn heuristic(&self, _state: &State) -> f64 {
        0.0 // placeholder heuristic
    }

    /// Load state and problem from JSON file
    fn load_state_from_json(json_path: &str) -> (State, ZenoTravelProblem) {
        let json_str = fs::read_to_string(json_path).expect("Failed to read JSON file");
        let json_value: JsonValue = serde_json::from_str(&json_str).expect("Failed to parse JSON");
        let state_val = json_value.get("state").expect("Missing 'state' field");
        let prob_val = json_value.get("problem").expect("Missing 'problem' field");
        let state: State =
            serde_json::from_value(state_val.clone()).expect("Failed to deserialize state");
        let problem: ZenoTravelProblem =
            serde_json::from_value(prob_val.clone()).expect("Failed to deserialize problem");
        (state, problem)
    }
}
