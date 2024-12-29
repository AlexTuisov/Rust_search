use crate::search::{state::StateTrait, state::Value, action::Action};
use serde_json::from_reader;
use serde_json::Value as JsonValue;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::BufReader;
use serde::{Deserialize, Serialize};
use crate::problems::problem::Problem;

include!("refined_heuristic.in");


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub bought: BTreeMap<String, i64>, // Item bought count per type
    pub camels: Vec<String>,           // List of camel identifiers
    pub drive_cost: BTreeMap<String, i64>, // Drive cost between locations
    pub goods: Vec<String>,            // List of goods available
    pub location: BTreeMap<String, String>, // Camel-to-location mapping
    pub markets: Vec<String>,          // List of markets
    pub on_sale: BTreeMap<String, BTreeMap<String, i64>>, // Goods available for sale in each market
    pub prices: BTreeMap<String, BTreeMap<String, i64>>,  // Prices of goods in each market
    pub resources: Resources,          // Encapsulated resource-related fields
}

impl State {
    pub fn can_drive(&self, from: &str, to: &str) -> bool {
        let key = format!("{}_{}", from, to);
        self.drive_cost.contains_key(&key)
    }

    pub fn drive_cost(&self, from: &str, to: &str) -> Option<f64> {
        let key = format!("{}_{}", from, to);
        self.drive_cost.get(&key).map(|cost| *cost as f64 / self.resources.scale_factor as f64)
    }

    pub fn is_camel_at(&self, camel: &str, location: &str) -> bool {
        self.location.get(camel).map_or(false, |camel_location| camel_location == location)
    }

    pub fn on_sale(&self, item: &str, market: &str) -> Option<i64> {
        self.on_sale.get(market).and_then(|market_sales| market_sales.get(item).copied())
    }

    pub fn price(&self, goods: &str, market: &str) -> Option<f64> {
        self.prices.get(market).and_then(|market_prices| {
            market_prices.get(goods).map(|price| *price as f64 / self.resources.scale_factor as f64)
        })
    }

    pub fn bought(&self, item: &str) -> Option<i64> {
        self.bought.get(item).copied()
    }

    pub fn cash(&self) -> f64 {
        self.resources.get_cash()
    }
}

impl StateTrait for State {

}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Resources {
    pub capacity: i64,    // Total capacity of the trader
    pub cash_scaled: i64,        // Available cash (scaled if necessary for decimals)
    pub fuel_scaled: i64,        // Fuel available (scaled if necessary)
    pub fuel_used_scaled: i64,   // Fuel already used
    pub scale_factor: i64
}

impl Resources {
    pub fn new(cash: f64, fuel: f64, fuel_used: f64, capacity: i64, scale_factor: i64) -> Self {
        Resources {
            cash_scaled: (cash * scale_factor as f64).round() as i64,
            fuel_scaled: (fuel * scale_factor as f64).round() as i64,
            fuel_used_scaled: (fuel_used * scale_factor as f64).round() as i64,
            capacity,
            scale_factor,
        }
    }

    // Getters
    pub fn get_cash(&self) -> f64 {
        self.cash_scaled as f64 / self.scale_factor as f64
    }

    pub fn get_fuel(&self) -> f64 {
        self.fuel_scaled as f64 / self.scale_factor as f64
    }

    pub fn get_fuel_used(&self) -> f64 {
        self.fuel_used_scaled as f64 / self.scale_factor as f64
    }

    // Setters
    pub fn set_cash(&mut self, cash: f64) {
        self.cash_scaled = (cash * self.scale_factor as f64).round() as i64;
    }

    pub fn set_fuel(&mut self, fuel: f64) {
        self.fuel_scaled = (fuel * self.scale_factor as f64).round() as i64;
    }

    pub fn set_fuel_used(&mut self, fuel_used: f64) {
        self.fuel_used_scaled = (fuel_used * self.scale_factor as f64).round() as i64;
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct AtomicGoal {
    pub parameter: String,
    pub value: Value,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Goal {
    pub conditions: Vec<AtomicGoal>,
}

impl Goal {
    pub fn new() -> Self {
        Goal { conditions: Vec::new() }
    }
}


pub struct MarketTraderProblem {
    pub goal: Goal,
}



impl MarketTraderProblem {

    pub fn possible_travel_actions(state: &State) -> Vec<Action> {
        let mut actions = Vec::new();

        // Ensure required data exists in the state
        if state.resources.get_cash() <= 0.0 {
            return actions; // No cash, no travel
        }

        for camel in &state.camels {
            for from in &state.markets {
                for to in &state.markets {
                    if from != to
                        && state.can_drive(from, to)
                        && state.resources.get_cash() >= state.drive_cost(from, to).unwrap_or(f64::INFINITY)
                        && state.is_camel_at(camel, from)
                    {
                        if let Some(cost) = state.drive_cost(from, to) {
                            let mut parameters = HashMap::new();
                            parameters.insert("camel".to_string(), Value::Text(camel.clone()));
                            parameters.insert("from".to_string(), Value::Text(from.clone()));
                            parameters.insert("to".to_string(), Value::Text(to.clone()));

                            // Format the action name as "travel_{from}_{to}_{camel}"
                            let action_name = format!("travel_{}_{}_{}", from, to, camel);

                            actions.push(Action::new(
                                action_name,
                                0, // Cost scaled using the state scaling logic
                                parameters,
                            ));
                        }
                    }
                }
            }
        }

        actions
    }


    pub fn possible_buy_actions(state: &State) -> Vec<Action> {
        let mut actions = Vec::new();

        // Ensure there is enough cash and capacity to consider buying
        if state.resources.get_cash() <= 7.0 || state.resources.capacity <= 0 {
            return actions;
        }

        for camel in &state.camels {
            for market in &state.markets {
                if state.is_camel_at(camel, market) {
                    for goods in &state.goods {
                        if let Some(price) = state.price(goods, market) {
                            let total_cost = price + 7.0;
                            if state.resources.get_cash() >= total_cost {
                                if let Some(on_sale) = state.on_sale(goods, market) {
                                    if on_sale > 0 {
                                        let mut parameters = HashMap::new();
                                        parameters.insert("camel".to_string(), Value::Text(camel.clone()));
                                        parameters.insert("goods".to_string(), Value::Text(goods.clone()));
                                        parameters.insert("market".to_string(), Value::Text(market.clone()));

                                        // Format the action name as "buy_{item}_{market}_{camel}"
                                        let action_name = format!("buy_{}_{}_{}", goods, market, camel);

                                        actions.push(Action::new(
                                            action_name,
                                            0, // Use scaling for cost
                                            parameters,
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        actions
    }


    pub fn possible_upgrade_actions(state: &State) -> Vec<Action> {
        let mut actions = Vec::new();

        // Check if there is enough cash for an upgrade
        if state.resources.get_cash() < 57.0 {
            return actions;
        }

        for camel in &state.camels {
            let mut parameters = HashMap::new();
            parameters.insert("camel".to_string(), Value::Text(camel.clone()));

            // Format the action name as "upgrade_{camel}"
            let action_name = format!("upgrade_{}", camel);

            actions.push(Action::new(
                action_name,
                0, // Fixed cost scaled for the upgrade action
                parameters,
            ));
        }

        actions
    }


    pub fn possible_sell_actions(state: &State) -> Vec<Action> {
        let mut actions = Vec::new();

        for camel in &state.camels {
            for market in &state.markets {
                if state.is_camel_at(camel, market) {
                    for item in &state.goods {
                        if let Some(bought_quantity) = state.bought.get(item) {
                            if *bought_quantity >= 1 {
                                let mut parameters = HashMap::new();
                                parameters.insert("camel".to_string(), Value::Text(camel.clone()));
                                parameters.insert("goods".to_string(), Value::Text(item.clone()));
                                parameters.insert("market".to_string(), Value::Text(market.clone()));

                                // Format the action name as "sell_{item}_{market}_{camel}"
                                let action_name = format!("sell_{}_{}_{}", item, market, camel);

                                actions.push(Action::new(
                                    action_name,
                                    0, // No specific cost for the sell action
                                    parameters,
                                ));
                            }
                        }
                    }
                }
            }
        }

        actions
    }


    //////////////////////////------------------------------------------

    pub fn apply_travel_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();

        // Retrieve parameters from the action
        if let (Some(Value::Text(from)), Some(Value::Text(to)), Some(Value::Text(camel))) =
            (action.parameters.get("from"), action.parameters.get("to"), action.parameters.get("camel"))
        {
            // Update cash by decreasing it by the drive cost
            if let Some(drive_cost) = state.drive_cost(from, to) {
                new_state.resources.set_cash(new_state.resources.get_cash()-drive_cost);
            }

            // Update the camel's location
            if let Some(location) = new_state.location.get_mut(camel) {
                *location = to.clone();
            }
        }

        new_state
    }


    pub fn apply_buy_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();

        // Retrieve parameters from the action
        if let (Some(Value::Text(camel)), Some(Value::Text(goods)), Some(Value::Text(market))) =
            (action.parameters.get("camel"), action.parameters.get("goods"), action.parameters.get("market"))
        {
            // Decrease capacity by 1
            new_state.resources.capacity -= 1;

            // Update the bought quantity for the goods
            *new_state.bought.entry(goods.clone()).or_insert(0) += 1;

            // Decrease cash by the price of the goods at the market
            if let Some(price) = state.price(goods, market) {
                new_state.resources.set_cash(new_state.resources.get_cash() - price);
            }
        }

        new_state
    }


    pub fn apply_upgrade_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();

        // Retrieve parameters from the action
        if let Some(Value::Text(_camel)) = action.parameters.get("camel") {
            // Increase capacity by 20
            new_state.resources.capacity += 20;

            // Decrease cash by 50
            new_state.resources.set_cash(new_state.resources.get_cash() - 50.0);
        }

        new_state
    }


    pub fn apply_sell_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();

        // Retrieve parameters from the action
        if let (Some(Value::Text(_camel)), Some(Value::Text(goods)), Some(Value::Text(market))) =
            (action.parameters.get("camel"), action.parameters.get("goods"), action.parameters.get("market"))
        {
            // Increase capacity by 1
            new_state.resources.capacity += 1;

            // Decrease the bought quantity of the goods item by 1
            if let Some(bought_quantity) = new_state.bought.get_mut(goods) {
                *bought_quantity -= 1;
            }

            // Increase cash by the price of the goods at the market
            if let Some(price) = state.price(goods, market) {
                new_state.resources.set_cash(new_state.resources.get_cash() + price);
            }
        }

        new_state
    }


    //     fn is_condition_met(state_value: &Value, goal_value: &Value) -> bool {
    //         match (state_value, goal_value) {
    //             // Integer comparison
    //             (Value::Int(state_int), Value::Int(goal_int)) => state_int >= goal_int,
    //
    //             // String equality or float comparison
    //             (Value::Text(state_str), Value::Text(goal_str)) => {
    //                 // Attempt to parse both strings as floats for comparison
    //                 if let (Ok(state_float), Ok(goal_float)) = (state_str.parse::<f64>(), goal_str.parse::<f64>()) {
    //                     state_float >= goal_float
    //                 } else {
    //                     state_str == goal_str // Fallback to string equality if parsing fails
    //                 }
    //             }
    //
    //             // Boolean equality
    //             (Value::Bool(state_bool), Value::Bool(goal_bool)) => state_bool == goal_bool,
    //
    //             // Array and map comparisons
    //             (Value::IntArray(state_array), Value::IntArray(goal_array)) => state_array == goal_array,
    //             (Value::StringArray(state_array), Value::StringArray(goal_array)) => state_array == goal_array,
    //             (Value::MapToInt(state_map), Value::MapToInt(goal_map)) => state_map == goal_map,
    //             (Value::MapToString(state_map), Value::MapToString(goal_map)) => state_map == goal_map,
    //             (Value::MapToBool(state_map), Value::MapToBool(goal_map)) => state_map == goal_map,
    //             (Value::MapToValue(state_map), Value::MapToValue(goal_map)) => state_map == goal_map,
    //             (Value::MapToMapToInt(state_map), Value::MapToMapToInt(goal_map)) => state_map == goal_map,
    //
    //             _ => false, // Mismatched types or unsupported comparison
    //         }
    //     }
    // }
}


impl Problem for MarketTraderProblem {
    fn get_possible_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        // Collect possible actions from each helper function
        actions.extend(Self::possible_travel_actions(state));
        actions.extend(Self::possible_buy_actions(state));
        actions.extend(Self::possible_sell_actions(state));
        actions.extend(Self::possible_upgrade_actions(state));

        actions
    }

    fn apply_action(&self, state: &State, action: &Action) -> State {
        // Parse the action name to determine which apply function to call
        if action.name.starts_with("travel_") {
            Self::apply_travel_action(state, action)
        } else if action.name.starts_with("buy_") {
            Self::apply_buy_action(state, action)
        } else if action.name.starts_with("sell_") {
            Self::apply_sell_action(state, action)
        } else if action.name.starts_with("upgrade_") {
            Self::apply_upgrade_action(state, action)
        } else {
            panic!("Unknown action type: {}", action.name)
        }
    }


    fn is_goal_state(&self, state: &State) -> bool {
        // Retrieve the goal cash threshold from the goal structure
        if let Some(atomic_goal) = self.goal.conditions.iter().find(|g| g.parameter == "cash") {
            // Retrieve the cash value from the state
            let state_cash = state.resources.get_cash();

            // Handle goal cash as either Int or Text
            match &atomic_goal.value {
                Value::Int(goal_cash) => state_cash >= *goal_cash as f64,
                Value::Text(goal_cash_str) => {
                    if let Ok(goal_cash) = goal_cash_str.parse::<i64>() {
                        state_cash >= goal_cash as f64
                    } else {
                        false // Invalid goal cash format
                    }
                }
                _ => false, // Unsupported goal value type
            }
        } else {
            false // No cash condition in the goal
        }
    }



    fn load_state_from_json(json_path: &str) -> (State, Self) {
        let file = File::open(json_path).expect("Failed to open JSON file");
        let reader = BufReader::new(file);
        let json_data: JsonValue = from_reader(reader).expect("Failed to parse JSON");

        let mut goal = Goal::new();

        // Extract data from the JSON
        let camels = json_data["init"]["camels"]
            .as_array()
            .expect("Missing or invalid 'camels'")
            .iter()
            .map(|v| v.as_str().expect("Invalid camel entry").to_string())
            .collect();

        let goods = json_data["init"]["goods"]
            .as_array()
            .expect("Missing or invalid 'goods'")
            .iter()
            .map(|v| v.as_str().expect("Invalid goods entry").to_string())
            .collect();

        let markets = json_data["init"]["markets"]
            .as_array()
            .expect("Missing or invalid 'markets'")
            .iter()
            .map(|v| v.as_str().expect("Invalid market entry").to_string())
            .collect();

        let drive_cost = json_data["init"]["drive_cost"]
            .as_object()
            .expect("Missing or invalid 'drive_cost'")
            .iter()
            .map(|(key, value)| {
                let scaled_cost = (value.as_f64().expect("Invalid drive cost") * 100.0).round() as i64;
                (key.clone(), scaled_cost)
            })
            .collect();

        let location = json_data["init"]["location"]
            .as_object()
            .expect("Missing or invalid 'location'")
            .iter()
            .map(|(camel, loc)| {
                let location_str = loc.as_str().expect("Invalid location entry").to_string();
                (camel.clone(), location_str)
            })
            .collect();

        let on_sale = json_data["init"]["on_sale"]
            .as_object()
            .expect("Missing or invalid 'on_sale'")
            .iter()
            .map(|(market, goods)| {
                let goods_map = goods
                    .as_object()
                    .expect("Invalid on_sale entry")
                    .iter()
                    .map(|(good, quantity)| {
                        let qty = quantity.as_i64().expect("Invalid quantity");
                        (good.clone(), qty)
                    })
                    .collect();
                (market.clone(), goods_map)
            })
            .collect();

        let prices = json_data["init"]["prices"]
            .as_object()
            .expect("Missing or invalid 'prices'")
            .iter()
            .map(|(market, goods)| {
                let goods_map = goods
                    .as_object()
                    .expect("Invalid prices entry")
                    .iter()
                    .map(|(good, price)| {
                        let scaled_price = (price.as_f64().expect("Invalid price") * 100.0).round() as i64;
                        (good.clone(), scaled_price)
                    })
                    .collect();
                (market.clone(), goods_map)
            })
            .collect();

        let bought = json_data["init"]["bought"]
            .as_object()
            .expect("Missing or invalid 'bought'")
            .iter()
            .map(|(item, quantity)| {
                let qty = quantity.as_i64().expect("Invalid bought quantity");
                (item.clone(), qty)
            })
            .collect();

        let resources = {
            let resources_obj = json_data["init"]["resources"]
                .as_object()
                .expect("Missing or invalid 'resources'");
            Resources::new(
                resources_obj["cash"].as_f64().expect("Invalid cash"),
                resources_obj["fuel"].as_f64().expect("Invalid fuel"),
                resources_obj["fuel_used"].as_f64().expect("Invalid fuel_used"),
                resources_obj["capacity"].as_i64().expect("Invalid capacity"),
                100, // Assume scale factor is 100
            )
        };

        // Populate the goal structure
        if let Some(goal_map) = json_data["goal"].as_object() {
            for (param, threshold) in goal_map {
                let goal_value = match threshold {
                    JsonValue::Number(n) => Value::Int(n.as_i64().expect("Invalid goal threshold") as i32),
                    JsonValue::String(s) => Value::Text(s.clone()),
                    JsonValue::Bool(b) => Value::Bool(*b),
                    _ => panic!("Invalid goal value"),
                };
                goal.conditions.push(AtomicGoal {
                    parameter: param.clone(),
                    value: goal_value,
                });
            }
        }

        let state = State {
            camels,
            goods,
            markets,
            drive_cost,
            location,
            on_sale,
            prices,
            bought,
            resources,
        };

        (state, MarketTraderProblem { goal })
    }


    fn heuristic(&self, state: &State) -> f64 {
        // heuristic is imported during build time from include!("refined_heuristic.in")
        heuristic(self, state)
    }

    type State = State;
}