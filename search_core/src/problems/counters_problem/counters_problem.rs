use crate::problems::problem::Problem;
use crate::search::{action::Action, state::StateTrait, state::Value};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub counters: Vec<Counter>, // List of all counters and their current values
}

impl State {}
impl StateTrait for State {}

/// Represents a linear expression of the form:
///     c₁·x₁ + c₂·x₂ + ... + cₙ·xₙ + constant
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinearExpr {
    pub terms: Vec<(i32, String)>, // Each term: (coefficient, counter name)
    pub constant: i32,             // Constant added at the end
}

impl LinearExpr {
    /// Evaluates the expression using the values from the given state.
    pub fn evaluate(&self, state: &State) -> i32 {
        let mut sum = self.constant;
        for (coeff, name) in &self.terms {
            if let Some(counter) = state.counters.iter().find(|c| c.name == *name) {
                sum += coeff * counter.value;
            } else {
                panic!("Counter {} not found in state", name);
            }
        }
        sum
    }
}

/// Represents a numeric condition between two linear expressions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Condition {
    pub left: LinearExpr,      // Left-hand side of the comparison
    pub operator: String,      // One of: "=", "<", "<=", ">", ">="
    pub right: LinearExpr,     // Right-hand side of the comparison
}

impl Condition {
    /// Checks whether the condition holds in the current state.
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

/// The goal is a conjunction of conditions that must all be satisfied.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Goal {
    pub conditions: Vec<Condition>, // All conditions that define the goal
}

impl Goal {
    /// Returns true if all goal conditions are satisfied in the given state.
    pub fn is_goal_state(&self, state: &State) -> bool {
        self.conditions.iter().all(|cond| cond.is_satisfied(state))
    }
}

/// A single numeric counter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Counter {
    pub value: i32,       // Current integer value of the counter
    pub name: String,     // Unique identifier for the counter
}

impl Counter {
    /// Constructs a new counter with a given name and initial value.
    pub fn new(name: String, value: i32) -> Self {
        Counter { name, value }
    }
}

/// Describes the entire problem instance for the counter domain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CountersProblem {
    pub max_value: i32,  // Maximum allowed value for any counter
    pub goal: Goal,      // Goal conditions to be satisfied
}

impl CountersProblem {
    /// Creates an action that increases a counter's value by 1.
    pub fn get_increase_action(counter: &Counter) -> Action {
        let mut parameters = std::collections::HashMap::new();
        let action_name = format!("increase_counter{}", counter.name);
        parameters.insert("increase".to_string(), Value::Text("increase".to_string()));
        parameters.insert("counter".to_string(), Value::Text(counter.name.clone()));
        Action::new(action_name, 1, parameters)
    }

    /// Creates an action that decreases a counter's value by 1.
    pub fn get_decrease_action(counter: &Counter) -> Action {
        let mut parameters = std::collections::HashMap::new();
        let action_name = format!("decrease_counter{}", counter.name);
        parameters.insert("decrease".to_string(), Value::Text("decrease".to_string()));
        parameters.insert("counter".to_string(), Value::Text(counter.name.clone()));
        Action::new(action_name, 1, parameters)
    }

    /// Returns all valid actions for counters in the current state.
    /// Actions are filtered by boundaries (1 to max_value).
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

    /// Applies an increase action and returns the resulting state.
    pub fn apply_increase_action(state: &State, action: &Action) -> State {
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

    /// Applies a decrease action and returns the resulting state.
    pub fn apply_decrease_action(state: &State, action: &Action) -> State {
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

    /// Lists all applicable actions from the current state.
    fn get_possible_actions(&self, state: &State) -> Vec<Action> {
        self.get_actions(state)
    }

    /// Applies an action to the current state and returns the resulting state.
    fn apply_action(&self, state: &State, action: &Action) -> State {
        if action.name.starts_with("increase_") {
            Self::apply_increase_action(state, action)
        } else if action.name.starts_with("decrease_") {
            Self::apply_decrease_action(state, action)
        } else {
            panic!("Unknown action type: {}", action.name)
        }
    }

    /// Checks whether the state satisfies the goal.
    fn is_goal_state(&self, state: &State) -> bool {
        self.goal.is_goal_state(state)
    }

    /// Returns the heuristic value (currently zero).
    fn heuristic(&self, _state: &State) -> f64 {
        0.0
    }

    /// Loads the state and problem from a JSON file.
    fn load_state_from_json(json_path: &str) -> (State, CountersProblem) {
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
        let problem: CountersProblem =
            serde_json::from_value(problem_value.clone()).expect("Failed to deserialize problem");

        (state, problem)
    }
}
