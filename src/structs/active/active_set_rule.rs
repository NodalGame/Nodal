pub mod active_set_rule {
    use bevy::{
        ecs::entity::Entity,
        sprite::{Sprite, SpriteBundle},
    };

    use crate::{
        structs::{
            active::{
                active_identifier::active_identifier::ActiveIdentifier,
                active_set::active_set::ActiveSet, traits::traits::Satisfiable,
            },
            immutable::{set_rule::set_rule::SetRule, solution::solution::Solution},
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
        pub fn check_satisfied(&self, set: &ActiveSet, solution: &Solution) -> bool {
            return self.rule.is_satisfied(&set.set, solution);
        }
    }

    impl Satisfiable for ActiveSetRule {
        fn identifier(&self) -> &ActiveIdentifier {
            &self.active_id
        }

        fn set_satisfied(&mut self, value: bool) {
            self.satisfied = value;
        }
        
        fn update_sprites(&mut self, sprites: Vec<&mut Sprite>) {
            for sprite in sprites {
                sprite.color = if self.satisfied {
                    COLOR_RULE_SAT
                } else {
                    COLOR_RULE_UNSAT
                }
            }
        }
    }
}
