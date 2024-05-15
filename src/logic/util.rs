use std::{
    collections::{HashMap, VecDeque},
    f32::consts::PI,
};

use bevy::{
    asset::Handle, prelude::*, render::texture::Image, sprite::SpriteBundle, utils::HashSet,
};

use crate::{
    objects::{
        active::{
            active_identifier::active_identifier::ActiveIdentifier,
            active_node::active_node::ActiveNode, active_set::active_set::ActiveSet,
        },
        immutable::{
            game_set::game_set::GameSet, puzzle::puzzle::Puzzle,
            solution::solution::active_nodes_to_solution,
        },
    },
    texture::Texture,
    SatisfiedStatesMap, SPRITE_SPACING, TILE_NODE_SPRITE_SIZE,
};

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
pub fn get_bg_tile(x: u8, y: u8, width: u8, height: u8, asset_server: AssetServer) -> SpriteBundle {
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

    let transform = Transform::from_xyz(x as f32 * SPRITE_SPACING, y as f32 * SPRITE_SPACING, -1.0);
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

pub fn is_left_edge(node: &u16, puzzle: &Puzzle) -> bool {
    *node < puzzle.height as u16
}

pub fn is_top_edge(node: &u16, puzzle: &Puzzle) -> bool {
    (*node + 1) % puzzle.height as u16 == 0
}

pub fn is_right_edge(node: &u16, puzzle: &Puzzle) -> bool {
    *node + puzzle.height as u16 >= puzzle.width as u16 * puzzle.height as u16
}

pub fn is_bottom_edge(node: &u16, puzzle: &Puzzle) -> bool {
    *node % puzzle.height as u16 == 0
}

pub fn get_node_left(node: &u16, puzzle: &Puzzle) -> Option<u16> {
    if !is_left_edge(node, puzzle) {
        Some(*node - puzzle.height as u16)
    } else {
        None
    }
}

pub fn get_node_up_left(node: &u16, puzzle: &Puzzle) -> Option<u16> {
    if !is_left_edge(node, puzzle) && !is_top_edge(node, puzzle) {
        Some(*node - puzzle.height as u16 + 1)
    } else {
        None
    }
}

pub fn get_node_up(node: &u16, puzzle: &Puzzle) -> Option<u16> {
    if !is_top_edge(node, puzzle) {
        Some(*node + 1)
    } else {
        None
    }
}

pub fn get_node_up_right(node: &u16, puzzle: &Puzzle) -> Option<u16> {
    if !is_top_edge(node, puzzle) && !is_right_edge(node, puzzle) {
        Some(*node + puzzle.height as u16 + 1)
    } else {
        None
    }
}

pub fn get_node_right(node: &u16, puzzle: &Puzzle) -> Option<u16> {
    if !is_right_edge(node, puzzle) {
        Some(*node + puzzle.height as u16)
    } else {
        None
    }
}

pub fn get_node_down_right(node: &u16, puzzle: &Puzzle) -> Option<u16> {
    if !is_right_edge(node, puzzle) && !is_bottom_edge(node, puzzle) {
        Some(*node + puzzle.height as u16 - 1)
    } else {
        None
    }
}

pub fn get_node_down(node: &u16, puzzle: &Puzzle) -> Option<u16> {
    if !is_bottom_edge(node, puzzle) {
        Some(*node - 1)
    } else {
        None
    }
}

pub fn get_node_down_left(node: &u16, puzzle: &Puzzle) -> Option<u16> {
    if !is_bottom_edge(node, puzzle) && !is_left_edge(node, puzzle) {
        Some(*node - puzzle.height as u16 - 1)
    } else {
        None
    }
}

pub fn get_adjacent_nodes(node: &u16, puzzle: &Puzzle) -> Vec<u16> {
    let mut adjacent = Vec::new();
    let height = puzzle.height as u16;
    let node = *node;

    let is_left_edge = node >= height;
    let is_top_edge = node + 1 % height == 0;
    let is_right_edge = node + height >= puzzle.width as u16 * height;
    let is_bottom_edge = node % height == 0;

    if !is_left_edge {
        adjacent.push(node - height);
    }
    if !is_left_edge && !is_top_edge {
        adjacent.push(node - height + 1);
    }
    if !is_top_edge {
        adjacent.push(node + 1);
    }
    if !is_top_edge && !is_right_edge {
        adjacent.push(node + height + 1);
    }
    if !is_right_edge {
        adjacent.push(node + height);
    }
    if !is_right_edge && !is_bottom_edge {
        adjacent.push(node + height - 1);
    }
    if !is_bottom_edge {
        adjacent.push(node - 1);
    }
    if !is_bottom_edge && !is_left_edge {
        adjacent.push(node - height - 1);
    }

    adjacent
}

pub fn node_to_position(node: &u16, puzzle: &Puzzle) -> (f32, f32) {
    let x = (node / puzzle.height as u16) as f32 * SPRITE_SPACING * 2. + SPRITE_SPACING;
    let y = (node % puzzle.height as u16) as f32 * SPRITE_SPACING * 2. + SPRITE_SPACING;

    (x, y)
}

fn get_set_tiles_vertical(
    node: &u16,
    node_x: f32,
    node_y: f32,
    set: &GameSet,
    puzzle: &Puzzle,
    asset_server: AssetServer,
) -> Vec<SpriteBundle> {
    let mut vertical_tiles = Vec::new();
    let tex_set_tile_vertical = asset_server.load(Texture::SetTileVertical.path());

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

        // let node_down = get_node_down(node, puzzle).unwrap_or(u16::MAX);
        // if !is_bottom_edge(node, puzzle) && !set.nodes.contains(&node_down) {
        //     // Below left
        //     let node_down_left = get_node_down_left(node, puzzle).unwrap_or(u16::MAX);
        //     if !set.nodes.contains(&node_down_left) {
        //         vertical_tiles.push(SpriteBundle {
        //             texture: tex_set_tile_vertical.clone(),
        //             sprite: Sprite {
        //                 custom_size: Some(Vec2::new(
        //                     TILE_NODE_SPRITE_SIZE,
        //                     TILE_NODE_SPRITE_SIZE,
        //                 )),
        //                 color: Color::GREEN,
        //                 ..Default::default()
        //             },
        //             transform: Transform::from_xyz(node_x - SPRITE_SPACING, node_y - SPRITE_SPACING, 0.0),
        //             ..default()
        //         });
        //     }

        //     // Below right
        //     let node_down_right = get_node_down_right(node, puzzle).unwrap_or(u16::MAX);
        //     if !set.nodes.contains(&node_down_right) {
        //         vertical_tiles.push(SpriteBundle {
        //             texture: tex_set_tile_vertical.clone(),
        //             sprite: Sprite {
        //                 custom_size: Some(Vec2::new(
        //                     TILE_NODE_SPRITE_SIZE,
        //                     TILE_NODE_SPRITE_SIZE,
        //                 )),
        //                 color: Color::GREEN,
        //                 ..Default::default()
        //             },
        //             transform: Transform::from_xyz(node_x + SPRITE_SPACING, node_y - SPRITE_SPACING, 0.0),
        //             ..default()
        //         });
        //     }
        // }
    }

    vertical_tiles
}

fn get_set_tiles_horizontal(
    node: &u16,
    node_x: f32,
    node_y: f32,
    set: &GameSet,
    puzzle: &Puzzle,
    asset_server: AssetServer,
) -> Vec<SpriteBundle> {
    let mut horizontal_tiles = Vec::new();
    let tex_set_tile_horizontal = asset_server.load(Texture::SetTileHorizontal.path());

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

    // let node_left = get_node_left(node, puzzle).unwrap_or(u16::MAX);
    // if !is_left_edge(node, &puzzle) && !set.nodes.contains(&node_left) {
    //     // Above left
    //     let node_up_left = get_node_up_left(node, puzzle).unwrap_or(u16::MAX);
    //     if !set.nodes.contains(&node_up_left) {
    //         horizontal_tiles.push(SpriteBundle {
    //             texture: tex_set_tile_horizontal.clone(),
    //             sprite: Sprite {
    //                 custom_size: Some(Vec2::new(
    //                     TILE_NODE_SPRITE_SIZE,
    //                     TILE_NODE_SPRITE_SIZE,
    //                 )),
    //                 color: Color::GREEN,
    //                 ..Default::default()
    //             },
    //             transform: Transform::from_xyz(node_x - SPRITE_SPACING, node_y + SPRITE_SPACING, 0.0),
    //             ..default()
    //         });
    //     }

    //     // Below left
    //     let node_down_left = get_node_down_left(node, puzzle).unwrap_or(u16::MAX);
    //     if !set.nodes.contains(&node_down_left) {
    //         horizontal_tiles.push(SpriteBundle {
    //             texture: tex_set_tile_horizontal.clone(),
    //             sprite: Sprite {
    //                 custom_size: Some(Vec2::new(
    //                     TILE_NODE_SPRITE_SIZE,
    //                     TILE_NODE_SPRITE_SIZE,
    //                 )),
    //                 color: Color::GREEN,
    //                 ..Default::default()
    //             },
    //             transform: Transform::from_xyz(node_x - SPRITE_SPACING, node_y - SPRITE_SPACING, 0.0),
    //             ..default()
    //         });
    //     }
    // }

    horizontal_tiles
}

fn get_set_tiles_bottom_right(
    node: &u16,
    node_x: f32,
    node_y: f32,
    set: &GameSet,
    puzzle: &Puzzle,
    asset_server: AssetServer,
) -> Vec<SpriteBundle> {
    let mut bottom_right_tiles = Vec::new();
    let tex_set_tile_bottom_right = asset_server.load(Texture::SetTileBottomRight.path());

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

fn get_set_tiles_bottom_left(
    node: &u16,
    node_x: f32,
    node_y: f32,
    set: &GameSet,
    puzzle: &Puzzle,
    asset_server: AssetServer,
) -> Vec<SpriteBundle> {
    let mut bottom_left_tiles = Vec::new();
    let tex_set_tile_bottom_left = asset_server.load(Texture::SetTileBottomLeft.path());

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

fn get_set_tiles_top_right(
    node: &u16,
    node_x: f32,
    node_y: f32,
    set: &GameSet,
    puzzle: &Puzzle,
    asset_server: AssetServer,
) -> Vec<SpriteBundle> {
    let mut top_right_tiles = Vec::new();
    let tex_set_tile_top_right = asset_server.load(Texture::SetTileTopRight.path());

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

fn get_set_tiles_top_left(
    node: &u16,
    node_x: f32,
    node_y: f32,
    set: &GameSet,
    puzzle: &Puzzle,
    asset_server: AssetServer,
) -> Vec<SpriteBundle> {
    let mut top_left_tiles = Vec::new();
    let tex_set_tile_top_left = asset_server.load(Texture::SetTileTopLeft.path());

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

    tiles
}

pub fn get_set_upper_left_node(set: &GameSet, puzzle: &Puzzle) -> u16 {
    let mut upper_left_most_node = u16::MAX;
    let mut upper_most_row = u8::MIN;
    set.nodes.iter().for_each(|node| {
        if node % puzzle.height as u16 > upper_most_row.into() && *node < upper_left_most_node {
            upper_most_row = (node % puzzle.height as u16) as u8;
            upper_left_most_node = *node;
        }
    });
    upper_left_most_node
}

pub fn clicked_on_sprite(sprite: &SpriteBundle, cursor: Vec2) -> bool {
    let node_pos = sprite.transform.translation.truncate();
    let distance = cursor.distance(node_pos);
    // Assuming the sprite size is a good proxy for click detection radius
    if distance < sprite.sprite.custom_size.unwrap_or(Vec2::MIN).x / 2.0 {
        return true;
    }
    false
}

pub fn get_cursor_world_position(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Vec2 {
    return window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
        .unwrap_or(Vec2::MIN);
}

/// Checks if the puzzle is solved. Verifies a few things:
/// - Is the connectivity of each node class spanning all nodes in the puzzle of that class?
/// - Is each node's set of conditions satisfied?  
/// - Is each set rule satisfied?
pub fn check_answer(active_nodes: Vec<&ActiveNode>, active_sets: Vec<&ActiveSet>) -> bool {
    // First verify that all nodes are connected to each other
    let mut visited: HashSet<u16> = HashSet::new();
    let mut queue: VecDeque<u16> = VecDeque::new();

    queue.push_back(active_nodes.get(0).unwrap().node.id);
    visited.insert(active_nodes.get(0).unwrap().node.id);

    while queue.len() > 0 {
        let curr_node_id = queue.pop_front().unwrap();
        let curr_node = active_nodes
            .iter()
            .find(|node| node.node.id == curr_node_id)
            .unwrap();
        for connection in curr_node.connections.iter() {
            if !visited.contains(connection) {
                visited.insert(*connection);
                queue.push_back(*connection);
            }
        }
    }

    // Check that all nodes in the class are connected to each other
    for node in active_nodes {
        if !visited
            .iter()
            .any(|visited_node| *visited_node == node.node.id)
        {
            println!("Node {} not connected to any other node", node.node.id);
            return false;
        }
    }

    let mut succeeds = true;
    // active_nodes.clone().into_iter().for_each(|node| {
    //     // Check failed node conditions
    //     let failed_conditions = node.get_failed_conditions(active_nodes.clone());
    //     if !failed_conditions.is_empty() {
    //         println!("Node failed conditions: {:?}", failed_conditions);
    //         succeeds = false;
    //     }
    // });

    // TODO check all sets for failed rules

    return succeeds;
}

/// Returns SatisfiedStatesMap containing all nodes, conditions, and set rules.
pub fn get_all_satisfied_states(
    active_nodes: &Vec<ActiveNode>,
    active_sets: &Vec<ActiveSet>,
) -> SatisfiedStatesMap {
    let mut satisfied_states: SatisfiedStatesMap = SatisfiedStatesMap::new();
    let solution = active_nodes_to_solution(&active_nodes);

    for node in active_nodes.iter() {
        satisfied_states.insert(node.active_id, node.check_satisfied());
        for condition in node.active_conditions.iter() {
            satisfied_states.insert(
                condition.active_id,
                condition.check_satisfied(&node, &solution),
            );
        }
        // TODO track which ones have been checked to not duplicate, this is reflexive
        for connected_condition in node.active_connected_conditions.iter() {
            satisfied_states.insert(
                connected_condition.active_id,
                connected_condition.check_satisfied(),
            );
        }
    }

    for set in active_sets.iter() {
        for rule in set.active_set_rules.iter() {
            satisfied_states.insert(rule.active_id, rule.check_satisfied());
        }
        // TODO track which ones have been checked to not duplicate, this is reflexive
        for connected_rule in set.active_connected_set_rules.iter() {
            satisfied_states.insert(connected_rule.active_id, connected_rule.check_satisfied());
        }
    }

    satisfied_states
}

/// Returns SatisfiedStatesMap containing relevant nodes, conditions, and set rules.
/// Uses the start and end nodes as a heuristic to avoid visiting all nodes to update their satisfied state.
pub fn get_filtered_satisfied_states(
    active_nodes: &Vec<ActiveNode>,
    active_sets: &Vec<ActiveSet>,
    start_node: &ActiveNode,
    end_node: &ActiveNode,
) -> SatisfiedStatesMap {
    // Getting networks starting from specific nodes
    let mut network_start_node = get_active_nodes_in_network(start_node, &active_nodes);

    // If end_node not in network, extend it
    if !network_start_node.contains(&end_node) {
        network_start_node.extend(get_active_nodes_in_network(end_node, &active_nodes));
    }

    let mut satisfied_states: SatisfiedStatesMap = SatisfiedStatesMap::new();
    let solution = active_nodes_to_solution(&network_start_node);

    for node in network_start_node.clone().into_iter() {
        satisfied_states.insert(node.active_id, node.check_satisfied());
        for condition in node.active_conditions.iter() {
            satisfied_states.insert(
                condition.active_id,
                condition.check_satisfied(&node, &solution),
            );
        }
        // TODO track which ones have been checked to not duplicate, this is reflexive
        for connected_condition in node.active_connected_conditions.iter() {
            satisfied_states.insert(
                connected_condition.active_id,
                connected_condition.check_satisfied(),
            );
        }
    }

    let network_sets = get_sets_in_network(active_sets, &network_start_node);

    for set in network_sets.into_iter() {
        for rule in set.active_set_rules.iter() {
            satisfied_states.insert(rule.active_id, rule.check_satisfied());
        }
        // TODO track which ones have been checked to not duplicate, this is reflexive
        for connected_rule in set.active_connected_set_rules.iter() {
            satisfied_states.insert(connected_rule.active_id, connected_rule.check_satisfied());
        }
    }

    satisfied_states

    // TODO for each set which encompasses at least one node in the network, check its satisfied states
}

fn get_sets_in_network<'a>(
    active_sets: &Vec<ActiveSet>,
    network: &Vec<ActiveNode>,
) -> Vec<ActiveSet> {
    let mut network_sets: Vec<ActiveSet> = Vec::new();

    // Convert network to list of node ids
    let network_ids: Vec<u16> = network.iter().map(|node| node.node.id).collect();

    for set in active_sets.iter() {
        if set.set.nodes.iter().any(|node| network_ids.contains(node)) {
            network_sets.push(set.clone());
        }
    }

    network_sets
}

fn get_active_node_from_id(id: u16, active_nodes: Vec<ActiveNode>) -> ActiveNode {
    active_nodes
        .iter()
        .find(|node| node.node.id == id)
        .unwrap()
        .clone()
}

fn get_active_nodes_in_network<'a>(
    start_node: &ActiveNode,
    active_nodes: &Vec<ActiveNode>,
) -> Vec<ActiveNode> {
    // Traverse the active_nodes from start_node and add them to network as discovered through connections.
    let mut visited: HashSet<u16> = HashSet::new();
    let mut queue: VecDeque<u16> = VecDeque::new();
    let mut network: Vec<ActiveNode> = Vec::new();

    queue.push_back(start_node.node.id);
    visited.insert(start_node.node.id);
    network.push(get_active_node_from_id(
        start_node.node.id,
        active_nodes.to_vec(),
    ));

    while queue.len() > 0 {
        let curr_node_id = queue.pop_front().unwrap();
        let curr_node = active_nodes
            .iter()
            .find(|node| node.node.id == curr_node_id)
            .unwrap();
        for connection in curr_node.connections.iter() {
            if !visited.contains(connection) {
                visited.insert(*connection);
                queue.push_back(*connection);
                network.push(get_active_node_from_id(*connection, active_nodes.to_vec()));
            }
        }
    }

    network
}
