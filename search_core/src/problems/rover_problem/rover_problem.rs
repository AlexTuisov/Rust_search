use crate::problems::problem::Problem;
use crate::search::{action::Action, state::StateTrait, state::Value};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rover {
    pub id: String,
    pub location: String, // Waypoint ID
    pub energy: i32,
    pub equipped_for_soil_analysis: bool,
    pub equipped_for_rock_analysis: bool,
    pub equipped_for_imaging: bool,
    pub available: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Store {
    pub id: String,
    pub rover_id: String, // Rover ID this store belongs to
    pub empty: bool,
    pub full: bool, // Added to match PDDL's (full ?s - store) predicate
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Camera {
    pub id: String,
    pub rover_id: String,             // Rover ID this camera is on
    pub calibration_target: String,   // Objective ID
    pub supported_modes: Vec<String>, // Mode IDs
    pub calibrated_objective: Option<String>, // New: Which objective this camera is calibrated for
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Waypoint {
    pub id: String,
    pub has_soil_sample: bool, // Matches PDDL's (at_soil_sample ?w - waypoint)
    pub has_rock_sample: bool, // Matches PDDL's (at_rock_sample ?w - waypoint)
    pub in_sun: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Objective {
    pub id: String,
    pub visible_from: Vec<String>, // Waypoint IDs
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Lander {
    pub id: String,
    pub location: String, // Waypoint ID
    pub channel_free: bool,
}

// State contains only dynamic elements that change during search
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub rovers: HashMap<String, Rover>,
    pub stores: HashMap<String, Store>,
    pub cameras: HashMap<String, Camera>,
    pub waypoints: HashMap<String, Waypoint>,
    pub landers: HashMap<String, Lander>,
    pub soil_analysis: HashMap<String, HashSet<String>>, // Rover ID -> Waypoints analyzed
    pub rock_analysis: HashMap<String, HashSet<String>>, // Rover ID -> Waypoints analyzed
    pub images: HashMap<String, HashMap<String, HashSet<String>>>, // Rover ID -> Objective ID -> Modes
    pub communicated_soil_data: HashSet<String>,                   // Waypoint IDs
    pub communicated_rock_data: HashSet<String>,                   // Waypoint IDs
    pub communicated_image_data: HashMap<String, HashSet<String>>, // Objective ID -> Modes
    pub recharges: i32,
}

impl StateTrait for State {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GoalCondition {
    SoilDataCommunicated { waypoint: String },
    RockDataCommunicated { waypoint: String },
    ImageDataCommunicated { objective: String, mode: String },
}

impl GoalCondition {
    pub fn is_satisfied(&self, state: &State) -> bool {
        match self {
            GoalCondition::SoilDataCommunicated { waypoint } => {
                state.communicated_soil_data.contains(waypoint)
            }
            GoalCondition::RockDataCommunicated { waypoint } => {
                state.communicated_rock_data.contains(waypoint)
            }
            GoalCondition::ImageDataCommunicated { objective, mode } => state
                .communicated_image_data
                .get(objective)
                .map_or(false, |modes| modes.contains(mode)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Goal {
    pub conditions: Vec<GoalCondition>,
}

impl Goal {
    pub fn is_goal_state(&self, state: &State) -> bool {
        self.conditions.iter().all(|cond| cond.is_satisfied(state))
    }
}

// Problem contains static elements that don't change during search
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoverProblem {
    pub goal: Goal,
    pub objectives: HashMap<String, Objective>, // Static data
    pub can_traverse: HashMap<String, HashMap<String, Vec<String>>>, // Static data
    pub visible: HashMap<String, Vec<String>>,  // Static data
}

impl RoverProblem {
    // Action generator methods
    pub fn get_navigate_action(rover: &Rover, from: &str, to: &str) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("rover".to_string(), Value::Text(rover.id.clone()));
        parameters.insert("from".to_string(), Value::Text(from.to_string()));
        parameters.insert("to".to_string(), Value::Text(to.to_string()));
        Action::new(
            format!("navigate_{}_{}_{}", rover.id, from, to),
            8,
            parameters,
        )
    }

    pub fn get_recharge_action(rover: &Rover, waypoint: &str) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("rover".to_string(), Value::Text(rover.id.clone()));
        parameters.insert("waypoint".to_string(), Value::Text(waypoint.to_string()));
        Action::new(format!("recharge_{}_{}", rover.id, waypoint), 1, parameters)
    }

    pub fn get_sample_soil_action(rover: &Rover, store: &Store, waypoint: &str) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("rover".to_string(), Value::Text(rover.id.clone()));
        parameters.insert("store".to_string(), Value::Text(store.id.clone()));
        parameters.insert("waypoint".to_string(), Value::Text(waypoint.to_string()));
        Action::new(
            format!("sample_soil_{}_{}_{}", rover.id, store.id, waypoint),
            3,
            parameters,
        )
    }

    pub fn get_sample_rock_action(rover: &Rover, store: &Store, waypoint: &str) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("rover".to_string(), Value::Text(rover.id.clone()));
        parameters.insert("store".to_string(), Value::Text(store.id.clone()));
        parameters.insert("waypoint".to_string(), Value::Text(waypoint.to_string()));
        Action::new(
            format!("sample_rock_{}_{}_{}", rover.id, store.id, waypoint),
            5,
            parameters,
        )
    }

    pub fn get_drop_action(rover: &Rover, store: &Store) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("rover".to_string(), Value::Text(rover.id.clone()));
        parameters.insert("store".to_string(), Value::Text(store.id.clone()));
        Action::new(format!("drop_{}_{}", rover.id, store.id), 1, parameters)
    }

    pub fn get_calibrate_action(
        rover: &Rover,
        camera: &Camera,
        objective: &str,
        waypoint: &str,
    ) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("rover".to_string(), Value::Text(rover.id.clone()));
        parameters.insert("camera".to_string(), Value::Text(camera.id.clone()));
        parameters.insert("objective".to_string(), Value::Text(objective.to_string()));
        parameters.insert("waypoint".to_string(), Value::Text(waypoint.to_string()));
        Action::new(
            format!(
                "calibrate_{}_{}_{}_{}",
                rover.id, camera.id, objective, waypoint
            ),
            2,
            parameters,
        )
    }

    pub fn get_take_image_action(
        rover: &Rover,
        waypoint: &str,
        objective: &str,
        camera: &Camera,
        mode: &str,
    ) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("rover".to_string(), Value::Text(rover.id.clone()));
        parameters.insert("waypoint".to_string(), Value::Text(waypoint.to_string()));
        parameters.insert("objective".to_string(), Value::Text(objective.to_string()));
        parameters.insert("camera".to_string(), Value::Text(camera.id.clone()));
        parameters.insert("mode".to_string(), Value::Text(mode.to_string()));
        Action::new(
            format!(
                "take_image_{}_{}_{}_{}_{}",
                rover.id, waypoint, objective, camera.id, mode
            ),
            1,
            parameters,
        )
    }

    pub fn get_communicate_soil_data_action(
        rover: &Rover,
        lander: &Lander,
        sample_waypoint: &str,
        rover_waypoint: &str,
        lander_waypoint: &str,
    ) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("rover".to_string(), Value::Text(rover.id.clone()));
        parameters.insert("lander".to_string(), Value::Text(lander.id.clone()));
        parameters.insert(
            "sample_waypoint".to_string(),
            Value::Text(sample_waypoint.to_string()),
        );
        parameters.insert(
            "rover_waypoint".to_string(),
            Value::Text(rover_waypoint.to_string()),
        );
        parameters.insert(
            "lander_waypoint".to_string(),
            Value::Text(lander_waypoint.to_string()),
        );
        Action::new(
            format!(
                "communicate_soil_data_{}_{}_{}_{}_{}",
                rover.id, lander.id, sample_waypoint, rover_waypoint, lander_waypoint
            ),
            4,
            parameters,
        )
    }

    pub fn get_communicate_rock_data_action(
        rover: &Rover,
        lander: &Lander,
        sample_waypoint: &str,
        rover_waypoint: &str,
        lander_waypoint: &str,
    ) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("rover".to_string(), Value::Text(rover.id.clone()));
        parameters.insert("lander".to_string(), Value::Text(lander.id.clone()));
        parameters.insert(
            "sample_waypoint".to_string(),
            Value::Text(sample_waypoint.to_string()),
        );
        parameters.insert(
            "rover_waypoint".to_string(),
            Value::Text(rover_waypoint.to_string()),
        );
        parameters.insert(
            "lander_waypoint".to_string(),
            Value::Text(lander_waypoint.to_string()),
        );
        Action::new(
            format!(
                "communicate_rock_data_{}_{}_{}_{}_{}",
                rover.id, lander.id, sample_waypoint, rover_waypoint, lander_waypoint
            ),
            4,
            parameters,
        )
    }

    pub fn get_communicate_image_data_action(
        rover: &Rover,
        lander: &Lander,
        objective: &str,
        mode: &str,
        rover_waypoint: &str,
        lander_waypoint: &str,
    ) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("rover".to_string(), Value::Text(rover.id.clone()));
        parameters.insert("lander".to_string(), Value::Text(lander.id.clone()));
        parameters.insert("objective".to_string(), Value::Text(objective.to_string()));
        parameters.insert("mode".to_string(), Value::Text(mode.to_string()));
        parameters.insert(
            "rover_waypoint".to_string(),
            Value::Text(rover_waypoint.to_string()),
        );
        parameters.insert(
            "lander_waypoint".to_string(),
            Value::Text(lander_waypoint.to_string()),
        );
        Action::new(
            format!(
                "communicate_image_data_{}_{}_{}_{}_{}_{}",
                rover.id, lander.id, objective, mode, rover_waypoint, lander_waypoint
            ),
            6,
            parameters,
        )
    }

    // Get all possible actions for the current state
    pub fn get_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();

        // Navigate actions
        for (rover_id, rover) in &state.rovers {
            if rover.available {
                let current_waypoint = &rover.location;

                if let Some(traversable_from_current) = self
                    .can_traverse
                    .get(rover_id)
                    .and_then(|waypoints| waypoints.get(current_waypoint))
                {
                    for to_waypoint in traversable_from_current {
                        if self
                            .visible
                            .get(current_waypoint)
                            .map_or(false, |visible_waypoints| {
                                visible_waypoints.contains(to_waypoint)
                            })
                            && rover.energy >= 8
                        {
                            actions.push(Self::get_navigate_action(
                                rover,
                                current_waypoint,
                                to_waypoint,
                            ));
                        }
                    }
                }
            }
        }

        // Recharge actions
        for rover in state.rovers.values() {
            let waypoint = &rover.location;
            if state.waypoints.get(waypoint).map_or(false, |w| w.in_sun) && rover.energy <= 80 {
                actions.push(Self::get_recharge_action(rover, waypoint));
            }
        }

        // Sample soil actions
        for rover in state.rovers.values() {
            if rover.equipped_for_soil_analysis && rover.energy >= 3 {
                let waypoint = &rover.location;
                if state
                    .waypoints
                    .get(waypoint)
                    .map_or(false, |w| w.has_soil_sample)
                {
                    for store in state.stores.values() {
                        if store.rover_id == rover.id && store.empty {
                            actions.push(Self::get_sample_soil_action(rover, store, waypoint));
                        }
                    }
                }
            }
        }

        // Sample rock actions
        for rover in state.rovers.values() {
            if rover.equipped_for_rock_analysis && rover.energy >= 5 {
                let waypoint = &rover.location;
                if state
                    .waypoints
                    .get(waypoint)
                    .map_or(false, |w| w.has_rock_sample)
                {
                    for store in state.stores.values() {
                        if store.rover_id == rover.id && store.empty {
                            actions.push(Self::get_sample_rock_action(rover, store, waypoint));
                        }
                    }
                }
            }
        }

        // Drop actions
        for rover in state.rovers.values() {
            for store in state.stores.values() {
                if store.rover_id == rover.id && store.full {
                    actions.push(Self::get_drop_action(rover, store));
                }
            }
        }

        
            // Calibrate actions
            for rover in state.rovers.values() {
                if rover.equipped_for_imaging && rover.energy >= 2 {
                    let waypoint: &String = &rover.location;

                    for camera in state.cameras.values() {
                        if camera.rover_id == rover.id && camera.calibrated_objective.is_none() {
                            let objective_id: &String = &camera.calibration_target;

                            if let Some(objective) = self.objectives.get(objective_id) {
                                if objective.visible_from.contains(waypoint) {
                                    actions.push(Self::get_calibrate_action(
                                        rover, camera, objective_id, waypoint,
                                    ));
                                }
                            }
                        }
                    }
                }
            }

            // Take image actions
            for rover in state.rovers.values() {
                if rover.equipped_for_imaging && rover.energy >= 1 {
                    let waypoint = &rover.location;

                    for camera in state.cameras.values() {
                        if camera.rover_id == rover.id {
                            if let Some(obj_id) = &camera.calibrated_objective {
                                if let Some(objective) = self.objectives.get(obj_id) {
                                    if objective.visible_from.contains(waypoint) {
                                        for mode in &camera.supported_modes {
                                            actions.push(Self::get_take_image_action(
                                                rover, waypoint, obj_id, camera, mode,
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

                    

            // Communicate soil data actions
            for rover in state.rovers.values() {
                if rover.available && rover.energy >= 4 {
                    let rover_waypoint = &rover.location;

                    if let Some(soil_analyzed) = state.soil_analysis.get(&rover.id) {
                        for sample_waypoint in soil_analyzed {
                            // Check if this soil data has already been communicated
                            if !state.communicated_soil_data.contains(sample_waypoint) {
                                for lander in state.landers.values() {
                                    let lander_waypoint = &lander.location;

                                    if self
                                        .visible
                                        .get(rover_waypoint)
                                        .map_or(false, |visible| visible.contains(lander_waypoint))
                                        && lander.channel_free
                                    {
                                        actions.push(Self::get_communicate_soil_data_action(
                                            rover,
                                            lander,
                                            sample_waypoint,
                                            rover_waypoint,
                                            lander_waypoint,
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
            }


            // Communicate rock data actions
            for rover in state.rovers.values() {
                if rover.available && rover.energy >= 4 {
                    let rover_waypoint = &rover.location;

                    if let Some(rock_analyzed) = state.rock_analysis.get(&rover.id) {
                        for sample_waypoint in rock_analyzed {
                            // Check if this rock data has already been communicated
                            if !state.communicated_rock_data.contains(sample_waypoint) {
                                for lander in state.landers.values() {
                                    let lander_waypoint = &lander.location;

                                    if self
                                        .visible
                                        .get(rover_waypoint)
                                        .map_or(false, |visible| visible.contains(lander_waypoint))
                                        && lander.channel_free
                                    {
                                        actions.push(Self::get_communicate_rock_data_action(
                                            rover,
                                            lander,
                                            sample_waypoint,
                                            rover_waypoint,
                                            lander_waypoint,
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
            }


            // Communicate image data actions
            for rover in state.rovers.values() {
                if rover.available && rover.energy >= 6 {
                    let rover_waypoint = &rover.location;

                    if let Some(images) = state.images.get(&rover.id) {
                        for (objective_id, modes) in images {
                            for mode in modes {
                                // Check if this image data has already been communicated
                                if !state.communicated_image_data
                                    .get(objective_id)
                                    .map_or(false, |communicated_modes| communicated_modes.contains(mode))
                                {
                                    for lander in state.landers.values() {
                                        let lander_waypoint = &lander.location;

                                        if self
                                            .visible
                                            .get(rover_waypoint)
                                            .map_or(false, |visible| visible.contains(lander_waypoint))
                                            && lander.channel_free
                                        {
                                            actions.push(Self::get_communicate_image_data_action(
                                                rover,
                                                lander,
                                                objective_id,
                                                mode,
                                                rover_waypoint,
                                                lander_waypoint,
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }


        actions
    }

    // Action application methods
    pub fn apply_navigate_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();

        let rover_id = match action.parameters.get("rover") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid rover ID in action"),
        };

        let to_waypoint = match action.parameters.get("to") {
            Some(Value::Text(waypoint)) => waypoint,
            _ => panic!("Invalid destination waypoint in action"),
        };

        if let Some(rover) = new_state.rovers.get_mut(rover_id) {
            rover.location = to_waypoint.clone();
            rover.energy -= 8;
        }

        new_state
    }

    pub fn apply_recharge_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();

        let rover_id = match action.parameters.get("rover") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid rover ID in action"),
        };

        if let Some(rover) = new_state.rovers.get_mut(rover_id) {
            rover.energy += 20;
            if rover.energy > 100 {
                rover.energy = 100; // Cap energy at 100
            }
        }

        new_state.recharges += 1;
        new_state
    }

    pub fn apply_sample_soil_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();

        let rover_id = match action.parameters.get("rover") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid rover ID in action"),
        };

        let store_id = match action.parameters.get("store") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid store ID in action"),
        };

        let waypoint_id = match action.parameters.get("waypoint") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid waypoint ID in action"),
        };

        // Update the store - mark as not empty and full
        if let Some(store) = new_state.stores.get_mut(store_id) {
            store.empty = false;
            store.full = true;
        }

        // Update the waypoint - remove soil sample
        if let Some(waypoint) = new_state.waypoints.get_mut(waypoint_id) {
            waypoint.has_soil_sample = false;
        }

        // Update the rover's energy and soil analysis
        if let Some(rover) = new_state.rovers.get_mut(rover_id) {
            rover.energy -= 3;

            // Record that this rover has analyzed soil at this waypoint
            new_state
                .soil_analysis
                .entry(rover_id.clone())
                .or_insert_with(HashSet::new)
                .insert(waypoint_id.clone());
        }

        new_state
    }

    pub fn apply_sample_rock_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();

        let rover_id = match action.parameters.get("rover") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid rover ID in action"),
        };

        let store_id = match action.parameters.get("store") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid store ID in action"),
        };

        let waypoint_id = match action.parameters.get("waypoint") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid waypoint ID in action"),
        };

        // Update the store - mark as not empty and full
        if let Some(store) = new_state.stores.get_mut(store_id) {
            store.empty = false;
            store.full = true;
        }

        // Update the waypoint - remove rock sample
        if let Some(waypoint) = new_state.waypoints.get_mut(waypoint_id) {
            waypoint.has_rock_sample = false;
        }

        // Update the rover's energy and rock analysis
        if let Some(rover) = new_state.rovers.get_mut(rover_id) {
            rover.energy -= 5;

            // Record that this rover has analyzed rock at this waypoint
            new_state
                .rock_analysis
                .entry(rover_id.clone())
                .or_insert_with(HashSet::new)
                .insert(waypoint_id.clone());
        }

        new_state
    }

    pub fn apply_drop_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();

        let store_id = match action.parameters.get("store") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid store ID in action"),
        };

        if let Some(store) = new_state.stores.get_mut(store_id) {
            store.empty = true;
            store.full = false;
        }

        new_state
    }

    pub fn apply_calibrate_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
    
        let rover_id = match action.parameters.get("rover") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid rover ID in action"),
        };
        let camera_id = match action.parameters.get("camera") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid camera ID in action"),
        };
        let objective_id = match action.parameters.get("objective") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid objective ID in action"),
        };
    
        // Update camera calibration only if it's the intended calibration target
        if let Some(camera) = new_state.cameras.get_mut(camera_id) {
            if camera.calibration_target == *objective_id {
                camera.calibrated_objective = Some(objective_id.to_string());
            }
        }
    
        // Reduce rover energy
        if let Some(rover) = new_state.rovers.get_mut(rover_id) {
            rover.energy -= 2;
        }
    
        new_state
    }
        

    pub fn apply_take_image_action(state: &State, action: &Action) -> State {
    let mut new_state = state.clone();

    let rover_id = match action.parameters.get("rover") {
        Some(Value::Text(id)) => id,
        _ => panic!("Invalid rover ID in action"),
    };
    let camera_id = match action.parameters.get("camera") {
        Some(Value::Text(id)) => id,
        _ => panic!("Invalid camera ID in action"),
    };
    let objective_id = match action.parameters.get("objective") {
        Some(Value::Text(id)) => id,
        _ => panic!("Invalid objective ID in action"),
    };
    let mode = match action.parameters.get("mode") {
        Some(Value::Text(id)) => id,
        _ => panic!("Invalid mode in action"),
    };

    if let Some(camera) = new_state.cameras.get_mut(camera_id) {
        camera.calibrated_objective = None; // Clear calibration after image
    }

    if let Some(rover) = new_state.rovers.get_mut(rover_id) {
        rover.energy -= 1;

        new_state
            .images
            .entry(rover_id.clone())
            .or_insert_with(HashMap::new)
            .entry(objective_id.clone())
            .or_insert_with(HashSet::new)
            .insert(mode.clone());
    }

    new_state
}


    pub fn apply_communicate_soil_data_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();

        let rover_id = match action.parameters.get("rover") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid rover ID in action"),
        };

        let sample_waypoint = match action.parameters.get("sample_waypoint") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid sample waypoint ID in action"),
        };

        // Update the rover's energy
        if let Some(rover) = new_state.rovers.get_mut(rover_id) {
            rover.energy -= 4;
        }

        // Record that this soil data has been communicated
        new_state
            .communicated_soil_data
            .insert(sample_waypoint.clone());

        new_state
    }

    pub fn apply_communicate_rock_data_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();

        let rover_id = match action.parameters.get("rover") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid rover ID in action"),
        };

        let sample_waypoint = match action.parameters.get("sample_waypoint") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid sample waypoint ID in action"),
        };

        // Update the rover's energy
        if let Some(rover) = new_state.rovers.get_mut(rover_id) {
            rover.energy -= 4;
        }

        // Record that this rock data has been communicated
        new_state
            .communicated_rock_data
            .insert(sample_waypoint.clone());

        new_state
    }

    pub fn apply_communicate_image_data_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();

        let rover_id = match action.parameters.get("rover") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid rover ID in action"),
        };

        let objective_id = match action.parameters.get("objective") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid objective ID in action"),
        };

        let mode = match action.parameters.get("mode") {
            Some(Value::Text(mode)) => mode,
            _ => panic!("Invalid mode in action"),
        };

        // Update the rover's energy
        if let Some(rover) = new_state.rovers.get_mut(rover_id) {
            rover.energy -= 6;
        }

        // Record that this image data has been communicated
        new_state
            .communicated_image_data
            .entry(objective_id.clone())
            .or_insert_with(HashSet::new)
            .insert(mode.clone());

        new_state
    }

    pub fn load_state_from_json(json_path: &str) -> (State, RoverProblem) {
        // Read the JSON file into a string
        let json_str = fs::read_to_string(json_path).expect("Failed to read JSON file");
    
        // Parse the JSON string into a serde_json::Value
        let json_value: JsonValue = serde_json::from_str(&json_str).expect("Failed to parse JSON");
    
        // Extract the "state" and "problem" fields
        let state_value = json_value
            .get("state")
            .expect("Missing 'state' field in JSON");
        let problem_value = json_value
            .get("problem")
            .expect("Missing 'problem' field in JSON");
    
        // Deserialize each part into the corresponding struct
        let state: State = serde_json::from_value(state_value.clone())
            .expect("Failed to deserialize state");
        
        let problem: RoverProblem = serde_json::from_value(problem_value.clone())
            .expect("Failed to deserialize problem");
    
        (state, problem)
    }
    
}

impl Problem for RoverProblem {
    type State = State;

    fn get_possible_actions(&self, state: &State) -> Vec<Action> {
        self.get_actions(state)
    }

    fn apply_action(&self, state: &State, action: &Action) -> State {
        if action.name.starts_with("navigate_") {
            Self::apply_navigate_action(state, action)
        } else if action.name.starts_with("recharge_") {
            Self::apply_recharge_action(state, action)
        } else if action.name.starts_with("sample_soil_") {
            Self::apply_sample_soil_action(state, action)
        } else if action.name.starts_with("sample_rock_") {
            Self::apply_sample_rock_action(state, action)
        } else if action.name.starts_with("drop_") {
            Self::apply_drop_action(state, action)
        } else if action.name.starts_with("calibrate_") {
            Self::apply_calibrate_action(state, action)
        } else if action.name.starts_with("take_image_") {
            Self::apply_take_image_action(state, action)
        } else if action.name.starts_with("communicate_soil_data_") {
            Self::apply_communicate_soil_data_action(state, action)
        } else if action.name.starts_with("communicate_rock_data_") {
            Self::apply_communicate_rock_data_action(state, action)
        } else if action.name.starts_with("communicate_image_data_") {
            Self::apply_communicate_image_data_action(state, action)
        } else {
            panic!("Unknown action type: {}", action.name)
        }
    }

    fn is_goal_state(&self, state: &State) -> bool {
        self.goal.is_goal_state(state)
    }

    fn heuristic(&self, _state: &State) -> f64 {
        0.0 // Simple heuristic - could be improved
    }

    fn load_state_from_json(json_path: &str) -> (State, RoverProblem) {
        Self::load_state_from_json(json_path)
    }
}
