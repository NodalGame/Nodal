pub mod node_condition_checks {
    use std::collections::{HashMap, VecDeque};

    use crate::objects::immutable::{game_node::game_node::GameNode, solution::solution::{solution_to_adjacency_list, Solution}};

    /// Checks if all nodes connected to the start node are of equal distance. 
    /// 
    /// NOTE: Currently does NOT consider Scope. 
    pub fn is_branch_equal(node: &GameNode, solution: &Solution) -> bool {
        let adj_matrix = solution_to_adjacency_list(solution);

        // Check if the node has no neighbors 
        let binding = Vec::new();
        let start_neighbors = adj_matrix.get(&node.id).unwrap_or(&binding);
        if start_neighbors.is_empty() {
            return true;
        }

        let mut distances = HashMap::new();
        let mut queue = VecDeque::new();

        queue.push_back(node.id);
        distances.insert(node.id, 0);

        while let Some(node_id) = queue.pop_front() {
            let curr_dist = distances[&node_id];

            if let Some(neighbors) = adj_matrix.get(&node_id) {
                for &neighbor in neighbors {
                    if distances.contains_key(&neighbor) {
                        if distances[&neighbor] != curr_dist + 1 {
                            println!("Found branch with unequal distances: {:?}", distances);
                            return false;
                        }
                    } else {
                        distances.insert(neighbor, curr_dist + 1);
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        // Check if all directly connected nodes to the start node have the same distance
        let first_distance = distances[&start_neighbors[0]];
        for &neighbor in start_neighbors {
            if distances[&neighbor] != first_distance {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::objects::immutable::game_node::game_node::GameNode;

    #[test]
    fn test_branch_equal_node_with_no_connections_returns_true() {
        // TODO
        assert!(false)
    }

    #[test]
    fn test_branch_equal_node_with_one_connection_returns_true() {
        // TODO
        assert!(false)
    }

    #[test]
    fn test_branch_equal_node_with_one_branch_returns_true() {
        // TODO
        assert!(false)
    }

    #[test]
    fn test_branch_equal_node_with_multiple_length_one_branches_returns_true() {
        // TODO
        assert!(false)
    }

    #[test]
    fn test_branch_equal_node_with_multiple_branches_with_multiple_length_one_branches_returns_true() {
        // TODO
        assert!(false)
    }

    #[test]
    fn test_branch_equal_node_with_length_one_and_length_two_branch_returns_false() {
        // TODO
        assert!(false)
    }

    #[test]
    fn test_branch_equal_node_in_cycle_returns_false() {
        // TODO
        assert!(false)
    }

    #[test]
    fn test_branch_equal_node_with_branch_containing_cycle_returns_false() {
        // TODO
        assert!(false)
    }
}
