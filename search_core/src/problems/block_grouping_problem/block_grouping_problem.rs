use crate::problems::problem::Problem;
use crate::search::{action::Action, state::StateTrait, state::Value};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub blocks: Vec<Block>,
}
impl State {}
impl StateTrait for State {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Block {
    pub index: i32,
    pub color_group: i32,
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Grid {
    pub max_x: i32,
    pub min_x: i32,
    pub max_y: i32,
    pub min_y: i32,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockGroupingProblem {
    pub grid: Grid,
}

impl BlockGroupingProblem {
    pub fn get_move_block_up_action(block: &Block) -> Action {
        let mut parameters = std::collections::HashMap::new();
        let action_name = format!("move_up_block{}", block.index);
        parameters.insert("block".to_string(), Value::Int(block.index));
        Action::new(action_name, 1, parameters)
    }

    pub fn get_move_block_down_action(block: &Block) -> Action {
        let mut parameters = std::collections::HashMap::new();
        let action_name = format!("move_down_block{}", block.index);
        parameters.insert("block".to_string(), Value::Int(block.index));
        Action::new(action_name, 1, parameters)
    }

    pub fn get_move_block_left_action(block: &Block) -> Action {
        let mut parameters = std::collections::HashMap::new();
        let action_name = format!("move_left_block{}", block.index);
        parameters.insert("block".to_string(), Value::Int(block.index));
        Action::new(action_name, 1, parameters)
    }

    pub fn get_move_block_right_action(block: &Block) -> Action {
        let mut parameters = std::collections::HashMap::new();
        let action_name = format!("move_right_block{}", block.index);
        parameters.insert("block".to_string(), Value::Int(block.index));
        Action::new(action_name, 1, parameters)
    }

    fn get_possible_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();

        for block in &state.blocks {
            if block.y + 1 <= self.grid.max_y {
                actions.push(Self::get_move_block_up_action(block))
            }

            if block.y - 1 >= self.grid.min_y {
                actions.push(Self::get_move_block_down_action(block))
            }

            if block.x + 1 <= self.grid.max_x {
                actions.push(Self::get_move_block_right_action(block))
            }

            if block.x - 1 >= self.grid.min_x {
                actions.push(Self::get_move_block_left_action(block))
            }
        }

        actions
    }

    pub fn apply_move_up_block_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let block_index = match action.parameters.get("block") {
            Some(Value::Int(index)) => *index,
            _ => panic!("Action parameters do not contain a valid index for block."),
        };
        if let Some(block) = new_state.blocks.iter_mut().find(|b| b.index == block_index) {
            block.y += 1;
        } else {
            panic!("Block with index {} not found", block_index);
        }
        new_state
    }
    pub fn apply_move_down_block_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let block_index = match action.parameters.get("block") {
            Some(Value::Int(index)) => *index,
            _ => panic!("Action parameters do not contain a valid index for block."),
        };
        if let Some(block) = new_state.blocks.iter_mut().find(|b| b.index == block_index) {
            block.y -= 1;
        } else {
            panic!("Block with index {} not found", block_index);
        }
        new_state
    }
    pub fn apply_move_right_block_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let block_index = match action.parameters.get("block") {
            Some(Value::Int(index)) => *index,
            _ => panic!("Action parameters do not contain a valid index for block."),
        };
        if let Some(block) = new_state.blocks.iter_mut().find(|b| b.index == block_index) {
            block.x += 1;
        } else {
            panic!("Block with index {} not found", block_index);
        }
        new_state
    }
    pub fn apply_move_left_block_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let block_index = match action.parameters.get("block") {
            Some(Value::Int(index)) => *index,
            _ => panic!("Action parameters do not contain a valid index for block."),
        };
        if let Some(block) = new_state.blocks.iter_mut().find(|b| b.index == block_index) {
            block.x -= 1;
        } else {
            panic!("Block with index {} not found", block_index);
        }
        new_state
    }
}

impl Problem for BlockGroupingProblem {
    type State = State;
    fn get_possible_actions(&self, state: &State) -> Vec<Action> {
        self.get_possible_actions(state)
    }

    fn apply_action(&self, state: &State, action: &Action) -> State {
        if action.name.starts_with("move_up_") {
            Self::apply_move_up_block_action(state, action)
        } else if action.name.starts_with("move_down_") {
            Self::apply_move_down_block_action(state, action)
        } else if action.name.starts_with("move_left_") {
            Self::apply_move_left_block_action(state, action)
        } else if action.name.starts_with("move_right_") {
            Self::apply_move_right_block_action(state, action)
        } else {
            panic!("Unknown action type: {}", action.name)
        }
    }
    fn is_goal_state(&self, state: &State) -> bool {
        for i in 0..state.blocks.len() - 1 {
            let current = &state.blocks[i];
            let next = &state.blocks[i + 1];
            if current.color_group == next.color_group {
                // Same group must be at the same position.
                if current.x != next.x || current.y != next.y {
                    return false;
                }
            } else {
                // Different groups must not be at the same position.
                if current.x == next.x && current.y == next.y {
                    return false;
                }
            }
        }
        true
    }

    fn heuristic(&self, state: &State) -> f64 {
        // heuristic is imported during build time from include!("refined_heuristic.in")
        //heuristic(self, state)
        0.0
    }
    fn load_state_from_json(json_path: &str) -> (State, BlockGroupingProblem) {
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
        let problem: BlockGroupingProblem =
            serde_json::from_value(problem_value.clone()).expect("Failed to deserialize problem");

        (state, problem)
    }
}
