pub mod connected_rule_checks {
    use crate::structs::immutable::{
        game_node::game_node::GameNodeId,
        game_set::game_set::GameSet,
        solution::{
            self,
            solution::{
                filter_solution_to_set, solution_to_adjacency_matrix, AdjacencyMatrix, Solution,
            },
        },
    };
    use std::{
        collections::{HashMap, HashSet},
        hash::Hash,
    };
    use itertools::Itertools;

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
    pub fn are_homomorphic(sets: Vec<&GameSet>, solution: &Solution) -> bool {
        // If set I homo J and J homo K, then I homo K. So we check sets (i, i+1) for all sets as homomorphic.
        for (set_i, set_j) in sets.iter().zip(sets.iter().skip(1)) {
            // This should be static for a puzzle, since it's unsolvable if there are sets with different sizes.
            if set_i.nodes.len() != set_j.nodes.len() {
                return false;
            }

            // Get adj matrices of only solutions within sets I and J.
            let set_i_sol = solution_to_adjacency_matrix(&filter_solution_to_set(set_i, solution));
            let set_j_sol = solution_to_adjacency_matrix(&filter_solution_to_set(set_j, solution));

            // If either don't have any lines, return false.
            if set_i_sol.is_empty() || set_j_sol.is_empty() {
                return false;
            }

            // Generate all mappings from nodes in I to nodes in J.
            let mappings = generate_all_mappings(&set_i.nodes, &set_j.nodes);

            // Check if any mapping is a homomorphism.
            let mut homomorphism = false;
            for mapping in mappings {
                if is_homomorphism(&mapping, &set_i_sol, &set_j_sol) {
                    println!("Homomorphism found {:?} with set_i {:?} and set_j {:?}", mapping, set_i_sol, set_j_sol);
                    homomorphism = true;
                    break;
                }
            }

            if !homomorphism {
                return false;
            }
        }

        return true;
    }

    // TODO this should be run and cached for each set if it has homomorphism when puzzle is loaded.
    fn generate_all_mappings(
        set_i: &Vec<GameNodeId>,
        set_j: &Vec<GameNodeId>,
    ) -> Vec<HashMap<GameNodeId, GameNodeId>> {
        let mut all_mappings = Vec::new();
        let set_j_perms = set_j.iter().cloned().permutations(set_i.len());
        for perm in set_j_perms {
            let mut mapping: HashMap<GameNodeId, GameNodeId> = HashMap::new();
            for (i, j) in set_i.iter().enumerate() {
                mapping.insert(*j, perm[i]);
            }
            all_mappings.push(mapping);
        }
        all_mappings
    }

    fn is_homomorphism(
        mapping: &HashMap<GameNodeId, GameNodeId>,
        set_i_sol: &AdjacencyMatrix,
        set_j_sol: &AdjacencyMatrix,
    ) -> bool {
        for (node, neighbors) in set_i_sol {
            for &neighbor in neighbors {
                let mapped_i = mapping.get(node).unwrap();
                let mapped_j = mapping.get(&neighbor).unwrap();
                if let Some(set_j_neighbors) = set_j_sol.get(mapped_i) {
                    if !set_j_neighbors.contains(mapped_j) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        logic::connected_rule_checks::connected_rule_checks::are_homomorphic,
        structs::immutable::{
            connected_set_rule::connected_set_rule::{ConnectedSetRule, RuleClass},
            game_line::game_line::GameLine,
            game_set::game_set::GameSet,
            solution::{self, solution::Solution},
        },
    };

    fn get_test_set(num_nodes: u16, first_node: u16, con_rules: Vec<ConnectedSetRule>) -> GameSet {
        GameSet {
            nodes: (first_node..num_nodes + first_node).collect(),
            id: 0,
            rules: Vec::new(),
            connected_rules: con_rules,
            bounded: false,
        }
    }

    #[test]
    fn test_homomorphic_sets_different_number_nodes_returns_false() {
        let set_i = get_test_set(
            3,
            0,
            [ConnectedSetRule::Homomorphic(RuleClass::Yellow)].to_vec(),
        );
        let set_j = get_test_set(
            4,
            0,
            [ConnectedSetRule::Homomorphic(RuleClass::Yellow)].to_vec(),
        );

        let solution = Solution::new();

        assert!(!are_homomorphic([&set_i, &set_j].to_vec(), &solution));
    }

    #[test]
    fn test_homomorphic_sets_empty_solution_returns_false() {
        let set_i = get_test_set(
            3,
            0,
            [ConnectedSetRule::Homomorphic(RuleClass::Yellow)].to_vec(),
        );
        let set_j = get_test_set(
            3,
            0,
            [ConnectedSetRule::Homomorphic(RuleClass::Yellow)].to_vec(),
        );

        let solution = Solution::new();

        assert!(!are_homomorphic([&set_i, &set_j].to_vec(), &solution));
    }

    #[test]
    fn test_homomorphic_sets_single_line_per_set_returns_true() {
        let set_i = get_test_set(
            3,
            0,
            [ConnectedSetRule::Homomorphic(RuleClass::Yellow)].to_vec(),
        );
        let set_j = get_test_set(
            3,
            3,
            [ConnectedSetRule::Homomorphic(RuleClass::Yellow)].to_vec(),
        );

        let solution = Solution::from([
            GameLine {
                node_a_id: 0,
                node_b_id: 1,
            },
            GameLine {
                node_a_id: 3,
                node_b_id: 4,
            },
        ]);

        assert!(are_homomorphic([&set_i, &set_j].to_vec(), &solution));
    }

    #[test]
    fn test_homomorphic_sets_overlapping_sets_horseshoe_returns_true() {
        let set_i = get_test_set(
            4,
            0,
            [ConnectedSetRule::Homomorphic(RuleClass::Yellow)].to_vec(),
        );
        let set_j = get_test_set(
            4,
            2,
            [ConnectedSetRule::Homomorphic(RuleClass::Yellow)].to_vec(),
        );

        let solution = Solution::from([
            GameLine {
                node_a_id: 0,
                node_b_id: 1,
            },
            GameLine {
                node_a_id: 1,
                node_b_id: 3,
            },
            GameLine {
                node_a_id: 2,
                node_b_id: 3,
            },
            GameLine {
                node_a_id: 3,
                node_b_id: 4,
            },
            GameLine {
                node_a_id: 4,
                node_b_id: 5,
            },
        ]);

        assert!(are_homomorphic([&set_i, &set_j].to_vec(), &solution));
    }

    #[test]
    fn test_homomorphic_sets_overlapping_sets_one_single_one_two_lines_returns_false() {
        let set_i = get_test_set(
            4,
            0,
            [ConnectedSetRule::Homomorphic(RuleClass::Yellow)].to_vec(),
        );
        let set_j = get_test_set(
            4,
            2,
            [ConnectedSetRule::Homomorphic(RuleClass::Yellow)].to_vec(),
        );

        let solution = Solution::from([
            GameLine {
                node_a_id: 0,
                node_b_id: 1,
            },
            GameLine {
                node_a_id: 1,
                node_b_id: 3,
            },
            GameLine {
                node_a_id: 4,
                node_b_id: 5,
            },
        ]);

        assert!(!are_homomorphic([&set_i, &set_j].to_vec(), &solution));
    }
}
