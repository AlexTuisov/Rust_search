use crate::problems::problem::Problem;
use crate::search::{action::Action, state::StateTrait, state::Value};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use ordered_float::OrderedFloat;
use std::fs;


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Boat {
    pub x: OrderedFloat<f64>, // x-coordinate
    pub y: OrderedFloat<f64>, // y-coordinate
    pub index: i32, // index of the boat
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Person {
    pub d: OrderedFloat<f64>, 
    pub saved: bool, 
    pub index: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub boats: Vec<Boat>,
    pub persons: Vec<Person>,
}

impl StateTrait for State {}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.boats == other.boats && self.persons == other.persons
    }
}

impl Eq for State {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SailingProblem {}

impl SailingProblem {
    pub fn go_north_east_action(boat: &Boat) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("boat".to_string(), Value::Int(boat.index));
        Action::new(format!("go_north_east_{}", boat.index), 1, parameters)
    }

    pub fn go_north_west_action(boat: &Boat) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("boat".to_string(), Value::Int(boat.index));
        Action::new(format!("go_north_west_{}", boat.index), 1, parameters)
    }

    pub fn go_east_action(boat: &Boat) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("boat".to_string(), Value::Int(boat.index));
        Action::new(format!("go_east_{}", boat.index), 1, parameters)
    }

    pub fn go_west_action(boat: &Boat) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("boat".to_string(), Value::Int(boat.index));
        Action::new(format!("go_west_{}", boat.index), 1, parameters)
    }

    pub fn go_south_west_action(boat: &Boat) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("boat".to_string(), Value::Int(boat.index));
        Action::new(format!("go_south_west_{}", boat.index), 1, parameters)
    }

    pub fn go_south_east_action(boat: &Boat) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("boat".to_string(), Value::Int(boat.index));
        Action::new(format!("go_south_east_{}", boat.index), 1, parameters)
    }

    pub fn go_south_action(boat: &Boat) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("boat".to_string(), Value::Int(boat.index));
        Action::new(format!("go_south_{}", boat.index), 1, parameters)
    }

    pub fn save_person_action(boat: &Boat, person: &Person) -> Option<Action> {
        if boat.x + boat.y >= person.d
            && boat.y - boat.x >= person.d
            && boat.x + boat.y <= person.d + OrderedFloat(25.0)
            && boat.y - boat.x <= person.d + OrderedFloat(25.0)
        {
            let mut parameters = HashMap::new();
            parameters.insert("boat".to_string(), Value::Int(boat.index));
            parameters.insert("person".to_string(), Value::Int(person.index));
            Some(Action::new(
                format!("save_person_{}_{}", boat.index, person.index),
                1,
                parameters,
            ))
        } else {
            None
        }
    }

    fn apply_go_north_east(state: &State, action: &Action) -> State {
        let boat_index = match action.parameters.get("boat").unwrap() {
            Value::Int(index) => *index,
            _ => panic!("Expected boat index to be an integer"),
        };
        let mut new_state = state.clone();
        if let Some(boat) = new_state.boats.iter_mut().find(|b| b.index == boat_index) {
            boat.x += OrderedFloat(1.5);
            boat.y += OrderedFloat(1.5);
        }
        new_state
    }

    fn apply_go_north_west(state: &State, action: &Action) -> State {
        let boat_index = match action.parameters.get("boat").unwrap() {
            Value::Int(index) => *index,
            _ => panic!("Expected boat index to be an integer"),
        };
        let mut new_state = state.clone();
        if let Some(boat) = new_state.boats.iter_mut().find(|b| b.index == boat_index) {
            boat.x -= OrderedFloat(1.5);
            boat.y += OrderedFloat(1.5);
        }
        new_state
    }

    fn apply_go_east(state: &State, action: &Action) -> State {
        let boat_index = match action.parameters.get("boat").unwrap() {
            Value::Int(index) => *index,
            _ => panic!("Expected boat index to be an integer"),
        };
        let mut new_state = state.clone();
        if let Some(boat) = new_state.boats.iter_mut().find(|b| b.index == boat_index) {
            boat.x += OrderedFloat(3.0);
        }
        new_state
    }

    fn apply_go_west(state: &State, action: &Action) -> State {
        let boat_index = match action.parameters.get("boat").unwrap() {
            Value::Int(index) => *index,
            _ => panic!("Expected boat index to be an integer"),
        };
        let mut new_state = state.clone();
        if let Some(boat) = new_state.boats.iter_mut().find(|b| b.index == boat_index) {
            boat.x -= OrderedFloat(3.0);
        }
        new_state
    }

    fn apply_go_south_west(state: &State, action: &Action) -> State {
        let boat_index = match action.parameters.get("boat").unwrap() {
            Value::Int(index) => *index,
            _ => panic!("Expected boat index to be an integer"),
        };
        let mut new_state = state.clone();
        if let Some(boat) = new_state.boats.iter_mut().find(|b| b.index == boat_index) {
            boat.x -= OrderedFloat(2.0);
            boat.y -= OrderedFloat(2.0);
        }
        new_state
    }

    fn apply_go_south_east(state: &State, action: &Action) -> State {
        let boat_index = match action.parameters.get("boat").unwrap() {
            Value::Int(index) => *index,
            _ => panic!("Expected boat index to be an integer"),
        };
        let mut new_state = state.clone();
        if let Some(boat) = new_state.boats.iter_mut().find(|b| b.index == boat_index) {
            boat.x += OrderedFloat(2.0);
            boat.y -= OrderedFloat(2.0);
        }
        new_state
    }

    fn apply_go_south(state: &State, action: &Action) -> State {
        let boat_index = match action.parameters.get("boat").unwrap() {
            Value::Int(index) => *index,
            _ => panic!("Expected boat index to be an integer"),
        };
        let mut new_state = state.clone();
        if let Some(boat) = new_state.boats.iter_mut().find(|b| b.index == boat_index) {
            boat.y -= OrderedFloat(2.0);
        }
        new_state
    }

    fn apply_save_person(state: &State, action: &Action) -> State {
        let _boat_index = match action.parameters.get("boat").unwrap() {
            Value::Int(index) => *index,
            _ => panic!("Expected boat index to be an integer"),
        };
        let person_index = match action.parameters.get("person").unwrap() {
            Value::Int(index) => *index,
            _ => panic!("Expected person index to be an integer"),
        };
        let mut new_state = state.clone();
        if let Some(person) = new_state
            .persons
            .iter_mut()
            .find(|p| p.index == person_index)
        {
            person.saved = true;
        }
        new_state
    }
}

impl Problem for SailingProblem {
    type State = State;

    fn get_possible_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();

        for boat in &state.boats {
            actions.push(Self::go_north_east_action(boat));
            actions.push(Self::go_north_west_action(boat));
            actions.push(Self::go_east_action(boat));
            actions.push(Self::go_west_action(boat));
            actions.push(Self::go_south_west_action(boat));
            actions.push(Self::go_south_east_action(boat));
            actions.push(Self::go_south_action(boat));

            for person in &state.persons {
                if !person.saved {
                    if let Some(action) = Self::save_person_action(boat, person) {
                        actions.push(action);
                    }
                }
            }
        }
        actions
    }

    fn apply_action(&self, state: &State, action: &Action) -> State {
        if action.name.starts_with("go_north_east") {
            Self::apply_go_north_east(state, action)
        } else if action.name.starts_with("go_north_west") {
            Self::apply_go_north_west(state, action)
        } else if action.name.starts_with("go_east") {
            Self::apply_go_east(state, action)
        } else if action.name.starts_with("go_west") {
            Self::apply_go_west(state, action)
        } else if action.name.starts_with("go_south_west") {
            Self::apply_go_south_west(state, action)
        } else if action.name.starts_with("go_south_east") {
            Self::apply_go_south_east(state, action)
        } else if action.name.starts_with("go_south") {
            Self::apply_go_south(state, action)
        } else if action.name.starts_with("save_person") {
            Self::apply_save_person(state, action)
        } else {
            panic!("Unknown action");
        }
    }

    fn is_goal_state(&self, state: &State) -> bool {
        state.persons.iter().all(|person| person.saved)
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
    let problem: SailingProblem = serde_json::from_value(problem_value.clone())
        .expect("Failed to deserialize problem");

    (state, problem)
}



    fn heuristic(&self, _state: &State) -> f64 {
        0.0
    }
}
