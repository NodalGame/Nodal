pub mod connected_condition_checks {
    use std::collections::{HashMap, HashSet};

    use crate::structs::immutable::{
        game_node::game_node::GameNode,
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

        let start_nodes: Vec<u16> = adj_matrix.keys().cloned().collect();
        let num_nodes = start_nodes.len();
        let mut index_map = HashMap::new();
        for (i, &node) in start_nodes.iter().enumerate() {
            index_map.insert(node, i);
        }

        let mut dist = vec![vec![usize::MAX; num_nodes]; num_nodes];
        for i in 0..num_nodes {
            dist[i][i] = 0;
        }

        for (u, neighbors) in adj_matrix {
            let u_idx = index_map[&u];
            for v in neighbors {
                let v_idx = index_map[&v];
                dist[u_idx][v_idx] = 1;
                dist[v_idx][u_idx] = 1; // because the graph is undirected
            }
        }

        // Floyd-Warshall algorithm
        for k in 0..num_nodes {
            for i in 0..num_nodes {
                for j in 0..num_nodes {
                    if dist[i][k] != usize::MAX && dist[k][j] != usize::MAX {
                        dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                    }
                }
            }
        }

        // Extract distances for the subset
        let mut distances = vec![];
        for i in 0..nodes.len() {
            for j in (i + 1)..nodes.len() {
                let u_idx = index_map[&nodes[i].id];
                let v_idx = index_map[&nodes[j].id];
                distances.push(dist[u_idx][v_idx]);
            }
        }

        // Check if all distances are the same
        distances[0] != usize::MAX && distances.iter().all(|&d| d == distances[0])
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
