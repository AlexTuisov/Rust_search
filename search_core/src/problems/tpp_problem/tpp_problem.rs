use crate::problems::problem::Problem;
use crate::search::{action::Action, state::StateTrait, state::Value};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::fs;

// Each State have airplanes, markets, and total cost
pub struct State {
    trucks: Vec<Truck>,
    markets: Vec<Market>,
    items_boughts: HashMap<i32, i32>, //ItemID -> Amount of already bought
    total_cost: i32,
}

impl State {}
impl StateTrait for State {}

pub struct Truck {
    name: String,
    location: i32,
}

pub struct Market {
    location: i32,
    items: HashMap<i32, (i32, i32)>, // itemID -> (on-sale amount, cost)
}

pub struct Goal {
    pub conditions: HashMap<i32, i32>, // itemID -> requested amount
}

pub struct TppProblem {
    pub distances: HashMap<(i32, i32), i32>, // (from,to) -> cost
    pub goal: Goal,
}

impl TppProblem {
    pub fn get_drive_action(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for to in 0..state.markets.len() {
            for truck in &state.trucks {
                let from = truck.location;
                if let Some(&action_cost) = self.distances.get(&(from, to)) {
                    let mut parameters = std::collections::HashMap::new();
                    let action_name = format!("drive_{}_from{}_to{}", truck.name, from, to);
                    parameters.insert("name".to_string(), Value::Text(truck.name.clone()));
                    parameters.insert("from".to_string(), Value::Int(from));
                    parameters.insert("to".to_string(), Value::Int(to));
                    parameters.insert("action_cost".to_string(), Value::Int(action_cost));

                    actions.push(Action::new(action_name, action_cost, parameters));
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
                    for (item_id, (on_sale, item_cost)) in &market.items {
                        let requested = *self.goal.conditions.get(item_id).unwrap_or(&0);
                        let already_bought = *state.items_boughts.get(item_id).unwrap_or(&0);
                        if on_sale > 0 && requested > already_bought {
                            let from = market.location;
                            let mut parameters = std::collections::HashMap::new();
                            parameters.insert("item_id".to_string(), Value::Int(item_id));
                            parameters.insert("from".to_string(), Value::Int(from));
                            let remaining_needed = requested - already_bought;
                            let amount_to_buy = if *on_sale > remaining_needed {
                                remaining_needed
                            } else {
                                *on_sale
                            };
                            let action_cost = amount_to_buy * item_cost;
                            let action_name =
                                format!("bought_{}_from{}_amount{}", item_id, from, amount_to_buy);
                            parameters
                                .insert("amount_to_buy".to_string(), Value::Int(amount_to_buy));
                            parameters.insert("action_cost".to_string(), Value::Int(action_cost));

                            actions.push(Action::new(action_name, action_cost, parameters));
                        }
                    }
                }
            }
        }

        actions
    }

    pub fn apply_drive_action(state: &State, action: &Action) -> State {
        let new_state = state.clone();
        let truck_name = match action.parameters.get("name") {
            Some(Value::Text(name)) => *name,
            _ => panic!("Action parameters do not contain a valid truck name."),
        };
        let to = match action.parameters.get("to") {
            Some(Value::Int(to)) => *to,
            _ => panic!("Action parameters do not contain a valid market location."),
        };
        let action_cost = match action.parameters.get("action_cost") {
            Some(Value::Int(action_cost)) => *action_cost,
            _ => panic!("Action parameters do not contain a valid action cost."),
        };

        if let Some(truck) = new_state.trucks.iter_mut().find(|t| t.name == truck_name) {
            truck.location = to;
            new_state.total_cost += action_cost;
        } else {
            panic!("Truck with name {} not found.", truck_name);
        }

        new_state
    }
    pub fn apply_buy_action(state: &State, action: &Action) -> State {
        let new_state = state.clone();
        let market_index = match action.parameters.get("from") {
            Some(Value::Int(i)) => *i as usize,
            _ => panic!("Action parameters do not contain a valid market location."),
        };
        let item_index = match action.parameters.get("item_id") {
            Some(Value::Int(i)) => *i,
            _ => panic!("Action parameters do not contain a valid item index."),
        };

        let amount_to_buy = match action.parameters.get("amount_to_buy") {
            Some(Value::Int(i)) => *i,
            _ => panic!("Action parameters do not contain a valid amount to buy."),
        };
        let action_cost = match action.parameters.get("action_cost") {
            Some(Value::Int(action_cost)) => *action_cost,
            _ => panic!("Action parameters do not contain a valid action cost."),
        };

        if let Some(market) = new_state.markets.get_mut(market_index) {
            if let Some((on_sale, _cost)) = market.items.get_mut(&item_id) {
                // Reduce the available amount in the market
                *on_sale -= amount_to_buy;
            } else {
                panic!("Market does not sell item {}.", item_id);
            }
        } else {
            panic!("Invalid market index {}.", market_index);
        }

        if let Some(count) = new_state.items_boughts.get_mut(&item_id) {
            *count += amount_to_buy;
        } else {
            // Key doesn't exist. You can choose to panic, log a warning, or do nothing.
            // For example, to panic:
            panic!("Item id {} does not exist in items_boughts.", item_id);
        }

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
            if truck.location != -1 {
                return false;
            }
        }
        for &(item_id, bought) in &state.items {
            if self.goal.get(item_id) > bought {
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

    fn load_state_from_json(json_path: &str) -> (State, TppProblem) {}
}
