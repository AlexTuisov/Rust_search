use crate::problems::problem::Problem;
use crate::search::{action::Action, state::StateTrait, state::Value};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub robots: Vec<Robot>,
    pub plants: Vec<Plant>,
    pub tap: Tap,
    pub total_poured: i32,
    pub total_loaded: i32,
}

impl State {}
impl StateTrait for State {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Robot {
    x: i32,
    y: i32,
    max_carry: i32,
    index: i32,
    carry: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Plant {
    x: i32,
    y: i32,
    index: i32,
    poured: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tap {
    x: i32,
    y: i32,
    water_amount: i32,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Goal {
    conditions: Vec<Condition>,
    total_operator: String,
}

impl Goal {
    pub fn is_goal_state(&self, state: &State) -> bool {
        // Check plant conditions
        for cond in &self.conditions {
            let plant_opt = state.plants.iter().find(|p| p.index == cond.plant_index);
            match plant_opt {
                Some(plant) => {
                    if plant.poured != cond.poured_amount {
                        return false;
                    }
                }
                None => return false,
            }
        }
        // Check total condition
        let total_ok = match self.total_operator.as_str() {
            "=" => state.total_poured == state.total_loaded,
            ">" => state.total_poured > state.total_loaded,
            "<" => state.total_poured < state.total_loaded,
            ">=" => state.total_poured >= state.total_loaded,
            "<=" => state.total_poured <= state.total_loaded,
            "!=" => state.total_poured != state.total_loaded,
            op => panic!("Unknown operator: {}", op),
        };
        total_ok
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Condition {
    plant_index: i32,
    poured_amount: i32,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtPlantWateringProblem {
    pub goal: Goal,
    pub max_x: i32,
    pub max_y: i32,
    pub min_x: i32,
    pub min_y: i32,
}

impl ExtPlantWateringProblem {
    pub fn get_robot_move_up_action(robot: &Robot) -> Action {
        let mut parameters = std::collections::HashMap::new();
        let action_name = format!("robot{}_move_up", robot.index);
        parameters.insert("move_up".to_string(), Value::Text("move_up".to_string()));
        parameters.insert("index".to_string(), Value::Int(robot.index));
        Action::new(action_name, 1, parameters)
    }
    pub fn get_robot_move_down_action(robot: &Robot) -> Action {
        let mut parameters = std::collections::HashMap::new();
        let action_name = format!("robot{}_move_down", robot.index);
        parameters.insert(
            "move_down".to_string(),
            Value::Text("move_down".to_string()),
        );
        parameters.insert("index".to_string(), Value::Int(robot.index));
        Action::new(action_name, 1, parameters)
    }
    pub fn get_robot_move_right_action(robot: &Robot) -> Action {
        let mut parameters = std::collections::HashMap::new();
        let action_name = format!("robot{}_move_right", robot.index);
        parameters.insert(
            "move_right".to_string(),
            Value::Text("move_right".to_string()),
        );
        parameters.insert("index".to_string(), Value::Int(robot.index));
        Action::new(action_name, 1, parameters)
    }
    pub fn get_robot_move_left_action(robot: &Robot) -> Action {
        let mut parameters = std::collections::HashMap::new();
        let action_name = format!("robot{}_move_left", robot.index);
        parameters.insert(
            "move_left".to_string(),
            Value::Text("move_left".to_string()),
        );
        parameters.insert("index".to_string(), Value::Int(robot.index));
        Action::new(action_name, 1, parameters)
    }
    pub fn get_robot_move_up_left_action(robot: &Robot) -> Action {
        let mut parameters = std::collections::HashMap::new();
        let action_name = format!("robot{}_move_up_left", robot.index);
        parameters.insert(
            "move_up_left".to_string(),
            Value::Text("move_up_left".to_string()),
        );
        parameters.insert("index".to_string(), Value::Int(robot.index));
        Action::new(action_name, 1, parameters)
    }
    pub fn get_robot_move_up_right_action(robot: &Robot) -> Action {
        let mut parameters = std::collections::HashMap::new();
        let action_name = format!("robot{}_move_up_right", robot.index);
        parameters.insert(
            "move_up_right".to_string(),
            Value::Text("move_up_right".to_string()),
        );
        parameters.insert("index".to_string(), Value::Int(robot.index));
        Action::new(action_name, 1, parameters)
    }
    pub fn get_robot_move_down_left_action(robot: &Robot) -> Action {
        let mut parameters = std::collections::HashMap::new();
        let action_name = format!("robot{}_move_down_left", robot.index);
        parameters.insert(
            "move_down_left".to_string(),
            Value::Text("move_down_left".to_string()),
        );
        parameters.insert("index".to_string(), Value::Int(robot.index));
        Action::new(action_name, 1, parameters)
    }
    pub fn get_robot_move_down_right_action(robot: &Robot) -> Action {
        let mut parameters = std::collections::HashMap::new();
        let action_name = format!("robot{}_move_down_right", robot.index);
        parameters.insert(
            "move_down_right".to_string(),
            Value::Text("move_down_right".to_string()),
        );
        parameters.insert("index".to_string(), Value::Int(robot.index));
        Action::new(action_name, 1, parameters)
    }
    pub fn get_robot_load_water_action(robot: &Robot) -> Action {
        let mut parameters = std::collections::HashMap::new();
        let action_name = format!("robot{}_loaded", robot.index);
        parameters.insert("load".to_string(), Value::Text("load".to_string()));
        parameters.insert("index".to_string(), Value::Int(robot.index));
        Action::new(action_name, 1, parameters)
    }
    pub fn get_robot_pour_water_action(robot: &Robot, plant: &Plant) -> Action {
        let mut parameters = std::collections::HashMap::new();
        let action_name = format!("robot{}_poured_water_to_plant{}", robot.index, plant.index);
        parameters.insert(
            "poured_water".to_string(),
            Value::Text("poured_water".to_string()),
        );
        parameters.insert("robot_index".to_string(), Value::Int(robot.index));
        parameters.insert("plant_index".to_string(), Value::Int(plant.index));
        Action::new(action_name, 1, parameters)
    }

    /// Generates all valid actions for all robots based on the state.
    /// Each block below documents the specific condition for that type of movement or action.
    /// This includes directional moves, water loading, and water pouring.
    pub fn get_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for robot in &state.robots {
            //Condition to move up
            if robot.y + 1 <= self.max_y {
                actions.push(Self::get_robot_move_up_action(robot));
            }
            //Condition to move down
            if robot.y - 1 >= self.min_y {
                actions.push(Self::get_robot_move_down_action(robot));
            }
            //Condition to move right
            if robot.x + 1 <= self.max_x {
                actions.push(Self::get_robot_move_right_action(robot));
            }
            //Condition to move left
            if robot.x - 1 >= self.min_x {
                actions.push(Self::get_robot_move_left_action(robot));
            }
            //Condition to move up left
            if robot.y + 1 <= self.max_y && robot.x - 1 >= self.min_x {
                actions.push(Self::get_robot_move_up_left_action(robot));
            }
            //Condition to move up right
            if robot.y + 1 <= self.max_y && robot.x + 1 <= self.max_x {
                actions.push(Self::get_robot_move_up_right_action(robot));
            }
            //Condition to move down left
            if robot.y - 1 >= self.min_y && robot.x - 1 >= self.min_x {
                actions.push(Self::get_robot_move_down_left_action(robot));
            }
            //Condition to move down right
            if robot.y - 1 >= self.min_y && robot.x + 1 <= self.max_x {
                actions.push(Self::get_robot_move_down_right_action(robot));
            }

            //Condition to load water
            if robot.y == state.tap.y && robot.x == state.tap.x {
                actions.push(Self::get_robot_load_water_action(robot));
            }

            for plant in &state.plants {
                //Condition to pour water
                if robot.y == plant.y && robot.x == plant.x && robot.carry > 0 {
                    actions.push(Self::get_robot_pour_water_action(robot, plant));
                }
            }
        }
        actions
    }
     /// Applies an action that moves the robot up by 1 unit.
    pub fn apply_move_up_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let robot_index = match action.parameters.get("index") {
            Some(Value::Int(index)) => *index,
            _ => panic!("Action parameters do not contain a valid index for robot."),
        };

        if let Some(robot) = new_state.robots.iter_mut().find(|v| v.index == robot_index) {
            robot.y += 1;
        } else {
            panic!("Robot with index {} not found", robot_index);
        }

        new_state
    }
    /// Applies an action that moves the robot down by 1 unit.
    pub fn apply_move_down_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let robot_index = match action.parameters.get("index") {
            Some(Value::Int(index)) => *index,
            _ => panic!("Action parameters do not contain a valid index for robot."),
        };

        if let Some(robot) = new_state.robots.iter_mut().find(|v| v.index == robot_index) {
            robot.y -= 1;
        } else {
            panic!("Robot with index {} not found", robot_index);
        }

        new_state
    }
    /// Applies an action that moves the robot left by 1 unit.
    pub fn apply_move_left_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let robot_index = match action.parameters.get("index") {
            Some(Value::Int(index)) => *index,
            _ => panic!("Action parameters do not contain a valid index for robot."),
        };

        if let Some(robot) = new_state.robots.iter_mut().find(|v| v.index == robot_index) {
            robot.x -= 1;
        } else {
            panic!("Robot with index {} not found", robot_index);
        }

        new_state
    }
    /// Applies an action that moves the robot right by 1 unit.
    pub fn apply_move_right_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let robot_index = match action.parameters.get("index") {
            Some(Value::Int(index)) => *index,
            _ => panic!("Action parameters do not contain a valid index for robot."),
        };

        if let Some(robot) = new_state.robots.iter_mut().find(|v| v.index == robot_index) {
            robot.x += 1;
        } else {
            panic!("Robot with index {} not found", robot_index);
        }

        new_state
    }

    /// Applies an action that moves the robot diagonally up and left.
    pub fn apply_move_up_left_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let robot_index = match action.parameters.get("index") {
            Some(Value::Int(index)) => *index,
            _ => panic!("Action parameters do not contain a valid index for robot."),
        };

        if let Some(robot) = new_state.robots.iter_mut().find(|v| v.index == robot_index) {
            robot.y += 1;
            robot.x -= 1;
        } else {
            panic!("Robot with index {} not found", robot_index);
        }

        new_state
    }
    /// Applies an action that moves the robot diagonally up and right.
    pub fn apply_move_up_right_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let robot_index = match action.parameters.get("index") {
            Some(Value::Int(index)) => *index,
            _ => panic!("Action parameters do not contain a valid index for robot."),
        };

        if let Some(robot) = new_state.robots.iter_mut().find(|v| v.index == robot_index) {
            robot.y += 1;
            robot.x += 1;
        } else {
            panic!("Robot with index {} not found", robot_index);
        }

        new_state
    }
    /// Applies an action that moves the robot diagonally down and left.
    pub fn apply_move_down_left_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let robot_index = match action.parameters.get("index") {
            Some(Value::Int(index)) => *index,
            _ => panic!("Action parameters do not contain a valid index for robot."),
        };

        if let Some(robot) = new_state.robots.iter_mut().find(|v| v.index == robot_index) {
            robot.y -= 1;
            robot.x -= 1;
        } else {
            panic!("Robot with index {} not found", robot_index);
        }

        new_state
    }
    /// Applies an action that moves the robot diagonally down and right.
    pub fn apply_move_down_right_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let robot_index = match action.parameters.get("index") {
            Some(Value::Int(index)) => *index,
            _ => panic!("Action parameters do not contain a valid index for robot."),
        };

        if let Some(robot) = new_state.robots.iter_mut().find(|v| v.index == robot_index) {
            robot.y -= 1;
            robot.x += 1;
        } else {
            panic!("Robot with index {} not found", robot_index);
        }

        new_state
    }
    /// Applies an action where the robot loads water from the tap.
    pub fn apply_load_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let robot_index = match action.parameters.get("index") {
            Some(Value::Int(index)) => *index,
            _ => panic!("Action parameters do not contain a valid index for robot."),
        };

        if let Some(robot) = new_state.robots.iter_mut().find(|v| v.index == robot_index) {
            new_state.tap.water_amount -= 1;
            new_state.total_loaded += 1;
            robot.carry += 1;
        } else {
            panic!("Robot with index {} not found", robot_index);
        }

        new_state
    }
    /// Applies an action where the robot pours water into a specific plant.
    pub fn apply_pour_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let robot_index = match action.parameters.get("robot_index") {
            Some(Value::Int(index)) => *index,
            _ => panic!("Action parameters do not contain a valid index for robot."),
        };

        let plant_index = match action.parameters.get("plant_index") {
            Some(Value::Int(index)) => *index,
            _ => panic!("Action parameters do not contain a valid index for plant."),
        };

        if let (Some(robot), Some(plant)) = (
            new_state.robots.iter_mut().find(|v| v.index == robot_index),
            new_state.plants.iter_mut().find(|v| v.index == plant_index),
        ) {
            robot.carry -= 1;
            plant.poured += 1;
            new_state.total_poured += 1;
        } else {
            panic!(
                "Robot with index {} or Plant with index {} not found",
                robot_index, plant_index
            );
        }

        new_state
    }
}

impl Problem for ExtPlantWateringProblem {
    type State = State;
     /// Returns all possible actions from the current state.
    fn get_possible_actions(&self, state: &State) -> Vec<Action> {
        self.get_actions(state)
    }
     /// Applies a named action to the state, dispatching to the correct handler.
    fn apply_action(&self, state: &State, action: &Action) -> State {
        if let Some((_, pure_action)) = action.name.split_once('_') {
            if pure_action.starts_with("poured_water_to_plant") {
                Self::apply_pour_action(state, action)
            } else {
                match pure_action {
                    "move_up" => Self::apply_move_up_action(state, action),
                    "move_down" => Self::apply_move_down_action(state, action),
                    "move_left" => Self::apply_move_left_action(state, action),
                    "move_right" => Self::apply_move_right_action(state, action),
                    "move_up_left" => Self::apply_move_up_left_action(state, action),
                    "move_up_right" => Self::apply_move_up_right_action(state, action),
                    "move_down_left" => Self::apply_move_down_left_action(state, action),
                    "move_down_right" => Self::apply_move_down_right_action(state, action),
                    "loaded" => Self::apply_load_action(state, action),
                    unknown => panic!("Unknown action: {}", unknown),
                }
            }
        } else {
            panic!(
                "Action name does not contain an underscore: {}",
                action.name
            );
        }
    }
     /// Checks if the current state satisfies the goal.
    fn is_goal_state(&self, state: &State) -> bool {
        self.goal.is_goal_state(state)
    }
    fn heuristic(&self, _state: &State) -> f64 {
        // heuristic is imported during build time from include!("refined_heuristic.in")
        //heuristic(self, state)
        0.0
    }

    fn load_state_from_json(json_path: &str) -> (State, ExtPlantWateringProblem) {
        // Read the JSON file into a string.
        let json_str = fs::read_to_string(json_path).expect("Failed to read JSON file");

        // Parse the JSON string into a serde_json::Value.
        let json_value: JsonValue = serde_json::from_str(&json_str).expect("Failed to parse JSON");

        // Extract the "state" and "problem" fields.
        let state_value = json_value
            .get("state")
            .expect("Missing 'state' field in JSON");
        let problem_value = json_value
            .get("problem")
            .expect("Missing 'problem' field in JSON");

        // Deserialize each part into the corresponding struct.
        let state: State =
            serde_json::from_value(state_value.clone()).expect("Failed to deserialize state");
        let problem: ExtPlantWateringProblem =
            serde_json::from_value(problem_value.clone()).expect("Failed to deserialize problem");

        (state, problem)
    }
}
