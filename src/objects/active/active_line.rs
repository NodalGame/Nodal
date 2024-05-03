pub mod active_line {
    use bevy::sprite::SpriteBundle;

    use crate::objects::active::active_node::active_node::ActiveNode;

    #[derive(Clone)]
    pub struct ActiveLine {
        pub start_node: ActiveNode,
        pub end_node: ActiveNode,
        pub sprite: SpriteBundle,
    }
}
