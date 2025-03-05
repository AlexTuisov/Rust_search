use crate::search::{state::StateTrait, state::Value, action::Action};
use serde_json::from_reader;
use serde_json::Value as JsonValue;
use std::collections::{HashMap};
use std::fs::File;
use std::io::BufReader;
use serde::{Deserialize, Serialize};
use crate::problems::problem::Problem;
use ordered_float::OrderedFloat;
use crate::problems::aff_navigation_problem::utils::{ZLevel, has_line_of_sight};

include!("refined_heuristic.in");


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub timestamp: i64,
    pub drones: HashMap<String, Drone>, // Drone ID -> Drone object
    pub fires: Vec<Fire>, //  Fire objects
    pub people: Vec<PeopleGroup>
}

impl StateTrait for State {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Drone {
    pub id: String,
    pub x: OrderedFloat<f64>,
    pub y: OrderedFloat<f64>,
    pub z: ZLevel, // discrete enum
    pub speed: SpeedLevel,
    pub theta: OrderedFloat<f64>, // Orientation
    pub fuel: OrderedFloat<f64>,
    pub status: DroneStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DroneStatus {
    Operational,
    OutOfOrder,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Fire {
    pub x: OrderedFloat<f64>,
    pub y: OrderedFloat<f64>,
    pub radius: OrderedFloat<f64>,
    pub intensity: OrderedFloat<f64>, // Between 0 and 1
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeopleGroup {
    pub x: i32,
    pub y: i32,
    pub count: i32,
    pub status: RescueStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RescueStatus {
    AwaitingRescue,
    Rescued,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Obstacle {
    pub boundaries: Vec<(i32, i32)>, // Closed shape boundary points
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpeedLevel {
    Low,
    Cruise,
    Maximum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapSize {
    pub x: i32,
    pub y: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AFFNavigationProblem {
    pub dtm: HashMap<(i32, i32), OrderedFloat<f64>>, // (x, y) -> minimum flyable z
    pub obstacles: Vec<Obstacle>,
    pub map_size: MapSize,
}


impl Problem for AFFNavigationProblem {
    type State = State;

    fn get_possible_actions(&self, state: &Self::State) -> Vec<Action> {
        todo!("Generate all valid actions for a given state")
    }

    fn apply_action(&self, state: &Self::State, action: &Action) -> Self::State {
        todo!("Apply an action to a state and return the resulting state")
    }

    fn is_goal_state(&self, state: &Self::State) -> bool {
        todo!("Define what constitutes a goal state (e.g., all rescues completed)")
    }

    fn heuristic(&self, state: &Self::State) -> f64 {
        todo!("Implement a heuristic function for guiding search algorithms")
    }

    fn load_state_from_json(json_path: &str) -> (Self::State, Self) {
        // Open the file and create a buffered reader.
        let file = File::open(json_path).expect("Unable to open JSON file");
        let reader = BufReader::new(file);
        let v: JsonValue = serde_json::from_reader(reader).expect("Error parsing JSON");

        // Parse "map size" from the JSON.
        let map_size: MapSize = serde_json::from_value(v["map size"].clone())
            .expect("Error parsing map size");

        // Parse drones (stored as a dictionary in the JSON).
        let drones: HashMap<String, Drone> = serde_json::from_value(v["drones"].clone())
            .expect("Error parsing drones");

        // Parse fires from the JSON (as a dictionary keyed by id) and collect the values.
        let fires_map: HashMap<String, Fire> = serde_json::from_value(v["fires"].clone())
            .expect("Error parsing fires");
        let fires: Vec<Fire> = fires_map.into_values().collect();

        // Parse people (stored as an array in the JSON).
        let people: Vec<PeopleGroup> = serde_json::from_value(v["people"].clone())
            .expect("Error parsing people");

        // Build the State (timestamp is always 0).
        let state = State {
            timestamp: 0,
            drones,
            fires,
            people,
        };

        // Parse dtm:
        // The JSON dtm is a dictionary with keys formatted as "x,y" and integer values (0 to 4).
        let dtm_json_map: HashMap<String, i32> = serde_json::from_value(v["dtm"].clone())
            .expect("Error parsing dtm");

        // Populate the full dtm for each coordinate in the map.
        let mut dtm: HashMap<(i32, i32), OrderedFloat<f64>> = HashMap::new();
        for x in 0..map_size.x {
            for y in 0..map_size.y {
                let key = format!("{},{}", x, y);
                let value = dtm_json_map.get(&key).cloned().unwrap_or(0);
                dtm.insert((x, y), OrderedFloat(value as f64));
            }
        }

        // Parse obstacles from the JSON (as a dictionary keyed by id) and collect their values.
        let obstacles_map: HashMap<String, Obstacle> = serde_json::from_value(v["obstacles"].clone())
            .expect("Error parsing obstacles");
        let obstacles: Vec<Obstacle> = obstacles_map.into_values().collect();

        let problem = AFFNavigationProblem {
            dtm,
            obstacles,
            map_size,
        };

        (state, problem)
    }
}