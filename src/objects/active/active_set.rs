pub mod active_set {
    use bevy::{
        ecs::entity::Entity,
        render::color::Color,
        sprite::{Sprite, SpriteBundle},
    };

    use crate::objects::{
        active::{
            active_connected_set_rule::active_connected_set_rule::ActiveConnectedSetRule, active_identifier::active_identifier::ActiveIdentifier, active_node::active_node::ActiveNode, active_set_rule::active_set_rule::ActiveSetRule, traits::traits::Satisfiable
        },
        immutable::game_set::game_set::GameSet,
    };

    #[derive(Clone)]
    pub struct ActiveSet {
        pub active_id: ActiveIdentifier,
        pub set: GameSet,
        pub sprites: Vec<SpriteBundle>,
        pub sprite_entity_ids: Vec<Entity>,
        pub active_set_rules: Vec<ActiveSetRule>,
        pub active_connected_set_rules: Vec<ActiveConnectedSetRule>,
        pub satisfied: bool,
    }

    impl ActiveSet {
        pub fn update_sprites(&mut self, sprites: &mut Vec<Sprite>) {
            sprites.iter_mut().for_each(|sprite| {
                sprite.color = if self.satisfied {
                    Color::WHITE
                } else {
                    Color::BLACK
                }
            })
        }
    }

    impl Satisfiable for ActiveSet {
        fn set_satisfied(&mut self, value: bool) {
            self.satisfied = value;
            println!("Setting satisfied to {} for set {}", value, self.set.id);
        }

        fn identifier(&self) -> &ActiveIdentifier {
            &self.active_id
        }
    }
}
