pub mod satisfiable_entity {
    use std::hash::{Hash, Hasher};

    use crate::objects::active::active_identifier::active_identifier::ActiveIdentifier;

    pub trait Satisfiable {
        /// Globally unique identifier among "active" objects in the game
        fn identifier(&self) -> &ActiveIdentifier;
        /// Computes if the object is satisfied 
        fn compute_satisfied(&self) -> bool;
        /// Sets the object satisfied property, updates any relevant sprites
        fn set_satisfied(&mut self, value: bool);
    }

    pub struct SatisfiableBox(Box<dyn Satisfiable>);

    impl SatisfiableBox {
        pub fn new<T: Satisfiable + 'static>(inner: T) -> SatisfiableBox {
            SatisfiableBox(Box::new(inner))
        }

        pub fn is_satisfied(&self) -> bool {
            self.0.compute_satisfied()
        }

        pub fn set_satisfied(&mut self, value: bool) {
            self.0.set_satisfied(value);
        }
    }

    impl PartialEq for SatisfiableBox {
        fn eq(&self, other: &Self) -> bool {
            self.0.compute_satisfied() == other.0.compute_satisfied()
        }
    }

    impl Eq for SatisfiableBox {}

    impl Hash for SatisfiableBox {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.compute_satisfied().hash(state);
        }
    }
}