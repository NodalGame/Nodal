pub mod active_connected_set_rule {
    use bevy::{
        ecs::entity::Entity,
        sprite::{Sprite, SpriteBundle},
    };

    use crate::{
        structs::{
            active::{
                active_identifier::active_identifier::ActiveIdentifier, traits::traits::Satisfiable,
            },
            immutable::connected_set_rule::connected_set_rule::{ConnectedSetRule, RuleClass},
        },
        COLOR_RULE_ORANGE_SAT, COLOR_RULE_ORANGE_UNSAT, COLOR_RULE_RED_SAT, COLOR_RULE_RED_UNSAT,
        COLOR_RULE_YELLOW_SAT, COLOR_RULE_YELLOW_UNSAT,
    };

    #[derive(Clone)]
    pub struct ActiveConnectedSetRule {
        pub active_id: ActiveIdentifier,
        pub rule: ConnectedSetRule,
        pub sprite: SpriteBundle,
        pub sprite_entity_id: Entity,
        pub satisfied: bool,
    }

    impl ActiveConnectedSetRule {
        pub fn update_sprite(&mut self, sprite: &mut Sprite) {
            sprite.color = match self.rule.rule_class() {
                RuleClass::Yellow => {
                    if self.satisfied {
                        COLOR_RULE_YELLOW_SAT
                    } else {
                        COLOR_RULE_YELLOW_UNSAT
                    }
                }
                RuleClass::Orange => {
                    if self.satisfied {
                        COLOR_RULE_ORANGE_SAT
                    } else {
                        COLOR_RULE_ORANGE_UNSAT
                    }
                }
                RuleClass::Red => {
                    if self.satisfied {
                        COLOR_RULE_RED_SAT
                    } else {
                        COLOR_RULE_RED_UNSAT
                    }
                }
            }
        }
    }

    impl Satisfiable for ActiveConnectedSetRule {
        fn identifier(&self) -> &ActiveIdentifier {
            &self.active_id
        }

        fn set_satisfied(&mut self, value: bool) {
            self.satisfied = value;
        }
    }
}
