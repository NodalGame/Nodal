pub mod solution_check {
    use crate::structs::active::{
        active_node::active_node::ActiveNode, active_set::active_set::ActiveSet,
    };

    /// Checks if a puzzle is solved based on its active state.
    pub fn is_puzzle_solved(active_nodes: Vec<ActiveNode>, active_sets: Vec<ActiveSet>) -> bool {
        for active_node in active_nodes {
            if !active_node.satisfied {
                return false;
            }
            for condition in &active_node.active_conditions {
                if !condition.satisfied {
                    return false;
                }
            }
            for connected_condition in &active_node.active_connected_conditions {
                if !connected_condition.satisfied {
                    return false;
                }
            }
        }
        for active_set in active_sets {
            for active_set_rule in &active_set.active_set_rules {
                if !active_set_rule.satisfied {
                    return false;
                }
            }
            for active_connected_set_rule in &active_set.active_connected_set_rules {
                if !active_connected_set_rule.satisfied {
                    return false;
                }
            }
        }
        true
    }
}
