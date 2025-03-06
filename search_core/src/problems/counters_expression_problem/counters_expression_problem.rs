use crate::problems::problem::Problem;
use crate::search::{action::Action, state::StateTrait, state::Value};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub counters: Vec<Counter>,
}

impl State {}

impl StateTrait for State {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Expression {
    Value(String),                         // e.g., represents counter "c0"
    Constant(i32),                         // a constant value
    Add(Box<Expression>, Box<Expression>), // addition: e1 + e2
    Sub(Box<Expression>, Box<Expression>), // subtraction: e1 - e2
    Mul(Box<Expression>, Box<Expression>), // multiplication: e1 * e2
}

impl Expression {
    /// Evaluates the expression based on the current state.
    pub fn evaluate(&self, state: &State) -> i32 {
        match self {
            Expression::Value(name) => {
                if let Some(counter) = state.counters.iter().find(|c| c.name == *name) {
                    counter.value
                } else {
                    panic!("Counter {} not found in state", name);
                }
            }
            Expression::Constant(n) => *n,
            Expression::Add(e1, e2) => e1.evaluate(state) + e2.evaluate(state),
            Expression::Sub(e1, e2) => e1.evaluate(state) - e2.evaluate(state),
            Expression::Mul(e1, e2) => e1.evaluate(state) * e2.evaluate(state),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Condition {
    pub left: Expression,
    pub operator: String, // e.g., "=", "<=", "<", ">", ">="
    pub right: Expression,
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
    /// Creates a new Counter with the given parameters.
    pub fn new(name: String, value: i32) -> Self {
        Counter { name, value }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CountersExpressionProblem {
    pub max_value: i32,
    pub goal: Goal,
}

impl CountersExpressionProblem {
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

impl Problem for CountersExpressionProblem {
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

    fn load_state_from_json(json_path: &str) -> (State, CountersExpressionProblem) {
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
        let problem: CountersExpressionProblem =
            serde_json::from_value(problem_value.clone()).expect("Failed to deserialize problem");

        (state, problem)
    }
}
