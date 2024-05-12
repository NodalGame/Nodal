pub mod solution {
    use std::collections::{HashMap, HashSet};

    use crate::objects::{active::active_node::active_node::ActiveNode, immutable::game_line::game_line::{connections_to_lines, GameLine}};

    // A proposed solution to a puzzle, aka, a list of game node connections.
    pub struct Solution {
        pub connections: Vec<GameLine>
    }

    pub fn active_nodes_to_solution(active_nodes: &Vec<&ActiveNode>) -> Solution {
        let mut lines_set = HashSet::new();

        for active_node in active_nodes {
            connections_to_lines(active_node).iter().for_each(|&line| {
                lines_set.insert(line);
            });
        }

        Solution { connections: lines_set.iter().cloned().map(|game_line_ref| game_line_ref.clone()).collect() }
    }

    pub fn solution_to_adjacency_list(solution: &Solution) -> HashMap<u16, Vec<u16>> {
        let mut graph = HashMap::new();
        for line in &solution.connections {
            graph.entry(line.node_a_id).or_insert(Vec::new()).push(line.node_b_id);
            graph.entry(line.node_b_id).or_insert(Vec::new()).push(line.node_a_id);
        }
        graph
    }
}