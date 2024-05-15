pub mod game_node {
    use serde::{Deserialize, Serialize};

    use crate::objects::immutable::{
        connected_node_condition::connected_node_condition::ConnectedNodeCondition,
        node_condition::node_condition::NodeCondition,
    };

    /// Id of a game node.
    pub type GameNodeId = u16;

    /// GameNode is a deserialized node spec in a puzzle json, consisting of ID for its
    /// location in the puzzle and conditions it must satisfy.
    #[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
    pub struct GameNode {
        pub id: GameNodeId,
        pub conditions: Vec<NodeCondition>,
        pub connected_conditions: Vec<ConnectedNodeCondition>,
    }
}
