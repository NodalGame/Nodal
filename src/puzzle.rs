use bevy::{
    a11y::accesskit::Vec2,
    asset::{AssetServer, Handle},
    render::texture::Image,
    sprite::{Sprite, SpriteBundle},
    utils::default,
};

use crate::texture::texture::Texture;

pub mod puzzle {
    use std::process::exit;

    use crate::{game_node::game_node::GameNode, texture::texture::Texture, MainCamera};
    use bevy::window::PrimaryWindow;
    use bevy::{prelude::*, render::camera::Viewport};
    use bevy_prototype_lyon::prelude::*;
    use serde::Deserialize;
    use uuid::Uuid;

    use crate::{
        despawn_screen, puzzle_manager::puzzle_manager::PuzzleManager, AppState, SelectedPuzzle,
    };

    // This plugin will contain a playable puzzle.
    pub fn puzzle_plugin(app: &mut App) {
        app.add_systems(OnEnter(AppState::Puzzle), puzzle_setup)
            .add_systems(OnExit(AppState::Puzzle), despawn_screen::<OnPuzzleScreen>)
            .add_systems(Update, line_system.run_if(in_state(AppState::Puzzle)))
            .insert_resource(ActiveNodes::default())
            .insert_resource(Lines::default())
            .insert_resource(CurrentLine::default());
    }

    #[derive(Deserialize, Debug)]
    pub struct Puzzle {
        pub uuid: Uuid,
        width: u8,
        height: u8,
        nodes: Vec<GameNode>,
    }

    // Tracks all nodes in the puzzle, including sprite and position
    #[derive(Default, Resource, Component, Clone)]
    struct ActiveNodes {
        active_nodes: Vec<ActiveNode>,
    }

    #[derive(Component, Clone)]
    struct ActiveNode {
        node: GameNode,
        sprite: SpriteBundle,
    }

    // Tracks all lines connecting nodes in puzzle
    #[derive(Default, Resource)]
    struct Lines {
        lines: Vec<NodeLine>,
    }

    struct NodeLine {
        start_node: ActiveNode,
        end_node: ActiveNode,
        line: shapes::Line,
    }

    // Line currently being drawn by user on the screen
    #[derive(Default, Resource)]
    struct CurrentLine {
        start_node: Option<ActiveNode>,
        line: Option<shapes::Line>,
    }

    // Tag component used to tag entities added on the puzzle screen
    #[derive(Component)]
    struct OnPuzzleScreen;

    fn puzzle_setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        puzzle_id: Res<SelectedPuzzle>,
        puzzle_manager: Res<PuzzleManager>,
        mut active_nodes: ResMut<ActiveNodes>,
        // Query to get camera transform
        q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    ) {
        // Get the puzzle by loading it
        let puzzle = puzzle_manager
            .load_puzzle(&puzzle_id.uuid)
            .unwrap_or_else(|| {
                println!("Failed to load puzzle with id {}", puzzle_id.uuid);
                exit(1); // TODO cause game to not crash, and do this check in menu BEFORE switching scenes
            });

        // Sort nodes by id (top to bottom, left to right)
        let mut ordered_nodes = puzzle.nodes.clone();
        ordered_nodes.sort_by(|a, b| a.id.cmp(&b.id));

        // Load node texture
        let tex_node = asset_server.load(Texture::NodeEmpty.path());

        // Create a width x height grid of nodes as sprite bundles, accounting for background tiles
        let spacing = 100.0;
        let sprite_size = 100.0;
        for x in 0..puzzle.width * 2 + 1 {
            for y in 0..puzzle.height * 2 + 1 {
                // If background tile, spawn it and continue
                if x % 2 == 0 || y % 2 == 0 {
                    commands.spawn(get_bg_tile(
                        x,
                        y,
                        puzzle.width,
                        puzzle.height,
                        sprite_size,
                        spacing,
                        asset_server.clone(),
                    ));
                    continue;
                }

                let node = ordered_nodes
                    .get((x / 2 * puzzle.height + y / 2) as usize)
                    .unwrap_or_else(|| {
                        println!("Error when adding nodes to screen, index out of range?");
                        exit(1);
                    });

                let x_pos = x as f32 * spacing;
                let y_pos = y as f32 * spacing;

                let sprite_bundle: SpriteBundle = SpriteBundle {
                    texture: tex_node.clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(sprite_size, sprite_size)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(x_pos, y_pos, 0.0),
                    ..default()
                };
                commands.spawn(sprite_bundle.clone()).insert(OnPuzzleScreen);
                active_nodes.active_nodes.push(ActiveNode {
                    node: node.clone(),
                    sprite: sprite_bundle.clone(),
                });
            }
        }

        // Get camera info and transform, assuming exacly 1 camera entity
        let (camera, camera_transform) = q_camera.single();

        println!("Camera transform is {:?}", camera_transform);

        // Move it to center of puzzle
        camera_transform.translation().x = (puzzle.width as f32 * spacing) / 2.0;
        camera_transform.translation().y = (puzzle.height as f32 * spacing) / 2.0;

        println!("Camera transform is now {:?}", camera_transform);
    }

    fn line_system(
        mut commands: Commands,
        active_nodes: ResMut<ActiveNodes>,
        mut current_line: ResMut<CurrentLine>,
        mut lines: ResMut<Lines>,
        // Track mouse inputs
        mouse_button_input: Res<ButtonInput<MouseButton>>,
        // Query to get the window, so we can read current cursor position
        q_window: Query<&Window, With<PrimaryWindow>>,
        // Query to get camera transform
        q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    ) {
        // Get camera info and transform, assuming exacly 1 camera entity
        let (camera, camera_transform) = q_camera.single();

        // Only one primary window, so get it from query
        let window = q_window.single();

        // Check if cursor inside window and get its position, convert to world coords, discard Z
        let world_position = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
            .unwrap_or_else(|| {
                // TODO fix this it causes game to crash when you hover out of it
                println!("Failed to get cursor position");
                exit(1);
            });

        // On left click, start new line on a clicked node, if exists
        if mouse_button_input.just_pressed(MouseButton::Left) {
            for active_node in active_nodes.active_nodes.iter() {
                if clicked_on_sprite(&active_node.sprite, world_position) {
                    println!("Left mouse button pressed on node {}", active_node.node.id);
                    current_line.start_node = Some(active_node.clone());
                    current_line.line = Some(shapes::Line(
                        active_node.sprite.transform.translation.truncate(),
                        world_position,
                    ));
                    println!(
                        "Current line start is {:?}",
                        current_line.line.clone().unwrap().0
                    );
                }
            }
        // If left click release, end the line on the released node, if exists
        } else if mouse_button_input.just_released(MouseButton::Left) {
            for active_node in active_nodes.active_nodes.iter() {
                if clicked_on_sprite(&active_node.sprite, world_position)
                    && current_line.start_node.is_some()
                    && active_node.node.id != current_line.start_node.clone().unwrap().node.id
                {
                    println!("Left mouse button released on node {}", active_node.node.id);
                    // Update list of lines
                    println!(
                        "Current line end is {:?}",
                        active_node.sprite.transform.translation.truncate()
                    );
                    let finished_line = shapes::Line(
                        current_line.line.clone().unwrap().0,
                        active_node.sprite.transform.translation.truncate(),
                    );
                    println!("Finished line is {:?}", finished_line);
                    lines.lines.push(NodeLine {
                        start_node: current_line.start_node.clone().unwrap(),
                        end_node: active_node.clone(),
                        line: finished_line,
                    });
                    // Add line to the screen
                    commands
                        .spawn(ShapeBundle {
                            path: GeometryBuilder::build_as(&finished_line),
                            spatial: SpatialBundle::default(),
                            ..default()
                        })
                        .insert(OnPuzzleScreen);
                }
            }
            // Regardless if we ended on a node or not, clear the current line
            current_line.line = None;
            current_line.start_node = None;
        }
    }

    fn clicked_on_sprite(sprite: &SpriteBundle, cursor: Vec2) -> bool {
        let node_pos = sprite.transform.translation.truncate();
        let distance = cursor.distance(node_pos);
        // Assuming the sprite size is a good proxy for click detection radius
        if distance
            < sprite
                .sprite
                .custom_size
                .unwrap_or_else(|| {
                    // TODO fix this
                    println!("Failed to get sprite size");
                    exit(1);
                })
                .x
                / 2.0
        {
            return true;
        }
        false
    }

    /// Returns a background tile as a sprite bundle.
    ///
    /// # Parameters
    ///
    /// - `x_pos`: The x position of the tile in the grid (0..width*2+1)
    /// - `y_pos`: The y position of the tile in the grid (0..height*2+1)
    /// - `width`: The width of the puzzle.
    /// - `height`: The height of the puzzle.
    /// - `sprite_size`: The size of the sprite in pixels.
    /// - `spacing`: The spacing between each sprite in the grid.
    ///
    /// # Returns
    ///
    /// A sprite bundle representing the background tile at the given position.
    fn get_bg_tile(
        x: u8,
        y: u8,
        width: u8,
        height: u8,
        sprite_size: f32,
        spacing: f32,
        asset_server: AssetServer,
    ) -> SpriteBundle {
        // Load background textures
        let bg_side_left: Handle<Image> = asset_server.load(Texture::BgTileSideLeft.path());
        let bg_side_right: Handle<Image> = asset_server.load(Texture::BgTileSideRight.path());
        let bg_side_bottom: Handle<Image> = asset_server.load(Texture::BgTileSideBottom.path());
        let bg_side_top: Handle<Image> = asset_server.load(Texture::BgTileSideTop.path());
        let bg_side_bottom_left: Handle<Image> = asset_server.load(Texture::BgTileSideLeft.path());
        let bg_side_bottom_right: Handle<Image> =
            asset_server.load(Texture::BgTileSideRight.path());
        let bg_side_top_left: Handle<Image> = asset_server.load(Texture::BgTileTopLeft.path());
        let bg_side_top_right: Handle<Image> = asset_server.load(Texture::BgTileTopRight.path());
        let bg_between_horizontal: Handle<Image> =
            asset_server.load(Texture::BgTileBetweenHorizontal.path());
        let bg_between_vertical: Handle<Image> =
            asset_server.load(Texture::BgTileBetweenVertical.path());
        let bg_between_cross: Handle<Image> = asset_server.load(Texture::BgTileBetweenCross.path());

        let transform = Transform::from_xyz(x as f32 * spacing, y as f32 * spacing, 0.0);
        let sprite = Sprite {
            custom_size: Some(Vec2::new(sprite_size, sprite_size)),
            ..Default::default()
        };

        // Bottom left corner
        if x == 0 && y == 0 {
            SpriteBundle {
                texture: bg_side_bottom_left,
                sprite: Sprite {
                    custom_size: Some(Vec2::new(sprite_size, sprite_size)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            }
        } else if x == 0 {
            // Top left corner
            if y == height * 2 {
                SpriteBundle {
                    texture: bg_side_top_left,
                    sprite,
                    transform,
                    ..default()
                }
            } else {
                // Left side
                SpriteBundle {
                    texture: bg_side_left,
                    sprite,
                    transform,
                    ..default()
                }
            }
        } else if y == 0 {
            // Bottom right corner
            if x == width * 2 {
                SpriteBundle {
                    texture: bg_side_bottom_right,
                    sprite,
                    transform,
                    ..default()
                }
            } else {
                // Bottom side
                SpriteBundle {
                    texture: bg_side_bottom,
                    sprite,
                    transform,
                    ..default()
                }
            }
        } else {
            // Top right corner
            if x == width * 2 && y == height * 2 {
                SpriteBundle {
                    texture: bg_side_top_right,
                    sprite,
                    transform,
                    ..default()
                }
            } else if x == width * 2 {
                // Right side
                SpriteBundle {
                    texture: bg_side_right,
                    sprite,
                    transform,
                    ..default()
                }
            } else if y == height * 2 {
                // Top side
                SpriteBundle {
                    texture: bg_side_top,
                    sprite,
                    transform,
                    ..default()
                }
            } else {
                if x % 2 == 0 && y % 2 == 0 {
                    // Between cross
                    SpriteBundle {
                        texture: bg_between_cross,
                        sprite,
                        transform,
                        ..default()
                    }
                } else if x & 2 == 0 {
                    // Between horizontal
                    SpriteBundle {
                        texture: bg_between_horizontal,
                        sprite,
                        transform,
                        ..default()
                    }
                } else {
                    // Between vertical
                    SpriteBundle {
                        texture: bg_between_vertical,
                        sprite,
                        transform,
                        ..default()
                    }
                }
            }
        }
    }
}
