use crate::problems::problem::Problem;
use crate::search::{action::Action, state::StateTrait, state::Value};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::fs;

// Each State have airplanes, markets, and total cost
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    farms: Vec<Farm>,
    number_of_cars: i32,
    cost: i32,
}

impl State {}
impl StateTrait for State {}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Farm {
    name: String,
    value: i32,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubCondition {
    farm_name: String,
    farm_constant: OrderedFloat<f64>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Goal {
    farms: Vec<SubCondition>,
    operator: String,
    value: i32,
}

impl Goal {
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
    
        let adjusted_sum = sum - state.cost as f64;
    
        match self.operator.as_str() {
            ">=" => adjusted_sum >= self.value as f64,
            "<=" => adjusted_sum <= self.value as f64,
            ">"  => adjusted_sum >  self.value as f64,
            "<"  => adjusted_sum <  self.value as f64,
            "==" | "=" => (adjusted_sum - self.value as f64).abs() < 1e-6,
            _ => panic!("Unsupported operator in goal: {}", self.operator),
        }
    }
    
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FarmLandProblem {
    pub adj: HashMap<String, Vec<String>>, // (from,to) -> cost
    pub goal: Goal,
}

impl FarmLandProblem {
    pub fn get_move_slow_action(&self, farm: &Farm) -> Vec<Action> {
        let mut actions = Vec::new();
        if let Some(farms) = self.adj.get(&farm.name) {
            for farm_adj in farms {
                if (farm.name != *farm_adj) {
                    let mut parameters = std::collections::HashMap::new();
                    let action_name = format!("move_slow_{}_{}", farm.name, farm_adj);
                    parameters.insert("from".to_string(), Value::Text(farm.name.clone()));
                    parameters.insert("to".to_string(), Value::Text(farm_adj.clone()));
                    actions.push(Action::new(action_name, 1, parameters));
                }
            }
        }
        actions
    }

    pub fn get_hire_car_action() -> Action {
        let mut parameters = std::collections::HashMap::new();
        let action_name = format!("hire_car");
        Action::new(action_name, 1, parameters)
    }
    pub fn get_move_by_car_action(&self, farm: &Farm, number_of_cars: &i32) -> Vec<Action> {
        let mut actions = Vec::new();
        if let Some(farms) = self.adj.get(&farm.name) {
            for farm_adj in farms {
                if (farm.name != *farm_adj) {
                    let mut parameters = std::collections::HashMap::new();
                    let action_name = format!("move_by_car_{}_{}", farm.name, farm_adj);
                    parameters.insert("from".to_string(), Value::Text(farm.name.clone()));
                    parameters.insert("to".to_string(), Value::Text(farm_adj.clone()));
                    parameters.insert("cars".to_string(), Value::Int(*number_of_cars));
                    let cost = (*number_of_cars as f64 * 0.4).round() as i32;
                    actions.push(Action::new(action_name, cost, parameters));
                }
            }
        }
        actions
    }

    pub fn get_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        actions.push(Self::get_hire_car_action());
        for farm in &state.farms {
            if farm.value >= 1 {
                actions.extend(self.get_move_slow_action(farm));
            }
            if farm.value >= 4 * state.number_of_cars {
                actions.extend(self.get_move_by_car_action(farm, &state.number_of_cars));
            }
        }
        actions
    }

    pub fn apply_move_slow_action(state: &State, action: &Action) -> State {
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

        new_state.farms[from_index].value -= 1;
        new_state.farms[to_index].value += 1;

        new_state
    }

    pub fn apply_move_by_car_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let farm_from_name = match action.parameters.get("from") {
            Some(Value::Text(name)) => name,
            _ => panic!("Action parameters do not contain a valid name for farm."),
        };

        let farm_to_name = match action.parameters.get("to") {
            Some(Value::Text(name)) => name,
            _ => panic!("Action parameters do not contain a valid name for farm."),
        };
        let number_of_cars = match action.parameters.get("cars") {
            Some(Value::Int(number)) => *number,
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

        new_state.farms[from_index].value -= 4 * number_of_cars;
        new_state.farms[to_index].value += 4 * number_of_cars;
        new_state.cost += action.cost;
        new_state
    }

    pub fn apply_hire_car_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        new_state.number_of_cars += 1;

        new_state
    }
}

impl Problem for FarmLandProblem {
    type State = State;

    fn get_possible_actions(&self, state: &State) -> Vec<Action> {
        self.get_actions(state)
    }

    fn apply_action(&self, state: &State, action: &Action) -> State {
        if action.name.starts_with("move_slow_") {
            Self::apply_move_slow_action(state, action)
        } else if action.name.starts_with("move_by_car_") {
            Self::apply_move_by_car_action(state, action)
        } else if action.name.starts_with("hire_car") {
            Self::apply_hire_car_action(state, action)
        } else {
            panic!("Unknown action type: {}", action.name)
        }
    }

    fn is_goal_state(&self, state: &State) -> bool {
        self.goal.is_goal_state(state)
    }

    fn heuristic(&self, _state: &State) -> f64 {
        // heuristic is imported during build time from include!("refined_heuristic.in")
        //heuristic(self, state)
        0.0
    }

    fn load_state_from_json(json_path: &str) -> (State, FarmLandProblem) {
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
        let problem: FarmLandProblem =
            serde_json::from_value(problem_value.clone()).expect("Failed to deserialize problem");

        (state, problem)
    }
}
