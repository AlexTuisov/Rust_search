use crate::search::{action::Action, state::StateTrait, state::Value};
use serde_json::{from_reader, Value as JsonValue};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub grid: Grid,
    pub vehicle_state: VehicleState,
    pub cost: i32,
}

impl State {
    fn new(row_size: usize, col_size: usize) -> Self {
        State {
            grid: Grid::new(row_size, col_size),
            vehicle_state: VehicleState::new(),
            cost: 0,
        }
    }
}

impl StateTrait for State {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VehicleState {
    pub vehicles: Vec<Vehicle>, // Stores all vehicles separately
}

impl VehicleState {
    pub fn new() -> Self {
        VehicleState {
            vehicles: Vec::new(),
        }
    }

    pub fn add_vehicle(&mut self, vehicle: Vehicle) {
        self.vehicles.push(vehicle);
    }

    pub fn find_vehicle(&self, name: &str) -> Option<&Vehicle> {
        self.vehicles.iter().find(|v| v.name == name)
    }

    pub fn find_vehicle_mut(&mut self, name: &str) -> Option<&mut Vehicle> {
        self.vehicles.iter_mut().find(|v| v.name == name)
    }
}

//Kind of Vehicles
#[derive(Clone, Copy, Debug)]
pub enum VehicleKind {
    HorizontalCar,
    HorizontalTruck,
    VerticalCar,
    VerticalTruck,
}
//Directions
#[derive(Clone, Copy, Debug)]
enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Vehicle {
    pub kind: VehicleKind,
    pub position: [usize; 2],
    pub name: String,
}

impl VehicleKind {
    fn length(self) -> usize {
        match self {
            Self::HorizontalCar => 2,
            Self::VerticalCar => 2,
            Self::HorizontalTruck => 3,
            Self::VerticalTruck => 3,
        }
    }

    fn direction(self) -> Direction {
        match self {
            Self::HorizontalCar | Self::HorizontalTruck => Direction::Horizontal,
            Self::VerticalCar | Self::VerticalTruck => Direction::Vertical,
        }
    }
}

impl Vehicle {
    pub fn new(kind: VehicleKind, position: [usize; 2], name: String) -> Self {
        Self {
            kind,
            position,
            name,
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    pub fn positions(&self) -> Vec<[usize; 2]> {
        let mut points_positions = Vec::with_capacity(3);
        match self.kind {
            VehicleKind::VerticalCar => {
                let end_point = [self.position[0] + 1, self.position[1]];
                points_positions.push(self.position);
                points_positions.push(end_point);
            }
            VehicleKind::HorizontalCar => {
                let end_point = [self.position[0], self.position[1] + 1];
                points_positions.push(self.position);
                points_positions.push(end_point);
            }
            VehicleKind::VerticalTruck => {
                let middle_point = [self.position[0] + 1, self.position[1]];
                let end_point = [self.position[0] + 2, self.position[1]];
                points_positions.push(self.position);
                points_positions.push(middle_point);
                points_positions.push(end_point);
            }
            VehicleKind::HorizontalTruck => {
                let middle_point = [self.position[0], self.position[1] + 1];
                let end_point = [self.position[0], self.position[1] + 2];
                points_positions.push(self.position);
                points_positions.push(middle_point);
                points_positions.push(end_point);
            }
        }
        points_positions
    }

    fn can_move(&self, direction: &str, grid: &Grid) -> bool {
        match self.kind.direction() {
            Direction::Horizontal => match direction {
                "right" => {
                    let end_position = self.position[1] + self.kind.length(); // Rightmost position of the vehicle
                    end_position < grid.col_size
                        && grid.cells[self.position[0]][end_position].is_none()
                }
                "left" => {
                    self.position[1] > 0
                        && grid.cells[self.position[0]][self.position[1] - 1].is_none()
                }
                _ => false, // Invalid direction
            },
            Direction::Vertical => match direction {
                "up" => {
                    self.position[0] > 0
                        && grid.cells[self.position[0] - 1][self.position[1]].is_none()
                }
                "down" => {
                    let end_position = self.position[0] + self.kind.length(); // Bottommost position of the vehicle
                    end_position < grid.row_size
                        && grid.cells[end_position][self.position[1]].is_none()
                }
                _ => false, // Invalid direction
            },
        }
    }

    fn move_vehicle(&mut self, direction: &str) -> Result<(), String> {
        match direction {
            "up" => self.position[0] -= 1,
            "down" => self.position[0] += 1,
            "left" => self.position[1] -= 1,
            "right" => self.position[1] += 1,
            _ => return Err(format!("Invalid move: {}", direction)),
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Grid {
    pub row_size: usize,
    pub col_size: usize, // Size of the grid (n x m)
    pub cells: HashMap<(usize, usize), String>,
}

impl Grid {
    // Constructor to create a new grid
    pub fn new(row_size: usize, col_size: usize) -> Self {
        Self {
            row_size,
            col_size,
            cells: HashMap::new(),
        }
    }

    pub fn display(&self) {
        for i in 0..self.row_size {
            for j in 0..self.col_size {
                if let Some(obj) = self.cells.get(&(i, j)) {
                    print!("{} ", obj);
                } else {
                    print!(".. "); // Empty cell
                }
            }
            println!();
        }
    }

    fn update(
        &mut self,
        old_positions: Vec<[usize; 2]>,
        new_positions: Vec<[usize; 2]>,
        name: String,
    ) {
        for pos in old_positions {
            self.cells.remove(&(pos[0], pos[1]));
        }
        for pos in new_positions {
            self.cells.insert((pos[0], pos[1]), name.clone());
        }
    }

    pub fn place_object(&mut self, object: &Vehicle) -> Result<String, String> {
        let positions = object.positions();

        for &[x, y] in &positions {
            if x >= self.row_size || y >= self.col_size {
                return Err(format!(
                    "Object {} is out of bounds at ({}, {}).",
                    object.name, x, y
                ));
            }
            if self.cells.contains_key(&(x, y)) {
                return Err(format!("Cell ({}, {}) is already occupied.", x, y));
            }
        }

        for &[x, y] in &positions {
            self.cells.insert((x, y), object.name.clone());
        }

        Ok(format!(
            "{} placed successfully at {:?}",
            object.name, positions
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RedCarProblem {}
impl RedCarProblem {
    pub fn generate_moves(state: &State) -> Vec<(Action)> {
        let mut actions = Vec::new();

        for object in &state.vehicle_state.vehicles {
            match object.kind.direction() {
                Direction::Vertical => {
                    // Check if the vehicle can move up
                    if object.can_move("up", &state.grid) {
                        let mut parameters = HashMap::new();
                        let action_name; // Format the action name as "Car/Truck_{name}_Move{action}"
                        match object.kind {
                            VehicleKind::HorizontalCar => {
                                action_name = format!("car_{}_move_{}", object.name, "up");
                            }
                            VehicleKind::HorizontalTruck => {
                                action_name = format!("truck_{}_move_{}", object.name, "up");
                            }
                        }
                        parameters.insert("object".to_string(), Value::Text(object.name.clone()));
                        parameters.insert("move".to_string(), Value::Text("up"));
                        actions.push(Action::new(action_name, 1, parameters));
                    }

                    // Check if the vehicle can move down
                    if object.can_move("down", &state.grid) {
                        let mut parameters = HashMap::new();
                        let action_name; // Format the action name as "Car/Truck_{name}_Move{action}"
                        match object.kind {
                            VehicleKind::HorizontalCar => {
                                action_name = format!("car_{}_move_{}", object.name, "down");
                            }
                            VehicleKind::HorizontalTruck => {
                                action_name = format!("truck_{}_move_{}", object.name, "down");
                            }
                        }
                        parameters.insert("object".to_string(), Value::Text(object.name.clone()));
                        parameters.insert("move".to_string(), Value::Text("down"));
                        actions.push(Action::new(action_name, 1, parameters));
                    }
                }

                Direction::Horizontal => {
                    // Check if the vehicle can move right
                    if object.can_move("right", &state.grid) {
                        let mut parameters = HashMap::new();
                        let action_name; // Format the action name as "Car/Truck_{name}_Move{action}"
                        match object.kind {
                            VehicleKind::HorizontalCar => {
                                action_name = format!("car_{}_move_{}", object.name, "right");
                            }
                            VehicleKind::HorizontalTruck => {
                                action_name = format!("truck_{}_move_{}", object.name, "right");
                            }
                        }
                        parameters.insert("object".to_string(), Value::Text(object.name.clone()));
                        parameters.insert("move".to_string(), Value::Text("right"));
                        actions.push(Action::new(action_name, 1, parameters));
                    }
                    // Check if the vehicle can move left
                    if object.can_move("left", &state.grid) {
                        let mut parameters = HashMap::new();
                        let action_name; // Format the action name as "Car/Truck_{name}_Move{action}"
                        match object.kind {
                            VehicleKind::HorizontalCar => {
                                action_name = format!("car_{}_move_{}", object.name, "left");
                            }
                            VehicleKind::HorizontalTruck => {
                                action_name = format!("truck_{}_move_{}", object.name, "left");
                            }
                        }
                        parameters.insert("object".to_string(), Value::Text(object.name.clone()));
                        parameters.insert("move".to_string(), Value::Text("left"));
                        actions.push(Action::new(action_name, 1, parameters));
                    }
                }
            }
        }

        actions // Return the vector of new grid configurations
    }

    fn apply_move_action(state: &State, action: &Action, direction: &str) -> State {
        let mut new_state = state.clone();

        // Extract vehicle name from parameters (now using "object" key)
        let vehicle_name = match action.parameters.get("object") {
            Some(Value::Text(name)) => name.clone(),
            _ => panic!("Action parameters do not contain a valid vehicle name."),
        };

        // Find the vehicle in the grid
        let vehicle_index = new_state
            .vehicle_state
            .vehicles
            .iter()
            .position(|v| v.name == vehicle_name);

        if let Some(index) = vehicle_index {
            let vehicle = &mut new_state.vehicle_state.vehicles[index];

            // Save the old positions before movement
            let old_positions = vehicle.positions();

            // Move the vehicle using its method
            vehicle.move_vehicle(direction);

            // Save the new positions after movement
            let new_positions = vehicle.positions();

            // Update the grid
            new_state
                .grid
                .update(old_positions, new_positions, vehicle.name.clone());
        } else {
            panic!("Vehicle '{}' not found in the grid.", vehicle_name);
        }

        new_state
    }
}

impl Problem for RedCarProblem {
    fn get_possible_actions(&self, state: &State) -> Vec<Action> {
        Self::generate_moves(state)
    }

    fn apply_action(&self, state: &State, action: &Action) -> State {
        let move_direction = match action.parameters.get("move") {
            Some(Value::Text(dir)) => dir.as_str(),
            _ => panic!("Invalid action: Missing `move` parameter"),
        };

        Self::apply_move_action(state, action, move_direction)
    }

    fn is_goal_state(&self, state: &State) -> bool {
        for vehicle in &state.vehicle_state.vehicles {
            if vehicle.name == "red_car" {
                let positions = vehicle.positions();
                return positions.contains(&[2, state.grid.col_size - 2])
                    && positions.contains(&[2, state.grid.col_size - 1]);
            }
        }
        false
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
        let json_data: JsonValue = from_reader(reader).expect("Failed to parse JSON");

        // We assume the JSON is an object where each key is a problem name.
        let (problem_name, problem_data) = json_data
            .as_object()
            .expect("Expected a JSON object")
            .iter()
            .next()
            .expect("JSON is empty");

        // Extract grid dimensions.
        let row_size = problem_data["grid"]["row_size"]
            .as_u64()
            .expect("Missing grid row_size") as usize;
        let col_size = problem_data["grid"]["col_size"]
            .as_u64()
            .expect("Missing grid col_size") as usize;

        // Initialize Grid and an empty VehicleState.
        let mut grid = Grid::new(row_size, col_size);
        let mut vehicle_state = VehicleState::new();

        // Load vehicles from the JSON array.
        if let Some(vehicles) = problem_data["vehicles"].as_array() {
            for v in vehicles {
                let name = v["name"]
                    .as_str()
                    .expect("Missing vehicle name")
                    .to_string();
                let kind_str = v["kind"].as_str().expect("Missing vehicle kind");
                let kind = match kind_str {
                    "HorizontalCar" => VehicleKind::HorizontalCar,
                    "VerticalCar" => VehicleKind::VerticalCar,
                    "HorizontalTruck" => VehicleKind::HorizontalTruck,
                    "VerticalTruck" => VehicleKind::VerticalTruck,
                    _ => panic!("Unknown vehicle kind: {}", kind_str),
                };
                let position = [
                    v["position"][0].as_u64().expect("Invalid position") as usize,
                    v["position"][1].as_u64().expect("Invalid position") as usize,
                ];

                let vehicle = Vehicle::new(kind, position, name);
                vehicle_state.add_vehicle(vehicle.clone());
                grid.place_object(&vehicle)
                    .expect("Failed to place vehicle on grid");
            }
        }

        // Extract cost; default to 0 if missing.
        let cost = problem_data["cost"].as_i64().unwrap_or(0) as i32;

        // Construct the state.
        let state = State {
            grid,
            vehicle_state,
            cost,
        };

        (state, Self {})
    }
}
