use crate::search::{node::Node, state::StateTrait, action::Action, state::Value};
use crate::algorithms::priority_queue::PriorityQueue;
pub struct SearchTree<S: StateTrait> {
    pub nodes: Vec<Node<S>>, // A vector to store all nodes
}

impl<S: StateTrait> SearchTree<S> {
    // Create a new empty tree with an initial node
    pub fn new(initial_state: S) -> Self {
        let root = Node {
            state: initial_state,
            parent: None,
            children: Vec::new(),
            action: None,
            cost: 0,
        };

        SearchTree {
            nodes: vec![root], // Add the root node to the nodes vector
        }
    }

    // Add a new node to the tree given a parent index and an action
    pub fn add_node<F>(&mut self, parent_index: usize, action: Action, apply_action: F) -> usize
    where
        F: Fn(&S, &Action) -> S,
    {
        let parent_node = &self.nodes[parent_index];
        let new_state = apply_action(&parent_node.state, &action);
        let new_cost = parent_node.cost + action.cost;

        let new_node = Node {
            state: new_state,
            parent: Some(parent_index),
            children: Vec::new(),
            action: Some(action),
            cost: new_cost,
        };

        let new_node_index = self.nodes.len();
        self.nodes.push(new_node);

        // Update the parent's children list
        self.nodes[parent_index].children.push(new_node_index);

        new_node_index
    }

    // Get the node by its index
    pub fn get_node(&self, index: usize) -> Option<&Node<S>> {
        self.nodes.get(index)
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

    pub fn expand_node<F, G>(&mut self, node_index: usize, get_possible_actions: F, apply_action: G) -> Vec<usize>
    where
        F: Fn(&S) -> Vec<Action>,
        G: Fn(&S, &Action) -> S,
    {
        let mut successors = Vec::new();
        if let Some(node) = self.get_node(node_index) {
            let actions = get_possible_actions(&node.state);
            for action in actions {
                let new_node_index = self.add_node(node_index, action, &apply_action);
                successors.push(new_node_index);
            }
        }
        successors
    }

    pub fn print_tree(&self, node_index: usize, indent: usize) {
        if let Some(node) = self.get_node(node_index) {
            // Print the current node details with indentation to show hierarchy
            println!(
                "{:indent$}Node Index: {}, Cost: {}, Action: {:?}, State: {:?}",
                "",
                node_index,
                node.cost,
                node.action.as_ref().map(|a| &a.name),
                node.state,
                indent = indent
            );

            // Recursively print all children of the current node
            for &child_index in &node.children {
                self.print_tree(child_index, indent + 4); // Increase indent for child nodes
            }
        }
    }
}




