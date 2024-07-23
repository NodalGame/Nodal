pub mod connected_condition_checks {
    use std::{collections::{HashMap, HashSet}, usize};

    use crate::structs::immutable::{
        game_node::game_node::{GameNode, GameNodeId},
        solution::solution::{solution_to_adjacency_matrix, Solution},
    };

    /// Checks if all nodes with the same condition and class have the same
    /// number of connected edges.
    ///
    /// # Parameters
    ///
    /// - `nodes`: Nodes of the same condition and class in the puzzle.
    /// - `solution`: Proposed solution in the puzzle to check against.
    ///
    /// # Returns
    ///
    /// Returns true if all nodes have equal degree, otherwise false.  
    pub fn is_degree_equal(nodes: Vec<&GameNode>, solution: &Solution) -> bool {
        let adj_matrix = solution_to_adjacency_matrix(solution);

        // This is safe since max degree is 8 in this game
        let mut degree: u8 = u8::MAX;
        for node in nodes.iter() {
            let node_degree = adj_matrix.get(&node.id).unwrap_or(&HashSet::new()).len() as u8;
            if degree == u8::MAX {
                degree = node_degree;
                if degree == 0 {
                    return false;
                }
            } else {
                if degree != node_degree {
                    return false;
                }
            }
        }

        return true;
    }

    /// Checks if all nodes with the same condition and class have the same
    /// shortest distance between each of them.
    ///
    /// # Parameters
    ///
    /// - `nodes`: Nodes of the same condition and class in the puzzle.
    /// - `solution`: Proposed solution in the puzzle to check against.
    ///
    /// # Returns
    ///
    /// Returns true if all nodes have equal distance, otherwise false.
    pub fn is_distance_equal(nodes: Vec<&GameNode>, solution: &Solution) -> bool {
        let adj_matrix = solution_to_adjacency_matrix(solution);

        // If only one node, check if it has any connections
        if nodes.len() == 1 {
            return adj_matrix.get(&nodes[0].id).is_some();
        }

        let mut distances_map: HashMap<GameNodeId, HashMap<GameNodeId, usize>> = HashMap::new();
    
        // Distance to itself is 0
        for node in &nodes {
            distances_map.insert(node.id, HashMap::from([(node.id, 0)]));
        }
    
        // Weight of every edge between nodes is 1
        for (node_u, neighbors_u) in adj_matrix {
            for node_v in neighbors_u {
                let mut distances_u: HashMap<GameNodeId, usize> = distances_map.get(&node_u).unwrap_or(&HashMap::new()).clone();
                distances_u.insert(node_v, 1);
                distances_map.insert(node_u, distances_u.clone());
            }
        }
    
        // Floyd-Warshall algorithm
        let distances: Vec<GameNodeId> = distances_map.keys().cloned().collect();
        for &k in &distances {
            for &i in &distances {
                for &j in &distances {
                    let new_dist = {
                        let dist_i = distances_map.get(&i).and_then(|m| m.get(&k)).copied().unwrap_or(usize::MAX);
                        let dist_j = distances_map.get(&k).and_then(|m| m.get(&j)).copied().unwrap_or(usize::MAX);
                        if dist_i == usize::MAX || dist_j == usize::MAX {
                            usize::MAX
                        } else {
                            dist_i.saturating_add(dist_j)
                        }
                    };
    
                    if new_dist < *distances_map.get(&i).and_then(|m| m.get(&j)).unwrap_or(&usize::MAX) {
                        distances_map.entry(i).or_insert_with(HashMap::new).insert(j, new_dist);
                    }
                }
            }
        }

        println!("got distance map {:?}", distances_map);

        // Verify that all distances of the relevant nodes are the same
        let mut common_distance: usize = 0;
        for node_u in &nodes {
            for node_v in &nodes {
                if node_u.id != node_v.id {
                    let distance_v = distances_map.get(&node_u.id).and_then(|m| m.get(&node_v.id)).copied().unwrap_or(usize::MAX);
                    if common_distance == 0 {
                        common_distance = distance_v;
                    } else if distance_v != common_distance {
                        return false;
                    }
                }
            }
        }

        // Final check to verify they aren't all disconnected 
        common_distance != usize::MAX
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        logic::connected_condition_checks::connected_condition_checks::{
            is_degree_equal, is_distance_equal,
        },
        structs::immutable::{
            connected_node_condition::connected_node_condition::{
                ConditionClass, ConnectedNodeCondition,
            },
            game_line::game_line::GameLine,
            game_node::game_node::{GameNode, GameNodeId},
            solution::solution::Solution,
        },
    };

    fn get_test_node(id: GameNodeId, con_cdtns: Vec<ConnectedNodeCondition>) -> GameNode {
        GameNode {
            id: id,
            conditions: Vec::new(),
            connected_conditions: con_cdtns,
        }
    }

    #[test]
    fn test_degree_equal_one_node_with_no_connections_returns_false() {
        let node = get_test_node(
            0,
            [ConnectedNodeCondition::DegreeEqual(ConditionClass::Blue)].to_vec(),
        );

        let solution = Solution::new();

        assert!(!is_degree_equal(Vec::from([&node]), &solution))
    }

    #[test]
    fn test_degree_equal_one_node_with_many_connections_returns_true() {
        let node = get_test_node(
            0,
            [ConnectedNodeCondition::DegreeEqual(ConditionClass::Blue)].to_vec(),
        );

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

        assert!(is_degree_equal(Vec::from([&node]), &solution))
    }

    #[test]
    fn test_degree_equal_two_nodes_with_no_connections_returns_false() {
        let node_a = get_test_node(
            0,
            [ConnectedNodeCondition::DegreeEqual(ConditionClass::Blue)].to_vec(),
        );
        let node_b = get_test_node(
            1,
            [ConnectedNodeCondition::DegreeEqual(ConditionClass::Blue)].to_vec(),
        );

        let solution = Solution::new();

        assert!(!is_degree_equal(Vec::from([&node_a, &node_b]), &solution))
    }

    #[test]

    fn test_degree_equal_two_nodes_with_two_connections_returns_true() {
        let node_a = get_test_node(
            0,
            [ConnectedNodeCondition::DegreeEqual(ConditionClass::Blue)].to_vec(),
        );
        let node_b = get_test_node(
            1,
            [ConnectedNodeCondition::DegreeEqual(ConditionClass::Blue)].to_vec(),
        );

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
                node_a_id: 1,
                node_b_id: 2,
            },
        ]);

        assert!(is_degree_equal(Vec::from([&node_a, &node_b]), &solution))
    }

    #[test]
    fn test_degree_equal_two_nodes_with_different_degree_returns_false() {
        let node_a = get_test_node(
            0,
            [ConnectedNodeCondition::DegreeEqual(ConditionClass::Blue)].to_vec(),
        );
        let node_b = get_test_node(
            1,
            [ConnectedNodeCondition::DegreeEqual(ConditionClass::Blue)].to_vec(),
        );

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

        assert!(!is_degree_equal(Vec::from([&node_a, &node_b]), &solution))
    }

    #[test]
    fn test_distance_equal_multiple_nodes_with_some_distance_equal_returns_false() {
        let node_a = get_test_node(
            1, 
        [ConnectedNodeCondition::DistanceEqual(ConditionClass::Blue)].to_vec(),
        );
        let node_b = get_test_node(
            3,
            [ConnectedNodeCondition::DistanceEqual(ConditionClass::Blue)].to_vec(),
        );
        let node_c = get_test_node(
            2,
            [ConnectedNodeCondition::DistanceEqual(ConditionClass::Blue)].to_vec(),
        );
        let node_d = get_test_node(
            0,
            [ConnectedNodeCondition::DistanceEqual(ConditionClass::Blue)].to_vec(),
        );

        let solution = Solution::from([
            GameLine {
                node_a_id: 1,
                node_b_id: 3,
            },
        ]);

        assert!(!is_distance_equal(Vec::from([&node_d, &node_a, &node_b, &node_c]), &solution))
    }

    #[test]
    fn test_distance_equal_one_node_with_no_connections_returns_false() {
        let node = get_test_node(
            0,
            [ConnectedNodeCondition::DistanceEqual(ConditionClass::Blue)].to_vec(),
        );

        let solution = Solution::new();

        assert!(!is_distance_equal(Vec::from([&node]), &solution))
    }

    #[test]
    fn test_distance_equal_one_node_with_many_connections_returns_true() {
        let node = get_test_node(
            0,
            [ConnectedNodeCondition::DistanceEqual(ConditionClass::Blue)].to_vec(),
        );

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

        assert!(is_distance_equal(Vec::from([&node]), &solution))
    }

    #[test]
    fn test_distance_equal_two_nodes_connected_returns_true() {
        let node_a = get_test_node(
            0,
            [ConnectedNodeCondition::DistanceEqual(ConditionClass::Blue)].to_vec(),
        );
        let node_b = get_test_node(
            1,
            [ConnectedNodeCondition::DistanceEqual(ConditionClass::Blue)].to_vec(),
        );

        let solution = Solution::from([GameLine {
            node_a_id: 0,
            node_b_id: 1,
        }]);

        assert!(is_distance_equal(Vec::from([&node_a, &node_b]), &solution))
    }

    #[test]
    fn test_distance_equal_two_nodes_disconnected_returns_false() {
        let node_a = get_test_node(
            0,
            [ConnectedNodeCondition::DistanceEqual(ConditionClass::Blue)].to_vec(),
        );
        let node_b = get_test_node(
            1,
            [ConnectedNodeCondition::DistanceEqual(ConditionClass::Blue)].to_vec(),
        );

        let solution = Solution::from([
            GameLine {
                node_a_id: 0,
                node_b_id: 2,
            },
            GameLine {
                node_a_id: 1,
                node_b_id: 3,
            },
        ]);

        assert!(!is_distance_equal(Vec::from([&node_a, &node_b]), &solution))
    }

    #[test]
    fn test_distance_equal_three_nodes_same_distance_returns_true() {
        let node_a = get_test_node(
            0,
            [ConnectedNodeCondition::DistanceEqual(ConditionClass::Blue)].to_vec(),
        );
        let node_b = get_test_node(
            1,
            [ConnectedNodeCondition::DistanceEqual(ConditionClass::Blue)].to_vec(),
        );
        let node_c = get_test_node(
            2,
            [ConnectedNodeCondition::DistanceEqual(ConditionClass::Blue)].to_vec(),
        );

        let solution = Solution::from([
            GameLine {
                node_a_id: 0,
                node_b_id: 3,
            },
            GameLine {
                node_a_id: 1,
                node_b_id: 3,
            },
            GameLine {
                node_a_id: 2,
                node_b_id: 3,
            },
        ]);

        assert!(is_distance_equal(
            Vec::from([&node_a, &node_b, &node_c]),
            &solution
        ))
    }

    #[test]
    fn test_distance_equal_three_nodes_different_distance_returns_false() {
        let node_a = get_test_node(
            0,
            [ConnectedNodeCondition::DistanceEqual(ConditionClass::Blue)].to_vec(),
        );
        let node_b = get_test_node(
            1,
            [ConnectedNodeCondition::DistanceEqual(ConditionClass::Blue)].to_vec(),
        );
        let node_c = get_test_node(
            2,
            [ConnectedNodeCondition::DistanceEqual(ConditionClass::Blue)].to_vec(),
        );

        let solution = Solution::from([
            GameLine {
                node_a_id: 0,
                node_b_id: 3,
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
                node_a_id: 0,
                node_b_id: 2,
            },
        ]);

        assert!(!is_distance_equal(
            Vec::from([&node_a, &node_b, &node_c]),
            &solution
        ))
    }

    #[test]
    fn test_distance_equal_three_nodes_same_distance_multiple_paths_longer_distance_returns_true() {
        let node_a = get_test_node(
            0,
            [ConnectedNodeCondition::DistanceEqual(ConditionClass::Blue)].to_vec(),
        );
        let node_b = get_test_node(
            1,
            [ConnectedNodeCondition::DistanceEqual(ConditionClass::Blue)].to_vec(),
        );
        let node_c = get_test_node(
            2,
            [ConnectedNodeCondition::DistanceEqual(ConditionClass::Blue)].to_vec(),
        );

        let solution = Solution::from([
            GameLine {
                node_a_id: 0,
                node_b_id: 3,
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
                node_a_id: 0,
                node_b_id: 4,
            },
            GameLine {
                node_a_id: 3,
                node_b_id: 4,
            },
        ]);

        assert!(is_distance_equal(
            Vec::from([&node_a, &node_b, &node_c]),
            &solution
        ))
    }
}
