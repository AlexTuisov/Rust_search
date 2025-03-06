use crate::problems::problem::Problem;
use crate::search::{action::Action, state::StateTrait, state::Value};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub grid: Grid,
    pub horizontalcars: Vec<HorizontalCar>,
    pub verticalcars: Vec<VerticalCar>,
    pub horizontaltrucks: Vec<HorizontalTruck>,
    pub verticaltrucks: Vec<VerticalTruck>,
}

impl State {}

impl StateTrait for State {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HorizontalCar {
    pub x: i32,
    pub y: i32,
    pub name: String,
}

impl HorizontalCar {
    /// Returns the positions occupied by the horizontal car.
    pub fn get_positions(&self) -> Vec<(i32, i32)> {
        vec![(self.x, self.y), (self.x + 1, self.y)]
    }

    // Example methods for moving and checking movement:
    pub fn can_move(&self, direction: &str, grid: &Grid) -> bool {
        match direction {
            "right" => {
                let end_position = self.x + 2; // The cell after the car's rightmost part
                end_position < grid.col_size && !grid.cells.contains_key(&(end_position, self.y))
            }
            "left" => self.x > 0 && !grid.cells.contains_key(&(self.x - 1, self.y)),
            _ => false,
        }
    }

    pub fn apply_action(&mut self, direction: &str) {
        match direction {
            "right" => self.x += 1,
            "left" => self.x -= 1,
            _ => panic!("Invalid move direction for HorizontalCar: {}", direction),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerticalCar {
    pub x: i32,
    pub y: i32,
    pub name: String,
}

impl VerticalCar {
    /// Returns the positions occupied by the vertical car.
    pub fn get_positions(&self) -> Vec<(i32, i32)> {
        vec![(self.x, self.y), (self.x, self.y + 1)]
    }

    pub fn can_move(&self, direction: &str, grid: &Grid) -> bool {
        match direction {
            "up" => {
                let end_position = self.y + 2;
                end_position < grid.row_size && !grid.cells.contains_key(&(self.x, end_position))
            }
            "down" => self.y > 0 && !grid.cells.contains_key(&(self.x, self.y - 1)),
            _ => false,
        }
    }

    pub fn apply_action(&mut self, direction: &str) {
        match direction {
            "up" => self.y += 1,
            "down" => self.y -= 1,
            _ => panic!("Invalid move direction for VerticalCar: {}", direction),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HorizontalTruck {
    pub x: i32,
    pub y: i32,
    pub name: String,
}

impl HorizontalTruck {
    /// Returns the positions occupied by the horizontal truck.
    pub fn get_positions(&self) -> Vec<(i32, i32)> {
        vec![(self.x, self.y), (self.x + 1, self.y), (self.x + 2, self.y)]
    }

    pub fn can_move(&self, direction: &str, grid: &Grid) -> bool {
        match direction {
            "right" => {
                let end_position = self.x + 3;
                end_position < grid.col_size && !grid.cells.contains_key(&(end_position, self.y))
            }
            "left" => self.x > 0 && !grid.cells.contains_key(&(self.x - 1, self.y)),
            _ => false,
        }
    }

    pub fn apply_action(&mut self, direction: &str) {
        match direction {
            "right" => self.x += 1,
            "left" => self.x -= 1,
            _ => panic!("Invalid move direction for HorizontalTruck: {}", direction),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerticalTruck {
    pub x: i32,
    pub y: i32,
    pub name: String,
}

impl VerticalTruck {
    /// Returns the positions occupied by the vertical truck.
    pub fn get_positions(&self) -> Vec<(i32, i32)> {
        vec![(self.x, self.y), (self.x, self.y + 1), (self.x, self.y + 2)]
    }

    pub fn can_move(&self, direction: &str, grid: &Grid) -> bool {
        match direction {
            "up" => {
                let end_position = self.y + 3;
                end_position < grid.row_size && !grid.cells.contains_key(&(self.x, end_position))
            }
            "down" => self.y > 0 && !grid.cells.contains_key(&(self.x, self.y - 1)),
            _ => false,
        }
    }

    pub fn apply_action(&mut self, direction: &str) {
        match direction {
            "up" => self.y += 1,
            "down" => self.y -= 1,
            _ => panic!("Invalid move direction for VerticalTruck: {}", direction),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Grid {
    pub row_size: i32,
    pub col_size: i32, // Size of the grid (n x m)
    pub cells: HashMap<(i32, i32), String>,
}

impl Grid {
    /// Creates a new empty grid with the given dimensions.
    pub fn new(row_size: i32, col_size: i32) -> Self {
        Grid {
            row_size,
            col_size,
            cells: HashMap::new(),
        }
    }

    /// Updates the grid by removing an object's old positions and adding its new positions.
    pub fn update(
        &mut self,
        old_positions: Vec<(i32, i32)>,
        new_positions: Vec<(i32, i32)>,
        name: &str,
    ) {
        for pos in old_positions {
            self.cells.remove(&pos);
        }
        for pos in new_positions {
            self.cells.insert(pos, name.to_string());
        }
    }

    /// Attempts to place an object on the grid.
    /// The `positions` slice contains the coordinates the object will occupy.
    /// Returns an error if any position is out of bounds or already occupied.
    pub fn place_object(&mut self, name: String, positions: &[(i32, i32)]) -> Result<(), String> {
        // Validate each position.
        for &(x, y) in positions {
            if x < 0 || x >= self.row_size || y < 0 || y >= self.col_size {
                return Err(format!(
                    "Object '{}' is out of bounds at ({}, {}).",
                    name, x, y
                ));
            }
            if self.cells.contains_key(&(x, y)) {
                return Err(format!("Cell ({}, {}) is already occupied.", x, y));
            }
        }
        // Place the object by marking its cells.
        for &(x, y) in positions {
            self.cells.insert((x, y), name.to_string());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RedCarProblem {}
impl RedCarProblem {
    pub fn get_possible_horizontal_cars_actions(state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for car in &state.horizontalcars {
            if car.can_move("right", &state.grid) {
                let mut parameters = HashMap::new();
                let action_name = format!("{}_move_{}", car.name, "right");
                parameters.insert("vehicle".to_string(), Value::Text(car.name.clone()));
                parameters.insert("move".to_string(), Value::Text("right".to_string()));
                actions.push(Action::new(action_name, 1, parameters));
            }
            // Check if the vehicle can move left
            if car.can_move("left", &state.grid) {
                let mut parameters = HashMap::new();
                let action_name = format!("{}_move_{}", car.name, "left"); // Format the action name as "Car/Truck_{name}_Move{action}"
                parameters.insert("vehicle".to_string(), Value::Text(car.name.clone()));
                parameters.insert("move".to_string(), Value::Text("left".to_string()));
                actions.push(Action::new(action_name, 1, parameters));
            }
        }
        actions
    }
    pub fn get_possible_vertical_cars_actions(state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for car in &state.verticalcars {
            if car.can_move("up", &state.grid) {
                let mut parameters = HashMap::new();
                let action_name = format!("{}_move_{}", car.name, "up");
                parameters.insert("vehicle".to_string(), Value::Text(car.name.clone()));
                parameters.insert("move".to_string(), Value::Text("up".to_string()));
                actions.push(Action::new(action_name, 1, parameters));
            }
            // Check if the vehicle can move left
            if car.can_move("down", &state.grid) {
                let mut parameters = HashMap::new();
                let action_name = format!("{}_move_{}", car.name, "down"); // Format the action name as "Car/Truck_{name}_Move{action}"
                parameters.insert("vehicle".to_string(), Value::Text(car.name.clone()));
                parameters.insert("move".to_string(), Value::Text("down".to_string()));
                actions.push(Action::new(action_name, 1, parameters));
            }
        }
        actions
    }
    pub fn get_possible_horizontal_trucks_actions(state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for truck in &state.horizontaltrucks {
            if truck.can_move("right", &state.grid) {
                let mut parameters = HashMap::new();
                let action_name = format!("{}_move_{}", truck.name, "right");
                parameters.insert("vehicle".to_string(), Value::Text(truck.name.clone()));
                parameters.insert("move".to_string(), Value::Text("right".to_string()));
                actions.push(Action::new(action_name, 1, parameters));
            }
            // Check if the vehicle can move left
            if truck.can_move("left", &state.grid) {
                let mut parameters = HashMap::new();
                let action_name = format!("{}_move_{}", truck.name, "left"); // Format the action name as "Car/Truck_{name}_Move{action}"
                parameters.insert("vehicle".to_string(), Value::Text(truck.name.clone()));
                parameters.insert("move".to_string(), Value::Text("left".to_string()));
                actions.push(Action::new(action_name, 1, parameters));
            }
        }
        actions
    }
    pub fn get_possible_vertical_trucks_actions(state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for truck in &state.verticaltrucks {
            if truck.can_move("up", &state.grid) {
                let mut parameters = HashMap::new();
                let action_name = format!("{}_move_{}", truck.name, "up");
                parameters.insert("vehicle".to_string(), Value::Text(truck.name.clone()));
                parameters.insert("move".to_string(), Value::Text("up".to_string()));
                actions.push(Action::new(action_name, 1, parameters));
            }
            // Check if the vehicle can move left
            if truck.can_move("down", &state.grid) {
                let mut parameters = HashMap::new();
                let action_name = format!("{}_move_{}", truck.name, "down"); // Format the action name as "Car/Truck_{name}_Move{action}"
                parameters.insert("vehicle".to_string(), Value::Text(truck.name.clone()));
                parameters.insert("move".to_string(), Value::Text("down".to_string()));
                actions.push(Action::new(action_name, 1, parameters));
            }
        }
        actions
    }
}

impl Problem for RedCarProblem {
    type State = State;
    fn get_possible_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        actions.extend(Self::get_possible_horizontal_cars_actions(state));
        actions.extend(Self::get_possible_vertical_cars_actions(state));
        actions.extend(Self::get_possible_horizontal_trucks_actions(state));
        actions.extend(Self::get_possible_vertical_trucks_actions(state));
        actions
    }

    fn apply_action(&self, state: &State, action: &Action) -> State {
        let move_direction = match action.parameters.get("move") {
            Some(Value::Text(dir)) => dir.as_str(),
            _ => panic!("Invalid action: Missing `move` parameter"),
        };
        let vehicle_name = match action.parameters.get("vehicle") {
            Some(Value::Text(vehicle)) => vehicle.as_str(),
            _ => panic!("Invalid action: Missing `vehicle` parameter"),
        };

        // Clone the state so we can modify it.
        let mut new_state = state.clone();

        if let Some(v) = new_state
            .horizontalcars
            .iter_mut()
            .find(|v| v.name == vehicle_name)
        {
            let old_positions = v.get_positions();
            v.apply_action(move_direction);
            let new_positions = v.get_positions();
            new_state
                .grid
                .update(old_positions, new_positions, vehicle_name);
        } else if let Some(v) = new_state
            .verticalcars
            .iter_mut()
            .find(|v| v.name == vehicle_name)
        {
            let old_positions = v.get_positions();
            v.apply_action(move_direction);
            let new_positions = v.get_positions();
            new_state
                .grid
                .update(old_positions, new_positions, vehicle_name);
        } else if let Some(v) = new_state
            .horizontaltrucks
            .iter_mut()
            .find(|v| v.name == vehicle_name)
        {
            let old_positions = v.get_positions();
            v.apply_action(move_direction);
            let new_positions = v.get_positions();
            new_state
                .grid
                .update(old_positions, new_positions, vehicle_name);
        } else if let Some(v) = new_state
            .verticaltrucks
            .iter_mut()
            .find(|v| v.name == vehicle_name)
        {
            let old_positions = v.get_positions();
            v.apply_action(move_direction);
            let new_positions = v.get_positions();
            new_state
                .grid
                .update(old_positions, new_positions, vehicle_name);
        } else {
            panic!(" Missing `vehicle` ");
        }

        new_state
    }

    fn is_goal_state(&self, state: &State) -> bool {
        for red_car in &state.horizontalcars {
            if red_car.name == "red-car" {
                return red_car.y == 2 && red_car.x == state.grid.col_size - 2;
            }
        }
        false
    }
    fn heuristic(&self, state: &State) -> f64 {
        // heuristic is imported during build time from include!("refined_heuristic.in")
        //heuristic(self, state)
        0.0
    }
    fn load_state_from_json(json_path: &str) -> (State, RedCarProblem) {
        // Read the JSON file into a string.
        let json_str = fs::read_to_string(json_path).expect("Failed to read JSON file");

        // Parse the JSON string into a serde_json::Value.
        let json_value: JsonValue = serde_json::from_str(&json_str).expect("Failed to parse JSON");

        // Extract the "state" field.
        let state_value = json_value
            .get("state")
            .expect("Missing 'state' field in JSON");

        // Deserialize into State.
        let mut state: State =
            serde_json::from_value(state_value.clone()).expect("Failed to deserialize state");

        // Now fill the grid.cells with the vehicles.
        // For each horizontal car:
        for car in &state.horizontalcars {
            for pos in car.get_positions() {
                state.grid.cells.insert(pos, car.name.clone());
            }
        }
        // For vertical cars:
        for car in &state.verticalcars {
            for pos in car.get_positions() {
                state.grid.cells.insert(pos, car.name.clone());
            }
        }
        // For horizontal trucks:
        for truck in &state.horizontaltrucks {
            for pos in truck.get_positions() {
                state.grid.cells.insert(pos, truck.name.clone());
            }
        }
        // For vertical trucks:
        for truck in &state.verticaltrucks {
            for pos in truck.get_positions() {
                state.grid.cells.insert(pos, truck.name.clone());
            }
        }

        (state, RedCarProblem {})
    }
}
