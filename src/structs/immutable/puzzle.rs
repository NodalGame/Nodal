pub mod puzzle {
    use serde::Deserialize;
    use uuid::Uuid;

    use crate::structs::immutable::{game_node::game_node::GameNode, game_set::game_set::GameSet};

    #[derive(Clone, Deserialize, Debug)]
    pub struct Puzzle {
        pub uuid: Uuid,
        pub width: u8,
        pub height: u8,
        pub nodes: Vec<GameNode>,
        pub sets: Vec<GameSet>,
    }
}
