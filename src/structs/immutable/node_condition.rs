pub mod node_condition {

    use bevy::{math::Vec2, sprite::Sprite};
    use serde::{Deserialize, Serialize};

    use crate::{
        logic::condition_checks::condition_checks::{is_branch_equal, is_leaf},
        structs::immutable::{
            game_node::game_node::GameNode,
            solution::solution::{Solution},
        },
        CDTN_RULE_SPRITE_SIZE, COLOR_CDTN_UNSAT,
    };

    /// Boolean indicating if condition is bounded by its encompassing set(s).
    pub type Bounded = bool;

    /// NodeCondition applies a condition to a single node in a puzzle, affecting its
    /// win condition.
    #[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
    pub enum NodeCondition {
        /// Every branching path connecting to this node must be of equal length (cycles disallowed).
        BranchEqual, 
        /// This node has only one line connected to it.
        Leaf, 
    }

    impl NodeCondition {
        // TODO implement bounded on each condition check (if keeping this way of doing it).
        // pub fn bounded(&self) -> &Bounded {
        //     match self {
        //         NodeCondition::BranchEqual(bounded) => bounded,
        //         NodeCondition::Leaf(bounded) => bounded,
        //     }
        // }

        pub fn sprite(&self) -> Sprite {
            Sprite {
                custom_size: Some(Vec2::new(CDTN_RULE_SPRITE_SIZE, CDTN_RULE_SPRITE_SIZE)),
                color: COLOR_CDTN_UNSAT,
                ..Default::default()
            }
        }

        // This takes static instead of active objects since this logic has to be re-used
        // to validate puzzle answers which aren't being actively displayed.
        pub fn is_satisfied(&self, node: &GameNode, solution: &Solution) -> bool {
            match self {
                // NodeCondition::BranchEqual(bounded) => is_branch_equal(node, solution),
                // NodeCondition::Leaf(bounded) => is_leaf(node, solution),
                NodeCondition::BranchEqual => is_branch_equal(node, solution),
                NodeCondition::Leaf => is_leaf(node, solution),
            }
        }
    }
}
