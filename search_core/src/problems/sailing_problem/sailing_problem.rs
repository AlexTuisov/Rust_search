use crate::problems::problem::Problem;
use crate::search::{action::Action, state::StateTrait, state::Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::from_reader;
use serde_json::Value as JsonValue;
use std::fs::File;
use std::io::BufReader;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Boat {
    pub x: f64,
    pub y: f64,
    pub index: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Person {
    pub d: f64,
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
        if boat.x + boat.y >= person.d && 
           boat.y - boat.x >= person.d && 
           boat.x + boat.y <= person.d + 25.0 && 
           boat.y - boat.x <= person.d + 25.0 {
            let mut parameters = HashMap::new();
            parameters.insert("boat".to_string(), Value::Int(boat.index));
            parameters.insert("person".to_string(), Value::Int(person.index));
            Some(Action::new(format!("save_person_{}_{}", boat.index, person.index), 1, parameters))
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
            boat.x += 1.5;
            boat.y += 1.5;
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
            boat.x -= 1.5;
            boat.y += 1.5;
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
            boat.x += 3.0;
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
            boat.x -= 3.0;
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
            boat.x += 2.0;
            boat.y -= 2.0;
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
            boat.x -= 2.0;
            boat.y -= 2.0;
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
        boat.y -= 2.0;
    }
    new_state
}

    fn apply_save_person(state: &State, action: &Action) -> State {
        let boat_index = match action.parameters.get("boat").unwrap() {
            Value::Int(index) => *index,
            _ => panic!("Expected boat index to be an integer"),
        };
        let person_index = match action.parameters.get("person").unwrap() {
            Value::Int(index) => *index,
            _ => panic!("Expected person index to be an integer"),
        };
        let mut new_state = state.clone();
        if let Some(person) = new_state.persons.iter_mut().find(|p| p.index == person_index) {
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
    let file = File::open(json_path).expect("Failed to open JSON file");
    let reader = BufReader::new(file);
    let json_data: JsonValue = from_reader(reader).expect("Failed to parse JSON");

    let boats = json_data["boats"]
        .as_array()
        .unwrap()
        .iter()
        .map(|boat| Boat {
            x: boat["x"].as_f64().unwrap(),
            y: boat["y"].as_f64().unwrap(),
            index: boat["index"].as_i64().unwrap() as i32,
        })
        .collect();

    let persons = json_data["persons"]
        .as_array()
        .unwrap()
        .iter()
        .map(|person| Person {
            d: person["d"].as_f64().unwrap(),
            saved: person["saved"].as_bool().unwrap(),
            index: person["index"].as_i64().unwrap() as i32,
        })
        .collect();

    let state = State { boats, persons };
    (state, SailingProblem {})
}


    fn heuristic(&self, state: &State) -> f64 {
        // Simple heuristic: count unsaved people
        //state.persons.iter().filter(|p| !p.saved).count() as f64
        0.0
    }
}
