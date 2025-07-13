// use std::cell::RefCell;
// use std::rc::{Rc, Weak};
use crate::search::{action::Action, state::StateTrait};

pub struct Node {
    pub parent:   Option<usize>,
    pub children: Vec<usize>,
    pub action:   Option<Action>,
    pub cost:     i32,
}

impl Node {
    pub fn new_root() -> Self {
        Self { parent: None, children: Vec::new(), action: None, cost: 0 }
    }
}
