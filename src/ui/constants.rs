use bevy::render::color::Color;

pub(crate) const TILE_NODE_SPRITE_SIZE: f32 = 100.0;
pub(crate) const CDTN_RULE_SPRITE_SIZE: f32 = 45.0;
pub(crate) const INTERNAL_SPACING_X: f32 = 25.0;
pub(crate) const INTERNAL_SPACING_Y: f32 = 25.0;
pub(crate) const STACK_CDTN_RULE_SPACING: f32 = 5.0;
pub(crate) const SPRITE_SPACING: f32 = 100.0;
pub(crate) const BG_SET_SPRITE_SIZE: f32 = 200.0;

pub(crate) const Z_BACKGROUND: f32 = -3.0;
pub(crate) const Z_SET_FILL: f32 = -2.0;
pub(crate) const Z_LINE: f32 = -1.0;
pub(crate) const Z_SET_RULE_BOX: f32 = 0.0;
pub(crate) const Z_RULE_CDTN_NODE: f32 = 1.0;

pub(crate) const COLOR_NODE_SAT: Color = Color::WHITE;
pub(crate) const COLOR_NODE_UNSAT: Color = Color::BLACK;
pub(crate) const COLOR_RULE_SAT: Color = Color::WHITE;
pub(crate) const COLOR_RULE_UNSAT: Color = Color::BLACK;
pub(crate) const COLOR_CDTN_SAT: Color = Color::WHITE;
pub(crate) const COLOR_CDTN_UNSAT: Color = Color::BLACK;
pub(crate) const COLOR_CDTN_BLUE_SAT: Color = Color::ALICE_BLUE;
pub(crate) const COLOR_CDTN_BLUE_UNSAT: Color = Color::MIDNIGHT_BLUE;
pub(crate) const COLOR_CDTN_PURPLE_SAT: Color = Color::PINK;
pub(crate) const COLOR_CDTN_PURPLE_UNSAT: Color = Color::PURPLE;
pub(crate) const COLOR_CDTN_GREEN_SAT: Color = Color::LIME_GREEN;
pub(crate) const COLOR_CDTN_GREEN_UNSAT: Color = Color::DARK_GREEN;
pub(crate) const COLOR_RULE_YELLOW_SAT: Color = Color::YELLOW;
pub(crate) const COLOR_RULE_YELLOW_UNSAT: Color = Color::GOLD;
pub(crate) const COLOR_RULE_ORANGE_SAT: Color = Color::ORANGE;
pub(crate) const COLOR_RULE_ORANGE_UNSAT: Color = Color::BEIGE;
pub(crate) const COLOR_RULE_RED_SAT: Color = Color::RED;
pub(crate) const COLOR_RULE_RED_UNSAT: Color = Color::CRIMSON;

pub(crate) const COLOR_SET_0: Color = Color::rgba(1.0, 0.0, 0.0, 0.1);
pub(crate) const COLOR_SET_1: Color = Color::rgba(0.0, 1.0, 0.0, 0.1);
pub(crate) const COLOR_SET_2: Color = Color::rgba(0.0, 0.0, 1.0, 0.1);