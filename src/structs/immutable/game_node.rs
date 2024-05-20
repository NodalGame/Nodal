pub mod game_node {
    use std::collections::{HashMap, HashSet, VecDeque};

    use serde::{Deserialize, Serialize};

    use crate::structs::immutable::{
        connected_node_condition::connected_node_condition::ConnectedNodeCondition,
        node_condition::node_condition::NodeCondition,
        solution::solution::{solution_to_adjacency_matrix, AdjacencyMatrix, Solution},
    };

    /// Id of a game node.
    pub type GameNodeId = u16;

    /// GameNode is a deserialized node spec in a puzzle json, consisting of ID for its
    /// location in the puzzle and conditions it must satisfy.
    #[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
    pub struct GameNode {
        pub id: GameNodeId,
        pub conditions: Vec<NodeCondition>,
        pub connected_conditions: Vec<ConnectedNodeCondition>,
    }

    impl GameNode {
        /// Returns true if node is part of a network and there exists only one network in the puzzle.
        /// TODO This is highly inefficient to call for every node, rather we can do one call for all
        /// nodes in the puzzle and update them at once:
        /// - get network(s)
        /// - if >1 network, all nodes false
        /// - else if node in network, set true, else false
        pub fn is_satisfied(&self, solution: &Solution) -> bool {
            let adj_matrix = solution_to_adjacency_matrix(solution);

            if !adj_matrix.contains_key(&self.id) {
                return false;
            }

            let mut visited = HashSet::new();

            // Function to perform BFS and mark visited nodes
            fn bfs(
                start_node: GameNodeId,
                adj_matrix: &AdjacencyMatrix,
                visited: &mut HashSet<u16>,
            ) {
                let mut queue = VecDeque::new();
                queue.push_back(start_node);

                while let Some(node) = queue.pop_front() {
                    if !visited.contains(&node) {
                        visited.insert(node);

                        if let Some(neighbors) = adj_matrix.get(&node) {
                            for &neighbor in neighbors {
                                if !visited.contains(&neighbor) {
                                    queue.push_back(neighbor);
                                }
                            }
                        }
                    }
                }
            }

            bfs(self.id, &adj_matrix, &mut visited);
            for &node in adj_matrix.keys() {
                if !visited.contains(&node) {
                    // There must be more than one connected component in the solution
                    return false;
                }
            }

            true

            // If we need to get the number of components in the puzzle, we can uncomment the following code
            // for &node in adj_matrix.keys() {
            //     if !visited.contains(&node) {
            //         component_count += 1;
            //         bfs(node, graph, &mut visited);

            //         // Early exit if more than one component is found
            //         if component_count > 1 {
            //             return (true, true);
            //         }
            //     }
            // }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::immutable::{game_line::game_line::GameLine, solution::solution::Solution};

    use super::game_node::GameNode;

    fn get_test_node() -> GameNode {
        GameNode {
            id: 0,
            conditions: Vec::new(),
            connected_conditions: Vec::new(),
        }
    }

    #[test]
    fn test_is_satisfied_node_not_in_network_false() {
        let node = get_test_node();

        let solution = Solution::new();

        assert!(!node.is_satisfied(&solution));
    }

    #[test]
    fn test_is_satisfied_node_in_only_network_true() {
        let node = get_test_node();

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
                node_a_id: 1,
                node_b_id: 3,
            },
            GameLine {
                node_a_id: 2,
                node_b_id: 3,
            },
            GameLine {
                node_a_id: 2,
                node_b_id: 4,
            },
            GameLine {
                node_a_id: 3,
                node_b_id: 4,
            },
        ]);

        assert!(node.is_satisfied(&solution));
    }

    #[test]
    fn test_is_satisfied_node_in_network_multiple_networks_false() {
        let node = get_test_node();

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
                node_a_id: 1,
                node_b_id: 3,
            },
            GameLine {
                node_a_id: 2,
                node_b_id: 3,
            },
            GameLine {
                node_a_id: 4,
                node_b_id: 5,
            },
        ]);

        assert!(!node.is_satisfied(&solution));
    }
}
