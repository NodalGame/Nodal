pub mod connected_condition_checks {
    use crate::structs::immutable::{
        game_node::game_node::GameNode,
        solution::solution::{solution_to_adjacency_matrix, Solution},
    };

    /// Checks if all other nodes with the same condition and class have the same
    /// number of connected edges.
    ///
    /// NOTE: Currently does NOT consider Scope.
    pub fn is_degree_equal(nodes: Vec<&GameNode>, solution: &Solution) -> bool {
        let adj_matrix = solution_to_adjacency_matrix(solution);

        // This is safe since max degree is 8 in this game
        let mut degree: u8 = u8::MAX;
        for node in nodes.iter() {
            let node_degree = adj_matrix.get(&node.id).unwrap_or(&Vec::new()).len() as u8;
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
}

#[cfg(test)]
mod tests {
    use crate::{
        logic::connected_condition_checks::connected_condition_checks::is_degree_equal,
        structs::immutable::{
            connected_node_condition::connected_node_condition::{
                ConditionClass, ConnectedNodeCondition,
            },
            game_line::game_line::GameLine,
            game_node::game_node::{GameNode, GameNodeId},
            solution::{self, solution::Solution},
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
    fn test_degree_equal_two_nodes_same_class_with_no_connections_returns_false() {
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

    fn test_degree_equal_two_nodes_same_class_with_two_connections_returns_true() {
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
    fn test_degree_equal_two_nodes_same_class_with_different_degree_returns_false() {
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
}
