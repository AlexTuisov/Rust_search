use crate::problems::problem::Problem;
use crate::search::{action::Action, state::StateTrait, state::Value};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::fs;

/// Represents the state of the world, which consists of a list of counters.
/// Each counter has a name, value, and rate of change.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct State {
    pub counters: Vec<Counter>,
}

impl State {}
impl StateTrait for State {}

/// A linear expression of the form: c1*x1 + c2*x2 + ... + cn*xn + constant
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinearExpr {
    /// List of (coefficient, counter name) pairs
    pub terms: Vec<(i32, String)>,
    /// Constant term added to the expression
    pub constant: i32,
}

impl LinearExpr {
    /// Evaluates the linear expression over a given state.
    /// Will panic if any referenced counter is missing.
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

/// A condition compares two linear expressions with a relational operator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Condition {
    pub left: LinearExpr,
    pub operator: String, // supports "=", "<=", "<", ">", ">="
    pub right: LinearExpr,
}

impl Condition {
    /// Evaluates the condition in the context of the given state.
    /// Supports standard comparison operators.
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

/// A goal is a conjunction of conditions that must all be satisfied.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Goal {
    pub conditions: Vec<Condition>,
}

impl Goal {
    /// Returns true if all conditions in the goal are satisfied by the state.
    pub fn is_goal_state(&self, state: &State) -> bool {
        self.conditions.iter().all(|cond| cond.is_satisfied(state))
    }
}

/// A numeric counter with a value and a rate of change per step.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Counter {
    pub value: i32,
    pub rate_value: i32,
    pub name: String,
}

impl Counter {
    /// Creates a new Counter with name, rate, and initial value.
    pub fn new(name: String, rate_value: i32, value: i32) -> Self {
        Counter {
            name,
            rate_value,
            value,
        }
    }
}

/// FO-Counters problem, supporting both counter manipulation and rate control.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FoCountersProblem {
    /// Maximum value a counter can reach.
    pub max_value: i32,
    pub goal: Goal,
}

impl FoCountersProblem {
    // === ACTION GENERATORS ===

    /// Creates an action to increase a counter by its current rate.
    pub fn get_increase_action(counter: &Counter) -> Action {
        let mut parameters = std::collections::HashMap::new();
        let action_name = format!("increase_counter{}", counter.name);
        parameters.insert("increase".to_string(), Value::Text("increase".to_string()));
        parameters.insert("counter".to_string(), Value::Text(counter.name.clone()));
        Action::new(action_name, 1, parameters)
    }

    /// Creates an action to decrease a counter by its rate.
    pub fn get_decrease_action(counter: &Counter) -> Action {
        let mut parameters = std::collections::HashMap::new();
        let action_name = format!("decrease_counter{}", counter.name);
        parameters.insert("decrease".to_string(), Value::Text("decrease".to_string()));
        parameters.insert("counter".to_string(), Value::Text(counter.name.clone()));
        Action::new(action_name, 1, parameters)
    }

    /// Creates an action to increase a counter's rate by 1.
    pub fn get_increase_rate_action(counter: &Counter) -> Action {
        let mut parameters = std::collections::HashMap::new();
        let action_name = format!("rateI_counter{}", counter.name);
        parameters.insert(
            "increase_rate".to_string(),
            Value::Text("increase_rate".to_string()),
        );
        parameters.insert("counter".to_string(), Value::Text(counter.name.clone()));
        Action::new(action_name, 1, parameters)
    }

    /// Creates an action to decrease a counter's rate by 1.
    pub fn get_decrease_rate_action(counter: &Counter) -> Action {
        let mut parameters = std::collections::HashMap::new();
        let action_name = format!("rateD_counter{}", counter.name);
        parameters.insert(
            "decrease_rate".to_string(),
            Value::Text("decrease_rate".to_string()),
        );
        parameters.insert("counter".to_string(), Value::Text(counter.name.clone()));
        Action::new(action_name, 1, parameters)
    }

    /// Returns all applicable actions for the current state.
    /// Guards are used to prevent invalid or unsafe operations.
    pub fn get_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for counter in &state.counters {
            if counter.value + counter.rate_value <= self.max_value {
                actions.push(Self::get_increase_action(counter));
            }
            if counter.value - counter.rate_value >= 0 {
                actions.push(Self::get_decrease_action(counter));
            }
            if counter.rate_value + 1 <= 10 {
                actions.push(Self::get_increase_rate_action(counter));
            }
            if counter.rate_value >= 1 {
                actions.push(Self::get_decrease_rate_action(counter));
            }
        }
        actions
    }

    // === ACTION APPLICATION ===

    /// Applies an increase action: value += rate
    pub fn apply_increase_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let counter_name = match action.parameters.get("counter") {
            Some(Value::Text(name)) => name,
            _ => panic!("Missing or invalid 'counter' parameter"),
        };

        if let Some(counter) = new_state
            .counters
            .iter_mut()
            .find(|v| v.name == *counter_name)
        {
            counter.value += counter.rate_value;
        } else {
            panic!("Counter with name {} not found", counter_name);
        }

        new_state
    }

    /// Applies a decrease action: value -= rate
    pub fn apply_decrease_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let counter_name = match action.parameters.get("counter") {
            Some(Value::Text(name)) => name,
            _ => panic!("Missing or invalid 'counter' parameter"),
        };

        if let Some(counter) = new_state
            .counters
            .iter_mut()
            .find(|v| v.name == *counter_name)
        {
            counter.value -= counter.rate_value;
        } else {
            panic!("Counter with name {} not found", counter_name);
        }

        new_state
    }

    /// Applies a decrease rate action: rate -= 1
    pub fn apply_decrease_rate_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let counter_name = match action.parameters.get("counter") {
            Some(Value::Text(name)) => name,
            _ => panic!("Missing or invalid 'counter' parameter"),
        };

        if let Some(counter) = new_state
            .counters
            .iter_mut()
            .find(|v| v.name == *counter_name)
        {
            counter.rate_value -= 1;
        } else {
            panic!("Counter with name {} not found", counter_name);
        }

        new_state
    }

    /// Applies an increase rate action: rate += 1
    pub fn apply_increace_rate_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let counter_name = match action.parameters.get("counter") {
            Some(Value::Text(name)) => name,
            _ => panic!("Missing or invalid 'counter' parameter"),
        };

        if let Some(counter) = new_state
            .counters
            .iter_mut()
            .find(|v| v.name == *counter_name)
        {
            counter.rate_value += 1;
        } else {
            panic!("Counter with name {} not found", counter_name);
        }

        new_state
    }
}

// === Integration with the search engine ===
impl Problem for FoCountersProblem {
    type State = State;

    /// Returns all valid actions for the state.
    fn get_possible_actions(&self, state: &State) -> Vec<Action> {
        self.get_actions(state)
    }

    /// Dispatches the action to the correct application method based on prefix.
    fn apply_action(&self, state: &State, action: &Action) -> State {
        if action.name.starts_with("increase_") {
            Self::apply_increase_action(state, action)
        } else if action.name.starts_with("decrease_") {
            Self::apply_decrease_action(state, action)
        } else if action.name.starts_with("rateI_") {
            Self::apply_increace_rate_action(state, action)
        } else if action.name.starts_with("rateD_") {
            Self::apply_decrease_rate_action(state, action)
        } else {
            panic!("Unknown action type: {}", action.name)
        }
    }

    /// Checks whether the goal is satisfied.
    fn is_goal_state(&self, state: &State) -> bool {
        self.goal.is_goal_state(state)
    }

    /// Returns the heuristic estimate for the state.
    /// (Currently zero — can be overridden with a domain-specific heuristic.)
    fn heuristic(&self, _state: &State) -> f64 {
        0.0
    }

    /// Loads a state and problem instance from a JSON file.
    fn load_state_from_json(json_path: &str) -> (State, FoCountersProblem) {
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
        let problem: FoCountersProblem =
            serde_json::from_value(problem_value.clone()).expect("Failed to deserialize problem");

        (state, problem)
    }
}
