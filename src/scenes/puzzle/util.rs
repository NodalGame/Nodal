use std::f32::consts::PI;

use bevy::{asset::{AssetServer, Handle}, math::Vec2, prelude::default, render::{color::Color, texture::Image}, sprite::{Sprite, SpriteBundle}, transform::components::Transform};

use crate::{get_node_down, get_node_down_left, get_node_down_right, get_node_left, get_node_right, get_node_up, get_node_up_left, get_node_up_right, is_bottom_edge, is_left_edge, is_right_edge, is_top_edge, node_to_position, structs::{active::active_node::active_node::ActiveNode, immutable::{game_set::game_set::GameSet, puzzle::puzzle::Puzzle}}, texture::Texture, BG_SET_SPRITE_SIZE, COLOR_SET_0, COLOR_SET_1, COLOR_SET_2, SPRITE_SPACING, TILE_NODE_SPRITE_SIZE, Z_BACKGROUND, Z_SET_FILL};

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
pub fn get_puzzle_background_tile(x: u8, y: u8, width: u8, height: u8, asset_server: AssetServer) -> SpriteBundle {
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

    let transform = Transform::from_xyz(x as f32 * SPRITE_SPACING, y as f32 * SPRITE_SPACING, Z_BACKGROUND);
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
            sprite: Sprite {
                custom_size: Some(Vec2::new(TILE_NODE_SPRITE_SIZE, TILE_NODE_SPRITE_SIZE)),
                color: Color::GREEN,
                ..Default::default()
            },
            transform: Transform::from_xyz(node_x - SPRITE_SPACING, node_y, 0.0),
            ..default()
        })
    }

    let node_right = get_node_right(node, puzzle).unwrap_or(u16::MAX);
    if is_right_edge(node, puzzle) || !set.nodes.contains(&node_right) {
        vertical_tiles.push(SpriteBundle {
            texture: tex_set_tile_vertical.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(TILE_NODE_SPRITE_SIZE, TILE_NODE_SPRITE_SIZE)),
                color: Color::GREEN,
                ..Default::default()
            },
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
                sprite: Sprite {
                    custom_size: Some(Vec2::new(TILE_NODE_SPRITE_SIZE, TILE_NODE_SPRITE_SIZE)),
                    color: Color::GREEN,
                    ..Default::default()
                },
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
                sprite: Sprite {
                    custom_size: Some(Vec2::new(TILE_NODE_SPRITE_SIZE, TILE_NODE_SPRITE_SIZE)),
                    color: Color::GREEN,
                    ..Default::default()
                },
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
            sprite: Sprite {
                custom_size: Some(Vec2::new(TILE_NODE_SPRITE_SIZE, TILE_NODE_SPRITE_SIZE)),
                color: Color::GREEN,
                ..Default::default()
            },
            transform: Transform::from_xyz(node_x, node_y + SPRITE_SPACING, 0.0),
            ..default()
        });
    }

    // Directly below
    let node_down = get_node_down(node, puzzle).unwrap_or(u16::MAX);
    if is_bottom_edge(node, &puzzle) || !set.nodes.contains(&node_down) {
        horizontal_tiles.push(SpriteBundle {
            texture: tex_set_tile_horizontal.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(TILE_NODE_SPRITE_SIZE, TILE_NODE_SPRITE_SIZE)),
                color: Color::GREEN,
                ..Default::default()
            },
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
                sprite: Sprite {
                    custom_size: Some(Vec2::new(TILE_NODE_SPRITE_SIZE, TILE_NODE_SPRITE_SIZE)),
                    color: Color::GREEN,
                    ..Default::default()
                },
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
                sprite: Sprite {
                    custom_size: Some(Vec2::new(TILE_NODE_SPRITE_SIZE, TILE_NODE_SPRITE_SIZE)),
                    color: Color::GREEN,
                    ..Default::default()
                },
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
            sprite: Sprite {
                custom_size: Some(Vec2::new(TILE_NODE_SPRITE_SIZE, TILE_NODE_SPRITE_SIZE)),
                color: Color::GREEN,
                ..Default::default()
            },
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
            sprite: Sprite {
                custom_size: Some(Vec2::new(TILE_NODE_SPRITE_SIZE, TILE_NODE_SPRITE_SIZE)),
                color: Color::GREEN,
                ..Default::default()
            },
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
            sprite: Sprite {
                custom_size: Some(Vec2::new(TILE_NODE_SPRITE_SIZE, TILE_NODE_SPRITE_SIZE)),
                color: Color::GREEN,
                ..Default::default()
            },
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
            sprite: Sprite {
                custom_size: Some(Vec2::new(TILE_NODE_SPRITE_SIZE, TILE_NODE_SPRITE_SIZE)),
                color: Color::GREEN,
                ..Default::default()
            },
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
            sprite: Sprite {
                custom_size: Some(Vec2::new(TILE_NODE_SPRITE_SIZE, TILE_NODE_SPRITE_SIZE)),
                color: Color::GREEN,
                ..Default::default()
            },
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
            sprite: Sprite {
                custom_size: Some(Vec2::new(TILE_NODE_SPRITE_SIZE, TILE_NODE_SPRITE_SIZE)),
                color: Color::GREEN,
                ..Default::default()
            },
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
                color: Color::GREEN,
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
            sprite: Sprite {
                custom_size: Some(Vec2::new(TILE_NODE_SPRITE_SIZE, TILE_NODE_SPRITE_SIZE)),
                color: Color::GREEN,
                ..Default::default()
            },
            transform: Transform::from_xyz(node_x + SPRITE_SPACING, node_y - SPRITE_SPACING, 0.0),
            ..default()
        });
    }

    top_left_tiles
}

fn get_set_bg_tiles(
    set: &GameSet,
    puzzle: &Puzzle,
    asset_server: AssetServer,
) -> Vec<SpriteBundle> {
    // TODO count number of overlapping sets to determine if background tiles need different color/orientation

    let mut bg_tiles: Vec<SpriteBundle> = Vec::new();
    let tex = match set.id {
        0 => asset_server.load(Texture::BgSet0.path()),
        1 => asset_server.load(Texture::BgSet1.path()),
        2 => asset_server.load(Texture::BgSet2.path()),
        _ => asset_server.load(Texture::BgSet3.path()),
    };
    set.nodes.iter().for_each(|node| {
        let (node_x, node_y) = node_to_position(node, puzzle);
        bg_tiles.push(SpriteBundle {
            texture: tex.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(BG_SET_SPRITE_SIZE, BG_SET_SPRITE_SIZE)),
                color: match set.id {
                    0 => COLOR_SET_0,
                    1 => COLOR_SET_1,
                    2 => COLOR_SET_2,
                    _ => Color::BLACK // TODO error
                },
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