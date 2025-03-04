// use crate::problems::problem::Problem;
// use crate::search::{action::Action, state::StateTrait, state::Value};
// use serde::{Deserialize, Serialize};
// use serde_json::{from_reader, Value as JsonValue};
// use std::collections::HashMap;
// use std::error::Error;
// use std::fs::File;
// use std::io::BufReader;

// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct State {
//     /// Number of cities in the problem.
//     pub num_cities: i32,

//     pub airplanes: Vec<Airplane>,

//     /// Distances between cities.
//     /// The key is a tuple (city1, city2) and the value is the distance between them.
//     pub distances: HashMap<(i32, i32), i32>,

//     //The persons in the problem
//     pub persons: Vec<Person>,

//     /// The total fuel used so far.
//     pub total_fuel_used: i32,

//     /// The total time elapsed (this value can be a float).
//     pub total_time: f64,
// }

// impl State {}

// impl StateTrait for State {}

// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Airplane {
//     /// Unique identifier for the airplane.
//     pub index: i32,

//     /// Slow attributes.
//     pub slow_burn: i32,
//     pub slow_speed: i32,

//     /// Fast attributes.
//     pub fast_burn: i32,
//     pub fast_speed: i32,

//     /// Capacity attributes.
//     pub capacity: i32,
//     pub fuel: i32,

//     /// Miscellaneous attributes.
//     pub location: i32,
//     pub zoom_limit: i32,
//     pub onboard: i32,
// }

// impl Airplane {
//     pub fn new(
//         index: i32,
//         slow_burn: i32,
//         slow_speed: i32,
//         fast_burn: i32,
//         fast_speed: i32,
//         capacity: i32,
//         fuel: i32,
//         location: i32,
//         zoom_limit: i32,
//         onboard: i32,
//     ) -> Self {
//         Airplane {
//             index,
//             slow_burn,
//             slow_speed,
//             fast_burn,
//             fast_speed,
//             capacity,
//             fuel,
//             location,
//             zoom_limit,
//             onboard,
//         }
//     }
// }

// impl Airplane {
//     pub fn new(
//         slow_burn: i32,
//         slow_speed: i32,
//         fast_burn: i32,
//         fast_speed: i32,
//         capacity: i32,
//         fuel: i32,
//         location: i32,
//         zoom_limit: i32,
//         onboard: i32,
//     ) -> Self {
//         Airplane {
//             slow_burn,
//             slow_speed,
//             fast_burn,
//             fast_speed,
//             capacity,
//             fuel,
//             location,
//             zoom_limit,
//             onboard,
//         }
//     }
// }

// pub struct Person {
//     /// -1 if he is on airplane (i.e in a airplane)
//     pub loc: i32,
//     /// -1 if on the ground (i.e. in a city)
//     pub on_airpane: i32,
// }

// impl Person {
//     /// Creates a new `Person` with the given location and air status.
//     pub fn new(loc: i32, is_in_the_air: i32) -> Self {
//         Person { loc, is_in_the_air }
//     }
// }

// pub struct Goal {
//     pub airplanes: Vec<(i32, i32)>,
//     pub persons: Vec<(i32, i32)>,
//     pub minize: String,
// }

// #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// pub struct ZenoTravelProblem {
//     pub goal: Goal,
// }

// impl ZenoTravelProblem {
//     pub fn get_refuel_actions(state: &State) -> Vec<Action> {
//         let mut actions = Vec::new();
//         // Iterate directly over each airplane in the vector.
//         for airplane in &state.airplanes {
//             let mut parameters = std::collections::HashMap::new();
//             let action_name = format!("refuel_airplane{}", airplane.index);
//             parameters.insert("refuel".to_string(), Value::Text("refuel".to_string()));
//             parameters.insert("airplane".to_string(), Value::Int(airplane.index));
//             actions.push(Action::new(action_name, 1, parameters));
//         }
//         actions
//     }

//     pub fn get_possible_board_persons(state: &State) -> Vec<Action> {
//         let mut actions = Vec::new();

//         // Iterate over persons with their indices.
//         for (j, person) in state.persons.iter().enumerate() {
//             // Only consider persons who are on the ground (i.e. on_airpane == -1).
//             if person.on_airpane == -1 {
//                 // Iterate over each airplane in the vector.
//                 for airplane in &state.airplanes {
//                     // Check if the airplane is at the same location as the person.
//                     if airplane.location == person.loc {
//                         let mut parameters = std::collections::HashMap::new();
//                         // Create an action name using the person's index (j) and the airplane's unique index.
//                         let action_name =
//                             format!("board_person{}_to_airplane{}", j, airplane.index);
//                         parameters.insert("board".to_string(), Value::Text("board".to_string()));
//                         parameters.insert("person".to_string(), Value::Int(j as i32));
//                         parameters.insert("airplane".to_string(), Value::Int(airplane.index));

//                         // Add the action to the list.
//                         actions.push(Action::new(action_name, 1, parameters));
//                     }
//                 }
//             }
//         }

//         actions
//     }

//     pub fn get_possible_debark_persons(state: &State) -> Vec<Action> {
//         let mut actions = Vec::new();

//         // Iterate over persons with their indices.
//         for (j, person) in state.persons.iter().enumerate() {
//             // Only consider persons who are not already on a plane.
//             if person.on_airplane != -1 {
//                 // Iterate over each airplane in the vector.
//                 for airplane in &state.airplanes {
//                     // Check if the airplane is at the same location as the person.
//                     if airplane.index == person.on_airplane {
//                         let mut parameters = std::collections::HashMap::new();
//                         // Create an action name using the person's index (j) and the airplane's unique index.
//                         let action_name =
//                             format!("debark_person{}_from_airplane{}", j, airplane.index);
//                         parameters.insert("debark".to_string(), Value::Text("debark".to_string()));
//                         parameters.insert("person".to_string(), Value::Int(j as i32));
//                         parameters.insert("airplane".to_string(), Value::Int(airplane.index));
//                         parameters.insert(
//                             "airplane_location".to_string(),
//                             Value::Int(airplane.location),
//                         );

//                         // Create a new action with cost 1 and add it to the actions vector.
//                         actions.push(Action::new(action_name, 1, parameters));
//                     }
//                 }
//             }
//         }

//         actions
//     }

//     pub fn get_possible_airplane_flys(state: &State) -> Vec<Action> {
//         let mut actions = Vec::new();

//         // Iterate over each airplane in the vector.
//         for airplane in &state.airplanes {
//             // Iterate over all cities (destination candidates).
//             for j in 0..state.num_cities {
//                 // Only consider flying to a city different from the airplane's current location.
//                 if j != airplane.location {
//                     // Check for slow flight:
//                     // Condition: available fuel >= (distance * slow-burn consumption rate)
//                     if airplane.fuel >= airplane.slow_burn * state.distance(&airplane.location, &j)
//                     {
//                         let mut parameters = std::collections::HashMap::new();
//                         // Create an action name using the airplane's index, its current location, and destination city.
//                         let action_name = format!(
//                             "fly_slow_airplane{}_from{}_to{}",
//                             airplane.index, airplane.location, j
//                         );
//                         parameters.insert("fly".to_string(), Value::Text("fly_slow".to_string()));
//                         parameters.insert("airplane".to_string(), Value::Int(airplane.index));
//                         parameters.insert("from".to_string(), Value::Int(airplane.location));
//                         parameters.insert("to".to_string(), Value::Int(j));
//                         parameters.insert(
//                             "distance".to_string(),
//                             Value::Int(state.distance(&airplane.location, &j)),
//                         );
//                         actions.push(Action::new(action_name, 1, parameters));
//                     }

//                     // Check for fast flight:
//                     // Condition: available fuel >= (distance * fast-burn consumption rate)
//                     //            and onboard count is less than or equal to the zoom limit.
//                     if airplane.fuel >= airplane.fast_burn * state.distance(&airplane.location, &j)
//                         && airplane.onboard <= airplane.zoom_limit
//                     {
//                         let mut parameters = std::collections::HashMap::new();
//                         let action_name = format!(
//                             "fly_fast_airplane{}_from{}_to{}",
//                             airplane.index, airplane.location, j
//                         );
//                         parameters.insert("fly".to_string(), Value::Text("fly_fast".to_string()));
//                         parameters.insert("airplane".to_string(), Value::Int(airplane.index));
//                         parameters.insert("from".to_string(), Value::Int(airplane.location));
//                         parameters.insert("to".to_string(), Value::Int(j));
//                         parameters.insert(
//                             "distance".to_string(),
//                             Value::Int(state.distance(&airplane.location, &j)),
//                         );
//                         actions.push(Action::new(action_name, 1, parameters));
//                     }
//                 }
//             }
//         }

//         actions
//     }

//     pub fn apply_refuel_action(state: &State, action: &Action) -> State {
//         let mut new_state = state.clone();

//         let airplane_index = match action.parameters.get("airplane") {
//             Some(Value::Int(i)) => *i as usize, // Dereference and cast to usize.
//             _ => panic!("Action parameters do not contain a valid airplane index."),
//         };

//         // Set the airplane's fuel to its capacity.
//         new_state.airplanes[airplane_index].fuel = new_state.airplanes[airplane_index].capacity;

//         new_state
//     }

//     pub fn apply_board_action(state: &State, action: &Action) -> State {
//         let mut new_state = state.clone();

//         let airplane_index = match action.parameters.get("airplane") {
//             Some(Value::Int(i)) => *i as usize, // Dereference and cast to usize.
//             _ => panic!("Action parameters do not contain a valid airplane index."),
//         };

//         let person_index = match action.parameters.get("person") {
//             Some(Value::Int(i)) => *i as usize, // Dereference and cast to usize.
//             _ => panic!("Action parameters do not contain a valid person index."),
//         };

//         // Set the airplane's fuel to its capacity.
//         new_state.persons[person_index].on_airplane = airplane_index;
//         new_state.persons[person_index].location = -1;
//         new_state.airplanes[airplane_index].onboard += 1;
//         new_state
//     }

//     pub fn apply_debark_action(state: &State, action: &Action) -> State {
//         let mut new_state = state.clone();

//         let airplane_index = match action.parameters.get("airplane") {
//             Some(Value::Int(i)) => *i as usize, // Dereference and cast to usize.
//             _ => panic!("Action parameters do not contain a valid airplane index."),
//         };

//         let person_index = match action.parameters.get("person") {
//             Some(Value::Int(i)) => *i as usize, // Dereference and cast to usize.
//             _ => panic!("Action parameters do not contain a valid person index."),
//         };

//         // Set the airplane's fuel to its capacity.
//         new_state.persons[person_index].on_airplane = -1;
//         new_state.persons[person_index].location = new_state.airplanes[airplane_index].location;
//         new_state.airplanes[airplane_index].onboard -= 1;
//         new_state
//     }

//     pub fn apply_fast_fly_action(state: &State, action: &Action) -> State {
//         let mut new_state = state.clone();
//         let distance = match action.parameters.get("distance") {
//             Some(Value::Int(i)) => *i as usize, // Dereference and cast to usize.
//             _ => panic!("Action parameters do not contain a valid person index."),
//         };

//         let airplane_index = match action.parameters.get("airplane") {
//             Some(Value::Int(i)) => *i as usize, // Dereference and cast to usize.
//             _ => panic!("Action parameters do not contain a valid airplane index."),
//         };
//         new_state.airplanes[airplane_index].fuel -=
//             new_state.airplanes[airplane_index].fast_burn * distance;
//         new_state.total_fuel_used += new_state.airplanes[airplane_index].fast_burn * distance;
//         new_state.total_time += distance / new_state.airplanes[airplane_index].fast_speed;
//     }

//     pub fn apply_slow_fly_action(state: &State, action: &Action) -> State {
//         let mut new_state = state.clone();
//         let distance = match action.parameters.get("distance") {
//             Some(Value::Int(i)) => *i as usize, // Dereference and cast to usize.
//             _ => panic!("Action parameters do not contain a valid person index."),
//         };

//         let airplane_index = match action.parameters.get("airplane") {
//             Some(Value::Int(i)) => *i as usize, // Dereference and cast to usize.
//             _ => panic!("Action parameters do not contain a valid airplane index."),
//         };
//         new_state.airplanes[airplane_index].fuel -=
//             new_state.airplanes[airplane_index].slow_burn * distance;
//         new_state.total_fuel_used += new_state.airplanes[airplane_index].slow_burn * distance;
//         new_state.total_time += distance / new_state.airplanes[airplane_index].slow_speed;
//         new_state
//     }

//     pub fn apply_fly_action(state: &State, action: &Action) -> State {
//         let mut new_state = state.clone();

//         match action.parameters.get("fly") {
//             Some(Value::Text(mode)) if mode == "fly_fast" => {
//                 new_state = apply_fast_fly_action(new_state, action);
//             }
//             Some(Value::Text(mode)) if mode == "fly_slow" => {
//                 new_state = apply_slow_fly_action(new_state, action);
//             }
//             _ => panic!("Action parameters do not contain a valid fly parameter."),
//         }

//         new_state
//     }
// }

// impl Problem for ZenoTravelProblem {
//     type State = State;

//     fn get_possible_actions(&self, state: &State) -> Vec<Action> {
//         let mut actions = Vec::new();
//         actions.extend(Self::get_refuel_actions(state));
//         actions.extend(Self::get_possible_airplane_flys(state));
//         actions.extend(Self::get_possible_board_persons(state));
//         actions.extend(Self::get_possible_debark_persons(state));
//         actions
//     }

//     fn apply_action(&self, state: &State, action: &Action) -> State {
//         // Parse the action name to determine which apply function to call
//         if action.name.starts_with("refuel_") {
//             Self::apply_refuel_action(state, action)
//         } else if action.name.starts_with("board_") {
//             Self::apply_board_action(state, action)
//         } else if action.name.starts_with("debark_") {
//             Self::apply_debark_action(state, action)
//         } else if action.name.starts_with("fly_") {
//             Self::apply_fly_action(state, action)
//         } else {
//             panic!("Unknown action type: {}", action.name)
//         }
//     }
//     /// need to check with Alex T how to do minimize
//     /* fn is_goal_state(&self, state: &State) -> bool {
//         for vehicle in &state.vehicle_state.vehicles {
//             if vehicle.name == "red-car" {
//                 let positions = vehicle.positions();
//                 return positions.contains(&[2, state.grid.col_size - 2])
//                     && positions.contains(&[2, state.grid.col_size - 1]);
//             }
//         }
//         false
//     } */

//     fn heuristic(&self, state: &State) -> f64 {
//         // heuristic is imported during build time from include!("refined_heuristic.in")
//         //heuristic(self, state)
//         0.0
//     }

//     pub fn load_state_from_json(json_path: &str) -> (State, Self) {
//         // Open the file and create a buffered reader.
//         let file = File::open(json_path).expect("Failed to open JSON file");
//         let reader = BufReader::new(file);
//         // Parse the JSON.
//         let json_data: JsonValue = from_reader(reader).expect("Failed to parse JSON");

//         // Expect the root to be an object.
//         let json_obj = json_data.as_object().expect("Expected a JSON object");

//         // --- Extract Goal ---
//         let goal_value = json_obj.get("goal").expect("Missing goal field in JSON");
//         let goal_obj = goal_value
//             .as_object()
//             .expect("Goal field must be an object");

//         // Parse airplane goals (if missing, use an empty vector).
//         let airplane_goals: Vec<(i32, i32)> =
//             if let Some(air_obj) = goal_obj.get("airplanes").and_then(|v| v.as_object()) {
//                 air_obj
//                     .iter()
//                     .map(|(key, value)| {
//                         let plane_idx: i32 = key.parse().expect("Invalid airplane index");
//                         let dest = value.as_i64().expect("Invalid airplane destination") as i32;
//                         (plane_idx, dest)
//                     })
//                     .collect()
//             } else {
//                 Vec::new()
//             };

//         // Parse person goals (if missing, use an empty vector).
//         let person_goals: Vec<(i32, i32)> =
//             if let Some(person_obj) = goal_obj.get("persons").and_then(|v| v.as_object()) {
//                 person_obj
//                     .iter()
//                     .map(|(key, value)| {
//                         let person_idx: i32 = key.parse().expect("Invalid person index");
//                         let dest = value.as_i64().expect("Invalid person destination") as i32;
//                         (person_idx, dest)
//                     })
//                     .collect()
//             } else {
//                 Vec::new()
//             };

//         // Parse the metric.
//         let minize = goal_obj
//             .get("minimize")
//             .and_then(|v| v.as_str())
//             .unwrap_or("")
//             .to_string();

//         let goal = Goal {
//             airplanes: airplane_goals,
//             persons: person_goals,
//             minize,
//         };

//         // --- Extract Global Fields ---
//         let num_cities = json_obj
//             .get("num_cities")
//             .expect("Missing num_cities")
//             .as_i64()
//             .expect("num_cities not an integer") as i32;

//         // --- Parse Airplanes ---
//         // Here we expect "airplanes" to be a JSON array.
//         let airplanes_json = json_obj
//             .get("airplanes")
//             .expect("Missing airplanes field")
//             .as_array()
//             .expect("Airplanes is not an array");
//         let mut airplanes = Vec::new();
//         for airplane_value in airplanes_json {
//             let airplane_obj = airplane_value
//                 .as_object()
//                 .expect("Expected airplane to be an object");
//             let index = airplane_obj
//                 .get("index")
//                 .expect("Missing airplane index")
//                 .as_i64()
//                 .expect("Invalid airplane index") as i32;
//             let slow_burn = airplane_obj
//                 .get("slow_burn")
//                 .expect("Missing slow_burn")
//                 .as_i64()
//                 .expect("Invalid slow_burn") as i32;
//             let slow_speed = airplane_obj
//                 .get("slow_speed")
//                 .expect("Missing slow_speed")
//                 .as_i64()
//                 .expect("Invalid slow_speed") as i32;
//             let fast_burn = airplane_obj
//                 .get("fast_burn")
//                 .expect("Missing fast_burn")
//                 .as_i64()
//                 .expect("Invalid fast_burn") as i32;
//             let fast_speed = airplane_obj
//                 .get("fast_speed")
//                 .expect("Missing fast_speed")
//                 .as_i64()
//                 .expect("Invalid fast_speed") as i32;
//             let capacity = airplane_obj
//                 .get("capacity")
//                 .expect("Missing capacity")
//                 .as_i64()
//                 .expect("Invalid capacity") as i32;
//             let fuel = airplane_obj
//                 .get("fuel")
//                 .expect("Missing fuel")
//                 .as_i64()
//                 .expect("Invalid fuel") as i32;
//             let location = airplane_obj
//                 .get("location")
//                 .expect("Missing location")
//                 .as_i64()
//                 .expect("Invalid location") as i32;
//             let zoom_limit = airplane_obj
//                 .get("zoom_limit")
//                 .expect("Missing zoom_limit")
//                 .as_i64()
//                 .expect("Invalid zoom_limit") as i32;
//             let onboard = airplane_obj
//                 .get("onboard")
//                 .expect("Missing onboard")
//                 .as_i64()
//                 .expect("Invalid onboard") as i32;
//             let airplane = Airplane {
//                 index,
//                 slow_burn,
//                 slow_speed,
//                 fast_burn,
//                 fast_speed,
//                 capacity,
//                 fuel,
//                 location,
//                 zoom_limit,
//                 onboard,
//             };
//             airplanes.push(airplane);
//         }

//         // --- Parse Persons ---
//         // Here we expect "persons" to be a JSON array.
//         let persons_json = json_obj
//             .get("persons")
//             .expect("Missing persons field")
//             .as_array()
//             .expect("Persons is not an array");
//         let mut persons = Vec::new();
//         for person_value in persons_json {
//             let person_obj = person_value
//                 .as_object()
//                 .expect("Expected person to be an object");
//             let loc = person_obj
//                 .get("loc")
//                 .expect("Missing loc")
//                 .as_i64()
//                 .expect("Invalid loc") as i32;
//             let on_airpane = person_obj
//                 .get("on_airpane")
//                 .expect("Missing on_airpane")
//                 .as_i64()
//                 .expect("Invalid on_airpane") as i32;
//             let person = Person { loc, on_airpane };
//             persons.push(person);
//         }

//         // --- Parse Distances ---
//         let mut distances = HashMap::new();
//         if let Some(distances_json) = json_obj.get("distances").and_then(|v| v.as_object()) {
//             for (key, value) in distances_json {
//                 let parts: Vec<&str> = key.split('-').collect();
//                 if parts.len() == 2 {
//                     let city1 = parts[0]
//                         .parse::<i32>()
//                         .expect("Invalid city index in distances");
//                     let city2 = parts[1]
//                         .parse::<i32>()
//                         .expect("Invalid city index in distances");
//                     let distance = value.as_i64().expect("Invalid distance value") as i32;
//                     distances.insert((city1, city2), distance);
//                 }
//             }
//         }

//         // --- Parse Global Metrics ---
//         let total_fuel_used = json_obj
//             .get("total_fuel_used")
//             .unwrap_or(&JsonValue::Number(0.into()))
//             .as_i64()
//             .unwrap_or(0) as i32;
//         let total_time = json_obj
//             .get("total_time")
//             .unwrap_or(&JsonValue::Number(0.into()))
//             .as_f64()
//             .unwrap_or(0.0);

//         let state = State {
//             num_cities,
//             airplanes,
//             distances,
//             persons,
//             total_fuel_used,
//             total_time,
//         };

//         (state, ZenoTravelProblem { goal })
//     }
// }
