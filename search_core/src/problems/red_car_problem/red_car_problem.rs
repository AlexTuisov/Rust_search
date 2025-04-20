use crate::problems::problem::Problem;
use crate::search::{action::Action, state::StateTrait, state::Value};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub grid: Grid,                             // grid occupancy and dimensions
    pub horizontalcars: Vec<HorizontalCar>,     // all horizontal cars
    pub verticalcars: Vec<VerticalCar>,         // all vertical cars
    pub horizontaltrucks: Vec<HorizontalTruck>, // all horizontal trucks
    pub verticaltrucks: Vec<VerticalTruck>,     // all vertical trucks
}

impl State {}
impl StateTrait for State {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HorizontalCar {
    pub x: i32,       // leftmost x position
    pub y: i32,       // y (row) coordinate
    pub name: String, // unique identifier
}

impl HorizontalCar {
    /// Occupied grid cells for this car
    pub fn get_positions(&self) -> Vec<(i32, i32)> {
        vec![(self.x, self.y), (self.x + 1, self.y)]
    }

    /// Can it move "right" or "left" without collision/out of bounds?
    pub fn can_move(&self, direction: &str, grid: &Grid) -> bool {
        match direction {
            "right" => {
                let nx = self.x + 2; // cell just past the right end
                let key = format!("({},{})", nx, self.y);
                nx < grid.col_size && !grid.cells.contains_key(&key)
            }
            "left" => {
                let nx = self.x - 1;
                let key = format!("({},{})", nx, self.y);
                self.x > 0 && !grid.cells.contains_key(&key)
            }
            _ => false,
        }
    }

    /// Update position by shifting x coordinate
    pub fn apply_action(&mut self, direction: &str) {
        match direction {
            "right" => self.x += 1,
            "left" => self.x -= 1,
            _ => panic!("Invalid direction for HorizontalCar: {}", direction),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerticalCar {
    pub x: i32,       // column coordinate
    pub y: i32,       // topmost y position
    pub name: String, // unique identifier
}

impl VerticalCar {
    /// Occupied grid cells for this car
    pub fn get_positions(&self) -> Vec<(i32, i32)> {
        vec![(self.x, self.y), (self.x, self.y + 1)]
    }

    /// Can it move "up" or "down" without collision/out of bounds?
    pub fn can_move(&self, direction: &str, grid: &Grid) -> bool {
        match direction {
            "up" => {
                let ny = self.y + 2; // cell just past the bottom end
                let key = format!("({},{})", self.x, ny);
                ny < grid.row_size && !grid.cells.contains_key(&key)
            }
            "down" => {
                let ny = self.y - 1;
                let key = format!("({},{})", self.x, ny);
                self.y > 0 && !grid.cells.contains_key(&key)
            }
            _ => false,
        }
    }

    /// Update position by shifting y coordinate
    pub fn apply_action(&mut self, direction: &str) {
        match direction {
            "up" => self.y += 1,
            "down" => self.y -= 1,
            _ => panic!("Invalid direction for VerticalCar: {}", direction),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HorizontalTruck {
    pub x: i32,       // leftmost x position
    pub y: i32,       // y (row) coordinate
    pub name: String, // unique identifier
}

impl HorizontalTruck {
    /// Occupied grid cells for this truck
    pub fn get_positions(&self) -> Vec<(i32, i32)> {
        vec![(self.x, self.y), (self.x + 1, self.y), (self.x + 2, self.y)]
    }

    /// Can it move "right" or "left" without collision?
    pub fn can_move(&self, direction: &str, grid: &Grid) -> bool {
        match direction {
            "right" => {
                let nx = self.x + 3; // cell just past the right end
                let key = format!("({},{})", nx, self.y);
                nx < grid.col_size && !grid.cells.contains_key(&key)
            }
            "left" => {
                let nx = self.x - 1;
                let key = format!("({},{})", nx, self.y);
                self.x > 0 && !grid.cells.contains_key(&key)
            }
            _ => false,
        }
    }

    /// Update position by shifting x coordinate
    pub fn apply_action(&mut self, direction: &str) {
        match direction {
            "right" => self.x += 1,
            "left" => self.x -= 1,
            _ => panic!("Invalid direction for HorizontalTruck: {}", direction),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerticalTruck {
    pub x: i32,       // column coordinate
    pub y: i32,       // topmost y position
    pub name: String, // unique identifier
}

impl VerticalTruck {
    /// Occupied grid cells for this truck
    pub fn get_positions(&self) -> Vec<(i32, i32)> {
        vec![(self.x, self.y), (self.x, self.y + 1), (self.x, self.y + 2)]
    }

    /// Can it move "up" or "down" without collision?
    pub fn can_move(&self, direction: &str, grid: &Grid) -> bool {
        match direction {
            "up" => {
                let ny = self.y + 3; // cell just past the bottom end
                let key = format!("({},{})", self.x, ny);
                ny < grid.row_size && !grid.cells.contains_key(&key)
            }
            "down" => {
                let ny = self.y - 1;
                let key = format!("({},{})", self.x, ny);
                self.y > 0 && !grid.cells.contains_key(&key)
            }
            _ => false,
        }
    }

    /// Update position by shifting y coordinate
    pub fn apply_action(&mut self, direction: &str) {
        match direction {
            "up" => self.y += 1,
            "down" => self.y -= 1,
            _ => panic!("Invalid direction for VerticalTruck: {}", direction),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Grid {
    pub row_size: i32,
    pub col_size: i32,
    pub cells: HashMap<String, String>, // now keyed by "(x,y)" strings
}

impl Grid {
    pub fn new(row_size: i32, col_size: i32) -> Self {
        Grid {
            row_size,
            col_size,
            cells: HashMap::new(),
        }
    }

    pub fn update(
        &mut self,
        old_positions: Vec<(i32, i32)>,
        new_positions: Vec<(i32, i32)>,
        name: &str,
    ) {
        for (x, y) in old_positions {
            self.cells.remove(&format!("({},{})", x, y));
        }
        for (x, y) in new_positions {
            self.cells
                .insert(format!("({},{})", x, y), name.to_string());
        }
    }

    pub fn place_object(&mut self, name: String, positions: &[(i32, i32)]) -> Result<(), String> {
        for &(x, y) in positions {
            if x < 0 || x >= self.col_size || y < 0 || y >= self.row_size {
                return Err(format!("Object '{}' out of bounds at ({}, {})", name, x, y));
            }
            let key = format!("({},{})", x, y);
            if self.cells.contains_key(&key) {
                return Err(format!("Cell {} already occupied", key));
            }
        }
        for &(x, y) in positions {
            self.cells.insert(format!("({},{})", x, y), name.clone());
        }
        Ok(())
    }
}

// Main problem definition for the red‐car puzzle
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RedCarProblem {}

impl RedCarProblem {
    /// Generate left/right moves for each horizontal car
    pub fn get_possible_horizontal_cars_actions(state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for car in &state.horizontalcars {
            if car.can_move("right", &state.grid) {
                let mut params = HashMap::new();
                let name = format!("{}_move_right", car.name);
                params.insert("vehicle".into(), Value::Text(car.name.clone()));
                params.insert("move".into(), Value::Text("right".into()));
                actions.push(Action::new(name, 1, params));
            }
            if car.can_move("left", &state.grid) {
                let mut params = HashMap::new();
                let name = format!("{}_move_left", car.name);
                params.insert("vehicle".into(), Value::Text(car.name.clone()));
                params.insert("move".into(), Value::Text("left".into()));
                actions.push(Action::new(name, 1, params));
            }
        }
        actions
    }

    /// Generate up/down moves for each vertical car
    pub fn get_possible_vertical_cars_actions(state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for car in &state.verticalcars {
            if car.can_move("up", &state.grid) {
                let mut params = HashMap::new();
                let name = format!("{}_move_up", car.name);
                params.insert("vehicle".into(), Value::Text(car.name.clone()));
                params.insert("move".into(), Value::Text("up".into()));
                actions.push(Action::new(name, 1, params));
            }
            if car.can_move("down", &state.grid) {
                let mut params = HashMap::new();
                let name = format!("{}_move_down", car.name);
                params.insert("vehicle".into(), Value::Text(car.name.clone()));
                params.insert("move".into(), Value::Text("down".into()));
                actions.push(Action::new(name, 1, params));
            }
        }
        actions
    }

    /// Generate left/right moves for each horizontal truck
    pub fn get_possible_horizontal_trucks_actions(state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for truck in &state.horizontaltrucks {
            if truck.can_move("right", &state.grid) {
                let mut params = HashMap::new();
                let name = format!("{}_move_right", truck.name);
                params.insert("vehicle".into(), Value::Text(truck.name.clone()));
                params.insert("move".into(), Value::Text("right".into()));
                actions.push(Action::new(name, 1, params));
            }
            if truck.can_move("left", &state.grid) {
                let mut params = HashMap::new();
                let name = format!("{}_move_left", truck.name);
                params.insert("vehicle".into(), Value::Text(truck.name.clone()));
                params.insert("move".into(), Value::Text("left".into()));
                actions.push(Action::new(name, 1, params));
            }
        }
        actions
    }

    /// Generate up/down moves for each vertical truck
    pub fn get_possible_vertical_trucks_actions(state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for truck in &state.verticaltrucks {
            if truck.can_move("up", &state.grid) {
                let mut params = HashMap::new();
                let name = format!("{}_move_up", truck.name);
                params.insert("vehicle".into(), Value::Text(truck.name.clone()));
                params.insert("move".into(), Value::Text("up".into()));
                actions.push(Action::new(name, 1, params));
            }
            if truck.can_move("down", &state.grid) {
                let mut params = HashMap::new();
                let name = format!("{}_move_down", truck.name);
                params.insert("vehicle".into(), Value::Text(truck.name.clone()));
                params.insert("move".into(), Value::Text("down".into()));
                actions.push(Action::new(name, 1, params));
            }
        }
        actions
    }
}

impl Problem for RedCarProblem {
    type State = State;

    /// Combine all possible move actions into one list
    fn get_possible_actions(&self, state: &State) -> Vec<Action> {
        let mut acts = Vec::new();
        acts.extend(Self::get_possible_horizontal_cars_actions(state));
        acts.extend(Self::get_possible_vertical_cars_actions(state));
        acts.extend(Self::get_possible_horizontal_trucks_actions(state));
        acts.extend(Self::get_possible_vertical_trucks_actions(state));
        acts
    }

    /// Apply a chosen action: move the matching vehicle and update grid
    fn apply_action(&self, state: &State, action: &Action) -> State {
        let dir = action
            .parameters
            .get("move")
            .and_then(|v| match v {
                Value::Text(s) => Some(s.as_str()),
                _ => None,
            })
            .expect("Missing move");
        let veh = action
            .parameters
            .get("vehicle")
            .and_then(|v| match v {
                Value::Text(s) => Some(s.as_str()),
                _ => None,
            })
            .expect("Missing vehicle");
        let mut ns = state.clone();
        if let Some(v) = ns.horizontalcars.iter_mut().find(|v| v.name == veh) {
            let old = v.get_positions();
            v.apply_action(dir);
            let new = v.get_positions();
            ns.grid.update(old, new, veh);
        } else if let Some(v) = ns.verticalcars.iter_mut().find(|v| v.name == veh) {
            let old = v.get_positions();
            v.apply_action(dir);
            let new = v.get_positions();
            ns.grid.update(old, new, veh);
        } else if let Some(v) = ns.horizontaltrucks.iter_mut().find(|v| v.name == veh) {
            let old = v.get_positions();
            v.apply_action(dir);
            let new = v.get_positions();
            ns.grid.update(old, new, veh);
        } else if let Some(v) = ns.verticaltrucks.iter_mut().find(|v| v.name == veh) {
            let old = v.get_positions();
            v.apply_action(dir);
            let new = v.get_positions();
            ns.grid.update(old, new, veh);
        } else {
            panic!("Unknown vehicle {}", veh);
        }
        ns
    }

    /// Goal: the red-car must reach the right edge on row 2
    fn is_goal_state(&self, state: &State) -> bool {
        for car in &state.horizontalcars {
            if car.name == "red-car" {
                return car.y == 2 && car.x == state.grid.col_size - 2;
            }
        }
        false
    }

    fn heuristic(&self, _state: &State) -> f64 {
        0.0
    }
    /// Load state and problem from JSON
    fn load_state_from_json(json_path: &str) -> (State, RedCarProblem) {
        let js = fs::read_to_string(json_path).expect("Failed to read JSON file");
        let val: JsonValue = serde_json::from_str(&js).expect("Failed to parse JSON");
        let st = val.get("state").expect("Missing 'state'");
        let pr = val.get("problem").expect("Missing 'problem'");
        let state: State = serde_json::from_value(st.clone()).expect("Failed to deserialize state");
        let problem: RedCarProblem =
            serde_json::from_value(pr.clone()).expect("Failed to deserialize problem");
        (state, problem)
    }
}
