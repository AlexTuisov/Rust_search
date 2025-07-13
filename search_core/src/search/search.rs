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
    use std::collections::{HashSet, hash_map::DefaultHasher};
    use std::hash::{Hash, Hasher};

    queue.insert(0, 0, f64::MAX);
    let mut closed: HashSet<u64> = HashSet::new();
    let (mut nodes, mut uniq) = (0, 0);

    while let Some(cur_idx) = queue.pop() {
        for &succ_idx in &tree.expand_node(cur_idx, &get_possible_actions, &apply_action) {
            nodes += 1;
            let state = tree.get_state(succ_idx).unwrap();

            // hash-based closed list
            let mut h = DefaultHasher::new();
            state.hash(&mut h);
            if !closed.insert(h.finish()) { continue; }

            uniq += 1;
            if is_goal(state) {
                println!("nodes: {nodes}, unique: {uniq}");
                return Ok(tree.trace_actions(succ_idx));
            }

            queue.insert(succ_idx,
                         tree.get_node(succ_idx).unwrap().cost,
                         heuristic(state));
        }
    }
    Err("No solution found")
}
