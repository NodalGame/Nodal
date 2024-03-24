pub mod game_node {
    use serde::{Deserialize, Serialize};

    use crate::node_condition::node_condition::NodeCondition;

    /// GameNode is a deserialized node spec in a puzzle json, consisting of ID for its
    /// location, class for its node class, and conditions impacting win conditions.
    #[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
    pub struct GameNode {
        pub id: u16,
        pub class: NodeClass,
        pub conditions: Vec<NodeCondition>,
    }

    /// NodeClass is the connection class of a node, wherein all of the same class
    /// in a puzzle must be connected for the win condition.
    #[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
    pub enum NodeClass {
        Red,
        Blue,
        Yellow,
    }
}
