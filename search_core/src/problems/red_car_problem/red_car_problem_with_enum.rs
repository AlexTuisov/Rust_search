use crate::problems::problem::Problem;
use crate::search::{action::Action, state::StateTrait, state::Value};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::fs;

// --- Vehicle Types Unified in an Enum ---

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HorizontalCar {
    pub x: i32,       // leftmost x coordinate of the car
    pub y: i32,       // y coordinate (row) of the car
    pub name: String, // unique vehicle identifier
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerticalCar {
    pub x: i32,       // x coordinate (column) of the car
    pub y: i32,       // topmost y coordinate of the car
    pub name: String, // unique vehicle identifier
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HorizontalTruck {
    pub x: i32,       // leftmost x coordinate of the truck
    pub y: i32,       // y coordinate (row) of the truck
    pub name: String, // unique vehicle identifier
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerticalTruck {
    pub x: i32,       // x coordinate (column) of the truck
    pub y: i32,       // topmost y coordinate of the truck
    pub name: String, // unique vehicle identifier
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "vehicle_type", rename_all = "snake_case")]
pub enum Vehicle {
    HorizontalCar(HorizontalCar),     // two-cell horizontal vehicle
    VerticalCar(VerticalCar),         // two-cell vertical vehicle
    HorizontalTruck(HorizontalTruck), // three-cell horizontal vehicle
    VerticalTruck(VerticalTruck),     // three-cell vertical vehicle
}

impl Vehicle {
    /// Returns the list of grid cells occupied by this vehicle
    pub fn get_positions(&self) -> Vec<(i32, i32)> {
        match self {
            Vehicle::HorizontalCar(car) => vec![(car.x, car.y), (car.x + 1, car.y)],
            Vehicle::VerticalCar(car) => vec![(car.x, car.y), (car.x, car.y + 1)],
            Vehicle::HorizontalTruck(truck) => vec![
                (truck.x, truck.y),
                (truck.x + 1, truck.y),
                (truck.x + 2, truck.y),
            ],
            Vehicle::VerticalTruck(truck) => vec![
                (truck.x, truck.y),
                (truck.x, truck.y + 1),
                (truck.x, truck.y + 2),
            ],
        }
    }

    /// Checks if the vehicle can move in the given direction without collision or out of bounds
    pub fn can_move(&self, dir: &str, grid: &Grid) -> bool {
        match self {
            Vehicle::HorizontalCar(car) => match dir {
                "right" => {
                    let x = car.x + 2;
                    let key = format!("({},{})", x, car.y);
                    x < grid.col_size && !grid.cells.contains_key(&key)
                }
                "left" => {
                    let x = car.x - 1;
                    let key = format!("({},{})", x, car.y);
                    car.x > 0 && !grid.cells.contains_key(&key)
                }
                _ => false,
            },
            Vehicle::VerticalCar(car) => match dir {
                "up" => {
                    let y = car.y + 2;
                    let key = format!("({},{})", car.x, y);
                    y < grid.row_size && !grid.cells.contains_key(&key)
                }
                "down" => {
                    let y = car.y - 1;
                    let key = format!("({},{})", car.x, y);
                    car.y > 0 && !grid.cells.contains_key(&key)
                }
                _ => false,
            },
            Vehicle::HorizontalTruck(t) => match dir {
                "right" => {
                    let x = t.x + 3;
                    let key = format!("({},{})", x, t.y);
                    x < grid.col_size && !grid.cells.contains_key(&key)
                }
                "left" => {
                    let x = t.x - 1;
                    let key = format!("({},{})", x, t.y);
                    t.x > 0 && !grid.cells.contains_key(&key)
                }
                _ => false,
            },
            Vehicle::VerticalTruck(t) => match dir {
                "up" => {
                    let y = t.y + 3;
                    let key = format!("({},{})", t.x, y);
                    y < grid.row_size && !grid.cells.contains_key(&key)
                }
                "down" => {
                    let y = t.y - 1;
                    let key = format!("({},{})", t.x, y);
                    t.y > 0 && !grid.cells.contains_key(&key)
                }
                _ => false,
            },
        }
    }

    /// Applies a movement to the vehicle by updating its x or y coordinate
    pub fn apply_action(&mut self, direction: &str) {
        match self {
            Vehicle::HorizontalCar(car) => match direction {
                "right" => car.x += 1,
                "left" => car.x -= 1,
                _ => panic!("Invalid move for HorizontalCar: {}", direction),
            },
            Vehicle::VerticalCar(car) => match direction {
                "up" => car.y += 1,
                "down" => car.y -= 1,
                _ => panic!("Invalid move for VerticalCar: {}", direction),
            },
            Vehicle::HorizontalTruck(truck) => match direction {
                "right" => truck.x += 1,
                "left" => truck.x -= 1,
                _ => panic!("Invalid move for HorizontalTruck: {}", direction),
            },
            Vehicle::VerticalTruck(truck) => match direction {
                "up" => truck.y += 1,
                "down" => truck.y -= 1,
                _ => panic!("Invalid move for VerticalTruck: {}", direction),
            },
        }
    }

    /// Returns the vehicle's unique name
    pub fn get_name(&self) -> &str {
        match self {
            Vehicle::HorizontalCar(car) => &car.name,
            Vehicle::VerticalCar(car) => &car.name,
            Vehicle::HorizontalTruck(truck) => &truck.name,
            Vehicle::VerticalTruck(truck) => &truck.name,
        }
    }
}

// --- Grid Representation ---

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Grid {
    pub row_size: i32,                  // number of rows
    pub col_size: i32,                  // number of columns
    pub cells: HashMap<String, String>, // map from cell to occupying vehicle name
}

impl Grid {
    /// Initializes an empty grid of given dimensions
    pub fn new(row_size: i32, col_size: i32) -> Self {
        Grid {
            row_size,
            col_size,
            cells: HashMap::new(),
        }
    }

    /// Updates the grid when a vehicle moves: removes old cells and inserts new ones
    pub fn update(
        &mut self,
        old_positions: Vec<(i32, i32)>,
        new_positions: Vec<(i32, i32)>,
        name: &str,
    ) {
        // remove old
        for (x, y) in old_positions {
            let key = format!("({},{})", x, y);
            self.cells.remove(&key);
        }
        // insert new
        for (x, y) in new_positions {
            let key = format!("({},{})", x, y);
            self.cells.insert(key, name.to_string());
        }
    }
}

// Unified state using Vehicle enum
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub grid: Grid,             // grid after placement
    pub vehicles: Vec<Vehicle>, // all vehicles in a single vector
}

impl StateTrait for State {}

// --- Red Car Problem Implementation ---

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RedCarProblem {}

impl RedCarProblem {
    /// Generate all valid moves (up/down/left/right) for every vehicle
    pub fn get_possible_actions(state: &State) -> Vec<Action> {
        let directions = ["left", "right", "up", "down"];
        let mut actions = Vec::new();
        for vehicle in &state.vehicles {
            for &dir in &directions {
                if vehicle.can_move(dir, &state.grid) {
                    let mut params = HashMap::new();
                    let name = format!("{}_move_{}", vehicle.get_name(), dir);
                    params.insert(
                        "vehicle".to_string(),
                        Value::Text(vehicle.get_name().to_string()),
                    );
                    params.insert("move".to_string(), Value::Text(dir.to_string()));
                    actions.push(Action::new(name, 1, params));
                }
            }
        }
        actions
    }
}

impl Problem for RedCarProblem {
    type State = State;

    fn get_possible_actions(&self, state: &State) -> Vec<Action> {
        Self::get_possible_actions(state)
    }

    /// Applies the chosen action by moving the corresponding vehicle and updating the grid
    fn apply_action(&self, state: &State, action: &Action) -> State {
        let dir = match action.parameters.get("move") {
            Some(Value::Text(d)) => d.as_str(),
            _ => panic!("Missing 'move' parameter"),
        };
        let veh_name = match action.parameters.get("vehicle") {
            Some(Value::Text(v)) => v.as_str(),
            _ => panic!("Missing 'vehicle' parameter"),
        };
        let mut new_state = state.clone();
        for vehicle in new_state.vehicles.iter_mut() {
            if vehicle.get_name() == veh_name {
                let old_pos = vehicle.get_positions();
                vehicle.apply_action(dir);
                let new_pos = vehicle.get_positions();
                new_state.grid.update(old_pos, new_pos, veh_name);
                break;
            }
        }
        new_state
    }

    /// Goal check: red-car must end at right edge on row 2
    fn is_goal_state(&self, state: &State) -> bool {
        for v in &state.vehicles {
            if v.get_name() == "red-car" {
                let pos = v.get_positions();
                if let Some(&(x, y)) = pos.first() {
                    return x == state.grid.col_size - 2 && y == 2;
                }
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
