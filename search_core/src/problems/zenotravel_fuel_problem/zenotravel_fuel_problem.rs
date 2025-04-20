use crate::problems::problem::Problem;
use crate::search::{action::Action, state::StateTrait, state::Value};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub airplanes: Vec<Airplane>,

    //The persons in the problem
    pub persons: Vec<Person>,
}

impl State {}

impl StateTrait for State {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Airplane {
    /// Unique identifier for the airplane.
    pub index: i32,

    /// Slow attributes.
    pub slow_burn: i32,

    /// Fast attributes.
    pub fast_burn: i32,

    /// Capacity attributes.
    pub capacity: i32,
    pub fuel: i32,

    /// Miscellaneous attributes.
    pub location: i32,
    pub zoom_limit: i32,
    pub onboard: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Person {
    /// -1 if he is on airplane
    pub location: i32,
    /// -1 if on the ground
    pub on_airplane: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Goal {
    pub airplanes: Vec<(i32, i32)>,
    pub persons: Vec<(i32, i32)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZenoTravelFuelProblem {
    pub goal: Goal,
    pub num_cities: i32,
    pub distances: HashMap<String, Vec<(i32, i32)>>,
}

impl ZenoTravelFuelProblem {
    pub fn get_refuel_actions(state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        // Iterate directly over each airplane in the vector.
        for airplane in &state.airplanes {
            let mut parameters = std::collections::HashMap::new();
            let action_name = format!("refuel_airplane{}", airplane.index);
            parameters.insert("airplane".to_string(), Value::Int(airplane.index));
            actions.push(Action::new(action_name, 1, parameters));
        }
        actions
    }

    pub fn get_possible_board_persons(state: &State) -> Vec<Action> {
        let mut actions = Vec::new();

        // Iterate over persons with their indices.
        for (j, person) in state.persons.iter().enumerate() {
            // Only consider persons who are on the ground (i.e. on_airpane == -1).
            if person.on_airplane == -1 {
                // Iterate over each airplane in the vector.
                for airplane in &state.airplanes {
                    // Check if the airplane is at the same location as the person.
                    if airplane.location == person.location {
                        let mut parameters = std::collections::HashMap::new();
                        // Create an action name using the person's index (j) and the airplane's unique index.
                        let action_name =
                            format!("board_person{}_to_airplane{}", j, airplane.index);
                        parameters.insert("person".to_string(), Value::Int(j as i32));
                        parameters.insert("airplane".to_string(), Value::Int(airplane.index));

                        // Add the action to the list.
                        actions.push(Action::new(action_name, 1, parameters));
                    }
                }
            }
        }

        actions
    }

    pub fn get_possible_debark_persons(state: &State) -> Vec<Action> {
        let mut actions = Vec::new();

        // Iterate over persons with their indices.
        for (j, person) in state.persons.iter().enumerate() {
            // Only consider persons who are not already on a plane.
            if person.on_airplane != -1 {
                // Iterate over each airplane in the vector.
                for airplane in &state.airplanes {
                    // Check if the airplane is at the same location as the person.
                    if airplane.index == person.on_airplane {
                        let mut parameters = std::collections::HashMap::new();
                        // Create an action name using the person's index (j) and the airplane's unique index.
                        let action_name =
                            format!("debark_person{}_from_airplane{}", j, airplane.index);
                        parameters.insert("person".to_string(), Value::Int(j as i32));
                        parameters.insert("airplane".to_string(), Value::Int(airplane.index));
                        parameters.insert(
                            "airplane_location".to_string(),
                            Value::Int(airplane.location),
                        );

                        // Create a new action with cost 1 and add it to the actions vector.
                        actions.push(Action::new(action_name, 1, parameters));
                    }
                }
            }
        }

        actions
    }

    pub fn get_possible_airplane_flys(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();

        // Iterate over each airplane in the vector.
        for airplane in &state.airplanes {
            // Iterate over all cities (destination candidates).
            // `get` returns Option<&Vec<(String,i32)>>
            if let Some(pairs) = self.distances.get(&airplane.location.to_string()) {
                // pairs: &Vec<(String,i32)>
                for (city, distance) in pairs {
                    if airplane.fuel >= airplane.slow_burn * distance {
                        let mut parameters = std::collections::HashMap::new();
                        // Create an action name using the airplane's index, its current location, and destination city.
                        let action_name = format!(
                            "fly_slow_airplane{}_from{}_to_city{}",
                            airplane.index, airplane.location, city
                        );

                        parameters.insert("airplane".to_string(), Value::Int(airplane.index));

                        parameters.insert("to".to_string(), Value::Int(*city));
                        parameters.insert("distance".to_string(), Value::Int(*distance));
                        let total_fuel = airplane.slow_burn * distance;

                        actions.push(Action::new(action_name, total_fuel, parameters));
                    }

                    // Check for fast flight:
                    // Condition: available fuel >= (distance * fast-burn consumption rate)
                    //            and onboard count is less than or equal to the zoom limit.
                    if airplane.fuel >= airplane.fast_burn * distance
                        && airplane.onboard <= airplane.zoom_limit
                    {
                        let mut parameters = std::collections::HashMap::new();
                        let action_name = format!(
                            "fly_fast_airplane{}_from{}_to_city{}",
                            airplane.index, airplane.location, city
                        );

                        parameters.insert("airplane".to_string(), Value::Int(airplane.index));

                        parameters.insert("to".to_string(), Value::Int(*city));
                        parameters.insert("distance".to_string(), Value::Int(*distance));

                        let total_fuel = airplane.fast_burn * distance;
                        actions.push(Action::new(action_name, total_fuel, parameters));
                    }
                }
            } else {
                // no entry for this location
                println!("No distances recorded for {}", airplane.location);
            }
        }

        actions
    }

    pub fn apply_refuel_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();

        let airplane_index = match action.parameters.get("airplane") {
            Some(Value::Int(i)) => *i as usize, // Dereference and cast to usize.
            _ => panic!("Action parameters do not contain a valid airplane index."),
        };

        // Set the airplane's fuel to its capacity.
        new_state.airplanes[airplane_index].fuel = new_state.airplanes[airplane_index].capacity;

        new_state
    }

    pub fn apply_board_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();

        let airplane_index = match action.parameters.get("airplane") {
            Some(Value::Int(i)) => *i as usize,
            _ => panic!("Action parameters do not contain a valid airplane index."),
        };

        let person_index = match action.parameters.get("person") {
            Some(Value::Int(i)) => *i as usize,
            _ => panic!("Action parameters do not contain a valid person index."),
        };

        new_state.persons[person_index].on_airplane = airplane_index as i32;
        new_state.persons[person_index].location = -1;
        new_state.airplanes[airplane_index].onboard += 1;
        new_state
    }

    pub fn apply_debark_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();

        let airplane_index = match action.parameters.get("airplane") {
            Some(Value::Int(i)) => *i as usize,
            _ => panic!("Action parameters do not contain a valid airplane index."),
        };

        let person_index = match action.parameters.get("person") {
            Some(Value::Int(i)) => *i as usize,
            _ => panic!("Action parameters do not contain a valid person index."),
        };

        let airplane_location = match action.parameters.get("airplane_location") {
            Some(Value::Int(i)) => *i,
            _ => panic!("Action parameters do not contain a valid person index."),
        };

        new_state.persons[person_index].on_airplane = -1;
        new_state.persons[person_index].location = airplane_location;
        new_state.airplanes[airplane_index].onboard -= 1;
        new_state
    }

    pub fn apply_fast_fly_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let distance = match action.parameters.get("distance") {
            Some(Value::Int(i)) => *i,
            _ => panic!("Action parameters do not contain a valid person index."),
        };

        let airplane_index = match action.parameters.get("airplane") {
            Some(Value::Int(i)) => *i as usize,
            _ => panic!("Action parameters do not contain a valid airplane index."),
        };

        let city_location = match action.parameters.get("to") {
            Some(Value::Int(i)) => *i,
            _ => panic!("Action parameters do not contain a valid city_location."),
        };
        new_state.airplanes[airplane_index].location = city_location;
        new_state.airplanes[airplane_index].fuel -=
            new_state.airplanes[airplane_index].fast_burn * distance;
        new_state
    }

    pub fn apply_slow_fly_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let distance = match action.parameters.get("distance") {
            Some(Value::Int(i)) => *i,
            _ => panic!("Action parameters do not contain a valid person index."),
        };

        let airplane_index = match action.parameters.get("airplane") {
            Some(Value::Int(i)) => *i as usize,
            _ => panic!("Action parameters do not contain a valid airplane index."),
        };

        let city_location = match action.parameters.get("to") {
            Some(Value::Int(i)) => *i,
            _ => panic!("Action parameters do not contain a valid city_location."),
        };
        new_state.airplanes[airplane_index].location = city_location;
        new_state.airplanes[airplane_index].fuel -=
            new_state.airplanes[airplane_index].slow_burn * distance;

        new_state
    }

    pub fn apply_fly_action(state: &State, action: &Action) -> State {
        let new_state = match action.parameters.get("fly") {
            Some(Value::Text(mode)) if mode == "fly_fast" => {
                Self::apply_fast_fly_action(state, action)
            }
            Some(Value::Text(mode)) if mode == "fly_slow" => {
                Self::apply_slow_fly_action(state, action)
            }
            _ => panic!("Action parameters do not contain a valid fly parameter."),
        };
        new_state
    }
}

impl Problem for ZenoTravelFuelProblem {
    type State = State;

    fn get_possible_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        actions.extend(Self::get_refuel_actions(state));
        actions.extend(self.get_possible_airplane_flys(state));
        actions.extend(Self::get_possible_board_persons(state));
        actions.extend(Self::get_possible_debark_persons(state));
        actions
    }

    fn apply_action(&self, state: &State, action: &Action) -> State {
        // Parse the action name to determine which apply function to call
        if action.name.starts_with("refuel_") {
            Self::apply_refuel_action(state, action)
        } else if action.name.starts_with("board_") {
            Self::apply_board_action(state, action)
        } else if action.name.starts_with("debark_") {
            Self::apply_debark_action(state, action)
        } else if action.name.starts_with("fly_") {
            Self::apply_fly_action(state, action)
        } else {
            panic!("Unknown action type: {}", action.name)
        }
    }

    fn is_goal_state(&self, state: &State) -> bool {
        // Check airplane goals: For each (airplane_index, goal_city) pair,
        // verify that the airplane's current location matches the goal city.
        for &(airplane_index, goal_city) in &self.goal.airplanes {
            if state.airplanes[airplane_index as usize].location != goal_city {
                return false;
            }
        }

        // Check person goals: For each (person_index, goal_city) pair,
        // verify that the person is on the ground (on_airpane == -1) and located in the goal city.
        for &(person_index, goal_city) in &self.goal.persons {
            let person = &state.persons[person_index as usize];
            if person.on_airplane != -1 || person.location != goal_city {
                return false;
            }
        }

        true
    }

    fn heuristic(&self, _state: &State) -> f64 {
        // heuristic is imported during build time from include!("refined_heuristic.in")
        //heuristic(self, state)
        0.0
    }

    fn load_state_from_json(json_path: &str) -> (State, ZenoTravelFuelProblem) {
        // Read the JSON file into a string.
        let json_str = fs::read_to_string(json_path).expect("Failed to read JSON file");

        // Parse the JSON string into a serde_json::Value.
        let json_value: JsonValue = serde_json::from_str(&json_str).expect("Failed to parse JSON");

        // Extract the "state" and "problem" fields.
        let state_value = json_value
            .get("state")
            .expect("Missing 'state' field in JSON");
        let problem_value = json_value
            .get("problem")
            .expect("Missing 'problem' field in JSON");

        // Deserialize each part into the corresponding struct.
        let state: State =
            serde_json::from_value(state_value.clone()).expect("Failed to deserialize state");
        let problem: ZenoTravelFuelProblem =
            serde_json::from_value(problem_value.clone()).expect("Failed to deserialize problem");

        (state, problem)
    }
}
