pub mod active_connected_node_condition {
    use bevy::{
        ecs::entity::Entity,
        sprite::{Sprite, SpriteBundle},
    };

    use crate::{
        structs::{
            active::{
                active_identifier::active_identifier::ActiveIdentifier, traits::traits::Satisfiable,
            },
            immutable::connected_node_condition::connected_node_condition::{
                ConditionClass, ConnectedNodeCondition,
            },
        },
        COLOR_CDTN_BLUE_SAT, COLOR_CDTN_BLUE_UNSAT, COLOR_CDTN_GREEN_SAT, COLOR_CDTN_GREEN_UNSAT,
        COLOR_CDTN_PURPLE_SAT, COLOR_CDTN_PURPLE_UNSAT,
    };

    #[derive(Clone)]
    pub struct ActiveConnectedNodeCondition {
        pub active_id: ActiveIdentifier,
        pub condition: ConnectedNodeCondition,
        pub sprite: SpriteBundle,
        pub sprite_entity_id: Entity,
        pub satisfied: bool,
    }

    impl ActiveConnectedNodeCondition {
        pub fn update_sprite(&mut self, sprite: &mut Sprite) {
            sprite.color = match self.condition.condition_class() {
                ConditionClass::Blue => {
                    if self.satisfied {
                        COLOR_CDTN_BLUE_SAT
                    } else {
                        COLOR_CDTN_BLUE_UNSAT
                    }
                }
                ConditionClass::Purple => {
                    if self.satisfied {
                        COLOR_CDTN_PURPLE_SAT
                    } else {
                        COLOR_CDTN_PURPLE_UNSAT
                    }
                }
                ConditionClass::Green => {
                    if self.satisfied {
                        COLOR_CDTN_GREEN_SAT
                    } else {
                        COLOR_CDTN_GREEN_UNSAT
                    }
                }
            }
        }
    }

    impl Satisfiable for ActiveConnectedNodeCondition {
        fn identifier(&self) -> &ActiveIdentifier {
            &self.active_id
        }

        fn set_satisfied(&mut self, value: bool) {
            self.satisfied = value;
        }
    }
}
