use std::f32::consts::PI;

use bevy::{
    asset::{AssetServer, Handle},
    color::Color,
    math::Vec2,
    prelude::{default, Commands, Res},
    render::texture::Image,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
};
use tracing::error;

use crate::{
    get_node_down, get_node_down_left, get_node_down_right, get_node_left, get_node_right,
    get_node_up, get_node_up_left, get_node_up_right, get_set_order, get_sets_containing_node,
    is_bottom_edge, is_left_edge, is_right_edge, is_top_edge, node_to_position,
    structs::{
        active::{
            active_identifier::active_identifier::ActiveIdentifier,
            active_line::active_line::ActiveLine, active_node::active_node::ActiveNode,
        },
        immutable::{
            game_node::game_node::GameNodeId, game_set::game_set::GameSet, puzzle::puzzle::Puzzle,
        },
    },
    texture::Texture,
    BG_SET_SPRITE_SIZE, COLOR_SET_0, COLOR_SET_1, COLOR_SET_2, COLOR_SET_BORDER, SPRITE_SPACING,
    TILE_NODE_SPRITE_SIZE, Z_BACKGROUND, Z_LINE, Z_SET_FILL,
};

use super::scene::scene::OnPuzzleScene;

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
pub fn get_puzzle_background_tile(
    x: u8,
    y: u8,
    width: u8,
    height: u8,
    asset_server: AssetServer,
) -> SpriteBundle {
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

    let transform = Transform::from_xyz(
        x as f32 * SPRITE_SPACING,
        y as f32 * SPRITE_SPACING,
        Z_BACKGROUND,
    );
    let sprite = Sprite {
        custom_size: Some(Vec2::new(TILE_NODE_SPRITE_SIZE, TILE_NODE_SPRITE_SIZE)),
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

/// Returns the texture for a line connecting two active nodes.
///
/// # Parameters
///
/// - `start_node`: The active node at the start of the line.
/// - `end_node`: The active node at the end of the line.
///
/// # Returns
///
/// A texture for the line connecting the two active nodes, or `None` if the line is invalid
/// (non-adjacent nodes).
pub fn get_line_texture(
    start_node: &ActiveNode,
    end_node: &ActiveNode,
) -> Option<&'static Texture> {
    let start_pos = start_node.sprite.transform.translation.truncate();
    let end_pos = end_node.sprite.transform.translation.truncate();
    let direction = end_pos - start_pos;
    let distance = direction.length();
    let angle = direction.y.atan2(direction.x);

    // Determine if line is valid connection between adjacent nodes
    if distance > SPRITE_SPACING + TILE_NODE_SPRITE_SIZE && (angle == 0.0 || angle == PI / 2.0) {
        return None;
    } else if distance > (2.0 * (SPRITE_SPACING + TILE_NODE_SPRITE_SIZE).powi(2)).sqrt() {
        return None;
    }

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

/// Returns the set border sprite (not the texture).
fn set_border_sprite() -> Sprite {
    Sprite {
        custom_size: Some(Vec2::new(TILE_NODE_SPRITE_SIZE, TILE_NODE_SPRITE_SIZE)),
        color: COLOR_SET_BORDER,
        ..Default::default()
    }
}

/// Returns a vector of sprite bundles for vertical edge set tiles using one node
/// as the focal point. It is responsible for generating appropriate set tiles to
/// the immediate left, right, up left, and up right of the node.
///
/// # Parameters
///
/// - `node`: The node around which set tiles are being added.
/// - `node_x`: The node's X coordinate in the scene.
/// - `node_y`: The node's Y coordinate in the scene.
/// - `set`: The game set for which tiles are being added.
/// - `puzzle`: The puzzle for which the tiles are being added.
/// - `asset_server`: The asset server used to load textures.
///
/// # Returns
///
/// A vector of Sprite Bundles.
fn get_set_tiles_vertical(
    node: &u16,
    node_x: f32,
    node_y: f32,
    set: &GameSet,
    puzzle: &Puzzle,
    asset_server: AssetServer,
) -> Vec<SpriteBundle> {
    let mut vertical_tiles = Vec::new();
    let tex_set_tile_vertical = asset_server.load(if set.bounded {
        Texture::SetTileBoundedVertical.path()
    } else {
        Texture::SetTileVertical.path()
    });

    let node_left = get_node_left(node, puzzle).unwrap_or(u16::MAX);
    if is_left_edge(node, puzzle) || !set.nodes.contains(&node_left) {
        vertical_tiles.push(SpriteBundle {
            texture: tex_set_tile_vertical.clone(),
            sprite: set_border_sprite(),
            transform: Transform::from_xyz(node_x - SPRITE_SPACING, node_y, 0.0),
            ..default()
        })
    }

    let node_right = get_node_right(node, puzzle).unwrap_or(u16::MAX);
    if is_right_edge(node, puzzle) || !set.nodes.contains(&node_right) {
        vertical_tiles.push(SpriteBundle {
            texture: tex_set_tile_vertical.clone(),
            sprite: set_border_sprite(),
            transform: Transform::from_xyz(node_x + SPRITE_SPACING, node_y, 0.0),
            ..default()
        });
    }

    if !is_top_edge(node, puzzle) {
        let node_up = get_node_up(node, puzzle).unwrap_or(u16::MAX);

        // Above left
        let node_up_left = get_node_up_left(node, puzzle).unwrap_or(u16::MAX);
        if !set.nodes.contains(&node_up_left)
            && !set.nodes.contains(&node_left)
            && set.nodes.contains(&node_up)
        {
            vertical_tiles.push(SpriteBundle {
                texture: tex_set_tile_vertical.clone(),
                sprite: set_border_sprite(),
                transform: Transform::from_xyz(
                    node_x - SPRITE_SPACING,
                    node_y + SPRITE_SPACING,
                    0.0,
                ),
                ..default()
            });
        }

        // Above right
        let node_up_right = get_node_up_right(node, puzzle).unwrap_or(u16::MAX);
        if !set.nodes.contains(&node_up_right)
            && !set.nodes.contains(&node_right)
            && set.nodes.contains(&node_up)
        {
            vertical_tiles.push(SpriteBundle {
                texture: tex_set_tile_vertical.clone(),
                sprite: set_border_sprite(),
                transform: Transform::from_xyz(
                    node_x + SPRITE_SPACING,
                    node_y + SPRITE_SPACING,
                    0.0,
                ),
                ..default()
            });
        }
    }

    vertical_tiles
}

/// Returns a vector of sprite bundles for horizontal edge set tiles using one node
/// as the focal point. It is responsible for generating appropriate set tiles to
/// the immediate top, bottom, down right, and up right of the node.
///
/// # Parameters
///
/// - `node`: The node around which set tiles are being added.
/// - `node_x`: The node's X coordinate in the scene.
/// - `node_y`: The node's Y coordinate in the scene.
/// - `set`: The game set for which tiles are being added.
/// - `puzzle`: The puzzle for which the tiles are being added.
/// - `asset_server`: The asset server used to load textures.
///
/// # Returns
///
/// A vector of Sprite Bundles.
fn get_set_tiles_horizontal(
    node: &u16,
    node_x: f32,
    node_y: f32,
    set: &GameSet,
    puzzle: &Puzzle,
    asset_server: AssetServer,
) -> Vec<SpriteBundle> {
    let mut horizontal_tiles = Vec::new();
    let tex_set_tile_horizontal = asset_server.load(if set.bounded {
        Texture::SetTileBoundedHorizontal.path()
    } else {
        Texture::SetTileHorizontal.path()
    });

    // Directly above
    let node_up = get_node_up(node, puzzle).unwrap_or(u16::MAX);
    if is_top_edge(node, &puzzle) || !set.nodes.contains(&node_up) {
        horizontal_tiles.push(SpriteBundle {
            texture: tex_set_tile_horizontal.clone(),
            sprite: set_border_sprite(),
            transform: Transform::from_xyz(node_x, node_y + SPRITE_SPACING, 0.0),
            ..default()
        });
    }

    // Directly below
    let node_down = get_node_down(node, puzzle).unwrap_or(u16::MAX);
    if is_bottom_edge(node, &puzzle) || !set.nodes.contains(&node_down) {
        horizontal_tiles.push(SpriteBundle {
            texture: tex_set_tile_horizontal.clone(),
            sprite: set_border_sprite(),
            transform: Transform::from_xyz(node_x, node_y - SPRITE_SPACING, 0.0),
            ..default()
        });
    }

    if !is_right_edge(node, &puzzle) {
        let node_right = get_node_right(node, puzzle).unwrap_or(u16::MAX);

        // Above right
        let node_up_right = get_node_up_right(node, puzzle).unwrap_or(u16::MAX);
        if !set.nodes.contains(&node_up_right)
            && !set.nodes.contains(&node_up)
            && set.nodes.contains(&node_right)
        {
            horizontal_tiles.push(SpriteBundle {
                texture: tex_set_tile_horizontal.clone(),
                sprite: set_border_sprite(),
                transform: Transform::from_xyz(
                    node_x + SPRITE_SPACING,
                    node_y + SPRITE_SPACING,
                    0.0,
                ),
                ..default()
            });
        }

        // Below right
        let node_down_right = get_node_down_right(node, puzzle).unwrap_or(u16::MAX);
        if !set.nodes.contains(&node_down_right)
            && !set.nodes.contains(&node_down)
            && set.nodes.contains(&node_right)
        {
            horizontal_tiles.push(SpriteBundle {
                texture: tex_set_tile_horizontal.clone(),
                sprite: set_border_sprite(),
                transform: Transform::from_xyz(
                    node_x + SPRITE_SPACING,
                    node_y - SPRITE_SPACING,
                    0.0,
                ),
                ..default()
            });
        }
    }

    horizontal_tiles
}

/// Returns a vector of sprite bundles for bottom right set tiles using one node
/// as the focal point. It is responsible for generating appropriate set tiles to
/// the immediate up left and down right of the node.
///
/// # Parameters
///
/// - `node`: The node around which set tiles are being added.
/// - `node_x`: The node's X coordinate in the scene.
/// - `node_y`: The node's Y coordinate in the scene.
/// - `set`: The game set for which tiles are being added.
/// - `puzzle`: The puzzle for which the tiles are being added.
/// - `asset_server`: The asset server used to load textures.
///
/// # Returns
///
/// A vector of Sprite Bundles.
fn get_set_tiles_bottom_right(
    node: &u16,
    node_x: f32,
    node_y: f32,
    set: &GameSet,
    puzzle: &Puzzle,
    asset_server: AssetServer,
) -> Vec<SpriteBundle> {
    let mut bottom_right_tiles = Vec::new();
    let tex_set_tile_bottom_right = asset_server.load(if set.bounded {
        Texture::SetTileBoundedBottomRight.path()
    } else {
        Texture::SetTileBottomRight.path()
    });

    let node_down = get_node_down(node, puzzle).unwrap_or(u16::MAX);
    let node_right = get_node_right(node, puzzle).unwrap_or(u16::MAX);
    if !set.nodes.contains(&node_down) && !set.nodes.contains(&node_right) {
        bottom_right_tiles.push(SpriteBundle {
            texture: tex_set_tile_bottom_right.clone(),
            sprite: set_border_sprite(),
            transform: Transform::from_xyz(node_x + SPRITE_SPACING, node_y - SPRITE_SPACING, 0.0),
            ..default()
        });
    }

    let node_up_left = get_node_up_left(node, &puzzle).unwrap_or(u16::MAX);
    let node_up = get_node_up(node, puzzle).unwrap_or(u16::MAX);
    let node_left = get_node_left(node, puzzle).unwrap_or(u16::MAX);
    if set.nodes.contains(&node_up)
        && set.nodes.contains(&node_left)
        && !set.nodes.contains(&node_up_left)
    {
        bottom_right_tiles.push(SpriteBundle {
            texture: tex_set_tile_bottom_right.clone(),
            sprite: set_border_sprite(),
            transform: Transform::from_xyz(node_x - SPRITE_SPACING, node_y + SPRITE_SPACING, 0.0),
            ..default()
        });
    }

    bottom_right_tiles
}

/// Returns a vector of sprite bundles for bottom left set tiles using one node
/// as the focal point. It is responsible for generating appropriate set tiles to
/// the immediate up right and down left of the node.
///
/// # Parameters
///
/// - `node`: The node around which set tiles are being added.
/// - `node_x`: The node's X coordinate in the scene.
/// - `node_y`: The node's Y coordinate in the scene.
/// - `set`: The game set for which tiles are being added.
/// - `puzzle`: The puzzle for which the tiles are being added.
/// - `asset_server`: The asset server used to load textures.
///
/// # Returns
///
/// A vector of Sprite Bundles.
fn get_set_tiles_bottom_left(
    node: &u16,
    node_x: f32,
    node_y: f32,
    set: &GameSet,
    puzzle: &Puzzle,
    asset_server: AssetServer,
) -> Vec<SpriteBundle> {
    let mut bottom_left_tiles = Vec::new();
    let tex_set_tile_bottom_left = asset_server.load(if set.bounded {
        Texture::SetTileBoundedBottomLeft.path()
    } else {
        Texture::SetTileBottomLeft.path()
    });

    let node_down = get_node_down(node, puzzle).unwrap_or(u16::MAX);
    let node_left = get_node_left(node, puzzle).unwrap_or(u16::MAX);
    if !set.nodes.contains(&node_down) && !set.nodes.contains(&node_left) {
        bottom_left_tiles.push(SpriteBundle {
            texture: tex_set_tile_bottom_left.clone(),
            sprite: set_border_sprite(),
            transform: Transform::from_xyz(node_x - SPRITE_SPACING, node_y - SPRITE_SPACING, 0.0),
            ..default()
        });
    }

    let node_up_right = get_node_up_right(node, &puzzle).unwrap_or(u16::MAX);
    let node_up = get_node_up(node, puzzle).unwrap_or(u16::MAX);
    let node_right = get_node_right(node, puzzle).unwrap_or(u16::MAX);
    if set.nodes.contains(&node_up)
        && set.nodes.contains(&node_right)
        && !set.nodes.contains(&node_up_right)
    {
        bottom_left_tiles.push(SpriteBundle {
            texture: tex_set_tile_bottom_left.clone(),
            sprite: set_border_sprite(),
            transform: Transform::from_xyz(node_x + SPRITE_SPACING, node_y + SPRITE_SPACING, 0.0),
            ..default()
        });
    }

    bottom_left_tiles
}

/// Returns a vector of sprite bundles for top right set tiles using one node
/// as the focal point. It is responsible for generating appropriate set tiles to
/// the immediate up right and down left of the node.
///
/// # Parameters
///
/// - `node`: The node around which set tiles are being added.
/// - `node_x`: The node's X coordinate in the scene.
/// - `node_y`: The node's Y coordinate in the scene.
/// - `set`: The game set for which tiles are being added.
/// - `puzzle`: The puzzle for which the tiles are being added.
/// - `asset_server`: The asset server used to load textures.
///
/// # Returns
///
/// A vector of Sprite Bundles.
fn get_set_tiles_top_right(
    node: &u16,
    node_x: f32,
    node_y: f32,
    set: &GameSet,
    puzzle: &Puzzle,
    asset_server: AssetServer,
) -> Vec<SpriteBundle> {
    let mut top_right_tiles = Vec::new();
    let tex_set_tile_top_right = asset_server.load(if set.bounded {
        Texture::SetTileBoundedTopRight.path()
    } else {
        Texture::SetTileTopRight.path()
    });

    let node_up = get_node_up(node, puzzle).unwrap_or(u16::MAX);
    let node_right = get_node_right(node, puzzle).unwrap_or(u16::MAX);
    if !set.nodes.contains(&node_up) && !set.nodes.contains(&node_right) {
        top_right_tiles.push(SpriteBundle {
            texture: tex_set_tile_top_right.clone(),
            sprite: set_border_sprite(),
            transform: Transform::from_xyz(node_x + SPRITE_SPACING, node_y + SPRITE_SPACING, 0.0),
            ..default()
        });
    }

    let node_down_left = get_node_down_left(node, &puzzle).unwrap_or(u16::MAX);
    let node_down = get_node_down(node, puzzle).unwrap_or(u16::MAX);
    let node_left = get_node_left(node, puzzle).unwrap_or(u16::MAX);
    if set.nodes.contains(&node_down)
        && set.nodes.contains(&node_left)
        && !set.nodes.contains(&node_down_left)
    {
        top_right_tiles.push(SpriteBundle {
            texture: tex_set_tile_top_right.clone(),
            sprite: set_border_sprite(),
            transform: Transform::from_xyz(node_x - SPRITE_SPACING, node_y - SPRITE_SPACING, 0.0),
            ..default()
        });
    }

    top_right_tiles
}

/// Returns a vector of sprite bundles for top left set tiles using one node
/// as the focal point. It is responsible for generating appropriate set tiles to
/// the immediate up left and down right of the node.
///
/// # Parameters
///
/// - `node`: The node around which set tiles are being added.
/// - `node_x`: The node's X coordinate in the scene.
/// - `node_y`: The node's Y coordinate in the scene.
/// - `set`: The game set for which tiles are being added.
/// - `puzzle`: The puzzle for which the tiles are being added.
/// - `asset_server`: The asset server used to load textures.
///
/// # Returns
///
/// A vector of Sprite Bundles.
fn get_set_tiles_top_left(
    node: &u16,
    node_x: f32,
    node_y: f32,
    set: &GameSet,
    puzzle: &Puzzle,
    asset_server: AssetServer,
) -> Vec<SpriteBundle> {
    let mut top_left_tiles = Vec::new();
    let tex_set_tile_top_left = asset_server.load(if set.bounded {
        Texture::SetTileBoundedTopLeft.path()
    } else {
        Texture::SetTileTopLeft.path()
    });

    let node_up = get_node_up(node, puzzle).unwrap_or(u16::MAX);
    let node_left = get_node_left(node, puzzle).unwrap_or(u16::MAX);
    if !set.nodes.contains(&node_up) && !set.nodes.contains(&node_left) {
        top_left_tiles.push(SpriteBundle {
            texture: tex_set_tile_top_left.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(TILE_NODE_SPRITE_SIZE, TILE_NODE_SPRITE_SIZE)),
                color: COLOR_SET_BORDER,
                ..Default::default()
            },
            transform: Transform::from_xyz(node_x - SPRITE_SPACING, node_y + SPRITE_SPACING, 0.0),
            ..default()
        });
    }

    let node_down_right = get_node_down_right(node, &puzzle).unwrap_or(u16::MAX);
    let node_down = get_node_down(node, puzzle).unwrap_or(u16::MAX);
    let node_right = get_node_right(node, puzzle).unwrap_or(u16::MAX);
    if set.nodes.contains(&node_down)
        && set.nodes.contains(&node_right)
        && !set.nodes.contains(&node_down_right)
    {
        top_left_tiles.push(SpriteBundle {
            texture: tex_set_tile_top_left.clone(),
            sprite: set_border_sprite(),
            transform: Transform::from_xyz(node_x + SPRITE_SPACING, node_y - SPRITE_SPACING, 0.0),
            ..default()
        });
    }

    top_left_tiles
}

/// Returns the appropriate texture for a set tile behind a given node. The reason for this is that overlapping
/// sets need to be distinguished by interlaced colors with sprites that match that offset.
///
/// # Parameters
///
/// `set`: The game set to which the texture belongs.
/// `node_id`: The id of the game node which the texture will live behind.
/// `sets`: The other overlapping game sets at this node.
///
/// # Returns
///
/// A Texture for the set tile.
fn get_texture_for_set_tile_at_node(
    set: GameSet,
    node_id: GameNodeId,
    sets: Vec<GameSet>,
) -> Texture {
    let sets_containing_node: Vec<GameSet> = get_sets_containing_node(sets, node_id);
    let set_order = get_set_order(set, sets_containing_node.clone());
    if sets_containing_node.len() == 1 {
        Texture::BgSetOne
    } else if sets_containing_node.len() == 2 {
        match set_order {
            0 => Texture::BgSetTwoA,
            1 => Texture::BgSetTwoB,
            _ => Texture::Missing,
        }
    } else if sets_containing_node.len() == 3 {
        match set_order {
            0 => Texture::BgSetThreeA,
            1 => Texture::BgSetThreeB,
            2 => Texture::BgSetThreeC,
            _ => Texture::Missing,
        }
    } else {
        error!("Error getting texture for set tile at node, need to support more set overlaps.");
        Texture::Missing
    }
}

/// Returns the appropriate color for a set tile. The color is determined in order by its ID.
///
/// # Parameters
///
/// - `set`: The game set to which the returned color belongs.
/// - `sets`: All sets in the puzzle.
///
/// # Returns
///
/// The Color for the set tiles.
pub fn get_color_for_set_tile(set: GameSet, sets: Vec<GameSet>) -> Color {
    let set_order = get_set_order(set, sets);
    match set_order {
        0 => COLOR_SET_0,
        1 => COLOR_SET_1,
        2 => COLOR_SET_2,
        _ => {
            error!("Error getting color for set tile, need to add more colors.");
            Color::BLACK
        }
    }
}

/// Returns the colored background tiles for a set.
///
/// # Parameters
///
/// - `set`: The game set for the tiles.
/// - `puzzle`: The puzzle in which the set lives.
/// - `asset_server`: The asset server used to load textures.
///
/// # Returns
///
/// A vector of SpriteBundles.
fn get_set_bg_tiles(
    set: &GameSet,
    puzzle: &Puzzle,
    asset_server: AssetServer,
) -> Vec<SpriteBundle> {
    let mut bg_tiles: Vec<SpriteBundle> = Vec::new();
    let sets = &puzzle.sets;
    set.nodes.iter().for_each(|node| {
        let (node_x, node_y) = node_to_position(node, puzzle);
        let tex = get_texture_for_set_tile_at_node(set.clone(), *node, sets.to_vec());
        let tex_path = tex.path().to_string();
        bg_tiles.push(SpriteBundle {
            texture: asset_server.load(tex_path),
            sprite: Sprite {
                custom_size: Some(Vec2::new(BG_SET_SPRITE_SIZE, BG_SET_SPRITE_SIZE)),
                color: get_color_for_set_tile(set.clone(), sets.to_vec()),
                ..Default::default()
            },
            transform: Transform::from_xyz(node_x, node_y, Z_SET_FILL),
            ..Default::default()
        })
    });
    bg_tiles
}

/// Returns a vector of sprite bundles for all of the tiles for a set.
/// This includes both the backgrounds and the edges of each set.
///
/// # Parameters
///
/// - `set`: The game set for which tiles are being added.
/// - `puzzle`: The puzzle for which the tiles are being added.
/// - `asset_server`: The asset server used to load textures.
///
/// # Returns
///
/// A vector of Sprite Bundles.
pub fn get_set_tiles(
    set: &GameSet,
    puzzle: &Puzzle,
    asset_server: AssetServer,
) -> Vec<SpriteBundle> {
    let mut tiles = Vec::new();

    for node in set.nodes.iter() {
        let (node_x, node_y) = node_to_position(node, &puzzle);
        tiles.append(&mut get_set_tiles_vertical(
            node,
            node_x,
            node_y,
            set,
            puzzle,
            asset_server.clone(),
        ));
        tiles.append(&mut get_set_tiles_horizontal(
            node,
            node_x,
            node_y,
            set,
            puzzle,
            asset_server.clone(),
        ));
        tiles.append(&mut get_set_tiles_bottom_right(
            node,
            node_x,
            node_y,
            set,
            puzzle,
            asset_server.clone(),
        ));
        tiles.append(&mut get_set_tiles_bottom_left(
            node,
            node_x,
            node_y,
            set,
            puzzle,
            asset_server.clone(),
        ));
        tiles.append(&mut get_set_tiles_top_right(
            node,
            node_x,
            node_y,
            set,
            puzzle,
            asset_server.clone(),
        ));
        tiles.append(&mut get_set_tiles_top_left(
            node,
            node_x,
            node_y,
            set,
            puzzle,
            asset_server.clone(),
        ));
    }

    tiles.append(&mut get_set_bg_tiles(set, puzzle, asset_server));

    tiles
}

/// Removes a line connecting start_node and end_node from the current puzzle. Also updates
/// the game state to visually remove the line.
pub fn remove_line(
    commands: &mut Commands,
    start_node: &mut ActiveNode,
    end_node: &mut ActiveNode,
    active_lines: &mut Vec<ActiveLine>,
) {
    // Remove connection from start to end
    if let Some(pos) = start_node
        .connections
        .iter()
        .position(|node_id| *node_id == end_node.node.id)
    {
        start_node.connections.remove(pos);
    }

    // Remove connection from end to start
    if let Some(pos) = end_node
        .connections
        .iter()
        .position(|node_id| *node_id == start_node.node.id)
    {
        end_node.connections.remove(pos);
    }

    // Remove line from scene
    let first_node = if start_node.node.id < end_node.node.id {
        start_node.clone()
    } else {
        end_node.clone()
    };
    let second_node = if start_node.node.id < end_node.node.id {
        end_node.clone()
    } else {
        start_node.clone()
    };
    for idx in 0..active_lines.len() {
        if active_lines[idx].start_node.node.id == first_node.node.id
            && active_lines[idx].end_node.node.id == second_node.node.id
        {
            commands
                .entity(active_lines[idx].sprite_entity_id)
                .despawn();
            active_lines.remove(idx);
            break;
        }
    }
}

pub fn get_mut_start_end_nodes(
    active_nodes: &mut Vec<ActiveNode>,
    i: usize,
    j: usize,
) -> (&mut ActiveNode, &mut ActiveNode) {
    let (first, second) = if i < j {
        let (left, right) = active_nodes.split_at_mut(j);
        (&mut left[i], &mut right[0])
    } else {
        let (left, right) = active_nodes.split_at_mut(i);
        (&mut right[0], &mut left[j])
    };
    (first, second)
}

/// Adds a line connecting start_node and end_node to the current puzzle. Also updates
/// the game state to visually add the line.
pub fn add_line(
    commands: &mut Commands,
    asset_server: AssetServer,
    start_node: &mut ActiveNode,
    end_node: &mut ActiveNode,
    active_lines: &mut Vec<ActiveLine>,
) {
    let start_pos = start_node.sprite.transform.translation.truncate();
    let end_pos = end_node.sprite.transform.translation.truncate();

    // Get the appropriate line texture, if exists (otherwise invalid node pair)
    let line_texture = get_line_texture(start_node, end_node).unwrap_or(&Texture::Missing);

    if *line_texture == Texture::Missing {
        return;
    }

    let line_sprite = SpriteBundle {
        texture: asset_server.load(line_texture.path()),
        sprite: Sprite {
            custom_size: Some(Vec2::new(TILE_NODE_SPRITE_SIZE, TILE_NODE_SPRITE_SIZE)),
            ..Default::default()
        },
        transform: Transform::from_xyz(
            (end_pos.x + start_pos.x) / 2.0,
            (end_pos.y + start_pos.y) / 2.0,
            Z_LINE,
        ),
        ..Default::default()
    };

    // Update connections of both start and end node
    start_node.connections.push(end_node.node.id);
    end_node.connections.push(start_node.node.id);

    // Add line to the screen
    let line_entity_id = commands
        .spawn(line_sprite.clone())
        .insert(OnPuzzleScene)
        .id();

    // Update list of lines, putting the smallest ID first.
    let first_node = if start_node.node.id < end_node.node.id {
        start_node.clone()
    } else {
        end_node.clone()
    };
    let second_node = if start_node.node.id < end_node.node.id {
        end_node.clone()
    } else {
        start_node.clone()
    };
    active_lines.push(ActiveLine {
        start_node: first_node.clone(),
        end_node: second_node.clone(),
        sprite: line_sprite.clone(),
        active_id: ActiveIdentifier::new(),
        sprite_entity_id: line_entity_id,
    });
}

/// Clear all lines from a puzzle. 
pub fn clear_all_lines(
    commands: &mut Commands,
    active_nodes: &mut Vec<ActiveNode>,
    active_lines: &mut Vec<ActiveLine>,
) {
    active_nodes.iter_mut().for_each(|node| {
        node.connections.clear();
    });
    active_lines.iter_mut().for_each(|active_line| {
        commands.entity(active_line.sprite_entity_id).despawn();
    });
    active_lines.clear();
}

/// Unload all active elements from a puzzle.
pub fn unload_active_elements(
    commands: &mut Commands,
    active_nodes: &mut Vec<ActiveNode>,
    active_lines: &mut Vec<ActiveLine>,
) {
    active_nodes.iter_mut().for_each(|active_node| {
        commands.entity(active_node.sprite_entity_id).despawn();
    });
    active_nodes.clear();

    active_lines.iter_mut().for_each(|active_line| {
        commands.entity(active_line.sprite_entity_id).despawn();
    });
    active_lines.clear();
}