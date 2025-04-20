use crate::problems::problem::Problem;
use crate::search::{action::Action, state::StateTrait, state::Value};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::fs;

/// Represents the current world state with all farm values.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    farms: Vec<Farm>,
}

impl State {}
impl StateTrait for State {}

/// A farm with a name and a numeric value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Farm {
    name: String,
    value: i32,
}

/// A single weighted component in the goal condition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubCondition {
    farm_name: String,
    farm_constant: OrderedFloat<f64>,
}

/// Represents the numeric goal condition over farms.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Goal {
    farms: Vec<SubCondition>,
    operator: String,
    value: i32,
}

impl Goal {
    /// Checks whether the given state satisfies the goal.
    pub fn is_goal_state(&self, state: &State) -> bool {
        let mut sum = 0.0;

        for cond in &self.farms {
            let farm = state.farms.iter().find(|f| f.name == cond.farm_name);
            if let Some(f) = farm {
                sum += cond.farm_constant.into_inner() * f.value as f64;
            } else {
                panic!("Farm {} not found in state", cond.farm_name);
            }
        }

        // Compare total sum to the goal threshold using the specified operator.
        match self.operator.as_str() {
            ">=" => sum >= self.value as f64,
            "<=" => sum <= self.value as f64,
            ">" => sum > self.value as f64,
            "<" => sum < self.value as f64,
            "==" | "=" => (sum - self.value as f64).abs() < 1e-6,
            _ => panic!("Unsupported operator in goal: {}", self.operator),
        }
    }
}

/// Represents the problem definition, including the adjacency map and goal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FarmLandProblem {
    pub adj: HashMap<String, Vec<String>>, // adjacency map: farm name -> list of neighbors
    pub goal: Goal,
}

impl FarmLandProblem {
    /// Generates all legal `move_fast` actions from the given farm.
    /// Requires `farm.value >= 4` to be used (enforced externally).
    pub fn get_move_fast_action(&self, farm: &Farm) -> Vec<Action> {
        let mut actions = Vec::new();
        if let Some(farms) = self.adj.get(&farm.name) {
            for farm_adj in farms {
                if farm.name != *farm_adj {
                    let mut parameters = HashMap::new();
                    let action_name = format!("move_fast_{}_{}", farm.name, farm_adj);
                    parameters.insert("from".to_string(), Value::Text(farm.name.clone()));
                    parameters.insert("to".to_string(), Value::Text(farm_adj.clone()));
                    actions.push(Action::new(action_name, 1, parameters));
                }
            }
        }
        actions
    }

    /// Generates all legal `move_slow` actions from the given farm.
    /// Requires `farm.value >= 1` to be used (enforced externally).
    pub fn get_move_slow_action(&self, farm: &Farm) -> Vec<Action> {
        let mut actions = Vec::new();
        if let Some(farms) = self.adj.get(&farm.name) {
            for farm_adj in farms {
                if farm.name != *farm_adj {
                    let mut parameters = HashMap::new();
                    let action_name = format!("move_slow_{}_{}", farm.name, farm_adj);
                    parameters.insert("from".to_string(), Value::Text(farm.name.clone()));
                    parameters.insert("to".to_string(), Value::Text(farm_adj.clone()));
                    actions.push(Action::new(action_name, 1, parameters));
                }
            }
        }
        actions
    }

    /// Generates all applicable actions for the given state based on farm values.
    pub fn get_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for farm in &state.farms {
            if farm.value >= 1 {
                actions.extend(self.get_move_slow_action(farm));
            }
            if farm.value >= 4 {
                actions.extend(self.get_move_fast_action(farm));
            }
        }
        actions
    }

    /// Applies a `move_slow` action: transfers 1 unit from `from` to `to`.
    pub fn apply_move_slow_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();

        // Extract farm names from action parameters.
        let farm_from_name = match action.parameters.get("from") {
            Some(Value::Text(name)) => name,
            _ => panic!("Action parameters do not contain a valid name for farm."),
        };
        let farm_to_name = match action.parameters.get("to") {
            Some(Value::Text(name)) => name,
            _ => panic!("Action parameters do not contain a valid name for farm."),
        };

        // Find the indices of the farms.
        let from_index = new_state
            .farms
            .iter()
            .position(|v| v.name == *farm_from_name)
            .expect(&format!("Farm with name {} not found", farm_from_name));

        let to_index = new_state
            .farms
            .iter()
            .position(|v| v.name == *farm_to_name)
            .expect(&format!("Farm with name {} not found", farm_to_name));

        // Apply value transfer.
        new_state.farms[from_index].value -= 1;
        new_state.farms[to_index].value += 1;

        new_state
    }

    /// Applies a `move_fast` action: transfers 4 units from `from` and adds 2 to `to`.
    pub fn apply_move_fast_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();

        let farm_from_name = match action.parameters.get("from") {
            Some(Value::Text(name)) => name,
            _ => panic!("Action parameters do not contain a valid name for farm."),
        };
        let farm_to_name = match action.parameters.get("to") {
            Some(Value::Text(name)) => name,
            _ => panic!("Action parameters do not contain a valid name for farm."),
        };

        let from_index = new_state
            .farms
            .iter()
            .position(|v| v.name == *farm_from_name)
            .expect(&format!("Farm with name {} not found", farm_from_name));

        let to_index = new_state
            .farms
            .iter()
            .position(|v| v.name == *farm_to_name)
            .expect(&format!("Farm with name {} not found", farm_to_name));

        // Apply value transfer with fast penalty/reward ratio.
        new_state.farms[from_index].value -= 4;
        new_state.farms[to_index].value += 2;

        new_state
    }
}

impl Problem for FarmLandProblem {
    type State = State;

    /// Return all valid actions for a state.
    fn get_possible_actions(&self, state: &State) -> Vec<Action> {
        self.get_actions(state)
    }

    /// Apply the given action to the state.
    fn apply_action(&self, state: &State, action: &Action) -> State {
        if action.name.starts_with("move_slow_") {
            Self::apply_move_slow_action(state, action)
        } else if action.name.starts_with("move_fast_") {
            Self::apply_move_fast_action(state, action)
        } else {
            panic!("Unknown action type: {}", action.name)
        }
    }

    /// Check whether the state satisfies the goal.
    fn is_goal_state(&self, state: &State) -> bool {
        self.goal.is_goal_state(state)
    }

    /// Returns a heuristic estimate for the given state (currently zero).
    fn heuristic(&self, _state: &State) -> f64 {
        0.0
    }

    /// Loads a problem and state from a JSON file.
    fn load_state_from_json(json_path: &str) -> (State, FarmLandProblem) {
        let json_str = fs::read_to_string(json_path).expect("Failed to read JSON file");

        let json_value: JsonValue = serde_json::from_str(&json_str).expect("Failed to parse JSON");

        let state_value = json_value
            .get("state")
            .expect("Missing 'state' field in JSON");
        let problem_value = json_value
            .get("problem")
            .expect("Missing 'problem' field in JSON");

        let state: State =
            serde_json::from_value(state_value.clone()).expect("Failed to deserialize state");
        let problem: FarmLandProblem =
            serde_json::from_value(problem_value.clone()).expect("Failed to deserialize problem");

        (state, problem)
    }
}
