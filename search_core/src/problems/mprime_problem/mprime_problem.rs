// Domain-specific imports for planning problem traits and serialization
use crate::problems::problem::Problem;
use crate::search::{action::Action, state::StateTrait, state::Value};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::fs;

// Represents the full world state in MPrime:
// - pleasures: list of Pleasure entities
// - pains: list of Pain entities
// - foods: list of available Food items
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    /// List of all Pleasure entities in the current state
    pleasures: Vec<Pleasure>,
    /// List of all Pain entities in the current state
    pains: Vec<Pain>,
    /// List of all Food items and their locale counts
    foods: Vec<Food>,
}

// Empty impl block required by framework; no custom methods on State
impl State {}

// Implements the StateTrait so this State can be used by the search framework
impl StateTrait for State {}

// Represents a Food item with:
// - name: unique identifier string
// - locale: integer count of how many units are available
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Food {
    /// Unique name of the food item
    name: String,
    /// Quantity or availability count of this food
    locale: i32,
}

// Represents a Pleasure entity with:
// - name: unique identifier for the emotion
// - harmony: integer measure of current harmony level
// - craves: list of food names this pleasure currently craves
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pleasure {
    /// Unique name of the pleasure emotion
    name: String,
    /// Current harmony level (>=0)
    harmony: i32,
    /// List of food names this pleasure craves
    craves: Vec<String>,
}

// Represents a Pain entity with:
// - name: unique identifier for the emotion
// - harmony: integer measure of current disharmony
// - craves: list of food names that pain craves
// - fears: list of pleasure names that this pain fears
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pain {
    /// Unique name of the pain emotion
    name: String,
    /// Current harmony level for the pain (>=0)
    harmony: i32,
    /// List of food names this pain craves
    craves: Vec<String>,
    /// List of pleasure names this pain fears
    fears: Vec<String>,
}

// Single goal condition requiring an emotion to crave a food:
// - emotion: name of a Pleasure or Pain
// - food: name of the food that must be craved
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Condition {
    /// Name of the target emotion (Pleasure or Pain)
    emotion: String,
    /// Name of the required food item
    food: String,
}

// Overall goal is composed of multiple Conditions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Goal {
    /// Vector of conditions that must all hold true
    conditions: Vec<Condition>,
}

impl Goal {
    /// Returns true if every Condition in `self.conditions` is satisfied in `state`
    pub fn is_goal_state(&self, state: &State) -> bool {
        // Iterate through each goal condition
        for condition in &self.conditions {
            let emotion = &condition.emotion;
            let food = &condition.food;

            // Check if any Pleasure with matching name craves this food
            let found_in_pleasures = state
                .pleasures
                .iter()
                .find(|p| &p.name == emotion)
                .map_or(false, |p| p.craves.contains(food));

            // Otherwise, check if any Pain with matching name craves this food
            let found_in_pains = state
                .pains
                .iter()
                .find(|p| &p.name == emotion)
                .map_or(false, |p| p.craves.contains(food));

            // If neither pleasure nor pain satisfies the craving, goal fails
            if !found_in_pleasures && !found_in_pains {
                return false;
            }
        }
        // All conditions passed
        true
    }
}

// Main problem struct defining:
// - eats: compatibility map of which foods can lead to others
// - goal: the Goal instance to achieve
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MPrimeProblem {
    /// Mapping: for each food name, list of foods it can 'eat' to discover flavor
    eats: HashMap<String, Vec<String>>,
    /// The goal conditions for this problem
    goal: Goal,
}

impl MPrimeProblem {
    /// Generate all "over_come" actions:
    /// For each Pleasure with harmony >=1 that craves a food,
    /// and each Pain that also craves that same food,
    /// create an action to let the Pleasure overcome the Pain using that food.
    pub fn get_over_come_actions(state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for pleasure in &state.pleasures {
            if pleasure.harmony >= 1 {
                for pain in &state.pains {
                    for food in &state.foods {
                        if pleasure.craves.contains(&food.name) && pain.craves.contains(&food.name)
                        {
                            let mut parameters = HashMap::new();
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

    /// Generate all "succumb" actions:
    /// For each Pleasure and Pain pair,
    /// if Pleasure craves a food and Pain fears that Pleasure,
    /// create an action to let Pleasure succumb, increasing Pain's crave.
    pub fn get_succumb_actions(state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for pleasure in &state.pleasures {
            for pain in &state.pains {
                for food in &state.foods {
                    if pleasure.craves.contains(&food.name) && pain.fears.contains(&pleasure.name) {
                        let mut parameters = HashMap::new();
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

    /// Generate all "feast" actions:
    /// For each Pleasure that craves food1 and food1 has availability,
    /// and for each food2 that food1 can eat (per `eats` map),
    /// create an action to feast: consume one unit of food1 and gain food2 craving.
    pub fn get_feast_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for pleasure in &state.pleasures {
            for food1 in &state.foods {
                if pleasure.craves.contains(&food1.name) && food1.locale >= 1 {
                    if let Some(eats_vec) = self.eats.get(&food1.name) {
                        for food2 in &state.foods {
                            if eats_vec.contains(&food2.name) {
                                let mut parameters = HashMap::new();
                                let action_name = format!(
                                    "feast_{}_{},{}",
                                    pleasure.name, food1.name, food2.name
                                );
                                parameters.insert(
                                    "pleasure".to_string(),
                                    Value::Text(pleasure.name.clone()),
                                );
                                parameters
                                    .insert("food1".to_string(), Value::Text(food1.name.clone()));
                                parameters
                                    .insert("food2".to_string(), Value::Text(food2.name.clone()));
                                actions.push(Action::new(action_name, 1, parameters));
                            }
                        }
                    }
                }
            }
        }
        actions
    }

    /// Generate all "drink" actions:
    /// For each food1 with at least one unit,
    /// allow transferring one unit from food1 to any other food2.
    pub fn get_drink_actions(state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for food1 in &state.foods {
            if food1.locale >= 1 {
                for food2 in &state.foods {
                    if food1.name != food2.name {
                        let mut parameters = HashMap::new();
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

    /// Applies a "drink" action to a cloned state:
    /// - Decrement `locale` of food1 by 1
    /// - Increment `locale` of food2 by 1
    pub fn apply_drink_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let food1_name = match action.parameters.get("food1") {
            Some(Value::Text(name)) => name,
            _ => panic!("Missing or invalid 'food1' parameter"),
        };
        let food2_name = match action.parameters.get("food2") {
            Some(Value::Text(name)) => name,
            _ => panic!("Missing or invalid 'food2' parameter"),
        };
        let idx1 = new_state
            .foods
            .iter()
            .position(|f| f.name == *food1_name)
            .expect("Food1 not found");
        let idx2 = new_state
            .foods
            .iter()
            .position(|f| f.name == *food2_name)
            .expect("Food2 not found");
        new_state.foods[idx1].locale -= 1;
        new_state.foods[idx2].locale += 1;
        new_state
    }

    /// Applies a "succumb" action to a cloned state:
    /// - Increase the pleasure's harmony by 1
    /// - Add the food to the pain's craves
    /// - Remove the pleasure from the pain's fears
    pub fn apply_succumb_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let pain_name = match action.parameters.get("pain") {
            Some(Value::Text(n)) => n,
            _ => panic!("Missing or invalid 'pain' parameter"),
        };
        let food_name = match action.parameters.get("food") {
            Some(Value::Text(n)) => n,
            _ => panic!("Missing or invalid 'food' parameter"),
        };
        let pleasure_name = match action.parameters.get("pleasure") {
            Some(Value::Text(n)) => n,
            _ => panic!("Missing or invalid 'pleasure' parameter"),
        };
        let pleasure = new_state
            .pleasures
            .iter_mut()
            .find(|p| p.name == *pleasure_name)
            .expect("Pleasure not found");
        pleasure.harmony += 1;
        let pain = new_state
            .pains
            .iter_mut()
            .find(|p| p.name == *pain_name)
            .expect("Pain not found");
        pain.craves.push(food_name.clone());
        pain.fears.retain(|f| f != pleasure_name);
        new_state
    }

    /// Applies a "feast" action to a cloned state:
    /// - Decrement `locale` of food1
    /// - Remove food1 from pleasure's `craves`
    /// - Add food2 to pleasure's `craves`
    pub fn apply_feast_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let food1_name = match action.parameters.get("food1") {
            Some(Value::Text(n)) => n,
            _ => panic!("Missing or invalid 'food1' parameter"),
        };
        let food2_name = match action.parameters.get("food2") {
            Some(Value::Text(n)) => n,
            _ => panic!("Missing or invalid 'food2' parameter"),
        };
        let pleasure_name = match action.parameters.get("pleasure") {
            Some(Value::Text(n)) => n,
            _ => panic!("Missing or invalid 'pleasure' parameter"),
        };
        let pleasure = new_state
            .pleasures
            .iter_mut()
            .find(|p| p.name == *pleasure_name)
            .expect("Pleasure not found");
        let food1 = new_state
            .foods
            .iter_mut()
            .find(|f| f.name == *food1_name)
            .expect("Food1 not found");
        food1.locale -= 1;
        pleasure.craves.retain(|c| c != food1_name);
        pleasure.craves.push(food2_name.clone());
        new_state
    }

    /// Applies an "over_come" action to a cloned state:
    /// - Decrease pleasure's harmony by 1
    /// - Remove food from pain's `craves`
    /// - Add pleasure to pain's `fears`
    pub fn apply_over_come_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let pain_name = match action.parameters.get("pain") {
            Some(Value::Text(n)) => n,
            _ => panic!("Missing or invalid 'pain' param"),
        };
        let food_name = match action.parameters.get("food") {
            Some(Value::Text(n)) => n,
            _ => panic!("Missing or invalid 'food' param"),
        };
        let pleasure_name = match action.parameters.get("pleasure") {
            Some(Value::Text(n)) => n,
            _ => panic!("Missing or invalid 'pleasure' param"),
        };
        let pleasure = new_state
            .pleasures
            .iter_mut()
            .find(|p| p.name == *pleasure_name)
            .expect("Pleasure not found");
        let pain = new_state
            .pains
            .iter_mut()
            .find(|p| p.name == *pain_name)
            .expect("Pain not found");
        pleasure.harmony -= 1;
        pain.craves.retain(|c| c != food_name);
        pain.fears.push(pleasure_name.clone());
        new_state
    }
}

impl Problem for MPrimeProblem {
    type State = State;

    /// Returns the full set of applicable actions in `state`
    fn get_possible_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        actions.extend(Self::get_over_come_actions(state));
        actions.extend(self.get_feast_actions(state));
        actions.extend(Self::get_succumb_actions(state));
        actions.extend(Self::get_drink_actions(state));
        actions
    }

    /// Dispatches to the correct apply_* function based on action name prefix
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

    /// Checks whether `state` satisfies the problem goal
    fn is_goal_state(&self, state: &State) -> bool {
        self.goal.is_goal_state(state)
    }

    /// Heuristic value for informed search (currently zero for uniform-cost)
    fn heuristic(&self, _state: &State) -> f64 {
        0.0
    }

    /// Loads initial `State` and `MPrimeProblem` from a JSON file
    fn load_state_from_json(json_path: &str) -> (State, MPrimeProblem) {
        let json_str = fs::read_to_string(json_path).expect("Failed to read JSON file");
        let json_value: JsonValue = serde_json::from_str(&json_str).expect("Failed to parse JSON");
        let state_val = json_value.get("state").expect("Missing 'state' field");
        let prob_val = json_value.get("problem").expect("Missing 'problem' field");
        let state: State =
            serde_json::from_value(state_val.clone()).expect("Failed to deserialize state");
        let problem: MPrimeProblem =
            serde_json::from_value(prob_val.clone()).expect("Failed to deserialize problem");
        (state, problem)
    }
}
