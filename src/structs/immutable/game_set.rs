pub mod game_set {
    use serde::{Deserialize, Serialize};

    use crate::structs::immutable::{
        connected_set_rule::connected_set_rule::ConnectedSetRule, game_node::game_node::GameNodeId,
        set_rule::set_rule::SetRule,
    };

    /// Set contains nodes over which it applies SetRules.
    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct GameSet {
        pub id: u8,
        pub nodes: Vec<GameNodeId>,
        pub rules: Vec<SetRule>,
        pub connected_rules: Vec<ConnectedSetRule>,
        pub bounded: bool,
    }
}
