pub mod set_rule {
    use bevy::{math::Vec2, sprite::Sprite};
    use serde::{Deserialize, Serialize};

    use crate::{CDTN_RULE_SPRITE_SIZE, COLOR_RULE_UNSAT};

    /// SetRule applies rules to a set of nodes which it wraps in a puzzle, possibly also impacting
    /// their conditions (e.g. Scope).
    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub enum SetRule {
        /// All nodes in this set must have a path to every other node in the set.
        Connected,
        /// There must be two nodes in the set for which there is no path between them.
        Unconnected,
        /// No nodes in this set may have a path to each other.
        Disconnected,
        /// There must be a cycle present in the set.
        Cycle,
        /// There must be no cycles present in the set.
        NoCycle,
        /// Limits the scope of the condition of all nodes in the set to just the set.
        /// NOTE: This does not affect the game's universal win condition of all nodes of same class connecting!
        /// Ex: BranchEqual: stops counting branch length for lines connecting to nodes outside the set.
        ///     Leaf: the node is allowed to connect to more than one node granted its outside the set.
        Scope,
    }

    impl SetRule {
        pub fn sprite(&self) -> Sprite {
            Sprite {
                custom_size: Some(Vec2::new(CDTN_RULE_SPRITE_SIZE, CDTN_RULE_SPRITE_SIZE)),
                color: COLOR_RULE_UNSAT,
                ..Default::default()
            }
        }
    }
}
