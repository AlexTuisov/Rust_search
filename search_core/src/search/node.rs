// use std::cell::RefCell;
// use std::rc::{Rc, Weak};
use crate::search::{action::Action, state::StateTrait};


#[derive(Debug, Clone, PartialEq)]
pub struct Node<S: StateTrait> {
    pub state: S,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
    pub action: Option<Action>,
    pub cost: i32,
}

impl<S: StateTrait> Node<S> {
    pub fn new_empty(state: S) -> Self {
        Node {
            state,
            parent: None,
            children: Vec::new(),
            action: None,
            cost: 0,
        }
    }
}


