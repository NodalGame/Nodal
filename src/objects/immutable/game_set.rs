pub mod game_set {
    use serde::{Deserialize, Serialize};

    use crate::objects::immutable::{connected_set_rule::connected_set_rule::ConnectedSetRule, set_rule::set_rule::SetRule};

    /// Set contains nodes over which it applies SetRules.
    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct GameSet {
        pub id: u8,
        pub nodes: Vec<u16>,
        pub rules: Vec<SetRule>,
        pub connected_rules: Vec<ConnectedSetRule>,
    }
}
