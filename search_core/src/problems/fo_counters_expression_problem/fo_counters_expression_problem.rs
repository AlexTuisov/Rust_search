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
    pub rate_value: i32,
    pub name: String,
}

impl Counter {
    /// Creates a new Counter with the given parameters.
    pub fn new(name: String, rate_value: i32, value: i32) -> Self {
        Counter {
            name,
            rate_value,
            value,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FoCountersExpressionProblem {
    pub max_value: i32,
    pub goal: Goal,
}

impl FoCountersExpressionProblem {
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
        parameters.insert(
            "increase_rate".to_string(),
            Value::Text("increase_rate".to_string()),
        );
        parameters.insert("counter".to_string(), Value::Text(counter.name.clone()));
        Action::new(action_name, 1, parameters)
    }

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
            new_state.total_cost += 1;
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
    pub fn apply_increace_rate_action(state: &State, action: &Action) -> State {
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
            counter.rate_value += 1;
            new_state.total_cost += 1;
        } else {
            panic!("Counter with name {} not found", counter_name);
        }

        new_state
    }
    pub fn parse_expression(expr_str: &str) -> Expression {
        let tokens: Vec<&str> = expr_str.split_whitespace().collect();

        if tokens.len() == 1 {
            if let Ok(num) = tokens[0].parse::<i32>() {
                return Expression::Constant(num);
            }
            return Expression::Value(tokens[0].to_string());
        }

        // Find the last operator with two operands (to handle nested expressions correctly).
        let mut operator_idx = None;
        let mut depth = 0;

        for (i, &token) in tokens.iter().enumerate() {
            match token {
                "(" => depth += 1,
                ")" => depth -= 1,
                "+" | "-" | "*" if depth == 0 => operator_idx = Some(i),
                _ => {}
            }
        }

        if let Some(idx) = operator_idx {
            let left = Self::parse_expression(&tokens[..idx].join(" "));
            let right = Self::parse_expression(&tokens[idx + 1..].join(" "));

            match tokens[idx] {
                "+" => return Expression::Add(Box::new(left), Box::new(right)),
                "-" => return Expression::Sub(Box::new(left), Box::new(right)),
                "*" => return Expression::Mul(Box::new(left), Box::new(right)),
                _ => panic!("Unknown operator: {}", tokens[idx]),
            }
        }

        panic!("Invalid expression: {}", expr_str);
    }
    pub fn parse_condition(cond_str: &str) -> (Expression, String, Expression) {
        let cond_clean = cond_str
            .replace("(", "")
            .replace(")", "")
            .trim()
            .to_string();
        let parts: Vec<&str> = cond_clean.split_whitespace().collect();

        if parts.len() < 3 {
            panic!("Invalid condition format: {}", cond_clean);
        }

        // The last two tokens are always the operator and right-hand side.
        let operator = parts[parts.len() - 2].to_string();
        let right_expr = Self::parse_expression(parts[parts.len() - 1]);

        // The left-hand side is everything before the operator.
        let left_expr = Self::parse_expression(&parts[..parts.len() - 2].join(" "));

        (left_expr, operator, right_expr)
    }
}

impl Problem for FoCountersExpressionProblem {
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
        } else if action.name.starts_with("rateD_") {
            Self::apply_decrease_rate_action(state, action)
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
                let counter_value = value.get("value").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                let rate_value = value
                    .get("rate_value")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0) as i32;
                counters.push(Counter::new(format!("c{}", key), rate_value, counter_value));
            }
        }

        let state = State {
            counters,
            total_cost: 0,
        };

        // Parse the "Goal" section.
        let goal_map = json_value.get("Goal").expect("Goal key missing");
        let mut conditions = Vec::new();
        if let Some(obj) = goal_map.as_object() {
            for (key, value) in obj.iter() {
                let cond_str = value
                    .as_str()
                    .expect("Expected goal condition to be a string");

                // Parse condition using a recursive parser.
                let (left_expr, operator, right_expr) = Self::parse_condition(cond_str);

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

        let problem = FoCountersExpressionProblem { max_value, goal };

        (state, problem)
    }
}
