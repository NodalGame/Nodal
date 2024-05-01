pub mod active_node {
    use std::hash::{Hash, Hasher};

    use bevy::{ecs::entity::Entity, math::Vec2, render::color::Color, sprite::{Sprite, SpriteBundle}};

    use crate::{objects::{active::{active_identifier::active_identifier::ActiveIdentifier, satisfiable_entity::satisfiable_entity::Satisfiable}, immutable::game_node::game_node::GameNode}, TILE_NODE_SPRITE_SIZE};

    #[derive(Clone)]
    pub struct ActiveNode {
        // TODO this should not be pub
        pub active_id: ActiveIdentifier,
        pub node: GameNode,
        pub connections: Vec<u16>,
        pub sprite: SpriteBundle,
        pub sprite_entity_id: Entity,
        // TODO this should not be pub
        pub satisfied: bool,
    }

    impl ActiveNode {
        // // TODO this will need to be updated to take set rules as well
        // pub fn get_failed_conditions(&self, active_nodes: Vec<&ActiveNode>) -> Vec<NodeCondition> {
        //     self.node
        //         .conditions
        //         .iter()
        //         .filter(|c| !c.is_satisfied(self, &active_nodes))
        //         .cloned()
        //         .collect()
        // }

        pub fn update_sprite(&mut self, sprite: &mut Sprite) {
            sprite.color = if self.satisfied {
                Color::WHITE
            } else {
                Color::BLACK
            };
        }
    }

    impl Satisfiable for ActiveNode {
        fn compute_satisfied(&self) -> bool {
            true
            // TODO check if part of network (two networks -> none satisfied) check all conditions (which are also Satisfiable)
        }
    
        fn set_satisfied(&mut self, value: bool) {
            self.satisfied = value;
            println!("Setting satisfied to {} for node {}", value, self.node.id);
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