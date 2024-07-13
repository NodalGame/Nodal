pub mod condition_checks {
    use std::collections::{HashMap, HashSet, VecDeque};

    use crate::structs::immutable::{
        game_node::game_node::{GameNode, GameNodeId},
        solution::solution::{solution_to_adjacency_matrix, Solution},
    };

    /// Checks if all nodes connected to the start node are of equal distance.
    ///
    /// NOTE: Currently does NOT consider Scope.
    pub fn is_branch_equal(node: &GameNode, solution: &Solution) -> bool {
        let adj_matrix = solution_to_adjacency_matrix(solution);

        // Check if the node has no neighbors
        if adj_matrix
            .get(&node.id)
            .unwrap_or(&HashSet::new())
            .is_empty()
        {
            return false;
        }

        // Perform DFS and keep track of the deepest, returning false if depth is ever
        // different at the lowest node in a branch (no neighbors besides its parent).
        let mut visited: HashSet<GameNodeId> = HashSet::new();
        let mut stack: VecDeque<(GameNodeId, GameNodeId, u16)> = VecDeque::new(); // Store (node, parent, depth).
        let mut deepest: u16 = 0;

        stack.push_back((node.id, node.id, 0));
        visited.insert(node.id);

        while let Some((node_id, parent_id, node_depth)) = stack.pop_back() {
            if let Some(neighbors) = adj_matrix.get(&node_id) {
                // If no neighbors (parent will always exist), it's a leaf node and we check depth
                if neighbors.len() == 1 {
                    if deepest == 0 {
                        deepest = node_depth;
                    } else if node_depth != deepest {
                        return false;
                    }
                }
                for &neighbor in neighbors {
                    if visited.contains(&neighbor) {
                        if neighbor != parent_id {
                            // If it ran into a visited node that isn't the parent, it's a cycle
                            return false;
                        }
                    } else {
                        // Unvisited nodes pushed back with depth incremented
                        visited.insert(neighbor);
                        stack.push_back((neighbor, node_id, node_depth + 1));
                    }
                }
            }
        }

        true
    }

    pub fn is_leaf(node: &GameNode, solution: &Solution) -> bool {
        solution_to_adjacency_matrix(solution)
            .get(&node.id)
            .unwrap_or(&HashSet::new())
            .len()
            == 1
    }

    pub fn is_internal(node: &GameNode, solution: &Solution) -> bool {
        solution_to_adjacency_matrix(solution)
            .get(&node.id)
            .unwrap_or(&HashSet::new())
            .len()
            > 1
    }

    pub fn is_cycle(node: &GameNode, solution: &Solution) -> bool {
        let adj_matrix = solution_to_adjacency_matrix(solution);

        let mut visited = HashSet::new();

        for &start_node in adj_matrix.keys() {
            if !visited.contains(&start_node) {
                let mut stack = VecDeque::new();
                let mut parent_map = HashMap::new();

                stack.push_back((start_node, None));

                while let Some((curr_node, parent)) = stack.pop_back() {
                    if !visited.contains(&curr_node) {
                        visited.insert(curr_node);
                        parent_map.insert(curr_node, parent);

                        if let Some(neighbors) = adj_matrix.get(&curr_node) {
                            for &neighbor in neighbors {
                                if Some(neighbor) != parent {
                                    if visited.contains(&neighbor) {
                                        // Found a cycle, backtrack to collect all nodes in the cycle
                                        let mut cycle_node = curr_node;
                                        let mut cycle = HashSet::new();
                                        cycle.insert(cycle_node);

                                        while let Some(&p) = parent_map.get(&cycle_node) {
                                            cycle.insert(p.unwrap());
                                            if p.unwrap() == neighbor {
                                                break;
                                            }
                                            cycle_node = p.unwrap();
                                        }

                                        for cycle_node in cycle {
                                            if cycle_node == node.id {
                                                return true;
                                            }
                                        }
                                    } else {
                                        stack.push_back((neighbor, Some(curr_node)));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        logic::condition_checks::condition_checks::{
            is_branch_equal, is_cycle, is_internal, is_leaf,
        },
        structs::immutable::{
            game_line::game_line::GameLine, game_node::game_node::GameNode,
            node_condition::node_condition::NodeCondition, solution::solution::Solution,
        },
    };

    fn get_test_node(conditions: Vec<NodeCondition>) -> GameNode {
        GameNode {
            id: 0,
            conditions: conditions,
            connected_conditions: Vec::new(),
        }
    }

    #[test]
    fn test_branch_equal_node_with_no_connections_returns_false() {
        let node = get_test_node([NodeCondition::BranchEqual].to_vec());

        let solution = Solution::new();

        assert!(!is_branch_equal(&node, &solution))
    }

    #[test]
    fn test_branch_equal_node_with_one_connection_returns_true() {
        let node = get_test_node([NodeCondition::BranchEqual].to_vec());

        let solution = Solution::from([GameLine {
            node_a_id: 0,
            node_b_id: 1,
        }]);

        assert!(is_branch_equal(&node, &solution))
    }

    #[test]
    fn test_branch_equal_node_with_one_branch_returns_true() {
        let node = get_test_node([NodeCondition::BranchEqual].to_vec());

        let solution = Solution::from([
            GameLine {
                node_a_id: 0,
                node_b_id: 1,
            },
            GameLine {
                node_a_id: 1,
                node_b_id: 2,
            },
            GameLine {
                node_a_id: 2,
                node_b_id: 3,
            },
        ]);

        assert!(is_branch_equal(&node, &solution))
    }

    #[test]
    fn test_branch_equal_node_with_multiple_length_one_branches_returns_true() {
        let node = get_test_node([NodeCondition::BranchEqual].to_vec());

        let solution = Solution::from([
            GameLine {
                node_a_id: 0,
                node_b_id: 1,
            },
            GameLine {
                node_a_id: 0,
                node_b_id: 2,
            },
            GameLine {
                node_a_id: 0,
                node_b_id: 3,
            },
        ]);

        assert!(is_branch_equal(&node, &solution))
    }

    #[test]
    fn test_branch_equal_node_with_multiple_branches_with_subbranches_same_length_returns_true() {
        let node = get_test_node([NodeCondition::BranchEqual].to_vec());

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
                node_a_id: 1,
                node_b_id: 4,
            },
            GameLine {
                node_a_id: 0,
                node_b_id: 2,
            },
            GameLine {
                node_a_id: 2,
                node_b_id: 5,
            },
        ]);

        assert!(is_branch_equal(&node, &solution))
    }

    #[test]
    fn test_branch_equal_node_with_length_one_and_length_two_branch_returns_false() {
        let node = get_test_node([NodeCondition::BranchEqual].to_vec());

        let solution = Solution::from([
            GameLine {
                node_a_id: 0,
                node_b_id: 1,
            },
            GameLine {
                node_a_id: 1,
                node_b_id: 2,
            },
            GameLine {
                node_a_id: 0,
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

        assert!(!is_branch_equal(&node, &solution))
    }

    #[test]
    fn test_branch_equal_node_in_cycle_returns_false() {
        let node = get_test_node([NodeCondition::BranchEqual].to_vec());

        let solution = Solution::from([
            GameLine {
                node_a_id: 0,
                node_b_id: 1,
            },
            GameLine {
                node_a_id: 1,
                node_b_id: 2,
            },
            GameLine {
                node_a_id: 2,
                node_b_id: 3,
            },
            GameLine {
                node_a_id: 0,
                node_b_id: 3,
            },
        ]);

        assert!(!is_branch_equal(&node, &solution))
    }

    #[test]
    fn test_branch_equal_node_with_branch_containing_cycle_returns_false() {
        let node = get_test_node([NodeCondition::BranchEqual].to_vec());

        let solution = Solution::from([
            GameLine {
                node_a_id: 0,
                node_b_id: 1,
            },
            GameLine {
                node_a_id: 1,
                node_b_id: 2,
            },
            GameLine {
                node_a_id: 2,
                node_b_id: 3,
            },
            GameLine {
                node_a_id: 1,
                node_b_id: 3,
            },
        ]);

        assert!(!is_branch_equal(&node, &solution))
    }

    #[test]
    fn test_leaf_node_no_neighbors_returns_false() {
        let node = get_test_node([NodeCondition::Leaf].to_vec());

        let solution = Solution::new();

        assert!(!is_leaf(&node, &solution))
    }

    #[test]
    fn test_leaf_one_neighbor_returns_true() {
        let node = get_test_node([NodeCondition::Leaf].to_vec());

        let solution = Solution::from([GameLine {
            node_a_id: 0,
            node_b_id: 1,
        }]);

        assert!(is_leaf(&node, &solution))
    }

    #[test]
    fn test_leaf_two_neighbors_returns_false() {
        let node = get_test_node([NodeCondition::Leaf].to_vec());

        let solution = Solution::from([
            GameLine {
                node_a_id: 0,
                node_b_id: 1,
            },
            GameLine {
                node_a_id: 0,
                node_b_id: 2,
            },
        ]);

        assert!(!is_leaf(&node, &solution))
    }

    #[test]
    fn test_internal_node_no_neighbors_returns_false() {
        let node = get_test_node([NodeCondition::Internal].to_vec());

        let solution = Solution::new();

        assert!(!is_internal(&node, &solution))
    }

    #[test]
    fn test_internal_one_neighbor_returns_false() {
        let node = get_test_node([NodeCondition::Internal].to_vec());

        let solution = Solution::from([GameLine {
            node_a_id: 0,
            node_b_id: 1,
        }]);

        assert!(!is_internal(&node, &solution))
    }

    #[test]
    fn test_internal_two_neighbors_returns_true() {
        let node = get_test_node([NodeCondition::Internal].to_vec());

        let solution = Solution::from([
            GameLine {
                node_a_id: 0,
                node_b_id: 1,
            },
            GameLine {
                node_a_id: 0,
                node_b_id: 2,
            },
        ]);

        assert!(is_internal(&node, &solution))
    }

    #[test]
    fn test_cycle_no_neighbors_returns_false() {
        let node = get_test_node([NodeCondition::Cycle].to_vec());

        let solution = Solution::new();

        assert!(!is_cycle(&node, &solution))
    }

    #[test]
    fn test_cycle_leaf_returns_false() {
        let node = get_test_node([NodeCondition::Cycle].to_vec());

        let solution = Solution::from([GameLine {
            node_a_id: 0,
            node_b_id: 1,
        }]);

        assert!(!is_cycle(&node, &solution))
    }

    #[test]
    fn test_cycle_in_branch_no_cycle_returns_false() {
        let node = get_test_node([NodeCondition::Cycle].to_vec());

        let solution = Solution::from([
            GameLine {
                node_a_id: 0,
                node_b_id: 1,
            },
            GameLine {
                node_a_id: 0,
                node_b_id: 2,
            },
        ]);

        assert!(!is_cycle(&node, &solution))
    }

    #[test]
    fn test_cycle_in_cycle_returns_true() {
        let node = get_test_node([NodeCondition::Cycle].to_vec());

        let solution = Solution::from([
            GameLine {
                node_a_id: 0,
                node_b_id: 1,
            },
            GameLine {
                node_a_id: 1,
                node_b_id: 2,
            },
            GameLine {
                node_a_id: 2,
                node_b_id: 3,
            },
            GameLine {
                node_a_id: 0,
                node_b_id: 3,
            },
        ]);

        assert!(is_cycle(&node, &solution))
    }

    #[test]
    fn test_cycle_on_branch_connected_to_cycle_returns_false() {
        let node = get_test_node([NodeCondition::Cycle].to_vec());

        let solution = Solution::from([
            GameLine {
                node_a_id: 1,
                node_b_id: 3,
            },
            GameLine {
                node_a_id: 1,
                node_b_id: 2,
            },
            GameLine {
                node_a_id: 2,
                node_b_id: 3,
            },
            GameLine {
                node_a_id: 0,
                node_b_id: 1,
            },
        ]);

        assert!(!is_cycle(&node, &solution))
    }

    #[test]
    fn test_cycle_in_cycle_connected_to_branch_returns_true() {
        let node = get_test_node([NodeCondition::Cycle].to_vec());

        let solution = Solution::from([
            GameLine {
                node_a_id: 0,
                node_b_id: 1,
            },
            GameLine {
                node_a_id: 1,
                node_b_id: 2,
            },
            GameLine {
                node_a_id: 2,
                node_b_id: 0,
            },
            GameLine {
                node_a_id: 1,
                node_b_id: 3,
            },
        ]);

        assert!(is_cycle(&node, &solution))
    }
}
