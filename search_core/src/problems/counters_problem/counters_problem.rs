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
    pub name: String,
}

impl Counter {
    /// Creates a new Person with the given parameters.
    pub fn new(name: String, value: i32) -> Self {
        Counter { name, value }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CountersProblem {
    pub max_value: i32,
    pub goal: Goal,
}

impl CountersProblem {
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

    pub fn get_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for counter in &state.counters {
            if counter.value + 1 <= self.max_value {
                actions.push(Self::get_increase_action(counter));
            }
            if counter.value - 1 >= 1 {
                actions.push(Self::get_decrease_action(counter));
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
            counter.value += 1;
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
            counter.value -= 1;
        } else {
            panic!("Counter with name {} not found", counter_name);
        }

        new_state
    }
}

impl Problem for CountersProblem {
    type State = State;

    fn get_possible_actions(&self, state: &State) -> Vec<Action> {
        self.get_actions(state)
    }

    fn apply_action(&self, state: &State, action: &Action) -> State {
        if action.name.starts_with("increase_") {
            Self::apply_increase_action(state, action)
        } else if action.name.starts_with("decrease_") {
            Self::apply_decrease_action(state, action)
        } else {
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

                // Remove extra spaces and ensure a proper split.
                let cond_clean = cond_str
                    .replace("(", "")
                    .replace(")", "")
                    .trim()
                    .to_string();
                let parts: Vec<&str> = cond_clean.split_whitespace().collect();

                if parts.len() != 5 {
                    panic!(
                        "Invalid condition format (expected 5 tokens): {}",
                        cond_clean
                    );
                }

                let left_counter = parts[0].to_string(); // e.g., "c0"
                let left_op = parts[1]; // e.g., "+"
                let left_offset: i32 = parts[2].parse().expect("Invalid offset in left expression");
                let operator = parts[3].to_string(); // e.g., "<="
                let right_counter = parts[4].to_string(); // e.g., "c1"

                let left_expr = match left_op {
                    "+" => {
                        if left_offset == 0 {
                            LinearExpr {
                                terms: vec![(1, left_counter.clone())],
                                constant: 0,
                            }
                        } else {
                            LinearExpr {
                                terms: vec![(1, left_counter.clone())],
                                constant: left_offset,
                            }
                        }
                    }
                    "-" => LinearExpr {
                        terms: vec![(1, left_counter.clone())],
                        constant: -left_offset,
                    },
                    _ => panic!("Invalid operator in left expression: {}", left_op),
                };

                let right_expr = LinearExpr {
                    terms: vec![(1, right_counter)],
                    constant: 0,
                };

                let condition = Condition {
                    left: left_expr,
                    operator,
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

        let problem = CountersProblem { max_value, goal };

        (state, problem)
    }
}
