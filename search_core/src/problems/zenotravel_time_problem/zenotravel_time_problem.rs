use crate::problems::problem::Problem;
use crate::search::{action::Action, state::StateTrait, state::Value};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::fs;

// State holds current airplanes and persons info
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub airplanes: Vec<Airplane>, // list of airplanes and their attributes
    pub persons: Vec<Person>,     // list of persons and their current status
}

impl State {}
impl StateTrait for State {}

// Aircraft characteristics and status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Airplane {
    pub index: i32,      // unique airplane identifier
    pub slow_burn: i32,  // fuel consumed per distance unit when flying slow
    pub slow_speed: i32, // distance covered per time unit when flying slow
    pub fast_burn: i32,  // fuel consumed per distance unit when flying fast
    pub fast_speed: i32, // distance covered per time unit when flying fast
    pub capacity: i32,   // max fuel capacity
    pub fuel: i32,       // current fuel level
    pub location: i32,   // current city index
    pub zoom_limit: i32, // max passengers allowed for fast flight
    pub onboard: i32,    // number of passengers currently onboard
}

// Person status: ground location or onboard
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Person {
    pub location: i32,    // city index if on ground, -1 if onboard
    pub on_airplane: i32, // airplane index if onboard, -1 if on ground
}

// Goal positions for airplanes and persons
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Goal {
    pub airplanes: Vec<(i32, i32)>, // (airplane_index, target_city_index)
    pub persons: Vec<(i32, i32)>,   // (person_index, target_city_index)
}

// Problem definition: goals and map distances
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZenoTravelTimeProblem {
    pub goal: Goal,                                  // desired end-state
    pub num_cities: i32,                             // total city count
    pub distances: HashMap<String, Vec<(i32, i32)>>, // location -> [(city_index, distance)]
}

impl ZenoTravelTimeProblem {
    /// Refuel actions: refill fuel to capacity for each airplane
    pub fn get_refuel_actions(state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for airplane in &state.airplanes {
            let mut params = HashMap::new();
            let name = format!("refuel_airplane{}", airplane.index);
            params.insert("airplane".to_string(), Value::Int(airplane.index));
            // cost = 1 for refueling
            actions.push(Action::new(name, 1, params));
        }
        actions
    }

    /// Boarding actions: for each person on ground and matching airplane location
    pub fn get_possible_board_persons(state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for (i, person) in state.persons.iter().enumerate() {
            if person.on_airplane == -1 {
                for airplane in &state.airplanes {
                    if airplane.location == person.location {
                        let mut params = HashMap::new();
                        let name = format!("board_person{}_to_airplane{}", i, airplane.index);
                        params.insert("person".to_string(), Value::Int(i as i32));
                        params.insert("airplane".to_string(), Value::Int(airplane.index));
                        // cost = 1 for boarding
                        actions.push(Action::new(name, 1, params));
                    }
                }
            }
        }
        actions
    }

    /// Debarking actions: for each person onboard matching airplane
    pub fn get_possible_debark_persons(state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for (i, person) in state.persons.iter().enumerate() {
            if person.on_airplane != -1 {
                for airplane in &state.airplanes {
                    if airplane.index == person.on_airplane {
                        let mut params = HashMap::new();
                        let name = format!("debark_person{}_from_airplane{}", i, airplane.index);
                        params.insert("person".to_string(), Value::Int(i as i32));
                        params.insert("airplane".to_string(), Value::Int(airplane.index));
                        params.insert(
                            "airplane_location".to_string(),
                            Value::Int(airplane.location),
                        );
                        // cost = 1 for debarking
                        actions.push(Action::new(name, 1, params));
                    }
                }
            }
        }
        actions
    }

    /// Flight actions: cost = travel time (distance / speed)
    pub fn get_possible_airplane_flys(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for airplane in &state.airplanes {
            if let Some(conns) = self.distances.get(&airplane.location.to_string()) {
                for &(city, dist) in conns {
                    // slow flight if fuel sufficient
                    if airplane.fuel >= airplane.slow_burn * dist {
                        let mut params = HashMap::new();
                        let name = format!(
                            "fly_slow_airplane{}_from{}_to_city{}",
                            airplane.index, airplane.location, city
                        );
                        params.insert("airplane".to_string(), Value::Int(airplane.index));
                        params.insert("to".to_string(), Value::Int(city));
                        params.insert("distance".to_string(), Value::Int(dist));
                        let time = dist / airplane.slow_speed; // travel time
                        actions.push(Action::new(name, time, params));
                    }
                    // fast flight if onboard within limit and fuel sufficient
                    if airplane.fuel >= airplane.fast_burn * dist
                        && airplane.onboard <= airplane.zoom_limit
                    {
                        let mut params = HashMap::new();
                        let name = format!(
                            "fly_fast_airplane{}_from{}_to_city{}",
                            airplane.index, airplane.location, city
                        );
                        params.insert("airplane".to_string(), Value::Int(airplane.index));
                        params.insert("to".to_string(), Value::Int(city));
                        params.insert("distance".to_string(), Value::Int(dist));
                        let time = dist / airplane.fast_speed; // travel time
                        actions.push(Action::new(name, time, params));
                    }
                }
            }
        }
        actions
    }

    /// Apply refuel: refill fuel to capacity
    pub fn apply_refuel_action(state: &State, action: &Action) -> State {
        let mut ns = state.clone();
        let idx = action
            .parameters
            .get("airplane")
            .and_then(|v| match v {
                Value::Int(i) => Some(*i as usize),
                _ => None,
            })
            .expect("Missing airplane idx");
        ns.airplanes[idx].fuel = ns.airplanes[idx].capacity;
        ns
    }

    /// Apply boarding: update person.on_airplane and airplane.onboard
    pub fn apply_board_action(state: &State, action: &Action) -> State {
        let mut ns = state.clone();
        let plane = action
            .parameters
            .get("airplane")
            .and_then(|v| match v {
                Value::Int(i) => Some(*i as usize),
                _ => None,
            })
            .unwrap();
        let pers = action
            .parameters
            .get("person")
            .and_then(|v| match v {
                Value::Int(i) => Some(*i as usize),
                _ => None,
            })
            .unwrap();
        ns.persons[pers].on_airplane = plane as i32;
        ns.persons[pers].location = -1;
        ns.airplanes[plane].onboard += 1;
        ns
    }

    /// Apply debark: update person.location and airplane.onboard
    pub fn apply_debark_action(state: &State, action: &Action) -> State {
        let mut ns = state.clone();
        let plane = action
            .parameters
            .get("airplane")
            .and_then(|v| match v {
                Value::Int(i) => Some(*i as usize),
                _ => None,
            })
            .unwrap();
        let pers = action
            .parameters
            .get("person")
            .and_then(|v| match v {
                Value::Int(i) => Some(*i as usize),
                _ => None,
            })
            .unwrap();
        let loc = action
            .parameters
            .get("airplane_location")
            .and_then(|v| match v {
                Value::Int(i) => Some(*i),
                _ => None,
            })
            .unwrap();
        ns.persons[pers].on_airplane = -1;
        ns.persons[pers].location = loc;
        ns.airplanes[plane].onboard -= 1;
        ns
    }

    /// Apply fast flight: move airplane and deduct fast fuel
    pub fn apply_fast_fly_action(state: &State, action: &Action) -> State {
        let mut ns = state.clone();
        let dist = action
            .parameters
            .get("distance")
            .and_then(|v| match v {
                Value::Int(i) => Some(*i),
                _ => None,
            })
            .unwrap();
        let idx = action
            .parameters
            .get("airplane")
            .and_then(|v| match v {
                Value::Int(i) => Some(*i as usize),
                _ => None,
            })
            .unwrap();
        let to = action
            .parameters
            .get("to")
            .and_then(|v| match v {
                Value::Int(i) => Some(*i),
                _ => None,
            })
            .unwrap();
        ns.airplanes[idx].location = to;
        ns.airplanes[idx].fuel -= ns.airplanes[idx].fast_burn * dist;
        ns
    }

    /// Apply slow flight: move airplane and deduct slow fuel
    pub fn apply_slow_fly_action(state: &State, action: &Action) -> State {
        let mut ns = state.clone();
        let dist = action
            .parameters
            .get("distance")
            .and_then(|v| match v {
                Value::Int(i) => Some(*i),
                _ => None,
            })
            .unwrap();
        let idx = action
            .parameters
            .get("airplane")
            .and_then(|v| match v {
                Value::Int(i) => Some(*i as usize),
                _ => None,
            })
            .unwrap();
        let to = action
            .parameters
            .get("to")
            .and_then(|v| match v {
                Value::Int(i) => Some(*i),
                _ => None,
            })
            .unwrap();
        ns.airplanes[idx].location = to;
        ns.airplanes[idx].fuel -= ns.airplanes[idx].slow_burn * dist;
        ns
    }
}

impl Problem for ZenoTravelTimeProblem {
    type State = State;

    /// Gather all available actions for current state
    fn get_possible_actions(&self, state: &State) -> Vec<Action> {
        let mut acts = Vec::new();
        acts.extend(Self::get_refuel_actions(state));
        acts.extend(self.get_possible_airplane_flys(state));
        acts.extend(Self::get_possible_board_persons(state));
        acts.extend(Self::get_possible_debark_persons(state));
        acts
    }

    /// Dispatch to appropriate apply function based on action name
    fn apply_action(&self, state: &State, action: &Action) -> State {
        let name = &action.name;
        if name.starts_with("refuel_") {
            Self::apply_refuel_action(state, action)
        } else if name.starts_with("board_") {
            Self::apply_board_action(state, action)
        } else if name.starts_with("debark_") {
            Self::apply_debark_action(state, action)
        } else if name.starts_with("fly_fast_") {
            Self::apply_fast_fly_action(state, action)
        } else if name.starts_with("fly_slow_") {
            Self::apply_slow_fly_action(state, action)
        } else {
            panic!("Unknown action type: {}", name)
        }
    }

    /// Check if current state meets all goal requirements
    fn is_goal_state(&self, state: &State) -> bool {
        for &(ai, city) in &self.goal.airplanes {
            if state.airplanes[ai as usize].location != city {
                return false;
            }
        }
        for &(pi, city) in &self.goal.persons {
            let p = &state.persons[pi as usize];
            if p.on_airplane != -1 || p.location != city {
                return false;
            }
        }
        true
    }

    fn heuristic(&self, _state: &State) -> f64 {
        0.0 // placeholder
    }

    /// Load state and problem from JSON
    fn load_state_from_json(json_path: &str) -> (State, ZenoTravelTimeProblem) {
        let js = fs::read_to_string(json_path).expect("Failed to read JSON file");
        let val: JsonValue = serde_json::from_str(&js).expect("Failed to parse JSON");
        let st = val.get("state").expect("Missing 'state'");
        let pr = val.get("problem").expect("Missing 'problem'");
        let state: State = serde_json::from_value(st.clone()).expect("Failed to deserialize state");
        let problem: ZenoTravelTimeProblem =
            serde_json::from_value(pr.clone()).expect("Failed to deserialize problem");
        (state, problem)
    }
}
