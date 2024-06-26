//! This example will display a simple menu using Bevy UI where you can start a new game,
//! change some settings or quit. There is no actual game, it will just display the current
//! settings for 5 seconds before going back to the menu.

use std::env;

use backend::api::api;
use bevy::prelude::*;

pub mod backend;

pub mod logic;
use bevy::window::WindowMode;

use bevy_steamworks::SteamworksPlugin;
use logic::puzzle_manager::*;
use logic::util::*;

pub mod steam;
pub mod structs;

pub mod scenes;
use scenes::campaign::campaign::*;
use scenes::menu::menu::*;
use scenes::puzzle::scene::scene::puzzle_plugin;
use scenes::splash::splash::*;

pub mod ui;
use steam::steam::steam::steam_system;
use tracing::subscriber::set_global_default;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use ui::buttons::*;
use ui::constants::*;
use ui::texture::*;

use uuid::Uuid;

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum AppState {
    #[default]
    Splash,
    Menu,
    Campaign,
    Puzzle,
}

// Tag component used to mark which puzzle is currently selected, shared resource in the app
#[derive(Default, Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
struct SelectedPuzzle {
    uuid: Uuid,
}

// Main camera, shared resource in the app
#[derive(Resource, Component, Default)]
struct MainCamera;

fn get_steam_app_id_env_var() -> Result<u32, Box<dyn std::error::Error>> {
    Ok(env::var("STEAM_APP_ID")?.parse::<u32>()?)
}

fn main() {
    // Set up tracer logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    set_global_default(subscriber).expect("Setting default subscriber failed.");

    App::new()
        // Share the SelectedPuzzle resource between menu and puzzle plugin
        .init_resource::<SelectedPuzzle>()
        // Share the CameraControl resource
        .init_resource::<MainCamera>()
        // Create a new puzzle manager to store puzzles (do it here and allow other plugins to manage shared load/unload)
        .insert_resource(puzzle_manager::PuzzleManager::new())
        // Create a new api caller to interface with backend
        .insert_resource(api::NodalApi::new())
        // Adds the steamworks plugin (needs to be before Default for RenderPlugin)
        .add_plugins(SteamworksPlugin::init_app(3063380).unwrap())
        .add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Option::from(Window {
                    title: "Nodal".to_string(),
                    focused: true,
                    mode: WindowMode::Windowed, // TODO fullscreen
                    ..Default::default()
                }),
                ..Default::default()
            }
        ))
        // Set the background color
        .insert_resource(ClearColor(Color::WHITE))
        // Declare the game state, whose starting value is determined by the `Default` trait
        .init_state::<AppState>()
        .add_systems(Startup, (setup, steam_system))
        .add_systems(Update, buttons::button_system)
        // Adds the plugins for each state
        .add_plugins((splash_plugin, menu_plugin, campaign_plugin, puzzle_plugin))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
