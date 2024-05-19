pub mod active_line {
    use bevy::{ecs::entity::Entity, sprite::SpriteBundle};

    use crate::structs::active::{
        active_identifier::active_identifier::ActiveIdentifier,
        active_node::active_node::ActiveNode,
    };

    #[derive(Clone)]
    pub struct ActiveLine {
        pub active_id: ActiveIdentifier,
        pub start_node: ActiveNode,
        pub end_node: ActiveNode,
        pub sprite: SpriteBundle,
        pub sprite_entity_id: Entity,
    }
}
