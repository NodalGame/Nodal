pub mod connected_rule_checks {
    use std::collections::HashSet;
    use crate::structs::immutable::{game_node::game_node::GameNodeId, game_set::game_set::GameSet, solution::{self, solution::Solution}};

    /// Checks if all sets with the same rule and class are homomorphic.
    /// 
    /// # Parameters
    /// 
    /// - `sets`: Sets of the same rule and class in the puzzle. 
    /// - `solution`: Proposed solution in the puzzle to check against. 
    /// 
    /// # Returns
    /// 
    /// Returns true if all sets are homomorphic, otherwise false. 
    pub fn is_homomorphism(sets: Vec<&GameSet>, solution: &Solution) -> bool {
        // If set I homo J and J homo K, then I homo K. So we check sets (i, i+1) for all sets as homomorphic.
        for (set_i, set_j) in sets.iter().zip(sets.iter().skip(1)) {
            // This should be static for a puzzle, since it's unsolvable if there are sets with different sizes.
            if set_i.nodes.len() != set_j.nodes.len() {
                return false;
            }

            // Generate all mappings from nodes in I to nodes in J.
            let mut node_map_i_j: HashSet<(GameNodeId, GameNodeId)> = HashSet::new();
            for node_i in set_i.nodes.iter() {
                for node_j in set_j.nodes.iter() {
                    node_map_i_j.insert((*node_i, *node_j));
                }
            }

            // TODO im tired
        }

        return true;
    }
}