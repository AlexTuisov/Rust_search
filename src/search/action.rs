use std::collections::{BTreeMap, HashMap};
use super::state::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Action {
    pub name: String,
    pub cost: i32,
    pub parameters: HashMap<String, Value>,
}

impl Action {
    pub fn new(name: String, cost: i32, parameters: HashMap<String, Value>) -> Self {
        Action {name, cost, parameters}
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_action_with_parameters() {
        let mut parameters = HashMap::new();
        parameters.insert("speed".to_string(), Value::Int(10));
        parameters.insert("direction".to_string(), Value::Text("north".to_string()));

        let action = Action::new("move".to_string(), 5, parameters);

        assert_eq!(action.name, "move");
        assert_eq!(action.cost, 5);
        assert_eq!(action.parameters.get("speed"), Some(&Value::Int(10)));
        assert_eq!(action.parameters.get("direction"), Some(&Value::Text("north".to_string())));
    }
}


