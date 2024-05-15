pub mod solution {
    use std::collections::{HashMap, HashSet};

    use crate::objects::{
        active::active_node::active_node::ActiveNode,
        immutable::{
            game_line::game_line::{connections_to_lines, GameLine},
            game_node::game_node::{GameNode, GameNodeId},
        },
    };

    /// A proposed solution to a puzzle.
    pub type Solution = Vec<GameLine>;

    /// A hash of node ids to connecting nodes.
    pub type AdjacencyList = HashMap<GameNodeId, Vec<GameNodeId>>;

    pub fn active_nodes_to_solution(active_nodes: &Vec<ActiveNode>) -> Solution {
        let mut lines_set = HashSet::new();

        for active_node in active_nodes {
            connections_to_lines(active_node).iter().for_each(|&line| {
                println!("inserting line {:?}", line);
                lines_set.insert(line);
            });
        }

        lines_set
            .iter()
            .cloned()
            .map(|game_line_ref| game_line_ref.clone())
            .collect()
    }

    /// Converts a solution to a hash of node ids to connecting nodes. It is reflexive, so if
    /// node A is connected to node B, then node B is connected to node A, represented by
    /// {0: [1], 1: [0]} in the hash.
    ///
    /// TODO solution should be struct, impl Solution .to_adjacency_list fn
    pub fn solution_to_adjacency_list(solution: &Solution) -> AdjacencyList {
        let mut graph = HashMap::new();
        for line in solution {
            graph
                .entry(line.node_a_id)
                .or_insert(Vec::new())
                .push(line.node_b_id);
            graph
                .entry(line.node_b_id)
                .or_insert(Vec::new())
                .push(line.node_a_id);
        }
        graph
    }
}
