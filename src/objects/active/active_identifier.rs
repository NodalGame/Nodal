pub mod active_identifier {
    use uuid::Uuid;

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub struct ActiveIdentifier {
        id: Uuid,
    }

    impl ActiveIdentifier {
        pub fn new() -> Self {
            ActiveIdentifier { id: Uuid::new_v4() }
        }

        pub fn get_id(&self) -> &Uuid {
            &self.id
        }
    }
}
