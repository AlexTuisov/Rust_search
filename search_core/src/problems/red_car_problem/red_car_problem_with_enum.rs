use crate::problems::problem::Problem;
use crate::search::{action::Action, state::StateTrait, state::Value};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::fs;

// --- Vehicle Types and Enum ---

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HorizontalCar {
    pub x: i32,
    pub y: i32,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerticalCar {
    pub x: i32,
    pub y: i32,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HorizontalTruck {
    pub x: i32,
    pub y: i32,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerticalTruck {
    pub x: i32,
    pub y: i32,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "vehicle_type", rename_all = "snake_case")]
pub enum Vehicle {
    HorizontalCar(HorizontalCar),
    VerticalCar(VerticalCar),
    HorizontalTruck(HorizontalTruck),
    VerticalTruck(VerticalTruck),
}

impl Vehicle {
    pub fn get_positions(&self) -> Vec<(i32, i32)> {
        match self {
            Vehicle::HorizontalCar(car) => vec![(car.x, car.y), (car.x + 1, car.y)],
            Vehicle::VerticalCar(car) => vec![(car.x, car.y), (car.x, car.y + 1)],
            Vehicle::HorizontalTruck(truck) => {
                vec![
                    (truck.x, truck.y),
                    (truck.x + 1, truck.y),
                    (truck.x + 2, truck.y),
                ]
            }
            Vehicle::VerticalTruck(truck) => {
                vec![
                    (truck.x, truck.y),
                    (truck.x, truck.y + 1),
                    (truck.x, truck.y + 2),
                ]
            }
        }
    }

    pub fn can_move(&self, direction: &str, grid: &Grid) -> bool {
        match self {
            Vehicle::HorizontalCar(car) => match direction {
                "right" => {
                    let end_position = car.x + 2;
                    end_position < grid.col_size && !grid.cells.contains_key(&(end_position, car.y))
                }
                "left" => car.x > 0 && !grid.cells.contains_key(&(car.x - 1, car.y)),
                _ => false,
            },
            Vehicle::VerticalCar(car) => match direction {
                "up" => {
                    let end_position = car.y + 2;
                    end_position < grid.row_size && !grid.cells.contains_key(&(car.x, end_position))
                }
                "down" => car.y > 0 && !grid.cells.contains_key(&(car.x, car.y - 1)),
                _ => false,
            },
            Vehicle::HorizontalTruck(truck) => match direction {
                "right" => {
                    let end_position = truck.x + 3;
                    end_position < grid.col_size
                        && !grid.cells.contains_key(&(end_position, truck.y))
                }
                "left" => truck.x > 0 && !grid.cells.contains_key(&(truck.x - 1, truck.y)),
                _ => false,
            },
            Vehicle::VerticalTruck(truck) => match direction {
                "up" => {
                    let end_position = truck.y + 3;
                    end_position < grid.row_size
                        && !grid.cells.contains_key(&(truck.x, end_position))
                }
                "down" => truck.y > 0 && !grid.cells.contains_key(&(truck.x, truck.y - 1)),
                _ => false,
            },
        }
    }

    pub fn apply_action(&mut self, direction: &str) {
        match self {
            Vehicle::HorizontalCar(car) => match direction {
                "right" => car.x += 1,
                "left" => car.x -= 1,
                _ => panic!("Invalid move for HorizontalCar"),
            },
            Vehicle::VerticalCar(car) => match direction {
                "up" => car.y += 1,
                "down" => car.y -= 1,
                _ => panic!("Invalid move for VerticalCar"),
            },
            Vehicle::HorizontalTruck(truck) => match direction {
                "right" => truck.x += 1,
                "left" => truck.x -= 1,
                _ => panic!("Invalid move for HorizontalTruck"),
            },
            Vehicle::VerticalTruck(truck) => match direction {
                "up" => truck.y += 1,
                "down" => truck.y -= 1,
                _ => panic!("Invalid move for VerticalTruck"),
            },
        }
    }

    pub fn get_name(&self) -> &str {
        match self {
            Vehicle::HorizontalCar(car) => &car.name,
            Vehicle::VerticalCar(car) => &car.name,
            Vehicle::HorizontalTruck(truck) => &truck.name,
            Vehicle::VerticalTruck(truck) => &truck.name,
        }
    }
}

// --- Grid and State ---

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Grid {
    pub row_size: i32,
    pub col_size: i32,
    pub cells: HashMap<(i32, i32), String>,
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
        for pos in old_positions {
            self.cells.remove(&pos);
        }
        for pos in new_positions {
            self.cells.insert(pos, name.to_string());
        }
    }

    pub fn place_object(&mut self, name: String, positions: &[(i32, i32)]) -> Result<(), String> {
        for &(x, y) in positions {
            if x < 0 || x >= self.col_size || y < 0 || y >= self.row_size {
                return Err(format!(
                    "Object '{}' is out of bounds at ({}, {}).",
                    name, x, y
                ));
            }
            if self.cells.contains_key(&(x, y)) {
                return Err(format!("Cell ({}, {}) is already occupied.", x, y));
            }
        }
        for &(x, y) in positions {
            self.cells.insert((x, y), name.clone());
        }
        Ok(())
    }
}

// --- Legacy State Structure for Deserialization ---

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LegacyState {
    pub grid: Grid,
    pub horizontalcars: Vec<HorizontalCar>,
    pub verticalcars: Vec<VerticalCar>,
    pub horizontaltrucks: Vec<HorizontalTruck>,
    pub verticaltrucks: Vec<VerticalTruck>,
}

// The new unified State holds a grid and a vector of vehicles.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub grid: Grid,
    pub vehicles: Vec<Vehicle>,
}

impl StateTrait for State {}

// --- Red Car Problem Using Enum for Vehicles ---

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RedCarProblem {}

impl RedCarProblem {
    pub fn get_possible_actions(state: &State) -> Vec<Action> {
        let directions = vec!["left", "right", "up", "down"];
        let mut actions = Vec::new();
        for vehicle in &state.vehicles {
            for &direction in &directions {
                if vehicle.can_move(direction, &state.grid) {
                    let mut parameters = HashMap::new();
                    let action_name = format!("{}_move_{}", vehicle.get_name(), direction);
                    parameters.insert(
                        "vehicle".to_string(),
                        Value::Text(vehicle.get_name().to_string()),
                    );
                    parameters.insert("move".to_string(), Value::Text(direction.to_string()));
                    actions.push(Action::new(action_name, 1, parameters));
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

    fn apply_action(&self, state: &State, action: &Action) -> State {
        let move_direction = match action.parameters.get("move") {
            Some(Value::Text(dir)) => dir.as_str(),
            _ => panic!("Missing 'move' parameter"),
        };
        let vehicle_name = match action.parameters.get("vehicle") {
            Some(Value::Text(name)) => name.as_str(),
            _ => panic!("Missing 'vehicle' parameter"),
        };

        let mut new_state = state.clone();

        for vehicle in new_state.vehicles.iter_mut() {
            if vehicle.get_name() == vehicle_name {
                let old_positions = vehicle.get_positions();
                vehicle.apply_action(move_direction);
                let new_positions = vehicle.get_positions();
                new_state
                    .grid
                    .update(old_positions, new_positions, vehicle_name);
                break;
            }
        }
        new_state
    }

    fn is_goal_state(&self, state: &State) -> bool {
        // Assume the red car is a horizontal car and check its leftmost coordinate.
        for vehicle in &state.vehicles {
            if vehicle.get_name() == "red-car" {
                let positions = vehicle.get_positions();
                if let Some(&(x, y)) = positions.first() {
                    return x == state.grid.col_size - 2 && y == 2;
                }
            }
        }
        false
    }

    fn heuristic(&self, _state: &State) -> f64 {
        0.0
    }

    fn load_state_from_json(json_path: &str) -> (State, RedCarProblem) {
        // Read the legacy JSON.
        let json_str = fs::read_to_string(json_path).expect("Failed to read JSON file");
        let json_value: JsonValue = serde_json::from_str(&json_str).expect("Failed to parse JSON");

        let legacy_state_value = json_value.get("state").expect("Missing 'state' field");
        let legacy_state: LegacyState = serde_json::from_value(legacy_state_value.clone())
            .expect("Failed to deserialize legacy state");

        // Convert legacy state fields into the unified vehicles vector.
        let mut vehicles = Vec::new();
        for car in legacy_state.horizontalcars {
            vehicles.push(Vehicle::HorizontalCar(car));
        }
        for car in legacy_state.verticalcars {
            vehicles.push(Vehicle::VerticalCar(car));
        }
        for truck in legacy_state.horizontaltrucks {
            vehicles.push(Vehicle::HorizontalTruck(truck));
        }
        for truck in legacy_state.verticaltrucks {
            vehicles.push(Vehicle::VerticalTruck(truck));
        }

        let mut state = State {
            grid: legacy_state.grid,
            vehicles,
        };

        // Place each vehicle on the grid.
        for vehicle in &state.vehicles {
            state
                .grid
                .place_object(vehicle.get_name().to_string(), &vehicle.get_positions())
                .expect("failed to place car on grid");
        }

        (state, RedCarProblem {})
    }
}
