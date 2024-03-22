pub mod game_node {
    use serde::{Deserialize, Serialize};

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

    /// NodeModifier applies a condition to a single node in a puzzle, affecting its
    /// win condition.
    #[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
    pub enum NodeCondition {
        /// This node can connect to nodes of any class.
        Universal,
        /// Every branching path connecting to this node must be of equal length (cycles disallowed).
        BranchEqual,
        /// This node has only one line connected to it.
        Leaf,
        /// This node must connect to every other node with this condition.
        Linked,
    }
}
