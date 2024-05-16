pub mod active_node {
    use std::hash::{Hash, Hasher};

    use bevy::{
        ecs::entity::Entity,
        sprite::{Sprite, SpriteBundle},
    };

    use crate::{
        objects::{
            active::{
                active_connected_node_condition::active_connected_node_condition::ActiveConnectedNodeCondition,
                active_identifier::active_identifier::ActiveIdentifier,
                active_node_condition::active_node_condition::ActiveNodeCondition,
                traits::traits::Satisfiable,
            },
            immutable::{game_node::game_node::GameNode, solution::solution::Solution},
        },
        COLOR_NODE_SAT, COLOR_NODE_UNSAT,
    };

    #[derive(Clone)]
    pub struct ActiveNode {
        // TODO this should not be pub
        pub active_id: ActiveIdentifier,
        pub node: GameNode,
        pub connections: Vec<u16>,
        pub sprite: SpriteBundle,
        pub sprite_entity_id: Entity,
        pub active_conditions: Vec<ActiveNodeCondition>,
        pub active_connected_conditions: Vec<ActiveConnectedNodeCondition>,
        pub satisfied: bool,
    }

    impl ActiveNode {
        pub fn check_satisfied(&self, solution: &Solution) -> bool {
            return self.node.is_satisfied(solution);
        }

        pub fn update_sprite(&mut self, sprite: &mut Sprite) {
            sprite.color = if self.satisfied {
                COLOR_NODE_SAT
            } else {
                COLOR_NODE_UNSAT
            };
        }
    }

    impl Satisfiable for ActiveNode {
        fn set_satisfied(&mut self, value: bool) {
            self.satisfied = value;
        }

        fn identifier(&self) -> &ActiveIdentifier {
            &self.active_id
        }
    }

    impl PartialEq for ActiveNode {
        fn eq(&self, other: &Self) -> bool {
            self.node == other.node && self.connections == other.connections
            // Note: SpriteBundle is not compared
        }
    }

    impl Eq for ActiveNode {}

    impl Hash for ActiveNode {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.node.hash(state);
            self.connections.hash(state);
            // Note: SpriteBundle is not hashed
        }
    }
}
