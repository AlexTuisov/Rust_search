use crate::problems::problem::Problem;
use crate::search::{action::Action, state::StateTrait, state::Value};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::fs;

/// Represents the state of the sugar production system.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub mills_total_cost: i32,                  // Total cost incurred by mills.
    pub handling_cost: i32,                     // Total cost of handling operations.
    pub trucks: HashMap<String, Truck>,         // truck_id -> Truck
    pub mills: HashMap<String, Mill>,           // mill_id -> Mill
    pub cranes: HashMap<String, Crane>,         // crane_id -> Crane
    pub depots: HashMap<String, Depot>,         // depot_id -> Depot
    pub farmfields: HashMap<String, FarmField>, // farmfield_id -> FarmField
}

impl State {}

impl StateTrait for State {}

/// Represents a truck used for transporting sugar.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Truck {
    pub id: String,
    pub remaining_capacity: i32,             // Remaining load capacity
    pub raw_cane_load: HashMap<String, i32>, // raw_cane_id -> quantity loaded
    pub brand_load: HashMap<String, i32>,    // sugar_id -> quantity loaded
    pub location: i32,
}

/// Represents a crane used for loading and unloading sugar.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Crane {
    pub id: String,
    pub capacity: i32,    // Maximum load capacity of the crane
    pub location: i32,    // The location of the crane
    pub maintenance: i32, // Remaining maintenance time before servicing
}

/// Represents a mill where sugar is processed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Mill {
    pub id: String,
    pub available: bool,   // True if the mill is available for production
    pub process_cost: i32, // Cost of running the production process
    pub max_produce: i32,  // Maximum production capacity
    pub can_produce: HashMap<String, bool>, // brand_id -> true if the mill can produce this brand
    pub current_produce: String, // Current brand being produced
    pub place_order: Vec<String>, // vec of raw_cane_id
    pub raw_cane_storage: HashMap<String, i32>, // raw_cane_id -> quantity stored
    pub brand_storage: HashMap<String, i32>, // brand_id -> quantity stored
    pub location: i32,     // Location of the mill
    pub max_changes: i32,  // Maximum number of production process changes allowed
}

/// Represents a storage depot for sugar.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Depot {
    pub id: String,                             // Unique identifier for the depot
    pub raw_cane_storage: HashMap<String, i32>, // raw_cane_id -> quantity stored
    pub brand_storage: HashMap<String, i32>,    // brand_id -> quantity stored
    pub location: i32,                          // Location of the depot
}

/// Represents a farm field that grows raw cane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FarmField {
    pub id: String,
    pub near_mills: HashMap<String, bool>, // mill_id -> true if the farm field is near a specific mill
    pub raw_canes: HashMap<String, bool>,  // List of raw cane types available on this farm
    pub cane_yield: i32,                   // The yield of cane per harvest
    pub total_canes: i32,                  // Total cane available in the field
    pub location: i32,
}

/// Represents a type of raw cane used in sugar production.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RawCane {
    pub id: String, // Unique identifier for raw cane
}

/// Represents a brand of sugar.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Brand {
    pub id: String, // Unique identifier for the sugar brand
}

/// Represents a factory that creates sugar brands from raw cane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrandsFormulas {
    pub brands: HashMap<String, Vec<Formula>>, // brand_id -> formulas for producing that brand
}

/// Represents a formula for creating a sugar brand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Formula {
    pub raw_cane1: RawCane,         // Primary raw cane required
    pub raw_cane2: Option<RawCane>, // Optional secondary raw cane (None if not needed)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SugarProblem {
    pub connected: HashMap<String, HashMap<String, bool>>, // Maps locations; connected[from][to] = true if connected.
    pub connected_mills: HashMap<String, HashMap<String, bool>>, // Maps mills; connected_mills[m1][m2] = true if connected.
    pub brands_formulas: BrandsFormulas, // Stores formulas required to produce each sugar brand.
    pub changing_brands: HashMap<String, Vec<String>>, // Maps brand transitions; changing_brands[brand] = list of possible changes.
    pub goal: Goal, // Defines the goal conditions for the sugar production problem.
}

/// Represents the goal conditions for the sugar production system.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Goal {
    pub conditions: Vec<Condition>, // List of conditions that must be met to achieve the goal.
}

/// Represents a specific condition that must be satisfied in a depot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Condition {
    pub brand_id: String,    // The ID of the sugar brand that must be stored.
    pub depot_id: String,    // The ID of the depot where the brand must be stored.
    pub storage_amount: i32, // The required amount of the brand to be stored in the depot.
}

impl SugarProblem {
    pub fn get_produce_sugar_from_single_raw_action(
        cane_id: &String,
        mill_id: &String,
        brand_id: &String,
        process_cost: &i32,
        mode: &str, // "single", "resource", or "max"
    ) -> Action {
        let mut parameters = HashMap::new();
        let action_name = format!(
            "{}_produce_{}_using_{}_{}",
            mill_id, brand_id, mode, cane_id
        );

        parameters.insert(
            "produce_sugar_from_single_raw".to_string(),
            Value::Text(mode.to_string()),
        );
        parameters.insert("mill".to_string(), Value::Text(mill_id.clone()));
        parameters.insert("brand".to_string(), Value::Text(brand_id.clone()));
        parameters.insert("cane".to_string(), Value::Text(cane_id.clone()));

        // Adjust process cost based on mode
        let adjusted_cost = match mode {
            "resource" => 4 * (*process_cost),
            "max" => 5 * (*process_cost),
            _ => *process_cost, // Default case: "single"
        };

        Action::new(action_name, adjusted_cost, parameters)
    }

    pub fn get_produce_sugar_from_mixed_raw_action(
        cane1_id: &String,
        cane2_id: &String,
        mill_id: &String,
        brand_id: &String,
        process_cost: &i32,
        mode: &str, // "single", "resource", or "max"
    ) -> Action {
        let mut parameters = HashMap::new();
        let action_name = format!(
            "{}_produce_{}_using_{}_{}_{}",
            mill_id, brand_id, mode, cane1_id, cane2_id
        );

        parameters.insert(
            "produce_sugar_from_mixed_raw".to_string(),
            Value::Text(mode.to_string()),
        );
        parameters.insert("mill".to_string(), Value::Text(mill_id.clone()));
        parameters.insert("brand".to_string(), Value::Text(brand_id.clone()));
        parameters.insert("cane1".to_string(), Value::Text(cane1_id.clone()));
        parameters.insert("cane2".to_string(), Value::Text(cane2_id.clone()));

        // Adjust process cost based on mode
        let adjusted_cost = match mode {
            "resource" => 4 * (*process_cost),
            "max" => 5 * (*process_cost),
            _ => *process_cost, // Default case: "single"
        };
        Action::new(action_name, adjusted_cost, parameters)
    }

    pub fn get_setting_machine_action(mill_id: &String) -> Action {
        let mut parameters = HashMap::new();
        let action_name = format!("setting_machine_{}", mill_id);
        parameters.insert("mill".to_string(), Value::Text(mill_id.clone()));
        Action::new(action_name, 1, parameters)
    }

    pub fn get_possible_available_or_produce_sugar_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for mill in state.mills.values() {
            if mill.available {
                for brand_id in mill.can_produce.keys() {
                    if mill.current_produce == *brand_id {
                        if let Some(formulas) = self.brands_formulas.brands.get(brand_id) {
                            for formula in formulas {
                                match &formula.raw_cane2 {
                                    Some(raw_cane2) => {
                                        if let (
                                            Some(raw_cane1_available),
                                            Some(raw_cane2_available),
                                        ) = (
                                            mill.raw_cane_storage.get(&formula.raw_cane1.id),
                                            mill.raw_cane_storage.get(&raw_cane2.id),
                                        ) {
                                            if *raw_cane1_available > 0 && *raw_cane2_available > 0
                                            {
                                                actions.push(
                                                    Self::get_produce_sugar_from_mixed_raw_action(
                                                        &formula.raw_cane1.id,
                                                        &raw_cane2.id,
                                                        &mill.id,
                                                        brand_id,
                                                        &mill.process_cost,
                                                        "single",
                                                    ),
                                                );

                                                if *raw_cane1_available > mill.max_produce
                                                    && *raw_cane2_available > mill.max_produce
                                                {
                                                    actions.push(
                                                        Self::get_produce_sugar_from_mixed_raw_action(
                                                            &formula.raw_cane1.id,
                                                            &raw_cane2.id,
                                                            &mill.id,
                                                            brand_id,
                                                            &mill.process_cost,
                                                            "max",
                                                        ),
                                                    );
                                                } else {
                                                    if raw_cane1_available < raw_cane2_available {
                                                        actions.push(
                                                            Self::get_produce_sugar_from_mixed_raw_action(
                                                                &formula.raw_cane1.id,
                                                                &raw_cane2.id,
                                                                &mill.id,
                                                                brand_id,
                                                                &mill.process_cost,
                                                                "resource",
                                                            ),
                                                        );
                                                    } else {
                                                        actions.push(
                                                            Self::get_produce_sugar_from_mixed_raw_action(
                                                                &formula.raw_cane1.id,
                                                                &raw_cane2.id,
                                                                &mill.id,
                                                                brand_id,
                                                                &mill.process_cost,
                                                                "resource"
                                                            ),
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    None => {
                                        if let Some(raw_cane_available) =
                                            mill.raw_cane_storage.get(&formula.raw_cane1.id)
                                        {
                                            if *raw_cane_available > 0 {
                                                actions.push(
                                                    Self::get_produce_sugar_from_single_raw_action(
                                                        &formula.raw_cane1.id,
                                                        &mill.id,
                                                        brand_id,
                                                        &mill.process_cost,
                                                        "single",
                                                    ),
                                                );
                                                if *raw_cane_available > mill.max_produce {
                                                    actions.push(
                                                        Self::get_produce_sugar_from_single_raw_action(
                                                            &formula.raw_cane1.id,
                                                            &mill.id,
                                                            brand_id,
                                                            &mill.process_cost,
                                                            "max",
                                                        ),
                                                    );
                                                } else {
                                                    actions.push(
                                                        Self::get_produce_sugar_from_single_raw_action(
                                                            &formula.raw_cane1.id,
                                                            &mill.id,
                                                            brand_id,
                                                            &mill.process_cost,
                                                            "resource",
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                actions.push(Self::get_setting_machine_action(&mill.id));
            }
        }
        actions
    }

    pub fn get_switch_production_process_action(mill_id: &String, brand_id: &String) -> Action {
        let mut parameters = HashMap::new();
        let action_name = format!("switch_production_in_{}_to_{}", mill_id, brand_id);
        parameters.insert("mill".to_string(), Value::Text(mill_id.clone()));
        parameters.insert("brand".to_string(), Value::Text(brand_id.clone()));
        Action::new(action_name, 1, parameters)
    }
    pub fn get_place_order_action(mill_id: &String, cane_id: &String) -> Action {
        let mut parameters = HashMap::new();
        let action_name = format!("placed_order_for_{}_with_{}", mill_id, cane_id);
        parameters.insert("mill".to_string(), Value::Text(mill_id.clone()));
        parameters.insert("cane".to_string(), Value::Text(cane_id.clone()));
        Action::new(action_name, 1, parameters)
    }
    pub fn get_possible_place_order_or_switch_production_actions(
        &self,
        state: &State,
    ) -> Vec<Action> {
        let mut actions = Vec::new();

        for mill in state.mills.values() {
            for (cane_id, amount) in mill.raw_cane_storage.iter() {
                if *amount == 0 {
                    actions.push(Self::get_place_order_action(&mill.id, cane_id));
                }
            }
            if mill.max_changes > 0 {
                for (brand_id, brands) in self.changing_brands.iter() {
                    if mill.current_produce == *brand_id {
                        for brand in brands {
                            if brand != brand_id && *mill.can_produce.get(brand).unwrap_or(&false) {
                                actions.push(Self::get_switch_production_process_action(
                                    &mill.id, brand,
                                ));
                            }
                        }
                    }
                }
            }
        }
        actions
    }

    pub fn get_possible_harvest_cane_action(
        farmfield_id: &String,
        destination_id: &String, // Can be a mill or truck
        row_cane_id: &String,
        cane_yield: &i32,
        destination_type: &str, // "mill" or "truck"
    ) -> Action {
        let mut parameters = HashMap::new();
        let action_name = format!(
            "harvest_cane_{}_in_{}_to_{}_{}",
            row_cane_id, farmfield_id, destination_type, destination_id
        );

        parameters.insert("farmfield".to_string(), Value::Text(farmfield_id.clone()));
        parameters.insert(
            "destination".to_string(),
            Value::Text(destination_id.clone()),
        );
        parameters.insert("cane".to_string(), Value::Text(row_cane_id.clone()));
        parameters.insert(
            "destination_type".to_string(),
            Value::Text(destination_type.to_string()),
        );

        Action::new(action_name, *cane_yield, parameters)
    }

    pub fn get_possible_harvest_cane_actions(state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for mill in state.mills.values() {
            for raw_cane_id in &mill.place_order {
                for farmfield in state.farmfields.values() {
                    if farmfield.total_canes >= farmfield.cane_yield
                        && *farmfield.near_mills.get(&mill.id).unwrap_or(&false)
                        && *farmfield.raw_canes.get(raw_cane_id).unwrap_or(&false)
                    {
                        actions.push(Self::get_possible_harvest_cane_action(
                            &farmfield.id,
                            &mill.id,
                            raw_cane_id,
                            &farmfield.cane_yield,
                            "mill",
                        ));
                    }
                }
            }
        }
        for truck in state.trucks.values() {
            for farmfield in state.farmfields.values() {
                if truck.location == farmfield.location
                    && truck.remaining_capacity >= farmfield.cane_yield
                    && farmfield.total_canes >= farmfield.cane_yield
                {
                    for (raw_cane_id, is_available) in farmfield.raw_canes.iter() {
                        if *is_available {
                            actions.push(Self::get_possible_harvest_cane_action(
                                &farmfield.id,
                                &truck.id,
                                raw_cane_id,
                                &farmfield.cane_yield,
                                "truck",
                            ));
                        }
                    }
                }
            }
        }

        actions
    }

    pub fn get_possible_raw_cane_transfer_action(
        mill1_id: &String,
        mill2_id: &String,
        row_cane_id: &String,
        transfer_type: &str, // "transfer" or "aggregate"
    ) -> Action {
        let mut parameters = HashMap::new();
        let action_name = format!(
            "raw_cane_{}_between_mills_from_{}_to_{}",
            row_cane_id, mill1_id, mill2_id
        );
        parameters.insert(
            "between_mills".to_string(),
            Value::Text(transfer_type.to_string()),
        );
        parameters.insert("mill1".to_string(), Value::Text(mill1_id.clone()));
        parameters.insert("mill2".to_string(), Value::Text(mill2_id.clone()));
        parameters.insert("cane".to_string(), Value::Text(row_cane_id.clone()));

        Action::new(action_name, 1, parameters)
    }

    pub fn get_possible_raw_cane_between_mills_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for mill1 in state.mills.values() {
            for raw_cane_id in &mill1.place_order {
                for mill2 in state.mills.values() {
                    if mill1.id != mill2.id
                        && *mill2.raw_cane_storage.get(raw_cane_id).unwrap_or(&0) > 0
                        && *self
                            .connected_mills
                            .get(&mill1.id)
                            .and_then(|mills_map| mills_map.get(&mill2.id))
                            .unwrap_or(&false)
                    {
                        actions.push(Self::get_possible_raw_cane_transfer_action(
                            &mill1.id,
                            &mill2.id,
                            raw_cane_id,
                            "transfer",
                        ));
                        actions.push(Self::get_possible_raw_cane_transfer_action(
                            &mill1.id,
                            &mill2.id,
                            raw_cane_id,
                            "aggregate",
                        ));
                    }
                }
            }
        }

        actions
    }

    pub fn get_possible_load_truck_action(
        place_id: &String, // Mill or Depot ID
        brand_id: &String,
        truck_id: &String,
        load_method: &str,         // "crane" or "manual"
        place_type: &str,          // "mill" or "depot"
        crane_id: Option<&String>, // Some(crane_id) if using crane, None if manual
    ) -> Action {
        let mut parameters = HashMap::new();

        let action_name = if load_method == "crane" {
            format!(
                "brand_{}_loaded_with_{}_to_{}_from_{}_{}",
                brand_id,
                crane_id.unwrap(),
                truck_id,
                place_type,
                place_id
            )
        } else {
            format!(
                "brand_{}_loaded_to_{}_from_{}_{}",
                brand_id, truck_id, place_type, place_id
            )
        };

        parameters.insert(
            "load_truck".to_string(),
            Value::Text(load_method.to_string()),
        );
        parameters.insert("place".to_string(), Value::Text(place_id.clone()));
        parameters.insert("brand".to_string(), Value::Text(brand_id.clone()));
        parameters.insert("truck".to_string(), Value::Text(truck_id.clone()));
        parameters.insert(
            "place_type".to_string(),
            Value::Text(place_type.to_string()),
        );

        if let Some(crane) = crane_id {
            parameters.insert("crane".to_string(), Value::Text(crane.clone()));
        }

        Action::new(action_name, 1, parameters)
    }

    pub fn get_possible_load_truck_actions(state: &State) -> Vec<Action> {
        let mut actions = Vec::new();

        for truck in state.trucks.values() {
            if truck.remaining_capacity > 0 {
                // Check mills
                for mill in state.mills.values() {
                    if truck.location == mill.location {
                        for (brand_id, amount) in mill.brand_storage.iter() {
                            if *amount > 0 {
                                actions.push(Self::get_possible_load_truck_action(
                                    &mill.id, brand_id, &truck.id, "manual", "mill", None,
                                ));
                            }
                            for crane in state.cranes.values() {
                                if crane.location == truck.location
                                    && crane.maintenance > 0
                                    && truck.remaining_capacity > crane.capacity
                                    && *amount > crane.capacity
                                {
                                    actions.push(Self::get_possible_load_truck_action(
                                        &mill.id,
                                        brand_id,
                                        &truck.id,
                                        "crane",
                                        "mill",
                                        Some(&crane.id),
                                    ));
                                }
                            }
                        }
                    }
                }

                // Check depots
                for depot in state.depots.values() {
                    if truck.location == depot.location {
                        for (brand_id, amount) in depot.brand_storage.iter() {
                            if *amount > 0 {
                                actions.push(Self::get_possible_load_truck_action(
                                    &depot.id, brand_id, &truck.id, "manual", "depot", None,
                                ));
                            }
                            for crane in state.cranes.values() {
                                if crane.location == truck.location
                                    && crane.maintenance > 0
                                    && truck.remaining_capacity > crane.capacity
                                    && *amount > crane.capacity
                                {
                                    actions.push(Self::get_possible_load_truck_action(
                                        &depot.id,
                                        brand_id,
                                        &truck.id,
                                        "crane",
                                        "depot",
                                        Some(&crane.id),
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }

        actions
    }

    pub fn get_possible_drive_truck_action(from: &i32, to: &i32, truck_id: &String) -> Action {
        let mut parameters = HashMap::new();
        let action_name = format!("drive_truck_{}_from_{}_to_{}", truck_id, from, to);
        parameters.insert("to".to_string(), Value::Int(*to));
        parameters.insert("truck".to_string(), Value::Text(truck_id.clone()));
        Action::new(action_name, 1, parameters)
    }

    pub fn get_possible_drive_truck_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for truck in state.trucks.values() {
            for mill1 in state.mills.values() {
                for mill2 in state.mills.values() {
                    if mill1.id != mill2.id {
                        if truck.location == mill1.location {
                            if *self
                                .connected
                                .get(&mill1.id)
                                .and_then(|map| map.get(&mill2.id))
                                .unwrap_or(&false)
                            {
                                actions.push(Self::get_possible_drive_truck_action(
                                    &mill1.location,
                                    &mill2.location,
                                    &truck.id,
                                ));
                            }
                        }
                        if truck.location == mill2.location {
                            if *self
                                .connected
                                .get(&mill2.id)
                                .and_then(|map| map.get(&mill1.id))
                                .unwrap_or(&false)
                            {
                                actions.push(Self::get_possible_drive_truck_action(
                                    &mill2.location,
                                    &mill1.location,
                                    &truck.id,
                                ));
                            }
                        }
                    }
                }
                for depot in state.depots.values() {
                    if truck.location == mill1.location {
                        if *self
                            .connected
                            .get(&mill1.id)
                            .and_then(|map| map.get(&depot.id))
                            .unwrap_or(&false)
                        {
                            actions.push(Self::get_possible_drive_truck_action(
                                &mill1.location,
                                &depot.location,
                                &truck.id,
                            ));
                        }
                    }
                    if truck.location == depot.location {
                        if *self
                            .connected
                            .get(&depot.id)
                            .and_then(|map| map.get(&mill1.id))
                            .unwrap_or(&false)
                        {
                            actions.push(Self::get_possible_drive_truck_action(
                                &depot.location,
                                &mill1.location,
                                &truck.id,
                            ));
                        }
                    }
                }
            }
        }
        actions
    }

    pub fn get_possible_maintainence_crane_actions(state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for crane in state.cranes.values() {
            if crane.maintenance == 0 {
                let mut parameters = HashMap::new();
                let action_name = format!("maintainence_{}", crane.id);
                parameters.insert("crane".to_string(), Value::Text(crane.id.clone()));
                actions.push(Action::new(action_name, 1, parameters));
            }
        }
        actions
    }

    pub fn get_possible_unload_truck_action(
        place_id: &String, // Now works for both mills & depots
        truck_id: &String,
        brand_id: &String,
        mode: &str,       // "single" or "max"
        place_type: &str, // "mill" or "depot"
    ) -> Action {
        let mut parameters = HashMap::new();
        let action_name = format!(
            "unload_brand_{}_from_{}_to_{}_{}",
            brand_id, truck_id, place_type, place_id
        );

        parameters.insert("place".to_string(), Value::Text(place_id.clone())); // Generic place ID
        parameters.insert("truck".to_string(), Value::Text(truck_id.clone()));
        parameters.insert("brand".to_string(), Value::Text(brand_id.clone()));
        parameters.insert("unload".to_string(), Value::Text(mode.to_string()));
        parameters.insert(
            "place_type".to_string(),
            Value::Text(place_type.to_string()),
        ); // Mill or Depot

        Action::new(action_name, 1, parameters)
    }

    pub fn get_possible_unload_truck_actions(state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        for truck in state.trucks.values() {
            for (brand_id, amount) in truck.brand_load.iter() {
                if *amount > 0 {
                    for mill in state.mills.values() {
                        if mill.location == truck.location {
                            actions.push(Self::get_possible_unload_truck_action(
                                &mill.id, &truck.id, &brand_id, "single", "mill",
                            ));
                            actions.push(Self::get_possible_unload_truck_action(
                                &mill.id, &truck.id, &brand_id, "max", "mill",
                            ));
                        }
                    }
                    for depot in state.depots.values() {
                        if depot.location == truck.location {
                            actions.push(Self::get_possible_unload_truck_action(
                                &depot.id, &truck.id, &brand_id, "single", "depot",
                            ));
                            actions.push(Self::get_possible_unload_truck_action(
                                &depot.id, &truck.id, &brand_id, "max", "depot",
                            ));
                        }
                    }
                }
            }
        }
        actions
    }
}
impl Problem for SugarProblem {
    type State = State;
    fn get_possible_actions(&self, state: &Self::State) -> Vec<Action> {
        let mut actions = Vec::new();
        actions.extend(self.get_possible_available_or_produce_sugar_actions(state));
        actions.extend(self.get_possible_place_order_or_switch_production_actions(state));
        actions.extend(Self::get_possible_harvest_cane_actions(state));
        actions.extend(self.get_possible_raw_cane_between_mills_actions(state));
        actions.extend(self.get_possible_drive_truck_actions(state));
        actions.extend(Self::get_possible_load_truck_actions(state));
        actions.extend(Self::get_possible_unload_truck_actions(state));
        actions.extend(Self::get_possible_maintainence_crane_actions(state));

        actions
    }
    fn apply_action(&self, state: &Self::State, action: &Action) -> Self::State {
        todo!("complete")
    }
    fn is_goal_state(&self, state: &Self::State) -> bool {
        for condition in &self.goal.conditions {
            if let Some(depot) = state.depots.get(&condition.depot_id) {
                if let Some(&stored_amount) = depot.brand_storage.get(&condition.brand_id) {
                    // If the stored amount is less than required, return false
                    if stored_amount < condition.storage_amount {
                        return false;
                    }
                } else {
                    // Brand is not present in the depot at all, goal not met
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
    fn heuristic(&self, _state: &Self::State) -> f64 {
        0.0
    }
    fn load_state_from_json(json_path: &str) -> (Self::State, Self) {
        todo!("complete")
    }
}
