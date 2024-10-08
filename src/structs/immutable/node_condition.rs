pub mod node_condition {

    use bevy::{math::Vec2, sprite::Sprite};
    use serde::{Deserialize, Serialize};

    use crate::{
        logic::condition_checks::condition_checks::{
            is_branch_equal, is_cycle, is_internal, is_leaf,
        },
        structs::immutable::{game_node::game_node::GameNode, solution::solution::Solution},
        CDTN_RULE_SPRITE_SIZE, COLOR_CDTN_UNSAT,
    };

    /// NodeCondition applies a condition to a single node in a puzzle, affecting its
    /// win condition.
    #[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
    pub enum NodeCondition {
        /// Every branching path connecting to this node must be of equal length (cycles disallowed).
        BranchEqual,
        /// This node has only one line connected to it.
        Leaf,
        /// This node must have at least two lines connected to it.
        Internal,
        /// This node must be contained within a cycle.
        Cycle,
    }

    impl NodeCondition {
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
                NodeCondition::Internal => is_internal(node, solution),
                NodeCondition::Cycle => is_cycle(node, solution),
            }
        }
    }
}
