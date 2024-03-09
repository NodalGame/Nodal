pub mod game_node {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct GameNode {
        pub id: u16,
        pub class: NodeClass,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub enum NodeClass {
        Default,
    }
}
