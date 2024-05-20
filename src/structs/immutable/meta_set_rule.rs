pub mod meta_set_rule {
    use serde::{Deserialize, Serialize};

    /// MetaSetRule applies rules on conditions or rules within the set.
    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub enum MetaSetRule {
        /// Exactly one set rule must be met for the set to be satisfied.
        Xor,
        /// Limits the scope of the condition of all nodes in the set to just the set.
        /// NOTE: This does not affect the game's universal win condition of all nodes of same class connecting!
        /// Ex: BranchEqual: stops counting branch length for lines connecting to nodes outside the set.
        ///     Leaf: the node is allowed to connect to more than one node granted its outside the set.
        Scope,
    }
}
