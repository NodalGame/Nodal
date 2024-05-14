pub mod game_line {
    use std::hash::{Hash, Hasher};

    use crate::objects::active::active_node::active_node::ActiveNode;


    #[derive(Clone, Copy, Debug)]
pub struct GameLine {
        pub node_a_id: u16,
        pub node_b_id: u16,
    }

    // Enforce reflexive property on lines (they are directionless). 
    impl PartialEq for GameLine {
        fn eq(&self, other: &Self) -> bool {
            let ret = (self.node_a_id == other.node_a_id && self.node_b_id == other.node_b_id) ||
            (self.node_a_id == other.node_b_id && self.node_b_id == other.node_a_id);
            ret
        }
    }

    impl Eq for GameLine {}

    // Hash lines (smallest, largest) node id, such that hash sets don't double index. 
    impl Hash for GameLine {
        fn hash<H: Hasher>(&self, state: &mut H) {
            let (min, max) = if self.node_a_id < self.node_b_id { ( self.node_a_id, self.node_b_id ) } else { ( self.node_b_id, self.node_a_id ) };
            min.hash(state);
            max.hash(state);
        }
    }

    pub fn connections_to_lines(active_node: &ActiveNode) -> Vec<GameLine> {
        let mut lines: Vec<GameLine> = Vec::new();

        for connection in &active_node.connections {
            lines.push(GameLine {
                node_a_id: active_node.node.id,
                node_b_id: *connection,
            });
        }

        lines
    }
}