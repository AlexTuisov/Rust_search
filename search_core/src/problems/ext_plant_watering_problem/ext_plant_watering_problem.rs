use crate::problems::problem::Problem;
use crate::search::{action::Action, state::Position, state::StateTrait, state::Value};
use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, Value as JsonValue};
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::BufReader;

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Robot {
    location: Position,
    max_carry: i32,
    index: i32,
    carry: i32,
}

// Define a helper struct matching the JSON structure
#[derive(Deserialize)]
struct RobotHelper {
    index: i32,
    x: i32,
    y: i32,
    max_carry: i32,
    carry: i32,
}

// Implement custom deserialization for Robot, combining x and y into location.
impl<'de> Deserialize<'de> for Robot {
    fn deserialize<D>(deserializer: D) -> Result<Robot, D::Error>
    where
        D: Deserializer<'de>,
    {
        let helper = RobotHelper::deserialize(deserializer)?;
        Ok(Robot {
            index: helper.index,
            max_carry: helper.max_carry,
            carry: helper.carry,
            location: Position::new(helper.x, helper.y),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Plant {
    location: Position,
    index: i32,
    poured: i32,
}

// Intermediate helper to match the JSON structure
#[derive(Deserialize)]
struct PlantHelper {
    index: i32,
    x: i32,
    y: i32,
    poured: i32,
}

impl<'de> Deserialize<'de> for Plant {
    fn deserialize<D>(deserializer: D) -> Result<Plant, D::Error>
    where
        D: Deserializer<'de>,
    {
        let helper = PlantHelper::deserialize(deserializer)?;
        Ok(Plant {
            location: Position::new(helper.x, helper.y),
            index: helper.index,
            poured: helper.poured,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Tap {
    location: Position,
    water_amount: i32,
}

#[derive(Deserialize)]
struct TapHelper {
    x: i32,
    y: i32,
    water_amount: i32,
}

impl<'de> Deserialize<'de> for Tap {
    fn deserialize<D>(deserializer: D) -> Result<Tap, D::Error>
    where
        D: Deserializer<'de>,
    {
        let helper = TapHelper::deserialize(deserializer)?;
        Ok(Tap {
            location: Position::new(helper.x, helper.y),
            water_amount: helper.water_amount,
        })
    }
}
pub struct Goal {
    conditions: Vec<Condition>,
    total_operator: String,
}

impl<'de> Deserialize<'de> for Goal {
    fn deserialize<D>(deserializer: D) -> Result<Goal, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GoalVisitor;

        impl<'de> Visitor<'de> for GoalVisitor {
            type Value = Goal;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a goal with plant_goals and other_goals")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Goal, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut plant_goals: Option<BTreeMap<String, i32>> = None;
                let mut other_goals: Option<Vec<String>> = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "plant_goals" => {
                            let pg: BTreeMap<String, i32> = map.next_value()?;
                            plant_goals = Some(pg);
                        }
                        "other_goals" => {
                            let og: Vec<String> = map.next_value()?;
                            other_goals = Some(og);
                        }
                        _ => {
                            let _: de::IgnoredAny = map.next_value()?;
                        }
                    }
                }

                let plant_goals =
                    plant_goals.ok_or_else(|| de::Error::missing_field("plant_goals"))?;
                let other_goals =
                    other_goals.ok_or_else(|| de::Error::missing_field("other_goals"))?;

                // Convert plant_goals (map) to Vec<Condition>.
                let mut conditions = Vec::new();
                for (key, value) in plant_goals.into_iter() {
                    // The key is something like "plant1"; strip "plant" and parse the number.
                    let plant_index = key
                        .trim_start_matches("plant")
                        .parse::<i32>()
                        .map_err(|_| de::Error::custom(format!("Invalid plant key: {}", key)))?;
                    conditions.push(Condition {
                        plant_index,
                        poured_amount: value,
                    });
                }

                // Assume we have at least one entry in other_goals.
                // For simplicity, extract the operator from the first string.
                // For example, if the string is "(= (total_poured) (total_loaded))", we take the character at position 1.
                let total_operator = other_goals
                    .get(0)
                    .and_then(|s| s.chars().nth(1))
                    .map(|op| op.to_string())
                    .ok_or_else(|| de::Error::custom("Failed to extract total operator"))?;

                Ok(Goal {
                    conditions,
                    total_operator,
                })
            }
        }

        deserializer.deserialize_map(GoalVisitor)
    }
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

pub struct Condition {
    plant_index: i32,
    poured_amount: i32,
}

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

    pub fn get_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for robot in &state.robots {
            ///Condition to move up
            if robot.location.y + 1 <= self.max_y {
                actions.push(Self::get_robot_move_up_action(robot));
            }
            ///Condition to move down
            if robot.location.y - 1 >= self.min_y {
                actions.push(Self::get_robot_move_down_action(robot));
            }
            ///Condition to move right
            if robot.location.x + 1 <= self.max_x {
                actions.push(Self::get_robot_move_right_action(robot));
            }
            ///Condition to move left
            if robot.location.x - 1 >= self.min_x {
                actions.push(Self::get_robot_move_left_action(robot));
            }
            ///Condition to move up left
            if robot.location.y + 1 <= self.max_y && robot.location.x - 1 >= self.min_x {
                actions.push(Self::get_robot_move_up_left_action(robot));
            }
            ///Condition to move up right
            if robot.location.y + 1 <= self.max_y && robot.location.x + 1 <= self.max_x {
                actions.push(Self::get_robot_move_up_right_action(robot));
            }
            ///Condition to move down left
            if robot.location.y - 1 >= self.min_y && robot.location.x - 1 >= self.min_x {
                actions.push(Self::get_robot_move_down_left_action(robot));
            }
            ///Condition to move down right
            if robot.location.y - 1 >= self.min_y && robot.location.x + 1 <= self.max_x {
                actions.push(Self::get_robot_move_down_right_action(robot));
            }

            //Condition to load water
            if robot.location.y == state.tap.location.y && robot.location.x == state.tap.location.x
            {
                actions.push(Self::get_robot_load_water_action(robot));
            }

            for plant in &state.plants {
                ///Condition to pour water
                if robot.location.y == plant.location.y && robot.location.x == plant.location.x {
                    actions.push(Self::get_robot_pour_water_action(robot, plant));
                }
            }
        }
        actions
    }

    pub fn apply_move_up_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let robot_index = match action.parameters.get("index") {
            Some(Value::Int(index)) => *index,
            _ => panic!("Action parameters do not contain a valid index for robot."),
        };

        if let Some(robot) = new_state.robots.iter_mut().find(|v| v.index == robot_index) {
            robot.location.y += 1;
        } else {
            panic!("Robot with index {} not found", robot_index);
        }

        new_state
    }
    pub fn apply_move_down_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let robot_index = match action.parameters.get("index") {
            Some(Value::Int(index)) => *index,
            _ => panic!("Action parameters do not contain a valid index for robot."),
        };

        if let Some(robot) = new_state.robots.iter_mut().find(|v| v.index == robot_index) {
            robot.location.y -= 1;
        } else {
            panic!("Robot with index {} not found", robot_index);
        }

        new_state
    }
    pub fn apply_move_left_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let robot_index = match action.parameters.get("index") {
            Some(Value::Int(index)) => *index,
            _ => panic!("Action parameters do not contain a valid index for robot."),
        };

        if let Some(robot) = new_state.robots.iter_mut().find(|v| v.index == robot_index) {
            robot.location.x -= 1;
        } else {
            panic!("Robot with index {} not found", robot_index);
        }

        new_state
    }
    pub fn apply_move_right_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let robot_index = match action.parameters.get("index") {
            Some(Value::Int(index)) => *index,
            _ => panic!("Action parameters do not contain a valid index for robot."),
        };

        if let Some(robot) = new_state.robots.iter_mut().find(|v| v.index == robot_index) {
            robot.location.x += 1;
        } else {
            panic!("Robot with index {} not found", robot_index);
        }

        new_state
    }
    pub fn apply_move_up_left_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let robot_index = match action.parameters.get("index") {
            Some(Value::Int(index)) => *index,
            _ => panic!("Action parameters do not contain a valid index for robot."),
        };

        if let Some(robot) = new_state.robots.iter_mut().find(|v| v.index == robot_index) {
            robot.location.y += 1;
            robot.location.x -= 1;
        } else {
            panic!("Robot with index {} not found", robot_index);
        }

        new_state
    }
    pub fn apply_move_up_right_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let robot_index = match action.parameters.get("index") {
            Some(Value::Int(index)) => *index,
            _ => panic!("Action parameters do not contain a valid index for robot."),
        };

        if let Some(robot) = new_state.robots.iter_mut().find(|v| v.index == robot_index) {
            robot.location.y += 1;
            robot.location.x += 1;
        } else {
            panic!("Robot with index {} not found", robot_index);
        }

        new_state
    }
    pub fn apply_move_down_left_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let robot_index = match action.parameters.get("index") {
            Some(Value::Int(index)) => *index,
            _ => panic!("Action parameters do not contain a valid index for robot."),
        };

        if let Some(robot) = new_state.robots.iter_mut().find(|v| v.index == robot_index) {
            robot.location.y -= 1;
            robot.location.x -= 1;
        } else {
            panic!("Robot with index {} not found", robot_index);
        }

        new_state
    }
    pub fn apply_move_down_right_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let robot_index = match action.parameters.get("index") {
            Some(Value::Int(index)) => *index,
            _ => panic!("Action parameters do not contain a valid index for robot."),
        };

        if let Some(robot) = new_state.robots.iter_mut().find(|v| v.index == robot_index) {
            robot.location.y -= 1;
            robot.location.x += 1;
        } else {
            panic!("Robot with index {} not found", robot_index);
        }

        new_state
    }
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
    fn get_possible_actions(&self, state: &State) -> Vec<Action> {
        self.get_actions(state)
    }
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

    fn is_goal_state(&self, state: &State) -> bool {
        self.goal.is_goal_state(state)
    }
    fn heuristic(&self, state: &State) -> f64 {
        // heuristic is imported during build time from include!("refined_heuristic.in")
        //heuristic(self, state)
        0.0
    }

    fn load_state_from_json(json_path: &str) -> (State, ExtPlantWateringProblem) {
        // Read the JSON file into a string.
        let json_str = fs::read_to_string(json_path).expect("Failed to read JSON file");

        // Parse the JSON string into a serde_json::Value.
        let json_value: JsonValue = serde_json::from_str(&json_str).expect("Failed to parse JSON");

        // ----------------------------
        // Deserialize the State part.
        // ----------------------------

        // Deserialize the tap. (Custom deserialization will combine x and y into a Position.)
        let tap: Tap =
            serde_json::from_value(json_value.get("tap").expect("No 'tap' field found").clone())
                .expect("Failed to deserialize tap");

        // Deserialize robots.
        let mut robots = Vec::new();
        if let Some(robots_map) = json_value.get("robots").and_then(|v| v.as_object()) {
            for (_key, robot_value) in robots_map.iter() {
                let robot: Robot = serde_json::from_value(robot_value.clone())
                    .expect("Failed to deserialize a robot");
                robots.push(robot);
            }
        } else {
            panic!("No 'robots' field found in JSON");
        }

        // Deserialize plants.
        let mut plants = Vec::new();
        if let Some(plants_map) = json_value.get("plants").and_then(|v| v.as_object()) {
            for (_key, plant_value) in plants_map.iter() {
                let plant: Plant = serde_json::from_value(plant_value.clone())
                    .expect("Failed to deserialize a plant");
                plants.push(plant);
            }
        } else {
            panic!("No 'plants' field found in JSON");
        }

        // Construct the state.
        let state = State {
            robots,
            plants,
            tap,
            total_poured: 0, // starting at 0
            total_loaded: 0, // starting at 0
        };

        // ----------------------------
        // Deserialize the Problem part.
        // ----------------------------

        // Grid boundaries.
        let max_x = json_value
            .get("max_x")
            .and_then(|v| v.as_i64())
            .unwrap_or(0) as i32;
        let max_y = json_value
            .get("max_y")
            .and_then(|v| v.as_i64())
            .unwrap_or(0) as i32;
        let min_x = json_value
            .get("min_x")
            .and_then(|v| v.as_i64())
            .unwrap_or(0) as i32;
        let min_y = json_value
            .get("min_y")
            .and_then(|v| v.as_i64())
            .unwrap_or(0) as i32;

        // Deserialize the goal.
        let goal: Goal = serde_json::from_value(
            json_value
                .get("goal")
                .expect("No 'goal' field found")
                .clone(),
        )
        .expect("Failed to deserialize goal");

        let problem = ExtPlantWateringProblem {
            goal,
            max_x,
            max_y,
            min_x,
            min_y,
        };

        (state, problem)
    }
}
