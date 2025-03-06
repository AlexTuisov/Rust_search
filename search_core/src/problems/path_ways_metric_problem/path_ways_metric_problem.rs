use crate::problems::problem::Problem;
use crate::search::{action::Action, state::StateTrait, state::Value};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub simples: Vec<Simple>,
    pub complexes: Vec<Complex>,
    pub num_subs: i32,
}
impl State {}
impl StateTrait for State {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssociationReaction {
    pub molecule_1_name: String,
    pub need_molecule_1: i32,
    pub molecule_2_name: String,
    pub need_molecule_2: i32,
    pub molecule_3_name: String,
    pub prod: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CatalyzedAssociationReaction {
    pub molecule_1_name: String,
    pub need_molecule_1: i32,
    pub molecule_2_name: String,
    pub need_molecule_2: i32,
    pub molecule_3_name: String,
    pub prod: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CatalyzedSelfAssociationReaction {
    pub molecule_1_name: String,
    pub need_molecule_1: i32,
    pub molecule_2_name: String,
    pub prod: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SynthesisReaction {
    pub molecule_1_name: String,
    pub need_molecule_1: i32,
    pub molecule_2_name: String,
    pub prod: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Simple {
    pub name: String,
    pub chosen: bool,
    pub possible: bool,
    pub available: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Complex {
    pub name: String,
    pub available: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Goal {
    conditions: Vec<Condition>,
}

impl Goal {
    pub fn is_goal_state(&self, state: &State) -> bool {
        // Iterate over each condition in the goal.
        for condition in &self.conditions {
            // Look for molecule_1 in both simples and complexes.
            let ava1 = state
                .simples
                .iter()
                .find(|s| s.name == condition.molecule_1_name)
                .map(|s| s.available)
                .unwrap_or(-100)
                + state
                    .complexes
                    .iter()
                    .find(|c| c.name == condition.molecule_1_name)
                    .map(|c| c.available)
                    .unwrap_or(-100);

            // Look for molecule_2 in both simples and complexes.
            let ava2 = state
                .simples
                .iter()
                .find(|s| s.name == condition.molecule_2_name)
                .map(|s| s.available)
                .unwrap_or(-100)
                + state
                    .complexes
                    .iter()
                    .find(|c| c.name == condition.molecule_2_name)
                    .map(|c| c.available)
                    .unwrap_or(-100);

            // If the sum of available amounts is less than the required amount,
            // the condition is not met.
            if ava1 + ava2 < condition.amount_condition {
                return false;
            }
        }
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Condition {
    molecule_1_name: String,
    molecule_2_name: String,
    amount_condition: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PathWaysMetricProblem {
    pub goal: Goal,
    pub association_reactions: Vec<AssociationReaction>,
    pub catalyzed_association_reactions: Vec<CatalyzedAssociationReaction>,
    pub catalyzed_self_association_reactions: Vec<CatalyzedSelfAssociationReaction>,
    pub synthesis_reactions: Vec<SynthesisReaction>,
}

impl PathWaysMetricProblem {
    pub fn get_choose_simple_action(simple: &Simple) -> Action {
        let mut parameters = std::collections::HashMap::new();
        let action_name = format!("choose_{}", simple.name);
        parameters.insert("choose".to_string(), Value::Text("choose".to_string()));
        parameters.insert("simple_name".to_string(), Value::Text(simple.name.clone()));
        Action::new(action_name, 1, parameters)
    }

    pub fn get_initialize_simple_action(simple: &Simple) -> Action {
        let mut parameters = std::collections::HashMap::new();
        let action_name = format!("initialize_{}", simple.name);
        parameters.insert(
            "initialize".to_string(),
            Value::Text("initialize".to_string()),
        );
        parameters.insert("simple_name".to_string(), Value::Text(simple.name.clone()));
        Action::new(action_name, 1, parameters)
    }

    pub fn get_associate_action(association: &AssociationReaction) -> Action {
        let mut parameters = std::collections::HashMap::new();
        let action_name = format!(
            "associate_{}_{}_to_{}",
            association.molecule_1_name, association.molecule_2_name, association.molecule_3_name
        );
        parameters.insert(
            "association".to_string(),
            Value::Text("association".to_string()),
        );
        parameters.insert(
            "molecule_1_name".to_string(),
            Value::Text(association.molecule_1_name.clone()),
        );
        parameters.insert(
            "molecule_2_name".to_string(),
            Value::Text(association.molecule_2_name.clone()),
        );
        parameters.insert(
            "molecule_3_name".to_string(),
            Value::Text(association.molecule_3_name.clone()),
        );
        parameters.insert(
            "need_molecule_1".to_string(),
            Value::Int(association.need_molecule_1),
        );
        parameters.insert(
            "need_molecule_2".to_string(),
            Value::Int(association.need_molecule_2),
        );
        parameters.insert("prod".to_string(), Value::Int(association.prod));

        Action::new(action_name, 1, parameters)
    }

    pub fn get_associate_with_catalyze_action(
        association: &CatalyzedAssociationReaction,
    ) -> Action {
        let mut parameters = std::collections::HashMap::new();
        let action_name = format!(
            "associate_with_catalyze_{}_{}_to_{}",
            association.molecule_1_name, association.molecule_2_name, association.molecule_3_name
        );
        parameters.insert(
            "associate_with_catalyze".to_string(),
            Value::Text("associate_with_catalyze".to_string()),
        );
        parameters.insert(
            "molecule_1_name".to_string(),
            Value::Text(association.molecule_1_name.clone()),
        );
        parameters.insert(
            "molecule_3_name".to_string(),
            Value::Text(association.molecule_3_name.clone()),
        );
        parameters.insert(
            "need_molecule_1".to_string(),
            Value::Int(association.need_molecule_1),
        );
        parameters.insert("prod".to_string(), Value::Int(association.prod));

        Action::new(action_name, 1, parameters)
    }

    pub fn get_self_associate_with_catalyze_action(
        association: &CatalyzedSelfAssociationReaction,
    ) -> Action {
        let mut parameters = std::collections::HashMap::new();
        let action_name = format!(
            "self_associate_with_catalyze_{}_to_{}",
            association.molecule_1_name, association.molecule_2_name
        );
        parameters.insert(
            "self_associate_with_catalyze".to_string(),
            Value::Text("self_associate_with_catalyze".to_string()),
        );
        parameters.insert(
            "molecule_1_name".to_string(),
            Value::Text(association.molecule_1_name.clone()),
        );
        parameters.insert(
            "molecule_2_name".to_string(),
            Value::Text(association.molecule_2_name.clone()),
        );
        parameters.insert(
            "need_molecule_1".to_string(),
            Value::Int(association.need_molecule_1),
        );
        parameters.insert("prod".to_string(), Value::Int(association.prod));

        Action::new(action_name, 1, parameters)
    }
    pub fn get_synthesize_action(synthesize: &SynthesisReaction) -> Action {
        let mut parameters = std::collections::HashMap::new();
        let action_name = format!(
            "synthesize_{}_to_{}",
            synthesize.molecule_1_name, synthesize.molecule_2_name
        );
        parameters.insert(
            "synthesize".to_string(),
            Value::Text("synthesize".to_string()),
        );

        parameters.insert(
            "molecule_2_name".to_string(),
            Value::Text(synthesize.molecule_2_name.clone()),
        );
        parameters.insert("prod".to_string(), Value::Int(synthesize.prod));

        Action::new(action_name, 1, parameters)
    }

    pub fn get_choose_initialize_simple_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();

        for simple in &state.simples {
            if simple.possible {
                actions.push(Self::get_choose_simple_action(simple));
            }
            if simple.chosen {
                actions.push(Self::get_initialize_simple_action(simple));
            }
        }
        actions
    }
    pub fn get_associate_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for reaction in &self.association_reactions {
            // Look up molecules by name in state. Here we assume that the first two molecules are of type Simple
            // and the product is a Complex. Depending on your design you might want to support both types.
            let mol1_opt = state
                .simples
                .iter()
                .find(|s| s.name == reaction.molecule_1_name);
            let mol2_opt = state
                .simples
                .iter()
                .find(|s| s.name == reaction.molecule_2_name);

            if let (Some(mol1), Some(mol2)) = (mol1_opt, mol2_opt) {
                // Check preconditions:
                if mol1.available >= reaction.need_molecule_1
                    && mol2.available >= reaction.need_molecule_2
                {
                    actions.push(Self::get_associate_action(reaction));
                }
            }
        }
        actions
    }

    pub fn get_associate_with_catalyze_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for reaction in &self.catalyzed_association_reactions {
            // Look up molecules by name in state. Here we assume that the first two molecules are of type Simple
            // and the product is a Complex. Depending on your design you might want to support both types.
            let mol1_opt = state
                .simples
                .iter()
                .find(|s| s.name == reaction.molecule_1_name);
            let mol2_opt = state
                .simples
                .iter()
                .find(|s| s.name == reaction.molecule_2_name);

            if let (Some(mol1), Some(mol2)) = (mol1_opt, mol2_opt) {
                // Check preconditions:
                if mol1.available >= reaction.need_molecule_1
                    && mol2.available >= reaction.need_molecule_2
                {
                    actions.push(Self::get_associate_with_catalyze_action(reaction));
                }
            }
        }
        actions
    }

    pub fn get_self_associate_with_catalyze_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for reaction in &self.catalyzed_self_association_reactions {
            // Look up molecules by name in state. Here we assume that the first two molecules are of type Simple
            // and the product is a Complex. Depending on your design you might want to support both types.
            let mol1_opt = state
                .simples
                .iter()
                .find(|s| s.name == reaction.molecule_1_name);

            if let Some(mol1) = mol1_opt {
                // Check preconditions:
                if mol1.available >= reaction.need_molecule_1 {
                    actions.push(Self::get_self_associate_with_catalyze_action(reaction));
                }
            }
        }
        actions
    }
    pub fn get_synthesize_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for reaction in &self.synthesis_reactions {
            // Look up molecules by name in state. Here we assume that the first two molecules are of type Simple
            // and the product is a Complex. Depending on your design you might want to support both types.
            let mol1_opt = state
                .simples
                .iter()
                .find(|s| s.name == reaction.molecule_1_name);

            if let Some(mol1) = mol1_opt {
                // Check preconditions:
                if mol1.available >= reaction.need_molecule_1 {
                    actions.push(Self::get_synthesize_action(reaction));
                }
            }
        }
        actions
    }

    pub fn apply_choose_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let simple_name = match action.parameters.get("simple_name") {
            Some(Value::Text(name)) => name,
            _ => panic!("Action parameters do not contain a valid name for simple."),
        };
        if let Some(simple) = new_state
            .simples
            .iter_mut()
            .find(|s| s.name == *simple_name)
        {
            simple.chosen = true;
            simple.possible = false;
            new_state.num_subs += 1;
        } else {
            panic!("Simple with name {} not found", simple_name);
        }
        new_state
    }

    pub fn apply_initialize_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let simple_name = match action.parameters.get("simple_name") {
            Some(Value::Text(name)) => name,
            _ => panic!("Action parameters do not contain a valid name for simple."),
        };
        if let Some(simple) = new_state
            .simples
            .iter_mut()
            .find(|s| s.name == *simple_name)
        {
            simple.available += 1;
        } else {
            panic!("Simple with name {} not found", simple_name);
        }
        new_state
    }

    pub fn apply_associate_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let simple_1_name = match action.parameters.get("molecule_1_name") {
            Some(Value::Text(name)) => name,
            _ => panic!("Action parameters do not contain a valid name for simple_1."),
        };
        let simple_2_name = match action.parameters.get("molecule_2_name") {
            Some(Value::Text(name)) => name,
            _ => panic!("Action parameters do not contain a valid name for simple_2."),
        };
        let complex_name = match action.parameters.get("molecule_3_name") {
            Some(Value::Text(name)) => name,
            _ => panic!("Action parameters do not contain a valid name for complex."),
        };
        let simple_1_need = match action.parameters.get("need_molecule_1") {
            Some(Value::Int(need)) => *need,
            _ => panic!("Action parameters do not contain a valid need for simple_1."),
        };
        let simple_2_need = match action.parameters.get("need_molecule_2") {
            Some(Value::Int(need)) => *need,
            _ => panic!("Action parameters do not contain a valid need for simple_2."),
        };
        let complex_prod = match action.parameters.get("prod") {
            Some(Value::Int(prod)) => *prod,
            _ => panic!("Action parameters do not contain a valid prod for complex."),
        };
        // Get indices for the two simples in the vector.
        let idx1 = new_state
            .simples
            .iter()
            .position(|s| s.name == *simple_1_name)
            .expect(&format!("Simple with name {} not found", simple_1_name));
        let idx2 = new_state
            .simples
            .iter()
            .position(|s| s.name == *simple_2_name)
            .expect(&format!("Simple with name {} not found", simple_2_name));
        // Now get a mutable reference to the complex as before.
        if let Some(complex) = new_state
            .complexes
            .iter_mut()
            .find(|c| c.name == *complex_name)
        {
            new_state.simples[idx1].available -= simple_1_need;
            new_state.simples[idx2].available -= simple_2_need;
            complex.available += complex_prod;
        } else {
            panic!("Complex with name {} not found", complex_name);
        }

        new_state
    }

    pub fn apply_associate_with_catalyze_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let simple_1_name = match action.parameters.get("molecule_1_name") {
            Some(Value::Text(name)) => name,
            _ => panic!("Action parameters do not contain a valid name for simple_1."),
        };
        let complex_name = match action.parameters.get("molecule_3_name") {
            Some(Value::Text(name)) => name,
            _ => panic!("Action parameters do not contain a valid name for complex."),
        };
        let simple_1_need = match action.parameters.get("need_molecule_1") {
            Some(Value::Int(need)) => *need,
            _ => panic!("Action parameters do not contain a valid need for simple_1."),
        };
        let complex_prod = match action.parameters.get("prod") {
            Some(Value::Int(prod)) => *prod,
            _ => panic!("Action parameters do not contain a valid prod for complex."),
        };
        if let (Some(simple_1), Some(complex)) = (
            new_state
                .simples
                .iter_mut()
                .find(|s| s.name == *simple_1_name),
            new_state
                .complexes
                .iter_mut()
                .find(|c| c.name == *complex_name),
        ) {
            simple_1.available -= simple_1_need;
            complex.available += complex_prod;
        } else {
            panic!(
                "Simple_1 with name {}  or complex with name {}  not found",
                simple_1_name, complex_name
            );
        }
        new_state
    }

    pub fn apply_self_associate_with_catalyze_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let simple_1_name = match action.parameters.get("molecule_1_name") {
            Some(Value::Text(name)) => name,
            _ => panic!("Action parameters do not contain a valid name for simple_1."),
        };
        let complex_name = match action.parameters.get("molecule_2_name") {
            Some(Value::Text(name)) => name,
            _ => panic!("Action parameters do not contain a valid name for complex."),
        };
        let simple_1_need = match action.parameters.get("need_molecule_1") {
            Some(Value::Int(need)) => *need,
            _ => panic!("Action parameters do not contain a valid need for simple_1."),
        };
        let complex_prod = match action.parameters.get("prod") {
            Some(Value::Int(prod)) => *prod,
            _ => panic!("Action parameters do not contain a valid prod for complex."),
        };
        if let (Some(simple_1), Some(complex)) = (
            new_state
                .simples
                .iter_mut()
                .find(|s| s.name == *simple_1_name),
            new_state
                .complexes
                .iter_mut()
                .find(|c| c.name == *complex_name),
        ) {
            simple_1.available -= simple_1_need;
            complex.available += complex_prod;
        } else {
            panic!(
                "Simple_1 with name {}  or complex with name {}  not found",
                simple_1_name, complex_name
            );
        }
        new_state
    }

    pub fn apply_synthesize_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let simple_name = match action.parameters.get("molecule_2_name") {
            Some(Value::Text(name)) => name,
            _ => panic!("Action parameters do not contain a valid name for simple."),
        };
        let simple_prod = match action.parameters.get("prod") {
            Some(Value::Int(prod)) => *prod,
            _ => panic!("Action parameters do not contain a valid prod for simple."),
        };
        if let Some(simple) = new_state
            .simples
            .iter_mut()
            .find(|s| s.name == *simple_name)
        {
            simple.available += simple_prod;
        } else {
            panic!("Simple with name {} not found", simple_name);
        }
        new_state
    }
}
impl Problem for PathWaysMetricProblem {
    type State = State;
    fn get_possible_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        actions.extend(self.get_choose_initialize_simple_actions(state));
        actions.extend(self.get_associate_actions(state));
        actions.extend(self.get_associate_with_catalyze_actions(state));
        actions.extend(self.get_self_associate_with_catalyze_actions(state));
        actions.extend(self.get_synthesize_actions(state));
        actions
    }
    fn apply_action(&self, state: &State, action: &Action) -> State {
        if action.name.starts_with("synthesize_") {
            Self::apply_synthesize_action(state, action)
        } else if action.name.starts_with("self_associate_with_catalyze_") {
            Self::apply_self_associate_with_catalyze_action(state, action)
        } else if action.name.starts_with("associate_with_catalyze_") {
            Self::apply_associate_with_catalyze_action(state, action)
        } else if action.name.starts_with("associate_") {
            Self::apply_associate_action(state, action)
        } else if action.name.starts_with("initialize_") {
            Self::apply_initialize_action(state, action)
        } else if action.name.starts_with("choose_") {
            Self::apply_choose_action(state, action)
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
    fn load_state_from_json(json_path: &str) -> (State, PathWaysMetricProblem) {
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
        let problem: PathWaysMetricProblem =
            serde_json::from_value(problem_value.clone()).expect("Failed to deserialize problem");

        (state, problem)
    }
}
