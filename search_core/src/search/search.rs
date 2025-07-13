use crate::algorithms::priority_queue::PriorityQueue;
use crate::search::action::Action;
use crate::search::search_tree::SearchTree;
use crate::search::state::StateTrait;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;


// Generic search function that operates on a SearchTree and uses a priority queue for the search strategy


pub fn generic_search<F, G, H, Q, I, S>(
    tree: &mut SearchTree<S>,
    get_possible_actions: F,
    apply_action: G,
    is_goal: H,
    mut queue: Q,
    heuristic: I,
) -> Result<Vec<Action>, &'static str>
where
    F: Fn(&S) -> Vec<Action>,
    G: Fn(&S, &Action) -> S,
    H: Fn(&S) -> bool,
    Q: PriorityQueue,
    I: Fn(&S) -> f64,
    S: StateTrait + Hash,
{
    queue.insert(0, 0, f64::MAX);
    let mut closed_list: HashSet<u64> = HashSet::new();
    let mut node_count = 0;
    let mut unique_node_count = 0;

    while let Some(current_index) = queue.pop() {
        let successors = tree.expand_node(current_index, &get_possible_actions, &apply_action);
        for &successor_index in &successors {
            node_count += 1;
            let successor_node = tree.get_node(successor_index).unwrap();

            // Hash the state for the closed list
            let mut hasher = DefaultHasher::new();
            successor_node.state.hash(&mut hasher);
            let state_hash = hasher.finish();

            // Check if the state is already in the closed list
            if !closed_list.insert(state_hash) {
                continue;
            }

            unique_node_count += 1;
            if is_goal(&successor_node.state) {
                println!("Number of nodes created: {}", node_count);
                println!("Number of unique nodes created: {}", unique_node_count);
                return Ok(tree.trace_actions(successor_index));
            }

            let heuristic_value = heuristic(&successor_node.state);
            queue.insert(successor_index, successor_node.cost, heuristic_value);
        }
    }

    Err("No solution found")
}
