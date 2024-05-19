pub mod meta_set_rule {
    use serde::{Deserialize, Serialize};

    /// MetaSetRule applies rules on the rules of a set.
    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub enum MetaSetRule {
        /// Exactly one set rule must be met for the set to be satisfied.
        Xor,
    }
}
