use crate::problems::problem::Problem;
use crate::search::{action::Action, state::StateTrait, state::Value};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::fs;

// State holds positions of trucks, available markets, and purchase history
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    trucks: Vec<Truck>,                 // list of all trucks and their locations
    markets: Vec<Market>,               // list of markets and available items
    items_bought: HashMap<String, i32>, // map: ItemID -> amount already purchased
}

impl State {}
impl StateTrait for State {}

// Represents a truck with a unique name and current location
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Truck {
    name: String,     // unique truck identifier
    location: String, // current location of the truck
}

// Represents a market at a location selling multiple items
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Market {
    location: String,                   // location identifier for this market
    items: HashMap<String, MarketItem>, // map: ItemID -> details of item for sale
}

// Details of an item available in a market
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketItem {
    pub on_sale: i32,             // quantity of this item currently on sale
    pub price: OrderedFloat<f64>, // price per unit of this item
}

// Goal specifies requested quantities for each item
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Goal {
    pub goal_requests: HashMap<String, i32>, // map: ItemID -> total requested quantity
}

// The TppProblem encapsulates travel distances and purchase goals
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TppProblem {
    pub distances: HashMap<String, Vec<(String, OrderedFloat<f64>)>>, // map: location -> [(neighbor, travel_cost)]
    pub goal: Goal,                                                   // purchase goals for items
}

impl TppProblem {
    /// Generate drive actions for each truck to move between connected locations
    pub fn get_drive_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for truck in &state.trucks {
            if let Some(pairs) = self.distances.get(&truck.location) {
                for (to, cost) in pairs {
                    let mut parameters = HashMap::new();
                    let action_name =
                        format!("drive_{}_from{}_to{}", truck.name, truck.location, to);
                    parameters.insert("name".to_string(), Value::Text(truck.name.clone()));
                    parameters.insert("from".to_string(), Value::Text(truck.location.clone()));
                    parameters.insert("to".to_string(), Value::Text(to.clone()));
                    let cost_i32 = (cost.into_inner().round()) as i32;
                    // travel cost converted to integer
                    actions.push(Action::new(action_name, cost_i32, parameters));
                }
            }
        }
        actions
    }

    /// Generate buy actions for trucks at markets where they can purchase needed items
    pub fn get_buy_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();

        for truck in &state.trucks {
            for market in &state.markets {
                if truck.location == market.location {
                    for (item_id, market_item) in &market.items {
                        let on_sale = market_item.on_sale; // available quantity
                        let item_cost = market_item.price; // price per unit
                        let requested = *self.goal.goal_requests.get(item_id).unwrap_or(&0); // goal quantity
                        let already_bought = *state.items_bought.get(item_id).unwrap_or(&0); // purchased so far
                        if on_sale > 0 && requested > already_bought {
                            let from = &market.location; // market location
                            let mut parameters = HashMap::new();
                            parameters.insert("item_id".to_string(), Value::Text(item_id.clone()));
                            parameters.insert("from".to_string(), Value::Text(from.clone()));
                            let remaining_needed = requested - already_bought; // amount still needed
                            let amount_to_buy = if on_sale > remaining_needed {
                                remaining_needed
                            } else {
                                on_sale
                            }; // buy up to remaining need or available
                               // compute action cost: units * price
                            let action_cost = (amount_to_buy as f64) * item_cost.into_inner();
                            let action_name =
                                format!("bought_{}_from{}_amount{}", item_id, from, amount_to_buy);
                            parameters
                                .insert("amount_to_buy".to_string(), Value::Int(amount_to_buy));
                            let cost_i32 = action_cost.round() as i32; // round to integer
                            actions.push(Action::new(action_name, cost_i32, parameters));
                        }
                    }
                }
            }
        }

        actions
    }

    /// Apply a drive action, updating the truck's location
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

        if let Some(truck) = new_state.trucks.iter_mut().find(|t| t.name == *truck_name) {
            truck.location = to.to_string(); // update location
        } else {
            panic!("Truck with name {} not found.", truck_name);
        }

        new_state
    }

    /// Apply a buy action, reducing market stock and increasing purchased count
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

        if let Some(market) = new_state.markets.get_mut(market_index) {
            if let Some(market_item) = market.items.get_mut(item_id) {
                market_item.on_sale -= amount_to_buy; // decrement market stock
            } else {
                panic!("Market does not sell item {}.", item_id);
            }
        } else {
            panic!("Invalid market index {}.", market_index);
        }

        if let Some(count) = new_state.items_bought.get_mut(item_id) {
            *count += amount_to_buy; // increment bought count
        } else {
            panic!("Item id {} does not exist in items_bought.", item_id);
        }

        new_state
    }
}

// Problem trait implementation for TppProblem
impl Problem for TppProblem {
    type State = State;

    /// Gather all possible drive and buy actions
    fn get_possible_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        actions.extend(self.get_drive_actions(state));
        actions.extend(self.get_buy_actions(state));
        actions
    }

    /// Dispatch to appropriate apply_* based on action name prefix
    fn apply_action(&self, state: &State, action: &Action) -> State {
        if action.name.starts_with("bought_") {
            Self::apply_buy_action(state, action)
        } else if action.name.starts_with("drive_") {
            Self::apply_drive_action(state, action)
        } else {
            panic!("Unknown action type: {}", action.name)
        }
    }

    /// Check goal: all trucks at depot (-1) and all requests fulfilled
    fn is_goal_state(&self, state: &State) -> bool {
        for truck in &state.trucks {
            if truck.location != "-1" {
                return false;
            } // depot coded as "-1"
        }
        for (item_id, request) in &self.goal.goal_requests {
            let owned = state.items_bought.get(item_id).unwrap_or(&0);
            if *owned < *request {
                return false;
            } // not enough purchased
        }
        true
    }

    fn heuristic(&self, _state: &State) -> f64 {
        0.0 // placeholder heuristic
    }

    /// Load state and problem from JSON file
    fn load_state_from_json(json_path: &str) -> (State, TppProblem) {
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
        let problem: TppProblem =
            serde_json::from_value(problem_value.clone()).expect("Failed to deserialize problem");
        (state, problem)
    }
}
