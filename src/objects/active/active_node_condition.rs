pub mod active_node_condition {
    use bevy::{
        ecs::entity::Entity,
        sprite::{Sprite, SpriteBundle},
    };

    use crate::{
        objects::{
            active::{
                active_identifier::active_identifier::ActiveIdentifier, traits::traits::Satisfiable,
            },
            immutable::node_condition::node_condition::NodeCondition,
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
        pub fn check_satisfied(&self) -> bool {
            // TODO check if condition is satisfied
            true
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
            println!(
                "Setting satisfied to {} for condition {}",
                value,
                self.active_id.get_id()
            );
        }
    }
}
