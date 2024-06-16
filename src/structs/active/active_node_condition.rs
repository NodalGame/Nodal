pub mod active_node_condition {
    use bevy::{
        ecs::entity::Entity,
        sprite::{Sprite, SpriteBundle},
    };

    use crate::{
        structs::{
            active::{
                active_identifier::active_identifier::ActiveIdentifier,
                active_node::active_node::ActiveNode, traits::traits::Satisfiable,
            },
            immutable::{
                node_condition::node_condition::NodeCondition,
                solution::{solution::Solution},
            },
        },
        COLOR_CDTN_SAT, COLOR_CDTN_UNSAT,
    };

    #[derive(Clone)]
    pub struct ActiveNodeCondition {
        pub active_id: ActiveIdentifier,
        pub condition: NodeCondition,
        pub sprite: SpriteBundle,
        pub sprite_entity_id: Entity,
        pub satisfied: bool,
    }

    impl ActiveNodeCondition {
        pub fn check_satisfied(&self, node: &ActiveNode, solution: &Solution) -> bool {
            return self.condition.is_satisfied(&node.node, solution);
        }

        pub fn update_sprite(&mut self, sprite: &mut Sprite) {
            sprite.color = if self.satisfied {
                COLOR_CDTN_SAT
            } else {
                COLOR_CDTN_UNSAT
            }
        }
    }

    impl Satisfiable for ActiveNodeCondition {
        fn identifier(&self) -> &ActiveIdentifier {
            &self.active_id
        }

        fn set_satisfied(&mut self, value: bool) {
            self.satisfied = value;
        }
    }
}
