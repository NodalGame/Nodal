pub mod tile_placement {
    use crate::structs::immutable::{game_set::game_set::GameSet, puzzle::puzzle::Puzzle};

    pub fn get_set_upper_left_node(set: &GameSet, puzzle: &Puzzle) -> u16 {
        let mut upper_left_most_node = u16::MAX;
        let mut upper_most_row = u8::MIN;
        let mut left_most_column = u8::MAX;
        set.nodes.iter().for_each(|node| {
            if (node % puzzle.height as u16) > upper_most_row.into()
                || (node / puzzle.height as u16) < left_most_column.into()
            {
                upper_most_row = (node % puzzle.height as u16) as u8;
                left_most_column = (node / puzzle.height as u16) as u8;
                upper_left_most_node = *node;
            }
        });
        upper_left_most_node
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use uuid::Uuid;

    use crate::{
        logic::puzzle::tile_placement::tile_placement::get_set_upper_left_node,
        structs::immutable::{
            game_node::game_node::{GameNode, GameNodeId},
            game_set::game_set::GameSet,
            puzzle::puzzle::Puzzle,
        },
    };

    fn get_test_puzzle(sets: Vec<GameSet>) -> Puzzle {
        Puzzle {
            uuid: Uuid::new_v4(),
            width: 4,
            height: 4,
            nodes: (0..15)
                .map(|id| GameNode {
                    id: id,
                    conditions: Vec::new(),
                    connected_conditions: Vec::new(),
                })
                .collect_vec(),
            sets: sets,
        }
    }

    fn get_test_set(node_ids: Vec<GameNodeId>) -> GameSet {
        GameSet {
            id: 0,
            nodes: node_ids,
            rules: Vec::new(),
            connected_rules: Vec::new(),
            bounded: false,
        }
    }

    #[test]
    fn test_get_set_upper_left_node_single_node_set_returns_node() {
        let set = get_test_set([0].to_vec());

        let puzzle = get_test_puzzle([set.clone()].to_vec());

        assert!(get_set_upper_left_node(&set, &puzzle) == 0);
    }

    #[test]
    fn test_get_set_upper_left_node_vertical_set_returns_top_node() {
        let set = get_test_set([0, 1, 2, 3].to_vec());

        let puzzle = get_test_puzzle([set.clone()].to_vec());

        assert!(get_set_upper_left_node(&set, &puzzle) == 3);
    }

    #[test]
    fn test_get_set_upper_left_node_horizontal_set_returns_left_node() {
        let set = get_test_set([0, 4, 8, 12].to_vec());

        let puzzle = get_test_puzzle([set.clone()].to_vec());

        assert!(get_set_upper_left_node(&set, &puzzle) == 0);
    }

    #[test]
    fn test_get_set_upper_left_node_square_set_returns_top_left_node() {
        let set = get_test_set([0, 1, 4, 5].to_vec());

        let puzzle = get_test_puzzle([set.clone()].to_vec());

        assert!(get_set_upper_left_node(&set, &puzzle) == 1);
    }

    #[test]
    fn test_get_set_upper_left_node_horse_shoe_down_set_returns_top_left_node() {
        let set = get_test_set([0, 1, 5, 8, 9].to_vec());

        let puzzle = get_test_puzzle([set.clone()].to_vec());

        assert!(get_set_upper_left_node(&set, &puzzle) == 1);
    }

    #[test]
    fn test_get_set_upper_left_node_horse_shoe_up_set_returns_top_left_node() {
        let set = get_test_set([0, 1, 4, 8, 9].to_vec());

        let puzzle = get_test_puzzle([set.clone()].to_vec());

        assert!(get_set_upper_left_node(&set, &puzzle) == 1);
    }
}
