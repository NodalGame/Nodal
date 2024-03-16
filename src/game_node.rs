pub mod game_node {
    use serde::{Deserialize, Serialize};

    /// GameNode is a deserialized node spec in a puzzle json, consisting of ID for its
    /// location, class for its node class, and conditions impacting win conditions. 
    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct GameNode {
        pub id: u16,
        pub class: NodeClass,
        pub conditions: Vec<NodeCondition>,
    }

    /// NodeClass is the connection class of a node, wherein all of the same class
    /// in a puzzle must be connected for the win condition. 
    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub enum NodeClass {
        Red,
        Blue,
        Yellow,
    }

    /// NodeModifier applies a condition to a single node in a puzzle, affecting its
    /// win condition.
    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub enum NodeCondition {
        /// This node must not connect to another of the same class. 
        Inverter,
        /// Every branching path connecting to this node must be of equal length (cycles disallowed).
        BranchEqual,
    }

    /// SetRule applies rules to a set of nodes which it wraps in a puzzle.
    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub enum SetRule {
        /// No nodes in this set must connect to another node in the same set.
        Disconnected,
        /// There must be a cycle present in the set. 
        Cycle,
        /// There must be no cycles present in the set.
        NoCycle,
    }
}
