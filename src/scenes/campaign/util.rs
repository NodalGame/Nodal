use bevy::{
    a11y::accesskit::Vec2, math::Vec3, prelude::Mut, render::camera::OrthographicProjection,
    transform::components::Transform, window::Window,
};
use uuid::Uuid;

use crate::{
    logic::save_data_manager::save_data_manager::is_solved,
    scenes::campaign::scene::campaign::{CampaignPuzzle, ClickableCampaignPuzzle},
    CAMPAIGN_NODE_SPACING, SPRITE_SPACING, TILE_NODE_SPRITE_SIZE,
};

fn get_topmost_unlocked_puzzle_position(
    clickable_campaign_puzzles: &Vec<ClickableCampaignPuzzle>,
) -> Vec2 {
    let topmost_puzzle = clickable_campaign_puzzles
        .iter()
        .filter(|campaign_puzzle| campaign_puzzle.unlocked)
        .max_by(|a, b| a.campaign_puzzle.pos_y.cmp(&b.campaign_puzzle.pos_y))
        .unwrap();
    get_campaign_puzzle_position(topmost_puzzle.campaign_puzzle)
}

fn get_rightmost_unlocked_puzzle_position(
    clickable_campaign_puzzles: &Vec<ClickableCampaignPuzzle>,
) -> Vec2 {
    let rightmost_puzzle = clickable_campaign_puzzles
        .iter()
        .filter(|campaign_puzzle| campaign_puzzle.unlocked)
        .max_by(|a, b| a.campaign_puzzle.pos_x.cmp(&b.campaign_puzzle.pos_x))
        .unwrap();
    get_campaign_puzzle_position(rightmost_puzzle.campaign_puzzle)
}

fn get_center_position(topmost_position: Vec2, rightmost_position: Vec2) -> Vec2 {
    Vec2::new(rightmost_position.x / 2.0, topmost_position.y / 2.0)
}

pub(crate) fn get_campaign_puzzle_position(campaign_puzzle: CampaignPuzzle) -> Vec2 {
    Vec2::new(
        campaign_puzzle.pos_x as f64 * (TILE_NODE_SPRITE_SIZE + CAMPAIGN_NODE_SPACING) as f64,
        campaign_puzzle.pos_y as f64 * (TILE_NODE_SPRITE_SIZE + CAMPAIGN_NODE_SPACING) as f64,
    )
}

/// Sets the camera to focus on the full scope of the unlocked portion of the campaign
pub(crate) fn update_camera(
    window: &Window,
    camera_transform: &mut Mut<Transform>,
    camera_projection: &mut Mut<OrthographicProjection>,
    clickable_campaign_puzzles: &Vec<ClickableCampaignPuzzle>,
) {
    // Get the topmost and rightmost unlocked puzzle positions
    let topmost_position = get_topmost_unlocked_puzzle_position(clickable_campaign_puzzles);
    let rightmost_position = get_rightmost_unlocked_puzzle_position(clickable_campaign_puzzles);

    // Get the center and move camera there
    let focus_point = get_center_position(topmost_position, rightmost_position);

    **camera_transform = Transform {
        translation: Vec3::new(focus_point.x as f32, focus_point.y as f32, 0.0),
        ..Default::default()
    };

    // Get the required scale and fit to include all unlocked puzzles in the window
    let distance_x = rightmost_position.x - focus_point.x + TILE_NODE_SPRITE_SIZE as f64;
    let distance_y = topmost_position.y - focus_point.y + TILE_NODE_SPRITE_SIZE as f64;

    let scale = (distance_x / (window.width() / 2.0) as f64)
        .max(distance_y / (window.height() / 2.0) as f64) as f32;

    camera_projection.scale = scale;
}

/// Returns if a puzzle is unlocked by determining if one of the 4 adjacent puzzles is solved.
pub(crate) fn is_unlocked(puzzle_grid: Vec<Vec<Uuid>>, campaign_puzzle: CampaignPuzzle) -> bool {
    if campaign_puzzle.pos_x == 0 && campaign_puzzle.pos_y == 0 {
        return true;
    }
    let adjacent_puzzles = get_adjacent_puzzles(puzzle_grid, campaign_puzzle);
    adjacent_puzzles
        .iter()
        .any(|adjacent_puzzle| is_solved(*adjacent_puzzle))
}

/// Returns a list of the adjacent puzzles to this one in the campaign.
pub(crate) fn get_adjacent_puzzles(
    puzzle_grid: Vec<Vec<Uuid>>,
    campaign_puzzle: CampaignPuzzle,
) -> Vec<Uuid> {
    let mut adjacent_puzzles: Vec<Uuid> = Vec::new();
    if campaign_puzzle.pos_y != 0 {
        adjacent_puzzles.push(puzzle_grid[campaign_puzzle.pos_y - 1][campaign_puzzle.pos_x]);
    }
    if campaign_puzzle.pos_y + 1 < puzzle_grid.len() {
        adjacent_puzzles.push(puzzle_grid[campaign_puzzle.pos_y + 1][campaign_puzzle.pos_x]);
    }
    if campaign_puzzle.pos_x != 0 {
        adjacent_puzzles.push(puzzle_grid[campaign_puzzle.pos_y][campaign_puzzle.pos_x - 1]);
    }
    if campaign_puzzle.pos_x + 1 < puzzle_grid[campaign_puzzle.pos_y].len() {
        adjacent_puzzles.push(puzzle_grid[campaign_puzzle.pos_y][campaign_puzzle.pos_x + 1]);
    }
    adjacent_puzzles
}
