//! This example will display a simple menu using Bevy UI where you can start a new game,
//! change some settings or quit. There is no actual game, it will just display the current
//! settings for 5 seconds before going back to the menu.

use bevy::prelude::*;

pub mod buttons;
mod campaign;
pub mod constants;
mod game_node;
pub mod game_set;
mod node_condition;
mod puzzle;
pub mod puzzle_manager;
pub mod texture;
pub mod util;
use buttons::buttons::button_system;
use campaign::campaign::campaign_plugin;
use puzzle::puzzle::*;

mod splash;
use puzzle_manager::puzzle_manager::PuzzleManager;
use splash::splash::*;

mod menu;
use menu::menu::*;
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

fn main() {
    App::new()
        // Share the SelectedPuzzle resource between menu and puzzle plugin
        .init_resource::<SelectedPuzzle>()
        // Share the CameraControl resource
        .init_resource::<MainCamera>()
        // Create a new puzzle manager to store puzzles (do it here and allow other plugins to manage shared load/unload)
        .insert_resource(PuzzleManager::new())
        .add_plugins(DefaultPlugins)
        // Declare the game state, whose starting value is determined by the `Default` trait
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .add_systems(Update, button_system)
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
