use crate::problems::problem::Problem;
use crate::search::{action::Action, state::StateTrait, state::Value};
use serde::{Deserialize, Serialize};
use serde_json::from_reader;
use serde_json::Value as JsonValue;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Arm {
    pub is_free: bool,
    pub side: i32, // 0 for left, 1 for right
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bot {
    pub location: i32,
    pub load_limit: i32,
    pub current_load: i32,
    pub index: i32,
    pub arms: Vec<Arm>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Item {
    pub location: i32,
    pub weight: i32,
    pub in_arm: i32,  // -1 if not in arm
    pub in_tray: i32, // -1 if not in tray
    pub index: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub bots: Vec<Bot>,
    pub items: Vec<Item>,
    pub room_connections: BTreeMap<i32, Vec<i32>>,
    pub cost: i32,
}

impl StateTrait for State {}

pub struct DeliveryProblem {
    goal_locations: BTreeMap<i32, i32>, // item_id -> target_room_id
}

impl DeliveryProblem {
    pub fn possible_move_action(bot: i32, from: i32, to: i32) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("bot".to_string(), Value::Int(bot));
        parameters.insert("from".to_string(), Value::Int(from));
        parameters.insert("to".to_string(), Value::Int(to));
        Action::new(
            format!("move_bot_{}_from_{}_to_{}", bot, from, to),
            3,
            parameters,
        )
    }

    pub fn possible_pick_action(item: i32, room: i32, arm: i32, bot: i32) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("item".to_string(), Value::Int(item));
        parameters.insert("room".to_string(), Value::Int(room));
        parameters.insert("arm".to_string(), Value::Int(arm));
        parameters.insert("bot".to_string(), Value::Int(bot));
        Action::new(
            format!("pick_item_{}_arm_{}_bot_{}", item, arm, bot),
            2,
            parameters,
        )
    }

    pub fn possible_drop_action(item: i32, room: i32, arm: i32, bot: i32) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("item".to_string(), Value::Int(item));
        parameters.insert("room".to_string(), Value::Int(room));
        parameters.insert("arm".to_string(), Value::Int(arm));
        parameters.insert("bot".to_string(), Value::Int(bot));
        Action::new(
            format!("drop_item_{}_arm_{}_bot_{}", item, arm, bot),
            2,
            parameters,
        )
    }

    pub fn possible_to_tray_action(item: i32, arm: i32, bot: i32) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("item".to_string(), Value::Int(item));
        parameters.insert("arm".to_string(), Value::Int(arm));
        parameters.insert("bot".to_string(), Value::Int(bot));
        Action::new(
            format!("to_tray_item_{}_arm_{}_bot_{}", item, arm, bot),
            1,
            parameters,
        )
    }

    pub fn possible_from_tray_action(item: i32, arm: i32, bot: i32) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("item".to_string(), Value::Int(item));
        parameters.insert("arm".to_string(), Value::Int(arm));
        parameters.insert("bot".to_string(), Value::Int(bot));
        Action::new(
            format!("from_tray_item_{}_arm_{}_bot_{}", item, arm, bot),
            1,
            parameters,
        )
    }

    pub fn apply_move_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let bot_index = match action.parameters.get("bot").unwrap() {
            Value::Int(val) => *val,
            _ => panic!("Expected integer value for bot"),
        };
        let to = match action.parameters.get("to").unwrap() {
            Value::Int(val) => *val,
            _ => panic!("Expected integer value for destination"),
        };

        new_state
            .bots
            .iter_mut()
            .find(|bot| bot.index == bot_index)
            .unwrap()
            .location = to;
        new_state.cost += 3;
        new_state
    }

    pub fn apply_pick_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let item_index = match action.parameters.get("item").unwrap() {
            Value::Int(val) => *val,
            _ => panic!("Expected integer value for item"),
        };
        let arm_side = match action.parameters.get("arm").unwrap() {
            Value::Int(val) => *val,
            _ => panic!("Expected integer value for arm"),
        };
        let bot_index = match action.parameters.get("bot").unwrap() {
            Value::Int(val) => *val,
            _ => panic!("Expected integer value for bot"),
        };

        let item = new_state
            .items
            .iter_mut()
            .find(|item| item.index == item_index)
            .unwrap();
        let bot = new_state
            .bots
            .iter_mut()
            .find(|bot| bot.index == bot_index)
            .unwrap();
        let arm = bot
            .arms
            .iter_mut()
            .find(|arm| arm.side == arm_side)
            .unwrap();

        item.in_arm = arm_side;
        arm.is_free = false;
        bot.current_load += item.weight;
        new_state.cost += 2;
        new_state
    }

    pub fn apply_drop_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let item_index = match action.parameters.get("item").unwrap() {
            Value::Int(val) => *val,
            _ => panic!("Expected integer value for item"),
        };
        let room = match action.parameters.get("room").unwrap() {
            Value::Int(val) => *val,
            _ => panic!("Expected integer value for room"),
        };
        let arm_side = match action.parameters.get("arm").unwrap() {
            Value::Int(val) => *val,
            _ => panic!("Expected integer value for arm"),
        };
        let bot_index = match action.parameters.get("bot").unwrap() {
            Value::Int(val) => *val,
            _ => panic!("Expected integer value for bot"),
        };

        let item = new_state
            .items
            .iter_mut()
            .find(|item| item.index == item_index)
            .unwrap();
        let bot = new_state
            .bots
            .iter_mut()
            .find(|bot| bot.index == bot_index)
            .unwrap();
        let arm = bot
            .arms
            .iter_mut()
            .find(|arm| arm.side == arm_side)
            .unwrap();

        item.in_arm = -1;
        item.location = room;
        arm.is_free = true;
        bot.current_load -= item.weight;
        new_state.cost += 2;
        new_state
    }

    pub fn apply_to_tray_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let item_index = match action.parameters.get("item").unwrap() {
            Value::Int(val) => *val,
            _ => panic!("Expected integer value for item"),
        };
        let arm_side = match action.parameters.get("arm").unwrap() {
            Value::Int(val) => *val,
            _ => panic!("Expected integer value for arm"),
        };
        let bot_index = match action.parameters.get("bot").unwrap() {
            Value::Int(val) => *val,
            _ => panic!("Expected integer value for bot"),
        };

        let item = new_state
            .items
            .iter_mut()
            .find(|item| item.index == item_index)
            .unwrap();
        let bot = new_state
            .bots
            .iter_mut()
            .find(|bot| bot.index == bot_index)
            .unwrap();
        let arm = bot
            .arms
            .iter_mut()
            .find(|arm| arm.side == arm_side)
            .unwrap();

        item.in_arm = -1;
        item.in_tray = bot_index;
        arm.is_free = true;
        new_state.cost += 1;
        new_state
    }

    pub fn apply_from_tray_action(state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let item_index = match action.parameters.get("item").unwrap() {
            Value::Int(val) => *val,
            _ => panic!("Expected integer value for item"),
        };
        let arm_side = match action.parameters.get("arm").unwrap() {
            Value::Int(val) => *val,
            _ => panic!("Expected integer value for arm"),
        };
        let bot_index = match action.parameters.get("bot").unwrap() {
            Value::Int(val) => *val,
            _ => panic!("Expected integer value for bot"),
        };

        let item = new_state
            .items
            .iter_mut()
            .find(|item| item.index == item_index)
            .unwrap();
        let bot = new_state
            .bots
            .iter_mut()
            .find(|bot| bot.index == bot_index)
            .unwrap();
        let arm = bot
            .arms
            .iter_mut()
            .find(|arm| arm.side == arm_side)
            .unwrap();

        item.in_tray = -1;
        item.in_arm = arm_side;
        arm.is_free = false;
        new_state.cost += 1;
        new_state
    }
}

impl Problem for DeliveryProblem {
    type State = State;

    fn get_possible_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();

        // Move actions
        for bot in &state.bots {
            if let Some(connected_rooms) = state.room_connections.get(&bot.location) {
                for &next_room in connected_rooms {
                    actions.push(Self::possible_move_action(
                        bot.index,
                        bot.location,
                        next_room,
                    ));
                }
            }
        }

        // Pick actions - only for items in rooms (not in arms or trays)
        for bot in &state.bots {
            for item in &state.items {
                if bot.location == item.location && item.in_arm == -1 && item.in_tray == -1 {
                    for arm in &bot.arms {
                        if arm.is_free {
                            let new_load = bot.current_load + item.weight;
                            if new_load <= bot.load_limit {
                                actions.push(Self::possible_pick_action(
                                    item.index,
                                    item.location,
                                    arm.side,
                                    bot.index,
                                ));
                            }
                        }
                    }
                }
            }
        }

        // Drop actions - only for items in arms
        for bot in &state.bots {
            for arm in &bot.arms {
                if !arm.is_free {
                    let item = state
                        .items
                        .iter()
                        .find(|item| item.in_arm == arm.side)
                        .unwrap();
                    actions.push(Self::possible_drop_action(
                        item.index,
                        bot.location,
                        arm.side,
                        bot.index,
                    ));
                }
            }
        }

        // To-tray actions - from arm to tray
        for bot in &state.bots {
            for arm in &bot.arms {
                if !arm.is_free {
                    let item = state
                        .items
                        .iter()
                        .find(|item| item.in_arm == arm.side)
                        .unwrap();
                    actions.push(Self::possible_to_tray_action(
                        item.index, arm.side, bot.index,
                    ));
                }
            }
        }

        // From-tray actions - from tray to free arm
        for bot in &state.bots {
            for item in &state.items {
                if item.in_tray == bot.index {
                    for arm in &bot.arms {
                        if arm.is_free {
                            actions.push(Self::possible_from_tray_action(
                                item.index, arm.side, bot.index,
                            ));
                        }
                    }
                }
            }
        }

        actions
    }

    fn apply_action(&self, state: &State, action: &Action) -> State {
        if action.name.starts_with("move_") {
            Self::apply_move_action(state, action)
        } else if action.name.starts_with("pick_") {
            Self::apply_pick_action(state, action)
        } else if action.name.starts_with("drop_") {
            Self::apply_drop_action(state, action)
        } else if action.name.starts_with("to_tray_") {
            Self::apply_to_tray_action(state, action)
        } else if action.name.starts_with("from_tray_") {
            Self::apply_from_tray_action(state, action)
        } else {
            panic!("Unknown action");
        }
    }

    fn load_state_from_json(json_path: &str) -> (State, Self) {
        let file = File::open(json_path).expect("Failed to open JSON file");
        let reader = BufReader::new(file);
        let json_data: JsonValue = from_reader(reader).expect("Failed to parse JSON");

        let bots: Vec<Bot> = json_data["bots"]
            .as_array()
            .unwrap()
            .iter()
            .map(|bot| Bot {
                location: bot["location"].as_i64().unwrap() as i32,
                load_limit: bot["load_limit"].as_i64().unwrap() as i32,
                current_load: bot["current_load"].as_i64().unwrap() as i32,
                index: bot["index"].as_i64().unwrap() as i32,
                arms: bot["arms"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|arm| Arm {
                        is_free: arm["is_free"].as_bool().unwrap(),
                        side: arm["side"].as_i64().unwrap() as i32,
                    })
                    .collect(),
            })
            .collect();

        let items: Vec<Item> = json_data["items"]
            .as_array()
            .unwrap()
            .iter()
            .map(|item| Item {
                location: item["location"].as_i64().unwrap() as i32,
                weight: item["weight"].as_i64().unwrap() as i32,
                in_arm: item["in_arm"].as_i64().unwrap() as i32,
                in_tray: item["in_tray"].as_i64().unwrap() as i32,
                index: item["index"].as_i64().unwrap() as i32,
            })
            .collect();

        let room_connections: BTreeMap<i32, Vec<i32>> = json_data["room_connections"]
            .as_object()
            .unwrap()
            .iter()
            .map(|(k, v)| {
                (
                    k.parse::<i32>().unwrap(),
                    v.as_array()
                        .unwrap()
                        .iter()
                        .map(|x| x.as_i64().unwrap() as i32)
                        .collect(),
                )
            })
            .collect();

        let cost = json_data["cost"].as_i64().unwrap() as i32;

        let goal_locations: BTreeMap<i32, i32> = json_data["goal_locations"]
            .as_object()
            .unwrap()
            .iter()
            .map(|(k, v)| (k.parse::<i32>().unwrap(), v.as_i64().unwrap() as i32))
            .collect();

        (
            State {
                bots,
                items,
                room_connections,
                cost,
            },
            Self { goal_locations },
        )
    }

    fn is_goal_state(&self, state: &State) -> bool {
        self.goal_locations.iter().all(|(&item, &target_room)| {
            state
                .items
                .iter()
                .any(|i| i.index == item && i.location == target_room)
        })
    }

    fn heuristic(&self, state: &State) -> f64 {
        0.0 // Can be improved to estimate minimum cost to goal
    }
}
