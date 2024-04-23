pub mod game_set {
    use serde::{Deserialize, Serialize};

    /// Set contains nodes over which it applies SetRules.
    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct GameSet {
        pub nodes: Vec<u16>,
        pub rules: Vec<SetRule>,
    }

    /// SetRule applies rules to a set of nodes which it wraps in a puzzle.
    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub enum SetRule {
        /// All nodes in this set must have a path to every other node in the set.
        Connected,
        /// There must be two nodes in the set for which there is no path between them.
        Unconnected,
        /// No nodes in this set may have a path to each other. 
        Disconnected,
        /// There must be a cycle present in the set.
        Cycle,
        /// There must be no cycles present in the set.
        NoCycle,
        /// Exactly one node condition must be met in the set.
        Xor,
        /// Limits the scope of the condition of all nodes in the set to just the set.
        /// NOTE: This does not affect the game's universal win condition of all nodes of same class connecting!
        /// Ex: Universal: nodes outside the set of a different class cannot connect to this node.
        ///     BranchEqual: stops counting branch length for lines connecting to nodes outside the set.
        ///     Leaf: the node is allowed to connect to more than one node granted its outside the set.
        ///     Linked: only nodes in the set with linked condition must be connected to this node.
        Scope,
        /// Sets with this rule must be homomorphic to each other
        Homomorphic,
    }
}
