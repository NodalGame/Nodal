pub mod meta_node_condition {
    use serde::{Deserialize, Serialize};

    /// MetaNodeCondition applies conditions on the conditions of a node.
    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub enum MetaNodeCondition {
        /// Exactly one condition must be met for the node to be satisfied.
        Xor,
    }
}
