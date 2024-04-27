pub mod game_node {
    use serde::{Deserialize, Serialize};

    use crate::{
        node_condition::node_condition::NodeCondition,
        objects::connected_node_condition::connected_node_condition::ConnectedNodeCondition,
    };

    /// GameNode is a deserialized node spec in a puzzle json, consisting of ID for its
    /// location in the puzzle and conditions it must satisfy.
    #[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
    pub struct GameNode {
        pub id: u16,
        pub conditions: Vec<NodeCondition>,
        pub connected_conditions: Vec<ConnectedNodeCondition>,
    }
}
