pub mod steam {
    use bevy::prelude::Res;
    use bevy_steamworks::Client;

    pub fn steam_system(steam_client: Res<Client>) {
        let steam_id = steam_client.user().steam_id();
        println!("Steam ID: {}", steam_id.steamid32());
    }
}