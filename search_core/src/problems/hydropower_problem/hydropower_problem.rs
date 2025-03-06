use crate::problems::problem::Problem;
use crate::search::{action::Action, state::StateTrait, state::Value};
use serde::{Deserialize, Serialize};
use serde_json::from_reader;
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub current_time: i32,
    pub funds: i32,
    pub stored_units: i32,
    pub stored_capacity: i32,
    pub demand: HashMap<i32, i32>,
    pub time_end: i32,
}

impl StateTrait for State {}

pub struct HydropowerProblem {
    goal_funds: i32,
}

impl HydropowerProblem {
    pub fn possible_advance_time_actions(state: &State) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert(
            "from".to_string(),
            Value::Text(state.current_time.to_string()),
        );
        parameters.insert(
            "to".to_string(),
            Value::Text((state.current_time + 1).to_string()),
        );
        Action::new(
            format!(
                "advance_time_{}_{}",
                state.current_time,
                state.current_time + 1
            ),
            1,
            parameters,
        )
    }

    pub fn possible_pump_water_actions(state: &State) -> Action {
        let current_demand = &state.demand.get(&state.current_time).unwrap();
        let mut parameters = HashMap::new();
        parameters.insert(
            "time".to_string(),
            Value::Text(state.current_time.to_string()),
        );
        parameters.insert("demand".to_string(), Value::Int(**current_demand));
        Action::new(
            format!("pump_water_up_{}", state.current_time),
            1,
            parameters,
        )
    }

    pub fn possible_generate_actions(state: &State) -> Action {
        let current_demand = &state.demand.get(&state.current_time).unwrap();
        let mut parameters = HashMap::new();
        parameters.insert(
            "time".to_string(),
            Value::Text(state.current_time.to_string()),
        );
        parameters.insert("demand".to_string(), Value::Int(**current_demand));
        Action::new(format!("generate_{}", state.current_time), 1, parameters)
    }

    pub fn apply_advance_time_action(state: &State, action: &Action) -> State {
        let mut new_state: State = state.clone();
        new_state.current_time += 1;
        new_state
    }

    pub fn apply_pump_water_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let current_demand = state.demand.get(&state.current_time).unwrap();
        new_state.stored_units += 1;
        new_state.stored_capacity -= 1;
        new_state.funds -= current_demand * 105 / 100;
        new_state
    }

    pub fn apply_generate_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let current_demand = state.demand.get(&state.current_time).unwrap();
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
        actions.push(Self::possible_advance_time_actions(state));
        if state.stored_capacity > 0 {
            actions.push(Self::possible_pump_water_actions(state));
        }
        if state.stored_units > 0 {
            actions.push(Self::possible_generate_actions(state));
        }
        actions
    }

    fn apply_action(&self, state: &State, action: &Action) -> State {
        if action.name.starts_with("advance_time") {
            Self::apply_advance_time_action(state, action)
        } else if action.name.starts_with("pump_water_up") {
            Self::apply_pump_water_action(state, action)
        } else if action.name.starts_with("generate") {
            Self::apply_generate_action(state, action)
        } else {
            panic!("Unknown action");
        }
    }

    fn is_goal_state(&self, state: &State) -> bool {
        state.funds >= self.goal_funds
    }

    fn load_state_from_json(json_path: &str) -> (State, Self) {
        let file = File::open(json_path).expect("Failed to open JSON file");
        let reader = BufReader::new(file);
        let json_data: JsonValue = from_reader(reader).expect("Failed to parse JSON");

        let state = State {
            current_time: 0,
            funds: json_data["funds"].as_i64().unwrap() as i32,
            stored_units: 0,
            stored_capacity: json_data["capacity"].as_i64().unwrap() as i32,
            demand: json_data["demands"]
                .as_object()
                .unwrap()
                .iter()
                .map(|(k, v)| (k.parse::<i32>().unwrap(), v.as_i64().unwrap() as i32))
                .collect(),
            time_end: json_data["time_end"].as_i64().unwrap() as i32,
        };

        let goal_funds = json_data["goal_funds"].as_i64().unwrap() as i32;
        (state, Self { goal_funds })
    }

    fn heuristic(&self, state: &State) -> f64 {
        // heuristic is imported during build time from include!("refined_heuristic.in")
        0.0
    }
}
