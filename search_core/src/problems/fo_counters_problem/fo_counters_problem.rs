use crate::problems::problem::Problem;
use crate::search::{action::Action, state::StateTrait, state::Value};
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, Value as JsonValue};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub counters: Vec<Counter>,
    pub total_cost: i32,
}

impl State {}

impl StateTrait for State {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinearExpr {
    /// Each term is a pair: (coefficient, counter name).
    pub terms: Vec<(i32, String)>,
    /// A constant term to add.
    pub constant: i32,
}

impl LinearExpr {
    /// Evaluates the expression based on the current state.
    pub fn evaluate(&self, state: &State) -> i32 {
        let mut sum = self.constant;
        for (coeff, name) in &self.terms {
            // Assume that state.counters has a field `name`.
            if let Some(counter) = state.counters.iter().find(|c| c.name == *name) {
                sum += coeff * counter.value;
            } else {
                panic!("Counter {} not found in state", name);
            }
        }
        sum
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Condition {
    pub left: LinearExpr,
    pub operator: String, // e.g., "=", "<=", "<", ">", ">="
    pub right: LinearExpr,
}

impl Condition {
    /// Checks if this condition is satisfied given the state.
    pub fn is_satisfied(&self, state: &State) -> bool {
        let left_val = self.left.evaluate(state);
        let right_val = self.right.evaluate(state);
        match self.operator.as_str() {
            "=" => left_val == right_val,
            "<=" => left_val <= right_val,
            "<" => left_val < right_val,
            ">=" => left_val >= right_val,
            ">" => left_val > right_val,
            _ => panic!("Unknown operator: {}", self.operator),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Goal {
    pub conditions: Vec<Condition>,
}

impl Goal {
    /// Checks if all conditions in the goal are satisfied by the state.
    pub fn is_goal_state(&self, state: &State) -> bool {
        self.conditions.iter().all(|cond| cond.is_satisfied(state))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Counter {
    pub value: i32,
    pub rate_value:i32,
    pub name: String,
}

impl Counter {
    /// Creates a new Person with the given parameters.
    pub fn new(name: String, value: i32) -> Self {
        Counter { name, value }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FoCountersProblem {
    pub max_value: i32,
    pub goal: Goal,
}

impl FoCountersProblem  {
    pub fn get_increase_action(counter: &Counter) -> Action {
        let mut parameters = std::collections::HashMap::new();
        let action_name = format!("increase_counter{}", counter.name);
        parameters.insert("increase".to_string(), Value::Text("increase".to_string()));
        parameters.insert("counter".to_string(), Value::Text(counter.name.clone()));
        Action::new(action_name, 1, parameters)
    }
    pub fn get_decrease_action(counter: &Counter) -> Action {
        let mut parameters = std::collections::HashMap::new();
        let action_name = format!("decrease_counter{}", counter.name);
        parameters.insert("decrease".to_string(), Value::Text("decrease".to_string()));
        parameters.insert("counter".to_string(), Value::Text(counter.name.clone()));
        Action::new(action_name, 1, parameters)
    }


    pub fn get_increase_rate_action(counter: &Counter) -> Action {
        let mut parameters = std::collections::HashMap::new();
        let action_name = format!("rateI_counter{}", counter.name);
        parameters.insert("increase_rate".to_string(), Value::Text("increase_rate".to_string()));
        parameters.insert("counter".to_string(), Value::Text(counter.name.clone()));
        Action::new(action_name, 1, parameters)
    }


    pub fn get_decrease_rate_action(counter: &Counter) -> Action {
        let mut parameters = std::collections::HashMap::new();
        let action_name = format!("rateD_counter{}", counter.name);
        parameters.insert("decrease_rate".to_string(), Value::Text("decrease_rate".to_string()));
        parameters.insert("counter".to_string(), Value::Text(counter.name.clone()));
        Action::new(action_name, 1, parameters)
    }

    pub fn get_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for counter in &state.counters {
            if counter.value + counter.rate_value <= self.max_value {
                actions.push(Self::get_increase_action(counter));
            }
            if counter.value - counter.rate_value >= 0 {
                actions.push(Self::get_decrease_action(counter));
            }
            if counter.rate_value +1 <= 10
            {
                actions.push(Self::get_increase_rate_action(counter));
            }
            if counter.rate_value >= 1
            {
                actions.push(Self::get_decrement_rate_action(counter));
            }
        }
        actions
    }

    pub fn apply_increase_action(state: &State, action: &Action) -> State {
        // Start by cloning the current state.
        let mut new_state = state.clone();
        let counter_name = match action.parameters.get("counter") {
            Some(Value::Text(name)) => name,
            _ => panic!("Action parameters do not contain a valid name for counter."),
        };

        if let Some(counter) = new_state
            .counters
            .iter_mut()
            .find(|v| v.name == *counter_name)
        {
            let r_val = counter.rate_value;
            counter.value += r_val;
            new_state.total_cost +=1;
        } else {
            panic!("Counter with name {} not found", counter_name);
        }

        new_state
    }

    pub fn apply_decrease_action(state: &State, action: &Action) -> State {
        // Start by cloning the current state.
        let mut new_state = state.clone();
        let counter_name = match action.parameters.get("counter") {
            Some(Value::Text(name)) => name,
            _ => panic!("Action parameters do not contain a valid name for counter."),
        };

        if let Some(counter) = new_state
            .counters
            .iter_mut()
            .find(|v| v.name == *counter_name)
        {
            let r_val = counter.rate_value;
            counter.value -= r_val;
            new_state.total_cost += 1;
        } else {
            panic!("Counter with name {} not found", counter_name);
        }

        new_state
    }

    pub fn apply_decrease_rate_action(state: &State, action: &Action) -> State {
        // Start by cloning the current state.
        let mut new_state = state.clone();
        let counter_name = match action.parameters.get("counter") {
            Some(Value::Text(name)) => name,
            _ => panic!("Action parameters do not contain a valid name for counter."),
        };

        if let Some(counter) = new_state
            .counters
            .iter_mut()
            .find(|v| v.name == *counter_name)
        {

            counter.rate_value -= 1;
            new_state.total_cost += 1;
        } else {
            panic!("Counter with name {} not found", counter_name);
        }

        new_state
    }
    pub fn apply_incrase_rate_action(state: &State, action: &Action) -> State {
        // Start by cloning the current state.
        let mut new_state = state.clone();
        let counter_name = match action.parameters.get("counter") {
            Some(Value::Text(name)) => name,
            _ => panic!("Action parameters do not contain a valid name for counter."),
        };

        if let Some(counter) = new_state
            .counters
            .iter_mut()
            .find(|v| v.name == *counter_name)
        {

            counter.rate_value -= 1;
            new_state.total_cost += 1;
        } else {
            panic!("Counter with name {} not found", counter_name);
        }

        new_state
    }
}

impl Problem for FoCountersProblem  {
    type State = State;

    fn get_possible_actions(&self, state: &State) -> Vec<Action> {
        self.get_actions(state)
    }

    fn apply_action(&self, state: &State, action: &Action) -> State {
        if action.name.starts_with("increase_") {
            Self::apply_increase_action(state, action)
        } else if action.name.starts_with("decrease_") {
            Self::apply_decrease_action(state, action)
        } else if action.name.starts_with("rateI_") {
            Self::apply_increace_rate_action(state, action)
        }else if action.name.starts_with("rateD_") {
            Self::apply_decrease_rate_action(state, action)
        }else {
            
            panic!("Unknown action type: {}", action.name)
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

    fn load_state_from_json(json_path: &str) -> (State, Self) {
        let json_str = fs::read_to_string(json_path).expect("Failed to read JSON file");

        let json_value: serde_json::Value =
            serde_json::from_str(&json_str).expect("Failed to parse JSON");

        // Parse the "Counters" object.
        let counters_map = json_value.get("Counters").expect("Counters key missing");
        let mut counters = Vec::new();
        if let Some(obj) = counters_map.as_object() {
            for (key, value) in obj.iter() {
                let val = value
                    .as_i64()
                    .expect("Expected an integer for counter value")
                    as i32;
                // Here we name the counter as "c{key}".
                counters.push(Counter::new(format!("c{}", key), val));
            }
        } else {
        }

        let state = State { counters };

        // Parse the "Goal" section.
        let goal_map = json_value.get("Goal").expect("Goal key missing");
        let mut conditions = Vec::new();
        if let Some(obj) = goal_map.as_object() {
            for (key, value) in obj.iter() {
                let cond_str = value
                    .as_str()
                    .expect("Expected goal condition to be a string");
                // Expect condition string in the format "c0+1 <= c1"
                let parts: Vec<&str> = cond_str.split_whitespace().collect();
                if parts.len() != 3 {
                    panic!("Invalid condition format: {}", cond_str);
                }
                let left_expr_str = parts[0]; // e.g., "c0+1"
                let operator = parts[1]; // e.g., "<="
                let right_expr_str = parts[2]; // e.g., "c1"

                // Parse the left-hand side: split on '+'
                let left_parts: Vec<&str> = left_expr_str.split('+').collect();
                if left_parts.len() != 2 {
                    panic!("Invalid left expression format: {}", left_expr_str);
                }
                let left_counter = left_parts[0].to_string(); // "c0"
                let left_offset = left_parts[1]
                    .parse::<i32>()
                    .expect("Invalid offset in left expression");

                let left_expr = LinearExpr {
                    terms: vec![(1, left_counter)],
                    constant: left_offset,
                };

                let right_expr = LinearExpr {
                    terms: vec![(1, right_expr_str.to_string())],
                    constant: 0,
                };

                let condition = Condition {
                    left: left_expr,
                    operator: operator.to_string(),
                    right: right_expr,
                };

                conditions.push(condition);
            }
        }
        let goal = Goal { conditions };

        // Parse max_value if present, default to 48 otherwise.
        let max_value = json_value
            .get("max_value")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32)
            .unwrap_or(48);

        let problem = FoCountersProblem  { max_value, goal };

        (state, problem)
    }
}
