use crate::problems::problem::Problem;
use crate::search::{action::Action, state::StateTrait, state::Value};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub current_time: i32,
    pub funds: i32, // current funds
    pub stored_units: i32, // how much water is stored now
    pub stored_capacity: i32, // how much capacity left
}

impl StateTrait for State {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HydropowerProblem {
    pub goal_funds: i32, // goal funds
    pub before: Vec<i32>,  // before[i] gives the next time after i
    pub demand: Vec<i32>,  // demand[i] gives the demand at time i
}

impl HydropowerProblem { 
    pub fn possible_advance_time_actions(&self, state: &State) -> Option<Action> {
        let current = state.current_time as usize;
    
        if current < self.before.len() { // if current time is earlier then the final time
            let next_time = self.before[current];
            let mut parameters = HashMap::new();
            parameters.insert("from".to_string(), Value::Text(state.current_time.to_string()));
            parameters.insert("to".to_string(), Value::Text(next_time.to_string()));
    
            Some(Action::new(
                format!("advance_time_{}_{}", state.current_time, next_time),
                1,
                parameters,
            ))
        } else {
            None
        }
    }
    

    pub fn possible_pump_water_actions(&self, state: &State) -> Option<Action> {
        let t = state.current_time as usize;
        if let Some(current_demand) = self.demand.get(t) {
            let mut parameters = HashMap::new();
            parameters.insert("time".to_string(), Value::Text(state.current_time.to_string()));
            parameters.insert("demand".to_string(), Value::Int(*current_demand));
            Some(Action::new(
                format!("pump_water_up_{}", state.current_time),
                1,
                parameters,
            ))
        } else {
            None
        }
    }

    pub fn possible_generate_actions(&self, state: &State) -> Option<Action> {
        let t = state.current_time as usize;
        if let Some(current_demand) = self.demand.get(t) {
            let mut parameters = HashMap::new();
            parameters.insert("time".to_string(), Value::Text(state.current_time.to_string()));
            parameters.insert("demand".to_string(), Value::Int(*current_demand));
            Some(Action::new(
                format!("generate_{}", state.current_time),
                1,
                parameters,
            ))
        } else {
            None
        }
    }

    pub fn apply_advance_time_action(state: &State, _action: &Action) -> State {
        let mut new_state = state.clone();
        new_state.current_time += 1;
        new_state
    }

    pub fn apply_pump_water_action(&self, state: &State, _action: &Action) -> State {
        let t = state.current_time as usize;
        let mut new_state = state.clone();
        let current_demand = self.demand.get(t).unwrap();
        new_state.stored_units += 1;
        new_state.stored_capacity -= 1;
        new_state.funds -= current_demand * 105 / 100;
        new_state
    }

    pub fn apply_generate_action(&self, state: &State, _action: &Action) -> State {
        let t = state.current_time as usize;
        let mut new_state = state.clone();
        let current_demand = self.demand.get(t).unwrap();
        new_state.stored_units -= 1;
        new_state.stored_capacity += 1;
        new_state.funds += current_demand;
        new_state
    }
}

impl Problem for HydropowerProblem {
    type State = State;

    fn get_possible_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();

        if let Some(action) = self.possible_advance_time_actions(state) {
            actions.push(action);
        }

        if state.stored_capacity > 0 {
            if let Some(action) = self.possible_pump_water_actions(state) {
                actions.push(action);
            }
        }

        if state.stored_units > 0 {
            if let Some(action) = self.possible_generate_actions(state) {
                actions.push(action);
            }
        }

        actions
    }

    fn apply_action(&self, state: &State, action: &Action) -> State {
        if action.name.starts_with("advance_time") {
            Self::apply_advance_time_action(state, action)
        } else if action.name.starts_with("pump_water_up") {
            self.apply_pump_water_action(state, action)
        } else if action.name.starts_with("generate") {
            self.apply_generate_action(state, action)
        } else {
            panic!("Unknown action");
        }
    }

    fn is_goal_state(&self, state: &State) -> bool {
        state.funds >= self.goal_funds
    }

    fn load_state_from_json(json_path: &str) -> (State, Self) {
        let json_str = std::fs::read_to_string(json_path).expect("Failed to read JSON file");
        let json_value: JsonValue = serde_json::from_str(&json_str).expect("Failed to parse JSON");
    
        let state_value = json_value.get("state").expect("Missing 'state' field in JSON");
        let problem_value = json_value.get("problem").expect("Missing 'problem' field in JSON");
    
        let state: State = serde_json::from_value(state_value.clone())
            .expect("Failed to deserialize State");
        let problem: HydropowerProblem = serde_json::from_value(problem_value.clone())
            .expect("Failed to deserialize HydropowerProblem");
    
        (state, problem)
    }

    fn heuristic(&self, _state: &State) -> f64 {
        0.0
    }
}
