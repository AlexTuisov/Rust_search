# Black-Box Planning Framework
## Overview
This is a generic planning framework that solves various state-based problems using search algorithms like BFS, DFS, and A*. Define your custom problem by implementing the `Problem` trait, and the framework will handle the rest.

## Key features
- **Modular problem definition:** Implement the `Problem` trait to define initial states, actions, goal conditions, and heuristics.
- **Generic Search Algorithms:** Supports BFS, DFS, A*, and more.
- **Black-Box Approach:** You provide the problem logic; the framework manages the search and solution.

## Getting Started
1. **Create a Problem:** Implement the `Problem` trait in your own Rust file.
2. **Configure Initial State:** Set up your initial input via JSON or other methods.
3. **Run the solver:**  Use `main.rs` to execute the solver.
