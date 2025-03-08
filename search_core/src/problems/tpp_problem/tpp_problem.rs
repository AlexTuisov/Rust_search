use crate::problems::problem::Problem;
use crate::search::{action::Action, state::StateTrait, state::Value};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::fs;

// Each State have airplanes, markets, and total cost
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    trucks: Vec<Truck>,
    markets: Vec<Market>,
    items_bought: HashMap<String, i32>, //ItemID -> Amount of already bought
    total_cost: OrderedFloat<f64>,
}

impl State {}
impl StateTrait for State {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Truck {
    name: String,
    location: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Market {
    location: String,
    items: HashMap<String, MarketItem>, // itemID -> MarketItem
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketItem {
    pub on_sale: i32,
    pub price: OrderedFloat<f64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Goal {
    pub goal_requests: HashMap<String, i32>, // itemID -> requested amount
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LocationPair {
    pub from: String,
    pub to: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TppProblem {
    pub distances: HashMap<LocationPair, OrderedFloat<f64>>, // (from,to) -> cost
    pub goal: Goal,
}

impl TppProblem {
    pub fn get_drive_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for to in -1..(state.markets.len() as i32) {
            for truck in &state.trucks {
                let from = &truck.location;
                // Create a LocationPair key for lookup
                let key = LocationPair {
                    from: from.clone(),
                    to: to.to_string(),
                };
                if let Some(&action_cost) = self.distances.get(&key) {
                    let mut parameters = std::collections::HashMap::new();
                    let action_name = format!("drive_{}_from{}_to{}", truck.name, from, to);
                    parameters.insert("name".to_string(), Value::Text(truck.name.clone()));
                    parameters.insert("from".to_string(), Value::Text(from.clone()));
                    parameters.insert("to".to_string(), Value::Text(to.to_string()));
                    parameters.insert(
                        "action_cost".to_string(),
                        Value::OrderedFloat64(action_cost),
                    );
                    let cost_i32 = (action_cost.into_inner().round()) as i32;
                    actions.push(Action::new(action_name, cost_i32, parameters));
                }
            }
        }
        actions
    }
    pub fn get_buy_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();

        for truck in &state.trucks {
            for market in &state.markets {
                if truck.location == market.location {
                    for (item_id, market_item) in &market.items {
                        let on_sale = market_item.on_sale;
                        let item_cost = market_item.price;
                        let requested = *self.goal.goal_requests.get(item_id).unwrap_or(&0);
                        let already_bought = *state.items_bought.get(item_id).unwrap_or(&0);
                        if on_sale > 0 && requested > already_bought {
                            let from = &market.location;
                            let mut parameters = std::collections::HashMap::new();
                            parameters.insert("item_id".to_string(), Value::Text(item_id.clone()));
                            parameters.insert("from".to_string(), Value::Text(from.clone()));
                            let remaining_needed = requested - already_bought;
                            let amount_to_buy = if on_sale > remaining_needed {
                                remaining_needed
                            } else {
                                on_sale
                            };
                            // Multiply the amount to buy (as f64) by the inner f64 value from the OrderedFloat.
                            let action_cost = (amount_to_buy as f64) * item_cost.into_inner();
                            let action_name =
                                format!("bought_{}_from{}_amount{}", item_id, from, amount_to_buy);
                            parameters
                                .insert("amount_to_buy".to_string(), Value::Int(amount_to_buy));
                            parameters.insert(
                                "action_cost".to_string(),
                                Value::OrderedFloat64(OrderedFloat(action_cost)),
                            );
                            let cost_i32 = action_cost.round() as i32;
                            actions.push(Action::new(action_name, cost_i32, parameters));
                        }
                    }
                }
            }
        }

        actions
    }

    pub fn apply_drive_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let truck_name = match action.parameters.get("name") {
            Some(Value::Text(name)) => name,
            _ => panic!("Action parameters do not contain a valid truck name."),
        };
        let to = match action.parameters.get("to") {
            Some(Value::Text(to)) => to,
            _ => panic!("Action parameters do not contain a valid market location."),
        };
        let action_cost = match action.parameters.get("action_cost") {
            Some(&Value::OrderedFloat64(ref action_cost)) => action_cost.clone().into_inner(),
            _ => panic!("Action parameters do not contain a valid action cost."),
        };

        if let Some(truck) = new_state.trucks.iter_mut().find(|t| t.name == *truck_name) {
            truck.location = to.to_string();
            new_state.total_cost += action_cost;
        } else {
            panic!("Truck with name {} not found.", truck_name);
        }

        new_state
    }
    pub fn apply_buy_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let market_index = match action.parameters.get("from") {
            Some(Value::Text(name)) => name.parse::<usize>().expect("Invalid market index format"),
            _ => panic!("Action parameters do not contain a valid market location."),
        };
        let item_id = match action.parameters.get("item_id") {
            Some(Value::Text(item_id)) => item_id,
            _ => panic!("Action parameters do not contain a valid item index."),
        };

        let amount_to_buy = match action.parameters.get("amount_to_buy") {
            Some(Value::Int(i)) => *i,
            _ => panic!("Action parameters do not contain a valid amount to buy."),
        };
        let action_cost = match action.parameters.get("action_cost") {
            Some(&Value::OrderedFloat64(ref action_cost)) => action_cost.clone().into_inner(),
            _ => panic!("Action parameters do not contain a valid action cost."),
        };

        if let Some(market) = new_state.markets.get_mut(market_index) {
            if let Some(market_item) = market.items.get_mut(item_id.as_str()) {
                // Reduce the available amount in the market
                market_item.on_sale -= amount_to_buy;
            } else {
                panic!("Market does not sell item {}.", item_id);
            }
        } else {
            panic!("Invalid market index {}.", market_index);
        }

        if let Some(count) = new_state.items_bought.get_mut(item_id.as_str()) {
            *count += amount_to_buy;
        } else {
            // Key doesn't exist. You can choose to panic, log a warning, or do nothing.
            // For example, to panic:
            panic!("Item id {} does not exist in items_bought.", item_id);
        }
        new_state.total_cost += action_cost;

        new_state
    }
}

// Note for the user, in your jsonfile in the distance the depot need to be repesends as -1
impl Problem for TppProblem {
    type State = State;

    fn get_possible_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        actions.extend(self.get_drive_actions(state));
        actions.extend(self.get_buy_actions(state));
        
        actions
    }

    fn apply_action(&self, state: &State, action: &Action) -> State {
        // Parse the action name to determine which apply function to call
        if action.name.starts_with("bought_") {
            Self::apply_buy_action(state, action)
        } else if action.name.starts_with("drive_") {
            Self::apply_drive_action(state, action)
        } else {
            panic!("Unknown action type: {}", action.name)
        }
    }

    fn is_goal_state(&self, state: &State) -> bool {
        for truck in &state.trucks {
            if truck.location != "-1" {
               
                return false;
            }
        }
        for (item_id, bought) in &state.items_bought {
          
            if self.goal.goal_requests.get(item_id) > Some(bought) {
                return false;
            }
        }
        true
    }

    fn heuristic(&self, state: &State) -> f64 {
        // heuristic is imported during build time from include!("refined_heuristic.in")
        //heuristic(self, state)
        0.0
    }

    fn load_state_from_json(json_path: &str) -> (State, TppProblem) {
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

        //Problem in json to get key with 2 parametrs/ object so we convert it
        let distances_json = problem_value.get("distances").expect("Missing distances");
        let mut distances: HashMap<LocationPair, OrderedFloat<f64>> = HashMap::new();
        if let Some(dist_obj) = distances_json.as_object() {
            for (key, value) in dist_obj {
                // The key is a string like "0,1". Split it.
                let parts: Vec<&str> = key.split(',').collect();
                if parts.len() == 2 {
                    let from = parts[0]
                        .trim()
                        .parse::<i32>()
                        .expect("Failed to parse 'from' index");
                    let to = parts[1]
                        .trim()
                        .parse::<i32>()
                        .expect("Failed to parse 'to' index");
                    let d = value.as_f64().expect("Distance not a number");
                    let key = LocationPair { from: from.to_string(), to: to.to_string() };
                    distances.insert(key, OrderedFloat(d));
                }
            }
        }
        let goal: Goal  =  serde_json::from_value(problem_value.get("goal").expect("Missing goal").clone()).expect("Missing goal");

        let problem = TppProblem { distances, goal };
    
        (state, problem)
    }
}
