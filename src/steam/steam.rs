pub mod steam {
    use bevy::prelude::Res;
    use bevy_steamworks::Client;

    pub fn steam_system(_steam_client: Res<Client>) {
        // TODO do something with steam
    }
}