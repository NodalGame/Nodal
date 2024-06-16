pub mod traits {

    use bevy::sprite::Sprite;

    use crate::structs::active::active_identifier::active_identifier::ActiveIdentifier;

    pub trait Satisfiable {
        /// Globally unique identifier among "active" objects in the game
        fn identifier(&self) -> &ActiveIdentifier;
        
        /// Sets the object satisfied property, updates any relevant sprites
        fn set_satisfied(&mut self, value: bool);
        
        /// Update the sprites to reflect the object's current state
        fn update_sprites(&mut self, sprites: Vec<&mut Sprite>); 
    }
}
