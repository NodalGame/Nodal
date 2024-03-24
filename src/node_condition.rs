pub mod node_condition {
    use serde::{Deserialize, Serialize};

    use crate::ActiveNode;

    /// NodeCondition applies a condition to a single node in a puzzle, affecting its
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

    impl NodeCondition {
        /// Returns true if the condition is satisfied for the given node.
        ///
        /// # Parameters
        ///
        /// - `node`: The current node upon which the condition is placed.
        /// - `active_nodes`: The active nodes in the puzzle, including their connections.
        /// TODO node condition satisified check needs to include set rules 
        ///
        /// # Returns
        ///
        /// True if the condition is satisfied, false otherwise.
        pub fn is_satisfied(&self, node: &ActiveNode, active_nodes: &Vec<&ActiveNode>) -> bool {
            match self {
                NodeCondition::Universal => true,
                NodeCondition::BranchEqual => self.is_branch_equal(node, active_nodes.to_vec()),
                NodeCondition::Leaf => self.is_leaf(node, active_nodes.to_vec()),
                NodeCondition::Linked => self.is_linked(node, active_nodes.to_vec()),
            }
        }

        // TODO branch equal condition logic
        fn is_branch_equal(&self, node: &ActiveNode, active_nodes: Vec<&ActiveNode>) -> bool {
            false
        }

        // TODO leaf condition logic
        fn is_leaf(&self, node: &ActiveNode, active_nodes: Vec<&ActiveNode>) -> bool {
            true
        }

        // TODO linked condition logic
        fn is_linked(&self, node: &ActiveNode, active_nodes: Vec<&ActiveNode>) -> bool {
            false
        }
    }
}
