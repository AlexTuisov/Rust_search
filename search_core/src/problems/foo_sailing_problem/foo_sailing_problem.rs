use crate::problems::problem::Problem;
use crate::search::{action::Action, state::StateTrait, state::Value};
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, Value as JsonValue};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use ordered_float::OrderedFloat;


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Boat {
    pub x: OrderedFloat<f64>,
    pub y: OrderedFloat<f64>,
    pub v: OrderedFloat<f64>,
    pub index: i32,
}

impl Boat {
    /// Creates a new Boat with the given parameters.
    pub fn new(x: f64, y: f64, v: f64, index: i32) -> Self {
        Boat {
            x: OrderedFloat(x),
            y: OrderedFloat(y),
            v: OrderedFloat(v),
            index,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Person {
    pub d: OrderedFloat<f64>,
    pub saved: bool,
    pub index: i32,
}

impl Person {
    /// Creates a new Person with the given parameters.
    pub fn new(d: f64, saved: bool, index: i32) -> Self {
        Person {
            d: OrderedFloat(d),
            saved,
            index,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub boats: Vec<Boat>,
    pub persons: Vec<Person>,
}

impl State {}

impl StateTrait for State {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FooSailingProblem {}

impl FooSailingProblem {
    pub fn get_north_east_action(boat: &Boat) -> Action {
        let mut parameters = std::collections::HashMap::new();
        let action_name = format!("boat{}_sailed_north_east", boat.index);
        parameters.insert("sail".to_string(), Value::Text("north_east".to_string()));
        parameters.insert("boat".to_string(), Value::Int(boat.index));
        Action::new(action_name, 1, parameters)
    }

    pub fn get_north_west_action(boat: &Boat) -> Action {
        let mut parameters = std::collections::HashMap::new();
        let action_name = format!("boat{}_sailed_north_west", boat.index);
        parameters.insert("sail".to_string(), Value::Text("north_west".to_string()));
        parameters.insert("boat".to_string(), Value::Int(boat.index));
        Action::new(action_name, 1, parameters)
    }
    pub fn get_est_action(boat: &Boat) -> Action {
        let mut parameters = std::collections::HashMap::new();
        let action_name = format!("boat{}_sailed_est", boat.index);
        parameters.insert("sail".to_string(), Value::Text("est".to_string()));
        parameters.insert("boat".to_string(), Value::Int(boat.index));
        Action::new(action_name, 1, parameters)
    }
    pub fn get_west_action(boat: &Boat) -> Action {
        let mut parameters = std::collections::HashMap::new();
        let action_name = format!("boat{}_sailed_west", boat.index);
        parameters.insert("sail".to_string(), Value::Text("west".to_string()));
        parameters.insert("boat".to_string(), Value::Int(boat.index));
        Action::new(action_name, 1, parameters)
    }
    pub fn get_south_west_action(boat: &Boat) -> Action {
        let mut parameters = std::collections::HashMap::new();
        let action_name = format!("boat{}_sailed_south_west", boat.index);
        parameters.insert("sail".to_string(), Value::Text("south_west".to_string()));
        parameters.insert("boat".to_string(), Value::Int(boat.index));
        Action::new(action_name, 1, parameters)
    }

    pub fn get_south_east_action(boat: &Boat) -> Action {
        let mut parameters = std::collections::HashMap::new();
        let action_name = format!("boat{}_sailed_south_east", boat.index);
        parameters.insert("sail".to_string(), Value::Text("south_east".to_string()));
        parameters.insert("boat".to_string(), Value::Int(boat.index));
        Action::new(action_name, 1, parameters)
    }
    pub fn get_south_action(boat: &Boat) -> Action {
        let mut parameters = std::collections::HashMap::new();
        let action_name = format!("boat{}_sailed_south", boat.index);
        parameters.insert("sail".to_string(), Value::Text("south".to_string()));
        parameters.insert("boat".to_string(), Value::Int(boat.index));
        Action::new(action_name, 1, parameters)
    }
    pub fn get_accelerate_action(boat: &Boat) -> Action {
        let mut parameters = std::collections::HashMap::new();
        let action_name = format!("boat{}_accelerate", boat.index);
        parameters.insert(
            "accelerate".to_string(),
            Value::Text("accelerate".to_string()),
        );
        parameters.insert("boat".to_string(), Value::Int(boat.index));
        Action::new(action_name, 1, parameters)
    }

    pub fn get_decelerate_action(boat: &Boat) -> Action {
        let mut parameters = std::collections::HashMap::new();
        let action_name = format!("boat{}_decelerate", boat.index);
        parameters.insert(
            "decelerate".to_string(),
            Value::Text("decelerate".to_string()),
        );
        parameters.insert("boat".to_string(), Value::Int(boat.index));
        Action::new(action_name, 1, parameters)
    }

    pub fn get_save_person_action(boat: &Boat, person: &Person) -> Action {
        let mut parameters = std::collections::HashMap::new();
        let action_name = format!("person{}_saved_by_boat{}", person.index, boat.index);
        parameters.insert(
            "save_person".to_string(),
            Value::Text("save_person".to_string()),
        );
        parameters.insert("boat".to_string(), Value::Int(boat.index));
        parameters.insert("person".to_string(), Value::Int(person.index));
        Action::new(action_name, 1, parameters)
    }

    pub fn get_actions(state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for boat in &state.boats {
            actions.push(Self::get_north_east_action(boat));
            actions.push(Self::get_north_west_action(boat));
            actions.push(Self::get_est_action(boat));
            actions.push(Self::get_west_action(boat));
            actions.push(Self::get_south_west_action(boat));
            actions.push(Self::get_south_east_action(boat));
            actions.push(Self::get_south_action(boat));
            if (boat.v + ordered_float::OrderedFloat(1.0)) <= ordered_float::OrderedFloat(3.0) {
                actions.push(Self::get_accelerate_action(boat));
            }
            if (boat.v - ordered_float::OrderedFloat(1.0)) >= ordered_float::OrderedFloat(1.0) {
                actions.push(Self::get_decelerate_action(boat));
            }

            for person in &state.persons {
                if !person.saved {
                    if boat.x + boat.y >= person.d
                        && boat.y - boat.x >= person.d
                        && boat.x + boat.y <= person.d + ordered_float::OrderedFloat(25.0)
                        && boat.y - boat.x <= person.d + ordered_float::OrderedFloat(25.0)
                        && boat.v <= ordered_float::OrderedFloat(1.0)
                    {
                        actions.push(Self::get_save_person_action(boat, person));
                    }
                }
            }
        }

        actions
    }
    pub fn apply_north_east_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let boat_index = match action.parameters.get("boat") {
            Some(Value::Int(i)) => *i as usize, // Dereference and cast to usize.
            _ => panic!("Action parameters do not contain a valid boat index."),
        };
        let v_val = new_state.boats[boat_index].v;
        new_state.boats[boat_index].x +=v_val* ordered_float::OrderedFloat(1.5);
        new_state.boats[boat_index].y += v_val*ordered_float::OrderedFloat(1.5);

        new_state
    }
    pub fn apply_north_west_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let boat_index = match action.parameters.get("boat") {
            Some(Value::Int(i)) => *i as usize, // Dereference and cast to usize.
            _ => panic!("Action parameters do not contain a valid boat index."),
        };
        let v_val = new_state.boats[boat_index].v;
        new_state.boats[boat_index].x += v_val  * ordered_float::OrderedFloat(1.5);
        new_state.boats[boat_index].y -= v_val  * ordered_float::OrderedFloat(1.5);

        new_state
    }
    pub fn apply_est_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let boat_index = match action.parameters.get("boat") {
            Some(Value::Int(i)) => *i as usize, // Dereference and cast to usize.
            _ => panic!("Action parameters do not contain a valid boat index."),
        };
        let v_val = new_state.boats[boat_index].v;
        new_state.boats[boat_index].x += v_val * ordered_float::OrderedFloat(3.0);

        new_state
    }
    pub fn apply_west_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let boat_index = match action.parameters.get("boat") {
            Some(Value::Int(i)) => *i as usize, // Dereference and cast to usize.
            _ => panic!("Action parameters do not contain a valid boat index."),
        };
        let v_val = new_state.boats[boat_index].v;
        new_state.boats[boat_index].x -= v_val * ordered_float::OrderedFloat(3.0);

        new_state
    }
    pub fn apply_south_west_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let boat_index = match action.parameters.get("boat") {
            Some(Value::Int(i)) => *i as usize, // Dereference and cast to usize.
            _ => panic!("Action parameters do not contain a valid boat index."),
        };
        let v_val = new_state.boats[boat_index].v;
        new_state.boats[boat_index].x += v_val * ordered_float::OrderedFloat(2.0);
        new_state.boats[boat_index].y -= v_val * ordered_float::OrderedFloat(2.0);

        new_state
    }
    pub fn apply_south_east_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let boat_index = match action.parameters.get("boat") {
            Some(Value::Int(i)) => *i as usize, // Dereference and cast to usize.
            _ => panic!("Action parameters do not contain a valid boat index."),
        };
        let v_val = new_state.boats[boat_index].v; 

        new_state.boats[boat_index].x -= v_val * ordered_float::OrderedFloat(2.0);
        new_state.boats[boat_index].y -= v_val * ordered_float::OrderedFloat(2.0);

        new_state
    }
    pub fn apply_south_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let boat_index = match action.parameters.get("boat") {
            Some(Value::Int(i)) => *i as usize, // Dereference and cast to usize.
            _ => panic!("Action parameters do not contain a valid boat index."),
        };
        let v_val = new_state.boats[boat_index].v; 
        new_state.boats[boat_index].y -= v_val * ordered_float::OrderedFloat(2.0);

        new_state
    }
    pub fn apply_accelerate_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let boat_index = match action.parameters.get("boat") {
            Some(Value::Int(i)) => *i as usize, // Dereference and cast to usize.
            _ => panic!("Action parameters do not contain a valid boat index."),
        };

        new_state.boats[boat_index].v += ordered_float::OrderedFloat(1.0);

        new_state
    }
    pub fn apply_decelerate_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let boat_index = match action.parameters.get("boat") {
            Some(Value::Int(i)) => *i as usize, // Dereference and cast to usize.
            _ => panic!("Action parameters do not contain a valid boat index."),
        };

        new_state.boats[boat_index].v -= ordered_float::OrderedFloat(1.0);

        new_state
    }

    pub fn apply_save_person_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let person_index = match action.parameters.get("person") {
            Some(Value::Int(i)) => *i as usize, // Dereference and cast to usize.
            _ => panic!("Action parameters do not contain a valid person index."),
        };
        new_state.persons[person_index].saved = true;

        new_state
    }
}

impl Problem for FooSailingProblem {
    type State = State;

    fn get_possible_actions(&self, state: &State) -> Vec<Action> {
        Self::get_actions(state)
    }

    fn apply_action(&self, state: &State, action: &Action) -> State {
        // Start by cloning the current state.
        let mut new_state = state.clone();

        // Process the "sail" parameter if present.
        if let Some(Value::Text(direction)) = action.parameters.get("sail") {
            match direction.as_str() {
                "north_east" => new_state = Self::apply_north_east_action(state, action),
                "north_west" => new_state = Self::apply_north_west_action(state, action),
                "est" => new_state = Self::apply_est_action(state, action),
                "west" => new_state = Self::apply_west_action(state, action),
                "south_west" => new_state = Self::apply_south_west_action(state, action),
                "south_east" => new_state = Self::apply_south_east_action(state, action),
                "south" => new_state = Self::apply_south_action(state, action),
                _ => {} // Unknown sail direction: ignore or log if needed.
            }
        }

        // Process the "decelerate" parameter if present.
        if let Some(Value::Text(param)) = action.parameters.get("decelerate") {
            if param == "decelerate" {
                new_state = Self::apply_decelerate_action(state, action);
            }
        }

        // Process the "accelerate" parameter if present.
        if let Some(Value::Text(param)) = action.parameters.get("accelerate") {
            if param == "accelerate" {
                new_state = Self::apply_accelerate_action(state, action);
            }
        }

        // Process the "save_person" parameter if present.
        if let Some(Value::Text(_)) = action.parameters.get("save_person") {
            new_state = Self::apply_save_person_action(state, action);
        }

        new_state
    }

    fn is_goal_state(&self, state: &State) -> bool {
        for person in &state.persons {
            if !person.saved {
              return false;
            }
        }
        true
    }

    fn heuristic(&self, state: &State) -> f64 {
        // heuristic is imported during build time from include!("refined_heuristic.in")
        //heuristic(self, state)
        0.0
    }

    fn load_state_from_json(json_path: &str) -> (State, Self) {
        // Open the file and create a buffered reader.
        let file = File::open(json_path).expect("Failed to open JSON file");
        let reader = BufReader::new(file);
        // Parse the JSON using from_reader.
        let json_data: JsonValue = serde_json::from_reader(reader).expect("Failed to parse JSON");

        // Treat json_data as the problem data.
        let problem_data = json_data.as_object().expect("Expected a JSON object");

        // Create boats from the "boats" array.
        let mut boats = Vec::new();
        if let Some(boats_json) = problem_data.get("boats").and_then(|v| v.as_array()) {
            for boat in boats_json {
                let x = boat
                    .get("x")
                    .and_then(|v| v.as_f64())
                    .expect("Missing or invalid 'x' for a boat");
                let y = boat
                    .get("y")
                    .and_then(|v| v.as_f64())
                    .expect("Missing or invalid 'y' for a boat");
                let v_val = boat
                    .get("v")
                    .and_then(|v| v.as_f64())
                    .expect("Missing or invalid 'v' for a boat");
                let index = boat
                    .get("index")
                    .and_then(|v| v.as_u64())
                    .expect("Missing or invalid 'index' for a boat")
                    as i32;
                boats.push(Boat {
                    x : ordered_float::OrderedFloat(x),
                    y : ordered_float::OrderedFloat(y),
                    v: ordered_float::OrderedFloat(v_val),
                    index,
                });
            }
        } else {
            panic!("Missing 'boats' array in JSON");
        }

        // Create persons from the "persons" array.
        let mut persons = Vec::new();
        if let Some(persons_json) = problem_data.get("persons").and_then(|v| v.as_array()) {
            for person in persons_json {
                let d = person
                    .get("d")
                    .and_then(|v| v.as_f64())
                    .expect("Missing or invalid 'd' for a person");
                let saved = person
                    .get("saved")
                    .and_then(|v| v.as_bool())
                    .expect("Missing or invalid 'saved' for a person");
                let index = person
                    .get("index")
                    .and_then(|v| v.as_u64())
                    .expect("Missing or invalid 'index' for a person")
                    as i32;
                persons.push(Person { d : ordered_float::OrderedFloat(d), saved, index });
            }
        } else {
            panic!("Missing 'persons' array in JSON");
        }

        // Build the dynamic state.
        let state = State { boats, persons };

        (state, Self {})
    }
}
