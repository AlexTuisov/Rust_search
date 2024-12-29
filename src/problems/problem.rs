use crate::search::{node::Node, state::StateTrait, action::Action};
use std::rc::Rc;

pub trait Problem {
    type State: StateTrait; // Associated type for State

    fn get_possible_actions(&self, state: &Self::State) -> Vec<Action>;
    fn apply_action(&self, state: &Self::State, action: &Action) -> Self::State;
    fn is_goal_state(&self, state: &Self::State) -> bool;
    fn heuristic(&self, state: &Self::State) -> f64;
    fn load_state_from_json(json_path: &str) -> (Self::State, Self);
}


