use std::collections::HashMap;
use crate::problems::problem::Problem;
use crate::search::{action::Action, state::StateTrait, state::Value};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::fs;
use ordered_float::OrderedFloat;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    /// Number of cities in the problem.
    pub num_cities: i32,

    pub airplanes: Vec<Airplane>,

    /// Distances between cities.
    /// The key is a tuple (city1, city2) and the value is the distance between them.
    pub distances: HashMap<(i32, i32), i32>,

    //The persons in the problem
    pub persons: Vec<Person>,

    /// The total fuel used so far.
    pub total_fuel_used: i32,

    /// The total time elapsed (this value can be a float).
    pub total_time: OrderedFloat<f64>,
}

impl State {}

impl StateTrait for State {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Airplane {
    /// Unique identifier for the airplane.
    pub index: i32,

    /// Slow attributes.
    pub slow_burn: i32,
    pub slow_speed: i32,

    /// Fast attributes.
    pub fast_burn: i32,
    pub fast_speed: i32,

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
    /// -1 if he is on airplane (i.e in a airplane)
    pub loc: i32,
    /// -1 if on the ground (i.e. in a city)
    pub on_airpane: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Goal {
    pub airplanes: Vec<(i32, i32)>,
    pub persons: Vec<(i32, i32)>,
    pub minize: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZenoTravelProblem {
    pub goal: Goal,
}

impl ZenoTravelProblem {
    pub fn get_refuel_actions(state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        // Iterate directly over each airplane in the vector.
        for airplane in &state.airplanes {
            let mut parameters = std::collections::HashMap::new();
            let action_name = format!("refuel_airplane{}", airplane.index);
            parameters.insert("refuel".to_string(), Value::Text("refuel".to_string()));
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
            if person.on_airpane == -1 {
                // Iterate over each airplane in the vector.
                for airplane in &state.airplanes {
                    // Check if the airplane is at the same location as the person.
                    if airplane.location == person.loc {
                        let mut parameters = std::collections::HashMap::new();
                        // Create an action name using the person's index (j) and the airplane's unique index.
                        let action_name =
                            format!("board_person{}_to_airplane{}", j, airplane.index);
                        parameters.insert("board".to_string(), Value::Text("board".to_string()));
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
                        parameters.insert("debark".to_string(), Value::Text("debark".to_string()));
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

    pub fn get_possible_airplane_flys(state: &State) -> Vec<Action> {
        todo!("cost, minimize , AlexT");
        todo!("cost, minimize , AlexT");
        todo!("cost, minimize , AlexT");
        todo!("cost, minimize , AlexT");
        todo!("cost, minimize , AlexT");
        todo!("cost, minimize , AlexT");
        todo!("cost, minimize , AlexT");
        todo!("cost, minimize , AlexT");

        let mut actions = Vec::new();

        // Iterate over each airplane in the vector.
        for airplane in &state.airplanes {
            // Iterate over all cities (destination candidates).
            for j in 0..state.num_cities {
                // Only consider flying to a city different from the airplane's current location.
                if j != airplane.location {
                    // Check for slow flight:
                    // Condition: available fuel >= (distance * slow-burn consumption rate)
                    if airplane.fuel >= airplane.slow_burn * state.distance(&airplane.location, &j)
                    {
                        let mut parameters = std::collections::HashMap::new();
                        // Create an action name using the airplane's index, its current location, and destination city.
                        let action_name = format!(
                            "fly_slow_airplane{}_from{}_to{}",
                            airplane.index, airplane.location, j
                        );
                        parameters.insert("fly".to_string(), Value::Text("fly_slow".to_string()));
                        parameters.insert("airplane".to_string(), Value::Int(airplane.index));
                        parameters.insert("from".to_string(), Value::Int(airplane.location));
                        parameters.insert("to".to_string(), Value::Int(j));
                        parameters.insert(
                            "distance".to_string(),
                            Value::Int(state.distance(&airplane.location, &j)),
                        );
                        actions.push(Action::new(action_name, 1, parameters));
                    }

                    // Check for fast flight:
                    // Condition: available fuel >= (distance * fast-burn consumption rate)
                    //            and onboard count is less than or equal to the zoom limit.
                    if airplane.fuel >= airplane.fast_burn * state.distance(&airplane.location, &j)
                        && airplane.onboard <= airplane.zoom_limit
                    {
                        let mut parameters = std::collections::HashMap::new();
                        let action_name = format!(
                            "fly_fast_airplane{}_from{}_to{}",
                            airplane.index, airplane.location, j
                        );
                        parameters.insert("fly".to_string(), Value::Text("fly_fast".to_string()));
                        parameters.insert("airplane".to_string(), Value::Int(airplane.index));
                        parameters.insert("from".to_string(), Value::Int(airplane.location));
                        parameters.insert("to".to_string(), Value::Int(j));
                        parameters.insert(
                            "distance".to_string(),
                            Value::Int(state.distance(&airplane.location, &j)),
                        );
                        actions.push(Action::new(action_name, 1, parameters));
                    }
                }
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
            Some(Value::Int(i)) => *i as usize, // Dereference and cast to usize.
            _ => panic!("Action parameters do not contain a valid airplane index."),
        };

        let person_index = match action.parameters.get("person") {
            Some(Value::Int(i)) => *i as usize, // Dereference and cast to usize.
            _ => panic!("Action parameters do not contain a valid person index."),
        };

        // Set the airplane's fuel to its capacity.
        new_state.persons[person_index].on_airplane = airplane_index;
        new_state.persons[person_index].location = -1;
        new_state.airplanes[airplane_index].onboard += 1;
        new_state
    }

    pub fn apply_debark_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();

        let airplane_index = match action.parameters.get("airplane") {
            Some(Value::Int(i)) => *i as usize, // Dereference and cast to usize.
            _ => panic!("Action parameters do not contain a valid airplane index."),
        };

        let person_index = match action.parameters.get("person") {
            Some(Value::Int(i)) => *i as usize, // Dereference and cast to usize.
            _ => panic!("Action parameters do not contain a valid person index."),
        };

        // Set the airplane's fuel to its capacity.
        new_state.persons[person_index].on_airplane = -1;
        new_state.persons[person_index].location = new_state.airplanes[airplane_index].location;
        new_state.airplanes[airplane_index].onboard -= 1;
        new_state
    }

    pub fn apply_fast_fly_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let distance = match action.parameters.get("distance") {
            Some(Value::Int(i)) => *i as usize, // Dereference and cast to usize.
            _ => panic!("Action parameters do not contain a valid person index."),
        };

        let airplane_index = match action.parameters.get("airplane") {
            Some(Value::Int(i)) => *i as usize, // Dereference and cast to usize.
            _ => panic!("Action parameters do not contain a valid airplane index."),
        };
        new_state.airplanes[airplane_index].fuel -=
            new_state.airplanes[airplane_index].fast_burn * distance;
        new_state.total_fuel_used += new_state.airplanes[airplane_index].fast_burn * distance;
        new_state.total_time += distance / new_state.airplanes[airplane_index].fast_speed;
    }

    pub fn apply_slow_fly_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let distance = match action.parameters.get("distance") {
            Some(Value::Int(i)) => *i as usize, // Dereference and cast to usize.
            _ => panic!("Action parameters do not contain a valid person index."),
        };

        let airplane_index = match action.parameters.get("airplane") {
            Some(Value::Int(i)) => *i as usize, // Dereference and cast to usize.
            _ => panic!("Action parameters do not contain a valid airplane index."),
        };
        new_state.airplanes[airplane_index].fuel -=
            new_state.airplanes[airplane_index].slow_burn * distance;
        new_state.total_fuel_used += new_state.airplanes[airplane_index].slow_burn * distance;
        new_state.total_time += distance / new_state.airplanes[airplane_index].slow_speed;
        new_state
    }

    pub fn apply_fly_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();

        match action.parameters.get("fly") {
            Some(Value::Text(mode)) if mode == "fly_fast" => {
                new_state = apply_fast_fly_action(new_state, action);
            }
            Some(Value::Text(mode)) if mode == "fly_slow" => {
                new_state = apply_slow_fly_action(new_state, action);
            }
            _ => panic!("Action parameters do not contain a valid fly parameter."),
        }

        new_state
    }
}

impl Problem for ZenoTravelProblem {
    type State = State;

    fn get_possible_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        actions.extend(Self::get_refuel_actions(state));
        actions.extend(Self::get_possible_airplane_flys(state));
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
    /// need to check with Alex T how to do minimize
    /* fn is_goal_state(&self, state: &State) -> bool {
        for vehicle in &state.vehicle_state.vehicles {
            if vehicle.name == "red-car" {
                let positions = vehicle.positions();
                return positions.contains(&[2, state.grid.col_size - 2])
                    && positions.contains(&[2, state.grid.col_size - 1]);
            }
        }
        false
    } */
    fn heuristic(&self, state: &State) -> f64 {
        // heuristic is imported during build time from include!("refined_heuristic.in")
        //heuristic(self, state)
        0.0
    }

    fn load_state_from_json(json_path: &str) -> (State, ZenoTravelProblem) {
        todo!("create an convertor !!!!");

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
        let problem: ZenoTravelProblem =
            serde_json::from_value(problem_value.clone()).expect("Failed to deserialize problem");

        (state, problem)
    }
}
