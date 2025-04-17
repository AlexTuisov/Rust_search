use crate::problems::problem::Problem;
use crate::search::{action::Action, state::StateTrait, state::Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use serde_json::Value as JsonValue;


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Place {
    pub id: String,
    pub available_resources: HashMap<String, i32>, // Resource -> Amount
    pub carts: i32,
    pub housing: i32,
    pub has_cabin: bool,
    pub has_coal_stack: bool,
    pub has_quarry: bool,
    pub has_mine: bool,
    pub has_sawmill: bool,
    pub has_ironworks: bool,
    pub has_docks: bool,
    pub has_wharf: bool,
    pub is_woodland: bool,
    pub is_mountain: bool,
    pub is_metalliferous: bool,
    pub is_by_coast: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Vehicle {
    pub id: String,
    pub is_train: bool,
    pub is_ship: bool,
    pub space_in: i32,
    pub available_resources: HashMap<String, i32>, // Resource -> Amount
    pub location: String, // Place ID
    pub potential: bool,  // Whether the vehicle is still potential for building
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub places: HashMap<String, Place>, // Place ID -> Place
    pub vehicles: HashMap<String, Vehicle>, // Vehicle ID -> Vehicle
    pub labour: i32,
    pub resource_use: i32,
    pub pollution: i32,
    pub connections_by_rail: HashMap<String, Vec<String>>,
}

impl StateTrait for State {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Goal {
    pub conditions: Vec<Condition>,
}

impl Goal {
    pub fn is_goal_state(&self, state: &State) -> bool {
        self.conditions.iter().all(|cond| cond.is_satisfied(state))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Condition {
    // Resource condition fields
    #[serde(default)]
    pub place_id: String,
    #[serde(default)]
    pub resource: String,
    #[serde(default)]
    pub required_amount: i32,
    
    // Attribute condition fields
    #[serde(default)]
    pub attribute: String,
    
    // Predicate condition fields
    #[serde(default)]
    pub predicate: String,
    #[serde(default)]
    pub argument: String,
    
    // Connected-by-rail condition fields
    #[serde(default)]
    pub from: String,
    #[serde(default)]
    pub to: String,
}

impl Condition {
    pub fn is_satisfied(&self, state: &State) -> bool {
        // Check if this is a resource condition
        if !self.place_id.is_empty() && !self.resource.is_empty() && self.required_amount > 0 {
            if let Some(place) = state.places.get(&self.place_id) {
                return place.available_resources.get(&self.resource).unwrap_or(&0) >= &self.required_amount;
            }
            return false;
        }
        
        // Check if this is an attribute condition
        if !self.attribute.is_empty() && !self.place_id.is_empty() && self.required_amount > 0 {
            if let Some(place) = state.places.get(&self.place_id) {
                match self.attribute.as_str() {
                    "housing" => return place.housing >= self.required_amount,
                    _ => return false,
                }
            }
            return false;
        }
        
        // Check if this is a predicate condition
        if !self.predicate.is_empty() && !self.argument.is_empty() {
            if let Some(place) = state.places.get(&self.argument) {
                match self.predicate.as_str() {
                    "has-sawmill" => return place.has_sawmill,
                    "has-cabin" => return place.has_cabin,
                    "has-coal-stack" => return place.has_coal_stack,
                    "has-quarry" => return place.has_quarry,
                    "has-mine" => return place.has_mine,
                    "has-ironworks" => return place.has_ironworks,
                    "has-docks" => return place.has_docks,
                    "has-wharf" => return place.has_wharf,
                    _ => (),
                }
            }
        }
        
        // Check if this is a connected-by-rail condition
        if self.predicate == "connected-by-rail" && !self.from.is_empty() && !self.to.is_empty() {
            if let Some(connected_places) = state.connections_by_rail.get(&self.from) {
                return connected_places.contains(&self.to);
            }
            return false;
        }
        
        // If none of the condition types matched, return false
        false
    }
}



#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettlersNumericProblem {
    pub goal: Goal,
    pub connections_by_land: HashMap<String, Vec<String>>, // Place ID -> Connected Places
    pub connections_by_sea: HashMap<String, Vec<String>>,  // Place ID -> Connected Places
}

impl SettlersNumericProblem {
    /// Generates a `load` action.
    pub fn get_load_action(vehicle: &Vehicle, place: &Place, resource: &str) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("vehicle".to_string(), Value::Text(vehicle.id.clone()));
        parameters.insert("place".to_string(), Value::Text(place.id.clone()));
        parameters.insert("resource".to_string(), Value::Text(resource.to_string()));
        Action::new(format!("load_{}_{}", vehicle.id, resource), 1, parameters)
    }

    /// Generates an `unload` action.
    pub fn get_unload_action(vehicle: &Vehicle, place: &Place, resource: &str) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("vehicle".to_string(), Value::Text(vehicle.id.clone()));
        parameters.insert("place".to_string(), Value::Text(place.id.clone()));
        parameters.insert("resource".to_string(), Value::Text(resource.to_string()));
        Action::new(format!("unload_{}_{}", vehicle.id, resource), 1, parameters)
    }

    /// Generates a `move_train` action.
    pub fn get_move_train_action(vehicle: &Vehicle, from: &str, to: &str) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("vehicle".to_string(), Value::Text(vehicle.id.clone()));
        parameters.insert("from".to_string(), Value::Text(from.to_string()));
        parameters.insert("to".to_string(), Value::Text(to.to_string()));
        Action::new(format!("move_train_{}_{}_{}", vehicle.id, from, to), 1, parameters)
    }


    /// Generates a `build_cabin` action.
    pub fn get_build_cabin_action(place: &Place) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("place".to_string(), Value::Text(place.id.clone()));
        Action::new(format!("build_cabin_{}", place.id), 1, parameters)
    }

    /// Generates a `build_docks` action.
    pub fn get_build_docks_action(place: &Place) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("place".to_string(), Value::Text(place.id.clone()));
        Action::new(format!("build_docks_{}", place.id), 1, parameters)
    }

    /// Generates a `build_quarry` action.
    pub fn get_build_quarry_action(place: &Place) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("place".to_string(), Value::Text(place.id.clone()));
        Action::new(format!("build_quarry_{}", place.id), 1, parameters)
    }

    /// Generates a `build_sawmill` action.
    pub fn get_build_sawmill_action(place: &Place) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("place".to_string(), Value::Text(place.id.clone()));
        Action::new(format!("build_sawmill_{}", place.id), 1, parameters)
    }

    pub fn get_move_empty_cart_action(from: &Place, to: &Place) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("from".to_string(), Value::Text(from.id.clone()));
        parameters.insert("to".to_string(), Value::Text(to.id.clone()));
        Action::new(format!("move_empty_cart_{}_{}", from.id, to.id), 2, parameters)
    }

    pub fn get_move_laden_cart_action(from: &Place, to: &Place, resource: &str) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("from".to_string(), Value::Text(from.id.clone()));
        parameters.insert("to".to_string(), Value::Text(to.id.clone()));
        parameters.insert("resource".to_string(), Value::Text(resource.to_string()));
        Action::new(format!("move_laden_cart_{}_{}_{}", from.id, to.id, resource), 2, parameters)
    }


    pub fn get_build_house_action(place: &Place) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("place".to_string(), Value::Text(place.id.clone()));
        Action::new(format!("build_house_{}", place.id), 1, parameters)
    }

    pub fn get_build_cart_action(place: &Place) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("place".to_string(), Value::Text(place.id.clone()));
        Action::new(format!("build_cart_{}", place.id), 1, parameters)
    }

    /// Generates a `build_ironworks` action.
pub fn get_build_ironworks_action(place: &Place) -> Action {
    let mut parameters = HashMap::new();
    parameters.insert("place".to_string(), Value::Text(place.id.clone()));
    Action::new(format!("build_ironworks_{}", place.id), 1, parameters)
}

/// Generates a `build_coal_stack` action.
pub fn get_build_coal_stack_action(place: &Place) -> Action {
    let mut parameters = HashMap::new();
    parameters.insert("place".to_string(), Value::Text(place.id.clone()));
    Action::new(format!("build_coal_stack_{}", place.id), 1, parameters)
}

/// Generates a `build_mine` action.
pub fn get_build_mine_action(place: &Place) -> Action {
    let mut parameters = HashMap::new();
    parameters.insert("place".to_string(), Value::Text(place.id.clone()));
    Action::new(format!("build_mine_{}", place.id), 1, parameters)
}

/// Generates a `build_rail` action.
pub fn get_build_rail_action(from: &Place, to: &str) -> Action {
    let mut parameters = HashMap::new();
    parameters.insert("from".to_string(), Value::Text(from.id.clone()));
    parameters.insert("to".to_string(), Value::Text(to.to_string()));
    Action::new(format!("build_rail_{}_{}", from.id, to), 1, parameters)
}

/// Generates a `fell_timber` action.
pub fn get_fell_timber_action(place: &Place) -> Action {
    let mut parameters = HashMap::new();
    parameters.insert("place".to_string(), Value::Text(place.id.clone()));
    Action::new(format!("fell_timber_{}", place.id), 1, parameters)
}

/// Generates a `burn_coal` action.
pub fn get_burn_coal_action(place: &Place) -> Action {
    let mut parameters = HashMap::new();
    parameters.insert("place".to_string(), Value::Text(place.id.clone()));
    Action::new(format!("burn_coal_{}", place.id), 1, parameters)
}

/// Generates a `saw_wood` action.
pub fn get_saw_wood_action(place: &Place) -> Action {
    let mut parameters = HashMap::new();
    parameters.insert("place".to_string(), Value::Text(place.id.clone()));
    Action::new(format!("saw_wood_{}", place.id), 1, parameters)
}

/// Generates a `make_iron` action.
pub fn get_make_iron_action(place: &Place) -> Action {
    let mut parameters = HashMap::new();
    parameters.insert("place".to_string(), Value::Text(place.id.clone()));
    Action::new(format!("make_iron_{}", place.id), 1, parameters)
}

/// Generates a `build_train` action.
pub fn get_build_train_action(vehicle: &Vehicle, place: &Place) -> Action {
    let mut parameters = HashMap::new();
    parameters.insert("vehicle".to_string(), Value::Text(vehicle.id.clone()));
    parameters.insert("place".to_string(), Value::Text(place.id.clone()));
    Action::new(format!("build_train_{}_{}", vehicle.id, place.id), 1, parameters)
}

/// Generates a `move_ship` action.
pub fn get_move_ship_action(vehicle: &Vehicle, from: &str, to: &str) -> Action {
    let mut parameters = HashMap::new();
    parameters.insert("vehicle".to_string(), Value::Text(vehicle.id.clone()));
    parameters.insert("from".to_string(), Value::Text(from.to_string()));
    parameters.insert("to".to_string(), Value::Text(to.to_string()));
    Action::new(format!("move_ship_{}_{}_{}", vehicle.id, from, to), 2, parameters)
}

    pub fn get_build_ship_action(vehicle: &Vehicle, place: &Place) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("vehicle".to_string(), Value::Text(vehicle.id.clone()));
        parameters.insert("place".to_string(), Value::Text(place.id.clone()));
        Action::new(format!("build_ship_{}_{}", vehicle.id, place.id), 3, parameters)
    }

    pub fn get_mine_ore_action(place: &Place) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("place".to_string(), Value::Text(place.id.clone()));
        Action::new(format!("mine_ore_{}", place.id), 2, parameters)
    }

    pub fn get_build_wharf_action(place: &Place) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("place".to_string(), Value::Text(place.id.clone()));
        Action::new(format!("build_wharf_{}", place.id), 1, parameters)
    }

    pub fn get_break_stone_action(place: &Place) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("place".to_string(), Value::Text(place.id.clone()));
        Action::new(format!("break_stone_{}", place.id), 1, parameters)
    }

    pub fn get_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        
        // Check what resources/conditions are needed for the goal
        let mut need_housing = false;
        let mut need_sawmill = false;
        let mut need_rail_connection = false;
        let mut rail_from = String::new();
        let mut rail_to = String::new();
        let mut need_resources = HashMap::new();
        
        for condition in &self.goal.conditions {
            if !condition.attribute.is_empty() && condition.attribute == "housing" {
                need_housing = true;
                need_resources.insert("wood".to_string(), true);
                need_resources.insert("stone".to_string(), true);
            }
            
            if !condition.predicate.is_empty() {
                if condition.predicate == "has-sawmill" {
                    need_sawmill = true;
                    need_resources.insert("timber".to_string(), true);
                } else if condition.predicate == "connected-by-rail" {
                    need_rail_connection = true;
                    rail_from = condition.from.clone();
                    rail_to = condition.to.clone();
                    need_resources.insert("wood".to_string(), true);
                    need_resources.insert("iron".to_string(), true);
                }
            }
            
            if !condition.resource.is_empty() {
                need_resources.insert(condition.resource.clone(), true);
            }
        }
        
        // Only generate actions that help achieve the goal
        
        // 1. If we need housing, generate actions to build houses
        if need_housing {
            for place in state.places.values() {
                if place.available_resources.get("wood").unwrap_or(&0) >= &1
                    && place.available_resources.get("stone").unwrap_or(&0) >= &1
                {
                    actions.push(Self::get_build_house_action(place));
                }
            }
        }
        
        // 2. If we need sawmills, generate actions to build sawmills
        if need_sawmill {
            for place in state.places.values() {
                if place.is_woodland && !place.has_sawmill 
                    && place.available_resources.get("timber").unwrap_or(&0) >= &2 {
                    actions.push(Self::get_build_sawmill_action(place));
                }
            }
        }
        
        // 3. If we need a rail connection, generate actions to build rails
        if need_rail_connection {
            if let Some(from_place) = state.places.get(&rail_from) {
                if from_place.available_resources.get("wood").unwrap_or(&0) >= &1
                    && from_place.available_resources.get("iron").unwrap_or(&0) >= &1
                    && !state.connections_by_rail.get(&rail_from).unwrap_or(&vec![]).contains(&rail_to)
                {
                    actions.push(Self::get_build_rail_action(from_place, &rail_to));
                }
            }
        }
        
        // 4. Generate resource gathering actions only for needed resources
        for place in state.places.values() {
            // Only generate timber-related actions if we need timber or wood
            if need_resources.contains_key("timber") || need_resources.contains_key("wood") {
                // B: Don't build more than one cabin per woodland
                if place.is_woodland && !place.has_cabin && 
                   state.places.values().filter(|p| p.has_cabin).count() < 2 {
                    actions.push(Self::get_build_cabin_action(place));
                }
                
                // C: Don't fell more timber than needed
                if place.has_cabin && 
                   state.places.values().map(|p| 
                       p.available_resources.get("timber").unwrap_or(&0)).sum::<i32>() < 5 {
                    actions.push(Self::get_fell_timber_action(place));
                }
                
                if place.has_sawmill && place.available_resources.get("timber").unwrap_or(&0) >= &1 {
                    actions.push(Self::get_saw_wood_action(place));
                }
            }
            
            // Only generate stone-related actions if we need stone
            if need_resources.contains_key("stone") {
                // B: Don't build more than one quarry
                if place.is_mountain && !place.has_quarry && 
                   state.places.values().filter(|p| p.has_quarry).count() < 2 {
                    actions.push(Self::get_build_quarry_action(place));
                }
                
                // C: Don't break more stone than needed
                if place.has_quarry && 
                   state.places.values().map(|p| 
                       p.available_resources.get("stone").unwrap_or(&0)).sum::<i32>() < 3 {
                    actions.push(Self::get_break_stone_action(place));
                }
            }
            
            // Only generate iron-related actions if we need iron
            if need_resources.contains_key("iron") {
                if place.is_metalliferous && !place.has_mine {
                    actions.push(Self::get_build_mine_action(place));
                }
                
                // C: Don't mine more ore than needed
                if place.has_mine && 
                   state.places.values().map(|p| 
                       p.available_resources.get("ore").unwrap_or(&0)).sum::<i32>() < 3 {
                    actions.push(Self::get_mine_ore_action(place));
                }
                
                if place.available_resources.get("timber").unwrap_or(&0) >= &1 && !place.has_coal_stack {
                    actions.push(Self::get_build_coal_stack_action(place));
                }
                
                // C: Don't burn more coal than needed
                if place.has_coal_stack && place.available_resources.get("timber").unwrap_or(&0) >= &1 &&
                   state.places.values().map(|p| 
                       p.available_resources.get("coal").unwrap_or(&0)).sum::<i32>() < 4 {
                    actions.push(Self::get_burn_coal_action(place));
                }
                
                if place.has_ironworks
                    && place.available_resources.get("ore").unwrap_or(&0) >= &1
                    && place.available_resources.get("coal").unwrap_or(&0) >= &2
                {
                    actions.push(Self::get_make_iron_action(place));
                }
            }
        }
        
        // 5. Generate transport actions only for needed resources
        for (from_id, from_place) in &state.places {
            if from_place.carts > 0 {
                if let Some(connected_places) = self.connections_by_land.get(from_id) {
                    for to_id in connected_places {
                        if let Some(to_place) = state.places.get(to_id) {
                            // Move empty cart
                            actions.push(Self::get_move_empty_cart_action(from_place, to_place));
                            
                            // Move laden cart only for needed resources
                            for (resource, &amount) in &from_place.available_resources {
                                if amount > 0 && need_resources.contains_key(resource) {
                                    actions.push(Self::get_move_laden_cart_action(from_place, to_place, resource));
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Add cart building actions if we need to move resources
        if !need_resources.is_empty() {
            for place in state.places.values() {
                // B: Limit the number of carts to what's needed
                if place.available_resources.get("timber").unwrap_or(&0) >= &1 && 
                   state.places.values().map(|p| p.carts).sum::<i32>() < 3 {
                    actions.push(Self::get_build_cart_action(place));
                }
            }
        }
        
        actions
    }
    

    /// Applies a `load` action to the state.
    pub fn apply_load_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let vehicle_id = match action.parameters.get("vehicle") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid vehicle ID in action"),
        };
        let place_id = match action.parameters.get("place") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid place ID in action"),
        };
        let resource = match action.parameters.get("resource") {
            Some(Value::Text(res)) => res,
            _ => panic!("Invalid resource in action"),
        };

        let vehicle = new_state.vehicles.get_mut(vehicle_id).unwrap();
        let place = new_state.places.get_mut(place_id).unwrap();

      
        *place.available_resources.get_mut(resource).unwrap() -= 1;
        *vehicle.available_resources.entry(resource.clone()).or_insert(0) += 1;
        vehicle.space_in -= 1;
        new_state.labour += 1;
    

        new_state
    }

    /// Applies an `unload` action to the state.
    pub fn apply_unload_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let vehicle_id = match action.parameters.get("vehicle") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid vehicle ID in action"),
        };
        let place_id = match action.parameters.get("place") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid place ID in action"),
        };
        let resource = match action.parameters.get("resource") {
            Some(Value::Text(res)) => res,
            _ => panic!("Invalid resource in action"),
        };

        let vehicle = new_state.vehicles.get_mut(vehicle_id).unwrap();
        let place = new_state.places.get_mut(place_id).unwrap();

        
        *vehicle.available_resources.get_mut(resource).unwrap() -= 1;
        *place.available_resources.entry(resource.clone()).or_insert(0) += 1;
        vehicle.space_in += 1;
        new_state.labour += 1;
    

        new_state
    }

    /// Applies a `build_docks` action to the state.
    pub fn apply_build_docks_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let place_id = match action.parameters.get("place") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid place ID in action"),
        };

        let place = new_state.places.get_mut(place_id).unwrap();

        *place.available_resources.get_mut("stone").unwrap() -= 2;
        *place.available_resources.get_mut("wood").unwrap() -= 2;
        place.has_docks = true;
        new_state.labour += 2;

        new_state
    }

    /// Applies a `build_quarry` action to the state.
    pub fn apply_build_quarry_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let place_id = match action.parameters.get("place") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid place ID in action"),
        };

        let place = new_state.places.get_mut(place_id).unwrap();

        // Apply effects directly
        place.has_quarry = true;
        new_state.labour += 2;

        new_state
    }

    /// Applies a `build_sawmill` action to the state.
    pub fn apply_build_sawmill_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let place_id = match action.parameters.get("place") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid place ID in action"),
        };
    
        let place = new_state.places.get_mut(place_id).unwrap();
    
        if place.available_resources.get("timber").unwrap_or(&0) >= &2 {
            *place.available_resources.get_mut("timber").unwrap() -= 2;
            place.has_sawmill = true;
            new_state.labour += 2;
        }
    
        new_state
    }
    

    pub fn apply_move_empty_cart_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let from = match action.parameters.get("from") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid from place ID in action"),
        };
        let to = match action.parameters.get("to") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid to place ID in action"),
        };
    
        // Get the cart count from the "from" place
        let from_carts = new_state.places.get(from).unwrap().carts;
        
        if from_carts > 0 {
            // Update the "from" place
            new_state.places.get_mut(from).unwrap().carts -= 1;
            // Update the "to" place
            new_state.places.get_mut(to).unwrap().carts += 1;
            new_state.labour += 2;
        }
    
        new_state
    }
    

    pub fn apply_move_laden_cart_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let from = match action.parameters.get("from") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid from place ID in action"),
        };
        let to = match action.parameters.get("to") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid to place ID in action"),
        };
        let resource = match action.parameters.get("resource") {
            Some(Value::Text(res)) => res,
            _ => panic!("Invalid resource in action"),
        };
    
        // Get the cart count and resource amount from the "from" place
        let from_place = new_state.places.get(from).unwrap();
        let from_carts = from_place.carts;
        let resource_amount = *from_place.available_resources.get(resource).unwrap_or(&0);
        
        if from_carts > 0 && resource_amount > 0 {
            // Update the "from" place
            let from_place = new_state.places.get_mut(from).unwrap();
            from_place.carts -= 1;
            *from_place.available_resources.get_mut(resource).unwrap() -= 1;
            
            // Update the "to" place
            let to_place = new_state.places.get_mut(to).unwrap();
            to_place.carts += 1;
            *to_place.available_resources.entry(resource.clone()).or_insert(0) += 1;
            
            new_state.labour += 2;
        }
    
        new_state
    }
    
    
    
    pub fn apply_build_house_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let place_id = match action.parameters.get("place") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid place ID in action"),
        };
    
        let place = new_state.places.get_mut(place_id).unwrap();
    
        *place.available_resources.get_mut("wood").unwrap() -= 1;
        *place.available_resources.get_mut("stone").unwrap() -= 1;
        place.housing += 1;
    
        new_state
    }


    pub fn apply_mine_ore_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let place_id = match action.parameters.get("place") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid place ID in action"),
        };
    
        let place = new_state.places.get_mut(place_id).unwrap();
    
        if place.has_mine {
            *place.available_resources.entry("ore".to_string()).or_insert(0) += 1;
            new_state.labour += 1;
            new_state.resource_use += 2; // Increment resource_use by 2 as per PDDL
        }
    
        new_state
    }

    pub fn apply_build_coal_stack_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let place_id = match action.parameters.get("place") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid place ID in action"),
        };

        let place = new_state.places.get_mut(place_id).unwrap();

        *place.available_resources.get_mut("timber").unwrap() -= 1;
        place.has_coal_stack = true;
        new_state.labour += 2;

        new_state
    }

    pub fn apply_fell_timber_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let place_id = match action.parameters.get("place") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid place ID in action"),
        };

        let place = new_state.places.get_mut(place_id).unwrap();

        *place.available_resources.entry("timber".to_string()).or_insert(0) += 1;
        new_state.labour += 1;

        new_state
    }

    pub fn apply_burn_coal_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let place_id = match action.parameters.get("place") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid place ID in action"),
        };

        let place = new_state.places.get_mut(place_id).unwrap();

        *place.available_resources.get_mut("timber").unwrap() -= 1;
        *place.available_resources.entry("coal".to_string()).or_insert(0) += 1;
        new_state.pollution += 1;
    
        new_state
    }

    pub fn apply_build_mine_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let place_id = match action.parameters.get("place") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid place ID in action"),
        };

        let place = new_state.places.get_mut(place_id).unwrap();

        *place.available_resources.get_mut("wood").unwrap() -= 2;
        place.has_mine = true;
        new_state.labour += 3;

        new_state
    }

    pub fn apply_build_train_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let place_id = match action.parameters.get("place") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid place ID in action"),
        };
        let vehicle_id = match action.parameters.get("vehicle") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid vehicle ID in action"),
        };
    
        let place = new_state.places.get_mut(place_id).unwrap();
        let vehicle = new_state.vehicles.get_mut(vehicle_id).unwrap();
    
        // Apply effects directly
        *place.available_resources.get_mut("iron").unwrap() -= 2;
        vehicle.is_train = true;
        vehicle.potential = false;
        vehicle.location = place_id.clone(); // Set the location
        vehicle.space_in = 5;
        
        // Initialize resources to 0
        vehicle.available_resources.insert("timber".to_string(), 0);
        vehicle.available_resources.insert("wood".to_string(), 0);
        vehicle.available_resources.insert("coal".to_string(), 0);
        vehicle.available_resources.insert("stone".to_string(), 0);
        vehicle.available_resources.insert("iron".to_string(), 0);
        vehicle.available_resources.insert("ore".to_string(), 0);
        
        new_state.labour += 2;
    
        new_state
    }
    
    
    pub fn apply_make_iron_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let place_id = match action.parameters.get("place") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid place ID in action"),
        };

        let place = new_state.places.get_mut(place_id).unwrap();

        *place.available_resources.get_mut("ore").unwrap() -= 1;
        *place.available_resources.get_mut("coal").unwrap() -= 2;
        *place.available_resources.entry("iron".to_string()).or_insert(0) += 1;
        new_state.pollution += 2;

        new_state
    }

    pub fn apply_move_train_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let vehicle_id = match action.parameters.get("vehicle") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid vehicle ID in action"),
        };
        let from = match action.parameters.get("from") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid from place ID in action"),
        };
        let to = match action.parameters.get("to") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid to place ID in action"),
        };
    
        let vehicle = new_state.vehicles.get_mut(vehicle_id).unwrap();
        
        // Apply effects directly if preconditions are met
        if vehicle.is_train
            && vehicle.location == *from
            && vehicle.available_resources.get("coal").unwrap_or(&0) >= &1
            && new_state.connections_by_rail.get(from).unwrap_or(&vec![]).contains(to)
        {
            *vehicle.available_resources.get_mut("coal").unwrap() -= 1;
            vehicle.location = to.clone();
            new_state.pollution += 1;
        }
    
        new_state
    }
    

    pub fn apply_build_wharf_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let place_id = match action.parameters.get("place") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid place ID in action"),
        };

        let place = new_state.places.get_mut(place_id).unwrap();

        *place.available_resources.get_mut("stone").unwrap() -= 2;
        *place.available_resources.get_mut("iron").unwrap() -= 2;
        place.has_wharf = true;
        new_state.labour += 2;

        new_state
    }

    pub fn apply_break_stone_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let place_id = match action.parameters.get("place") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid place ID in action"),
        };
    
        let place = new_state.places.get_mut(place_id).unwrap();
    
        *place.available_resources.entry("stone".to_string()).or_insert(0) += 1;
        new_state.labour += 1;
        new_state.resource_use += 1;
    
        new_state
    }
    

    pub fn apply_saw_wood_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let place_id = match action.parameters.get("place") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid place ID in action"),
        };

        let place = new_state.places.get_mut(place_id).unwrap();

        *place.available_resources.get_mut("timber").unwrap() -= 1;
        *place.available_resources.entry("wood".to_string()).or_insert(0) += 1;

        new_state
    }

    pub fn apply_build_rail_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        
        let from = match action.parameters.get("from") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid from place ID in action"),
        };
        let to = match action.parameters.get("to") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid to place ID in action"),
        };
    
        let from_place = new_state.places.get_mut(from).unwrap();
    
        // Apply effects directly
        *from_place.available_resources.get_mut("wood").unwrap() -= 1;
        *from_place.available_resources.get_mut("iron").unwrap() -= 1;
        
        // Update the connections_by_rail in the state
        new_state
            .connections_by_rail
            .entry(from.clone())
            .or_insert_with(Vec::new)
            .push(to.clone());
            
        new_state.labour += 2;
    
        new_state
    }
    


    pub fn apply_build_ironworks_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let place_id = match action.parameters.get("place") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid place ID in action"),
        };
    
        let place = new_state.places.get_mut(place_id).unwrap();
    
        *place.available_resources.get_mut("stone").unwrap() -= 2;
        *place.available_resources.get_mut("wood").unwrap() -= 2;
        place.has_ironworks = true;
        new_state.labour += 3;
    
        new_state
    }

    pub fn apply_build_cabin_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let place_id = match action.parameters.get("place") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid place ID in action"),
        };
    
        let place = new_state.places.get_mut(place_id).unwrap();
    
        place.has_cabin = true;
        new_state.labour += 1;
    
        new_state
    }
    
    pub fn apply_move_ship_action(state: &State, action: &Action, problem: &SettlersNumericProblem) -> State {
        let mut new_state = state.clone();
        let vehicle_id = match action.parameters.get("vehicle") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid vehicle ID in action"),
        };
        let from = match action.parameters.get("from") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid from place ID in action"),
        };
        let to = match action.parameters.get("to") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid to place ID in action"),
        };

        let vehicle = new_state.vehicles.get_mut(vehicle_id).unwrap();

        if vehicle.is_ship 
            && vehicle.location == *from
            && vehicle.available_resources.get("coal").unwrap_or(&0) >= &2 
            && problem.connections_by_sea.get(from).unwrap_or(&vec![]).contains(to)
        {
            *vehicle.available_resources.get_mut("coal").unwrap() -= 2;
            vehicle.location = to.clone();
            new_state.pollution += 2;
        }

        new_state
    }
    
    pub fn apply_build_ship_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let vehicle_id = match action.parameters.get("vehicle") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid vehicle ID in action"),
        };
        let place_id = match action.parameters.get("place") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid place ID in action"),
        };
    
        let place = new_state.places.get_mut(place_id).unwrap();
        let vehicle = new_state.vehicles.get_mut(vehicle_id).unwrap();
    
        if vehicle.potential 
            && place.has_wharf 
            && place.available_resources.get("iron").unwrap_or(&0) >= &4 {
            *place.available_resources.get_mut("iron").unwrap() -= 4;
            vehicle.is_ship = true;
            vehicle.potential = false;
            vehicle.location = place_id.clone();
            vehicle.space_in = 10;
            
            // Initialize resources to 0
            vehicle.available_resources.insert("timber".to_string(), 0);
            vehicle.available_resources.insert("wood".to_string(), 0);
            vehicle.available_resources.insert("coal".to_string(), 0);
            vehicle.available_resources.insert("stone".to_string(), 0);
            vehicle.available_resources.insert("iron".to_string(), 0);
            vehicle.available_resources.insert("ore".to_string(), 0);
            
            new_state.labour += 3;
        }
    
        new_state
    }

    pub fn apply_build_cart_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let place_id = match action.parameters.get("place") {
            Some(Value::Text(id)) => id,
            _ => panic!("Invalid place ID in action"),
        };
    
        let place = new_state.places.get_mut(place_id).unwrap();
    
        *place.available_resources.get_mut("timber").unwrap() -= 1;
        place.carts += 1;
        new_state.labour += 1;
    
        new_state
    }
    
}
impl Problem for SettlersNumericProblem {
    type State = State;

    fn get_possible_actions(&self, state: &State) -> Vec<Action> {
        self.get_actions(state)
    }

    fn apply_action(&self, state: &State, action: &Action) -> State {
        if action.name.starts_with("load_") {
            Self::apply_load_action(state, action)
        } else if action.name.starts_with("unload_") {
            Self::apply_unload_action(state, action)
        } else if action.name.starts_with("move_train_") {
            Self::apply_move_train_action(state, action)
        } else if action.name.starts_with("move_empty_cart_") {
            Self::apply_move_empty_cart_action(state, action)
        } else if action.name.starts_with("move_laden_cart_") {
            Self::apply_move_laden_cart_action(state, action)
        } else if action.name.starts_with("build_coal_stack_") {
            Self::apply_build_coal_stack_action(state, action)
        } else if action.name.starts_with("fell_timber_") {
            Self::apply_fell_timber_action(state, action)
        } else if action.name.starts_with("burn_coal_") {
            Self::apply_burn_coal_action(state, action)
        } else if action.name.starts_with("build_docks_") {
            Self::apply_build_docks_action(state, action)
        } else if action.name.starts_with("build_house_") {
            Self::apply_build_house_action(state, action)
        } else if action.name.starts_with("build_quarry_") {
            Self::apply_build_quarry_action(state, action)
        } else if action.name.starts_with("build_sawmill_") {
            Self::apply_build_sawmill_action(state, action)
        } else if action.name.starts_with("build_wharf_") {
            Self::apply_build_wharf_action(state, action)
        } else if action.name.starts_with("build_mine_") {
            Self::apply_build_mine_action(state, action)
        } else if action.name.starts_with("build_train_") {
            Self::apply_build_train_action(state, action)
        } else if action.name.starts_with("make_iron_") {
            Self::apply_make_iron_action(state, action)
        } else if action.name.starts_with("break_stone_") {
            Self::apply_break_stone_action(state, action)
        } else if action.name.starts_with("saw_wood_") {
            Self::apply_saw_wood_action(state, action)
        } else if action.name.starts_with("build_rail_") {
            Self::apply_build_rail_action(state, action)
        } else if action.name.starts_with("build_ironworks_") {
            Self::apply_build_ironworks_action(state, action)
        } else if action.name.starts_with("build_cabin_") {
            Self::apply_build_cabin_action(state, action)
        } else if action.name.starts_with("move_ship_") {
            Self::apply_move_ship_action(state, action, self)
        } else if action.name.starts_with("build_ship_") {
            Self::apply_build_ship_action(state, action)
        } else if action.name.starts_with("mine_ore_") {
            Self::apply_mine_ore_action(state, action)
        } else if action.name.starts_with("build_cart_") {
            Self::apply_build_cart_action(state, action)
        } else {
            panic!("Unknown action type: {}", action.name)
        }
    }
    
    fn is_goal_state(&self, state: &State) -> bool {
        self.goal.is_goal_state(state)
    }

    fn heuristic(&self, _state: &State) -> f64 {
        0.0
    }
    
    fn load_state_from_json(json_path: &str) -> (State, Self) {
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
        let problem: SettlersNumericProblem =
            serde_json::from_value(problem_value.clone()).expect("Failed to deserialize problem");

        (state, problem)
    }
}
