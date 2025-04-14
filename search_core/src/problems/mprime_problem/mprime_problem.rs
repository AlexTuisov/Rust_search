use crate::problems::problem::Problem;
use crate::search::{action::Action, state::StateTrait, state::Value};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::fs;

// Each State have airplanes, markets, and total cost
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pleasures: Vec<Pleasure>,
    pains: Vec<Pain>,
    foods: Vec<Food>,
}

impl State {}
impl StateTrait for State {}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Food {
    name: String,
    locale: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pleasure {
    name: String,
    harmony: i32,
    craves: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pain {
    name: String,
    harmony: i32,
    craves: Vec<String>,
    fears: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Condition {
    emotion: String,
    food: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Goal {
    conditions: Vec<Condition>,
}
impl Goal {
    pub fn is_goal_state(state: &State) -> bool {
        todo!("Complete it")
    }
}

pub struct MPrimeProblem {
    eats: HashMap<String, Vec<String>>,
    goal: Goal,
}

impl MPrimeProblem {
    pub fn get_over_come_actions(state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for pleasure in &state.pleasures {
            if (pleasure.harmony >= 1) {
                for pain in &state.pains {
                    for food in &state.foods {
                        if pleasure.craves.contains(&food.name) && pain.craves.contains(&food.name)
                        {
                            let mut parameters = std::collections::HashMap::new();
                            let action_name = format!("over_come_{}_{}_{}", pleasure.name,pain.name,food.name);
                            parameters.insert("pleasure".to_string(), Value::Text(pleasure.name.clone()));
                            parameters.insert("pain".to_string(), Value::Text(pain.name.clone()));
                            parameters.insert("food".to_string(), Value::Text(food.name.clone()));    
                            actions.push(Action::new(action_name, 1, parameters));
                        }
                    }
                }
            }
        }
        actions
    }

    pub fn get_succumb_actions(state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for pleasure in &state.pleasures {
            
                for pain in &state.pains {
                    for food in &state.foods {
                        if pleasure.craves.contains(&food.name) && pain.craves.contains(&food.name)
                        {
                            let mut parameters = std::collections::HashMap::new();
                            let action_name = format!("over_come_{}_{}_{}", pleasure.name,pain.name,food.name);
                            parameters.insert("pleasure".to_string(), Value::Text(pleasure.name.clone()));
                            parameters.insert("pain".to_string(), Value::Text(pain.name.clone()));
                            parameters.insert("food".to_string(), Value::Text(food.name.clone()));    
                            actions.push(Action::new(action_name, 1, parameters));
                        }
                    }
                }
            
        }
        actions
    }
}

impl Problem for MPrimeProblem {
    type State = State;

    fn get_possible_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();

        actions.extend(Self::get_over_come_actions(state));
        actions.extend(self.get_feast_actions(state));
        actions.extend(Self::get_sccumb_actions(state));
        actions.extend(Self::get_drink_actions(state));
        actions
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

    fn load_state_from_json(json_path: &str) -> (State, MPrimeProblem) {
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
        let problem: MPrimeProblem =
            serde_json::from_value(problem_value.clone()).expect("Failed to deserialize problem");

        (state, problem)
    }
}
