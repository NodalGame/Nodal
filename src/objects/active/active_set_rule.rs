pub mod active_set_rule {
    use bevy::{
        ecs::entity::Entity,
        sprite::{Sprite, SpriteBundle},
    };

    use crate::{
        objects::{
            active::{
                active_identifier::active_identifier::ActiveIdentifier, traits::traits::Satisfiable,
            },
            immutable::set_rule::set_rule::SetRule,
        },
        COLOR_RULE_SAT, COLOR_RULE_UNSAT,
    };

    #[derive(Clone)]
    pub struct ActiveSetRule {
        pub active_id: ActiveIdentifier,
        pub rule: SetRule,
        pub sprite: SpriteBundle,
        pub sprite_entity_id: Entity,
        pub satisfied: bool,
    }

    impl ActiveSetRule {
        pub fn update_sprite(&mut self, sprite: &mut Sprite) {
            sprite.color = if self.satisfied {
                COLOR_RULE_SAT
            } else {
                COLOR_RULE_UNSAT
            }
        }
    }

    impl Satisfiable for ActiveSetRule {
        fn identifier(&self) -> &ActiveIdentifier {
            &self.active_id
        }

        fn set_satisfied(&mut self, value: bool) {
            self.satisfied = value;
            println!(
                "Setting satisfied to {} for rule {}",
                value,
                self.active_id.get_id()
            );
        }
    }
}
