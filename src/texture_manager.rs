pub mod texture_manager {
    use bevy::prelude::*;

    #[derive(Resource, Debug)]
    pub struct TextureManager {
        textures: HashMap<Texture, Handle<Image>>,
    }

    
}

