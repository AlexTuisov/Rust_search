use crate::search::{action::Action, node::Node, state::StateTrait};
pub struct SearchTree<S: StateTrait> {
    pub nodes:  Vec<Node>, // topology only
    pub states: Vec<S>,    // state payloads, kept at the same indices
}

impl<S: StateTrait> SearchTree<S> {
    pub fn new(initial_state: S) -> Self {
        Self { nodes: vec![Node::new_root()], states: vec![initial_state] }
    }

    pub fn get_node(&self, idx: usize) -> Option<&Node>        { self.nodes.get(idx) }
    pub fn get_state(&self, idx: usize) -> Option<&S>          { self.states.get(idx) }

    // Add a new node to the tree given a parent index and an action
    pub fn add_node<F>(&mut self,
                       parent_idx: usize,
                       action: Action,
                       apply_action: F) -> usize
    where
        F: Fn(&S, &Action) -> S,
    {
        let parent_state = &self.states[parent_idx];
        let new_state    = apply_action(parent_state, &action);
        let new_cost     = self.nodes[parent_idx].cost + action.cost;

        let new_idx = self.nodes.len();
        self.nodes.push(Node { parent: Some(parent_idx),
            children: Vec::new(),
            action: Some(action),
            cost: new_cost });
        self.states.push(new_state);
        self.nodes[parent_idx].children.push(new_idx);
        new_idx
    }

    pub fn trace_actions(&self, node_index: usize) -> Vec<Action> {
        let mut actions = Vec::new();
        let mut current_index = Some(node_index);
        while let Some(index) = current_index {
            if let Some(node) = self.get_node(index) {
                if let Some(action) = &node.action {
                    actions.push(action.clone());
                }
                current_index = node.parent;
            } else {
                break;
            }
        }
        actions.reverse();
        actions
    }

    pub fn expand_node<F, G>(&mut self,
                             idx: usize,
                             get_actions: F,
                             apply_action: G) -> Vec<usize>
    where
        F: Fn(&S) -> Vec<Action>,
        G: Fn(&S, &Action) -> S,
    {
        let mut succ = Vec::new();
        if let Some(state) = self.get_state(idx) {
            for act in get_actions(state) {
                succ.push(self.add_node(idx, act, &apply_action));
            }
        }
        succ
    }

    pub fn print_tree(&self, node_index: usize, indent: usize) {
        if let Some(node) = self.get_node(node_index) {
            // Print the current node details with indentation to show hierarchy
            println!(
                "{:indent$}Node Index: {}, Cost: {}, Action: {:?}",
                "",
                node_index,
                node.cost,
                node.action.as_ref().map(|a| &a.name),
                indent = indent
            );

            // Recursively print all children of the current node
            for &child_index in &node.children {
                self.print_tree(child_index, indent + 4); // Increase indent for child nodes
            }
        }
    }
}
