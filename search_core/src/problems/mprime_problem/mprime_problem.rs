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
    pub fn is_goal_state(&self,state: &State) -> bool {
        for condition in &self.conditions {
            let emotion = &condition.emotion;
            let food = &condition.food;

            // Try to find the emotion in pleasures
            let found_in_pleasures = state
                .pleasures
                .iter()
                .find(|p| &p.name == emotion)
                .map_or(false, |p| p.craves.contains(food));

            // If not in pleasures, try in pains
            let found_in_pains = state
                .pains
                .iter()
                .find(|p| &p.name == emotion)
                .map_or(false, |p| p.craves.contains(food));

            if !found_in_pleasures && !found_in_pains {
                return false;
            }
        }
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MPrimeProblem {
    eats: HashMap<String, Vec<String>>,
    goal: Goal,
}

impl MPrimeProblem {
    pub fn get_over_come_actions(state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for pleasure in &state.pleasures {
            if pleasure.harmony >= 1 {
                for pain in &state.pains {
                    for food in &state.foods {
                        if pleasure.craves.contains(&food.name) && pain.craves.contains(&food.name)
                        {
                            let mut parameters = std::collections::HashMap::new();
                            let action_name =
                                format!("over_come_{}_{}_{}", pleasure.name, pain.name, food.name);
                            parameters
                                .insert("pleasure".to_string(), Value::Text(pleasure.name.clone()));
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
                    if pleasure.craves.contains(&food.name) && pain.fears.contains(&pleasure.name) {
                        let mut parameters = std::collections::HashMap::new();
                        let action_name =
                            format!("succumb_{}_{}_{}", pleasure.name, pain.name, food.name);
                        parameters
                            .insert("pleasure".to_string(), Value::Text(pleasure.name.clone()));
                        parameters.insert("pain".to_string(), Value::Text(pain.name.clone()));
                        parameters.insert("food".to_string(), Value::Text(food.name.clone()));
                        actions.push(Action::new(action_name, 1, parameters));
                    }
                }
            }
        }
        actions
    }

    pub fn get_feast_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for pleasure in &state.pleasures {
            for food1 in &state.foods {
                if pleasure.craves.contains(&food1.name) && food1.locale >= 1 {
                    for food2 in &state.foods {
                        if let Some(eats_vec) = self.eats.get(&food1.name) {
                            if eats_vec.contains(&food2.name) {
                                let mut parameters = std::collections::HashMap::new();
                                let action_name =
                                    format!("feast_{}_{}_{}", pleasure.name, food1.name, food2.name);
                                parameters
                                    .insert("pleasure".to_string(), Value::Text(pleasure.name.clone()));
                                parameters.insert("food1".to_string(), Value::Text(food1.name.clone()));
                                parameters.insert("food2".to_string(), Value::Text(food2.name.clone()));
                                actions.push(Action::new(action_name, 1, parameters));
                            }
                        }
                    }
                }
            }
        }
        actions
    }

    pub fn get_drink_actions(state: &State) -> Vec<Action> {
        let mut actions = Vec::new();

        for food1 in &state.foods {
            if food1.locale >= 1 {
                for food2 in &state.foods {
                    if food1.name != food2.name {
                        let mut parameters = std::collections::HashMap::new();
                        let action_name = format!("drink_{}_{}", food1.name, food2.name);

                        parameters.insert("food1".to_string(), Value::Text(food1.name.clone()));
                        parameters.insert("food2".to_string(), Value::Text(food2.name.clone()));
                        actions.push(Action::new(action_name, 1, parameters));
                    }
                }
            }
        }
        actions
    }

    pub fn apply_drink_action(state: &State, action: &Action) -> State {
        // Start by cloning the current state.
        let mut new_state = state.clone();
        let food1_name = match action.parameters.get("food1") {
            Some(Value::Text(name)) => name,
            _ => panic!("Action parameters do not contain a valid name for food."),
        };
        let food2_name = match action.parameters.get("food2") {
            Some(Value::Text(name)) => name,
            _ => panic!("Action parameters do not contain a valid name for food."),
        };
        let food1_index = new_state
            .foods
            .iter()
            .position(|v| v.name == *food1_name)
            .expect(&format!("Food with name {} not found", food1_name));

        let food2_index = new_state
            .foods
            .iter()
            .position(|v| v.name == *food2_name)
            .expect(&format!("Food with name {} not found", food2_name));

        new_state.foods[food1_index].locale -= 1;
        new_state.foods[food2_index].locale += 1;

        new_state
    }

    pub fn apply_succumb_action(state: &State, action: &Action) -> State {
        // Start by cloning the current state.
        let mut new_state = state.clone();
        let pain_name = match action.parameters.get("pain") {
            Some(Value::Text(name)) => name,
            _ => panic!("Action parameters do not contain a valid name for pain."),
        };
        let food_name = match action.parameters.get("food") {
            Some(Value::Text(name)) => name,
            _ => panic!("Action parameters do not contain a valid name for food."),
        };
        let pleasure_name = match action.parameters.get("pleasure") {
            Some(Value::Text(name)) => name,
            _ => panic!("Action parameters do not contain a valid name for pleasure."),
        };
        let pleasure = new_state
            .pleasures
            .iter_mut()
            .find(|v| v.name == *pleasure_name)
            .expect(&format!("Pleasure with name {} not found", pleasure_name));

        pleasure.harmony += 1;

        let pain = new_state
            .pains
            .iter_mut()
            .find(|v| v.name == *pleasure_name)
            .expect(&format!("Pain with name {} not found", pain_name));
        pain.craves.push(food_name.clone());
        pain.fears.retain(|f| f != pleasure_name);

        new_state
    }
    pub fn apply_feast_action(state: &State, action: &Action) -> State {
        // Start by cloning the current state.
        let mut new_state = state.clone();
        let food1_name = match action.parameters.get("food1") {
            Some(Value::Text(name)) => name,
            _ => panic!("Action parameters do not contain a valid name for food."),
        };
        let food2_name = match action.parameters.get("food2") {
            Some(Value::Text(name)) => name,
            _ => panic!("Action parameters do not contain a valid name for food."),
        };
        let pleasure_name = match action.parameters.get("pleasure") {
            Some(Value::Text(name)) => name,
            _ => panic!("Action parameters do not contain a valid name for pleasure."),
        };
        let pleasure = new_state
            .pleasures
            .iter_mut()
            .find(|v| v.name == *pleasure_name)
            .expect(&format!("Pleasure with name {} not found", pleasure_name));
        let food1 = new_state
            .foods
            .iter_mut()
            .find(|v| v.name == *food1_name)
            .expect(&format!("Food with name {} not found", food1_name));

        food1.locale -= 1;

        pleasure.craves.retain(|f| f != food1_name);
        pleasure.craves.push(food2_name.clone());

        new_state
    }

    pub fn apply_over_come_action(state: &State, action: &Action) -> State {
        // Start by cloning the current state.
        let mut new_state = state.clone();
        let pain_name = match action.parameters.get("pain") {
            Some(Value::Text(name)) => name,
            _ => panic!("Action parameters do not contain a valid name for pain."),
        };
        let food_name = match action.parameters.get("food") {
            Some(Value::Text(name)) => name,
            _ => panic!("Action parameters do not contain a valid name for food."),
        };
        let pleasure_name = match action.parameters.get("pleasure") {
            Some(Value::Text(name)) => name,
            _ => panic!("Action parameters do not contain a valid name for pleasure."),
        };
        let pleasure = new_state
            .pleasures
            .iter_mut()
            .find(|v| v.name == *pleasure_name)
            .expect(&format!("Pleasure with name {} not found", pleasure_name));
        let pain = new_state
            .pains
            .iter_mut()
            .find(|v| v.name == *pain_name)
            .expect(&format!("Pain with name {} not found", pain_name));

        pleasure.harmony -= 1;

        pain.craves.retain(|f| f != food_name);
        pain.fears.push(pleasure_name.clone());

        new_state
    }
}

impl Problem for MPrimeProblem {
    type State = State;

    fn get_possible_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();

        actions.extend(Self::get_over_come_actions(state));
        actions.extend(self.get_feast_actions(state));
        actions.extend(Self::get_succumb_actions(state));
        actions.extend(Self::get_drink_actions(state));
        actions
    }

    fn apply_action(&self, state: &State, action: &Action) -> State {
        if action.name.starts_with("drink_") {
            Self::apply_drink_action(state, action)
        } else if action.name.starts_with("feast_") {
            Self::apply_feast_action(state, action)
        } else if action.name.starts_with("succumb_") {
            Self::apply_succumb_action(state, action)
        } else if action.name.starts_with("over_come_") {
            Self::apply_over_come_action(state, action)
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
