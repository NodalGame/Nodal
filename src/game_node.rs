pub mod game_node {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct GameNode {
        x: u8,
        y: u8,
        class: NodeClass,
    }

    #[derive(Serialize, Deserialize, Debug)]
    enum NodeClass {
        Default,
    }
}