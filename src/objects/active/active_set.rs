pub mod active_set {
    use crate::objects::{active::{active_identifier::active_identifier::ActiveIdentifier, satisfiable_entity::satisfiable_entity::Satisfiable}, immutable::game_set::game_set::GameSet};

    #[derive(Clone)]
    pub struct ActiveSet {
        pub active_id: ActiveIdentifier,
        pub set: GameSet,
        // TODO each rule should be tracked here as satisfiable, and compose the satisfiable state for this set
        pub satisfied: bool,
    }

    impl ActiveSet {}

    impl Satisfiable for ActiveSet {
        fn compute_satisfied(&self) -> bool {
            true
            // TODO this should also check all set rules (which are also Satisfiable)
        }
    
        fn set_satisfied(&mut self, value: bool) {
            self.satisfied = value;
        }
    
        fn identifier(&self) -> &ActiveIdentifier {
            &self.active_id
        }
    }
}