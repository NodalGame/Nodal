pub mod set_rule {
    use bevy::{math::Vec2, sprite::Sprite};
    use serde::{Deserialize, Serialize};

    use crate::{
        logic::rule_checks::rule_checks::{is_disconnected, is_leaf},
        structs::immutable::{game_set::game_set::GameSet, solution::solution::Solution},
        CDTN_RULE_SPRITE_SIZE, COLOR_RULE_UNSAT,
    };

    /// SetRule applies rules to a set of nodes which it wraps in a puzzle, possibly also impacting
    /// their conditions (e.g. Scope).
    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub enum SetRule {
        /// None of the nodes in the set may directly connect to any node also in the set.
        Disconnected,
        /// Only one connection may be made between a node within the set and any node outside the set.
        Leaf,
    }

    impl SetRule {
        pub fn sprite(&self) -> Sprite {
            Sprite {
                custom_size: Some(Vec2::new(CDTN_RULE_SPRITE_SIZE, CDTN_RULE_SPRITE_SIZE)),
                color: COLOR_RULE_UNSAT,
                ..Default::default()
            }
        }

        pub fn is_satisfied(&self, set: &GameSet, solution: &Solution) -> bool {
            match self {
                SetRule::Disconnected => is_disconnected(set, solution),
                SetRule::Leaf => is_leaf(set, solution),
            }
        }
    }
}
