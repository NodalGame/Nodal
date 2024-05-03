pub mod traits {

    use crate::objects::active::active_identifier::active_identifier::ActiveIdentifier;

    pub trait Satisfiable {
        /// Globally unique identifier among "active" objects in the game
        fn identifier(&self) -> &ActiveIdentifier;
        /// Sets the object satisfied property, updates any relevant sprites
        fn set_satisfied(&mut self, value: bool);
    }
}
