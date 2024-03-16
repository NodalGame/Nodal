use bevy::{
    a11y::accesskit::Vec2,
    asset::{AssetServer, Handle},
    render::texture::Image,
    sprite::{Sprite, SpriteBundle},
    utils::default,
};

use crate::texture::texture::Texture;

pub mod puzzle {
    use std::f32::consts::PI;
    use std::process::exit;

    use crate::game_node::game_node::NodeClass;
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
            .insert_resource(ActiveLines::default())
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
    struct ActiveLines {
        lines: Vec<ActiveLine>,
    }

    struct ActiveLine {
        start_node: ActiveNode,
        end_node: ActiveNode,
        sprite: SpriteBundle,
    }

    // Line currently being drawn by user on the screen
    #[derive(Default, Resource)]
    struct CurrentLine {
        start_node: Option<ActiveNode>,
    }

    // Tag component used to tag entities added on the puzzle screen
    #[derive(Component)]
    struct OnPuzzleScreen;

    const SPRITE_SIZE: f32 = 100.0;
    const SPRITE_SPACING: f32 = 100.0;

    fn puzzle_setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        puzzle_id: Res<SelectedPuzzle>,
        puzzle_manager: Res<PuzzleManager>,
        mut active_nodes: ResMut<ActiveNodes>,
        // Query to get camera transform
        mut q_camera: Query<&mut Transform, With<MainCamera>>,
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

        // Load node textures
        let tex_node_red = asset_server.load(Texture::ClassRed.path());
        let tex_node_blue = asset_server.load(Texture::ClassBlue.path());
        let tex_node_yellow = asset_server.load(Texture::ClassYellow.path());

        // Create a width x height grid of nodes as sprite bundles, accounting for background tiles
        for x in 0..puzzle.width * 2 + 1 {
            for y in 0..puzzle.height * 2 + 1 {
                // If background tile, spawn it and continue
                if x % 2 == 0 || y % 2 == 0 {
                    commands.spawn(get_bg_tile(
                        x,
                        y,
                        puzzle.width,
                        puzzle.height,
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

                let x_pos = x as f32 * SPRITE_SPACING;
                let y_pos = y as f32 * SPRITE_SPACING;

                let texture = match node.class {
                    NodeClass::Red => tex_node_red.clone(),
                    NodeClass::Blue => tex_node_blue.clone(),
                    NodeClass::Yellow => tex_node_yellow.clone(),
                    _ => {
                        println!("Error when adding nodes to screen, invalid class?");
                        exit(1);
                    }
                };

                let sprite_bundle: SpriteBundle = SpriteBundle {
                    texture,
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(SPRITE_SIZE, SPRITE_SIZE)),
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

        // Get camera transform
        for mut transform in q_camera.iter_mut() {
            // Move it to center of puzzle
            *transform = Transform {
                translation: Vec3::new(
                    puzzle.width as f32 * SPRITE_SPACING,
                    puzzle.height as f32 * SPRITE_SPACING,
                    0.0,
                ),
                ..default()
            };
        }
    }

    /// A system for handling lines added to the puzzle.
    ///
    /// # Parameters
    ///
    /// - `commands`: Bevy's command system, used to spawn new entities.
    /// - `active_nodes`: Resource containing all active nodes in the puzzle.
    /// - `current_line`: Resource containing the current line being drawn by the user.
    /// - `lines`: Resource containing all lines in the puzzle.
    /// - `mouse_button_input`: Bevy's mouse button input system, used to check if the left mouse button is pressed.
    /// - `q_window`: Bevy's query system, used to get the window, so we can read current cursor position.
    /// - `q_camera`: Bevy's query system, used to get the camera transform to also read current cursor position.
    /// - `asset_server`: Bevy's asset server, used to load textures.
    fn line_system(
        mut commands: Commands,
        active_nodes: ResMut<ActiveNodes>,
        mut current_line: ResMut<CurrentLine>,
        mut lines: ResMut<ActiveLines>,
        mouse_button_input: Res<ButtonInput<MouseButton>>,
        q_window: Query<&Window, With<PrimaryWindow>>,
        q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
        asset_server: Res<AssetServer>,
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
                    let start_pos = current_line
                        .start_node.clone()
                        .unwrap()
                        .sprite
                        .transform
                        .translation
                        .truncate();
                    let end_pos = active_node.sprite.transform.translation.truncate();

                    // Get the appropriate line texture, if exists (otherwise invalid node pair)
                    let line_texture = get_line_texture(
                        current_line.start_node.clone().unwrap(),
                        active_node.clone(),
                    )
                    .unwrap_or(&Texture::Missing);

                    if *line_texture == Texture::Missing {
                        break;
                    }

                    let line_sprite = SpriteBundle {
                        texture: asset_server.load(line_texture.path()),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(SPRITE_SIZE, SPRITE_SIZE)),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(
                            (end_pos.x + start_pos.x) / 2.0,
                            (end_pos.y + start_pos.y) / 2.0,
                            0.0,
                        ),
                        ..default()
                    };

                    // Update list of lines
                    println!(
                        "Current line end is {:?}",
                        active_node.sprite.transform.translation.truncate()
                    );

                    lines.lines.push(ActiveLine {
                        start_node: current_line.start_node.clone().unwrap(),
                        end_node: active_node.clone(),
                        sprite: line_sprite.clone(),
                    });
                    // Add line to the screen
                    commands.spawn(line_sprite).insert(OnPuzzleScreen);

                    // Break since only one node could've been released on
                    break;
                }
            }
            // Regardless if we ended on a node or not, clear the current line
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
    fn get_bg_tile(x: u8, y: u8, width: u8, height: u8, asset_server: AssetServer) -> SpriteBundle {
        // Load background textures
        let bg_side_left: Handle<Image> = asset_server.load(Texture::BgTileSideLeft.path());
        let bg_side_right: Handle<Image> = asset_server.load(Texture::BgTileSideRight.path());
        let bg_side_bottom: Handle<Image> = asset_server.load(Texture::BgTileSideBottom.path());
        let bg_side_top: Handle<Image> = asset_server.load(Texture::BgTileSideTop.path());
        let bg_bottom_left: Handle<Image> = asset_server.load(Texture::BgTileBottomLeft.path());
        let bg_bottom_right: Handle<Image> = asset_server.load(Texture::BgTileBottomRight.path());
        let bg_top_left: Handle<Image> = asset_server.load(Texture::BgTileTopLeft.path());
        let bg_top_right: Handle<Image> = asset_server.load(Texture::BgTileTopRight.path());
        let bg_between_horizontal: Handle<Image> =
            asset_server.load(Texture::BgTileBetweenHorizontal.path());
        let bg_between_vertical: Handle<Image> =
            asset_server.load(Texture::BgTileBetweenVertical.path());
        let bg_between_cross: Handle<Image> = asset_server.load(Texture::BgTileBetweenCross.path());

        let transform =
            Transform::from_xyz(x as f32 * SPRITE_SPACING, y as f32 * SPRITE_SPACING, 0.0);
        let sprite = Sprite {
            custom_size: Some(Vec2::new(SPRITE_SIZE, SPRITE_SIZE)),
            ..Default::default()
        };

        // Bottom left corner
        if x == 0 && y == 0 {
            SpriteBundle {
                texture: bg_bottom_left,
                sprite,
                transform,
                ..default()
            }
        } else if x == 0 {
            // Top left corner
            if y == height * 2 {
                SpriteBundle {
                    texture: bg_top_left,
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
                    texture: bg_bottom_right,
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
                    texture: bg_top_right,
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
                } else if x % 2 == 0 {
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

    fn get_line_texture(start_node: ActiveNode, end_node: ActiveNode) -> Option<&'static Texture> {
        let start_pos = start_node.sprite.transform.translation.truncate();
        let end_pos = end_node.sprite.transform.translation.truncate();
        let direction = end_pos - start_pos;
        let distance = direction.length();
        let angle = direction.y.atan2(direction.x);

        // Determine if line is valid connection between adjacent nodes
        if distance > SPRITE_SPACING + SPRITE_SIZE && (angle == 0.0 || angle == PI / 2.0) {
            return None;
        } else if distance > (2.0 * (SPRITE_SPACING + SPRITE_SIZE).powi(2)).sqrt() {
            return None;
        }

        println!("angle {}", angle);

        if angle.abs() == 0.0 || angle.abs() == PI {
            Some(&Texture::LineHorizontal)
        } else if angle.abs() == PI / 2.0 {
            Some(&Texture::LineVertical)
        } else if angle == PI / 4.0 || angle == -3.0 * PI / 4.0 {
            Some(&Texture::LineDiagonalBottomLeftTopRight)
        } else {
            Some(&Texture::LineDiagonalTopLeftBottomRight)
        }
    }
}
