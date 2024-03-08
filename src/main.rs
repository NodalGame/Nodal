//! This example will display a simple menu using Bevy UI where you can start a new game,
//! change some settings or quit. There is no actual game, it will just display the current
//! settings for 5 seconds before going back to the menu.

use bevy::prelude::*;

mod puzzle;
mod game_node;
pub mod puzzle_manager;
use puzzle::puzzle::*;

mod splash;
use splash::splash::*;

mod menu;
use menu::menu::*;
use uuid::Uuid;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum AppState {
    #[default]
    Splash,
    Menu,
    Puzzle,
}

// Tag component used to mark which puzzle is currently selected, shared resource in the app
#[derive(Default, Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
struct SelectedPuzzle {
    uuid: Uuid,
}

// Camera control, shared resource in the app
#[derive(Resource, Default)]
struct CameraControl {
    focus_point: Vec2,
    // Add zoom level? 
}

fn main() {
    App::new()
        // Share the SelectedPuzzle resource between menu and puzzle plugin
        .init_resource::<SelectedPuzzle>()
        // Share the CameraControl resource
        .init_resource::<CameraControl>()
        .add_plugins(DefaultPlugins)
        // Declare the game state, whose starting value is determined by the `Default` trait
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        // Adds the plugins for each state
        .add_plugins((splash_plugin, menu_plugin, puzzle_plugin))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}