pub mod rule_checks {

    use crate::structs::immutable::{
        game_set::game_set::GameSet,
        solution::solution::{solution_to_adjacency_matrix, Solution},
    };

    /// Checks if there are any nodes in the set which directly connect to a node also in the set.
    pub fn is_disconnected(set: &GameSet, solution: &Solution) -> bool {
        if solution.is_empty() {
            return false;
        }

        let adj_matrix = solution_to_adjacency_matrix(solution);

        for (node, neighbors) in adj_matrix.iter() {
            for neighbor in neighbors.iter() {
                if set.nodes.contains(neighbor) && set.nodes.contains(node) {
                    return false;
                }
            }
        }

        true
    }

    /// Checks that there is exactly one node within the set that connects to a node outside the set.
    pub fn is_leaf(set: &GameSet, solution: &Solution) -> bool {
        let adj_matrix = solution_to_adjacency_matrix(solution);

        let mut external_connection = false;
        for (node, neighbors) in adj_matrix.iter() {
            for neighbor in neighbors.iter() {
                if !set.nodes.contains(neighbor) && set.nodes.contains(node) {
                    if external_connection == true {
                        return false;
                    }
                    external_connection = true;
                }
            }
        }

        external_connection
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        logic::rule_checks::rule_checks::{is_disconnected, is_leaf},
        structs::immutable::{
            game_line::game_line::GameLine,
            game_node::game_node::GameNode,
            game_set::game_set::GameSet,
            set_rule::set_rule::SetRule,
            solution::{self, solution::Solution},
        },
    };

    fn get_test_set(num_nodes: u16, rules: Vec<SetRule>) -> GameSet {
        GameSet {
            nodes: (0..num_nodes).collect(),
            id: 0,
            rules: rules,
            connected_rules: Vec::new(),
        }
    }

    #[test]
    fn test_disconnected_no_connected_nodes_returns_false() {
        let set = get_test_set(4, [SetRule::Disconnected].to_vec());

        let solution = Solution::new();

        assert!(!is_disconnected(&set, &solution));
    }

    #[test]
    fn test_disconnected_no_connected_nodes_inside_set_returns_true() {
        let set = get_test_set(4, [SetRule::Disconnected].to_vec());

        let solution = Solution::from([GameLine {
            node_a_id: 5,
            node_b_id: 6,
        }]);

        assert!(is_disconnected(&set, &solution));
    }

    #[test]
    fn test_disconnected_connected_node_outside_set_returns_true() {
        let set = get_test_set(4, [SetRule::Disconnected].to_vec());

        let solution = Solution::from([GameLine {
            node_a_id: 0,
            node_b_id: 5,
        }]);

        assert!(is_disconnected(&set, &solution));
    }

    #[test]
    fn test_disconnected_two_connected_nodes_returns_false() {
        let set = get_test_set(4, [SetRule::Disconnected].to_vec());

        let solution = Solution::from([GameLine {
            node_a_id: 0,
            node_b_id: 1,
        }]);

        assert!(!is_disconnected(&set, &solution));
    }

    #[test]
    fn test_leaf_no_nodes_connected_outside_set_returns_false() {
        let set = get_test_set(4, [SetRule::Leaf].to_vec());

        let solution = Solution::new();

        assert!(!is_leaf(&set, &solution));
    }

    #[test]
    fn test_leaf_one_node_connected_outside_set_returns_true() {
        let set = get_test_set(4, [SetRule::Leaf].to_vec());

        let solution = Solution::from([GameLine {
            node_a_id: 0,
            node_b_id: 5,
        }]);

        assert!(is_leaf(&set, &solution));
    }

    #[test]
    fn test_leaf_two_nodes_connected_outside_set_returns_false() {
        let set = get_test_set(4, [SetRule::Leaf].to_vec());

        let solution = Solution::from([
            GameLine {
                node_a_id: 0,
                node_b_id: 5,
            },
            GameLine {
                node_a_id: 1,
                node_b_id: 6,
            },
        ]);

        assert!(!is_leaf(&set, &solution));
    }
}
