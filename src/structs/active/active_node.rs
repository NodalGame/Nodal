pub mod active_node {
    use std::{
        collections::HashSet,
        hash::{Hash, Hasher},
    };

    use bevy::{
        ecs::entity::Entity,
        math::Vec2,
        sprite::{Sprite, SpriteBundle},
    };

    use crate::{
        is_mouse_over_sprite,
        structs::{
            active::{
                active_connected_node_condition::active_connected_node_condition::ActiveConnectedNodeCondition,
                active_identifier::active_identifier::ActiveIdentifier,
                active_node_condition::active_node_condition::ActiveNodeCondition,
                traits::traits::Satisfiable,
            },
            immutable::{
                game_node::game_node::{GameNode, GameNodeId},
                solution::solution::Solution,
            },
        },
        COLOR_NODE_SAT, COLOR_NODE_UNSAT, SCALE_NODE_DEFAULT, SCALE_NODE_HOVERED,
    };

    #[derive(Clone)]
    pub struct ActiveNode {
        // TODO this should not be pub
        pub active_id: ActiveIdentifier,
        pub node: GameNode,
        pub sprite: SpriteBundle,
        pub connections: HashSet<GameNodeId>,
        pub sprite_entity_id: Entity,
        pub active_conditions: Vec<ActiveNodeCondition>,
        pub active_connected_conditions: Vec<ActiveConnectedNodeCondition>,
        pub satisfied: bool,
    }

    impl ActiveNode {
        pub fn check_satisfied(&self, solution: &Solution) -> bool {
            return self.node.is_satisfied(solution);
        }
    }

    impl Satisfiable for ActiveNode {
        fn set_satisfied(&mut self, value: bool) {
            self.satisfied = value;
        }

        fn identifier(&self) -> &ActiveIdentifier {
            &self.active_id
        }

        fn update_sprites(&mut self, sprites: Vec<&mut Sprite>) {
            for sprite in sprites {
                sprite.color = if self.satisfied {
                    COLOR_NODE_SAT
                } else {
                    COLOR_NODE_UNSAT
                }
            }
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
            // Note: SpriteBundle is not hashed
        }
    }
}
