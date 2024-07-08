use bevy::color::palettes;
use bevy::color::Color;

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

pub(crate) const COLOR_SET_BORDER: Color = Color::BLACK;

pub(crate) const COLOR_NODE_SAT: Color = Color::BLACK;
pub(crate) const COLOR_NODE_UNSAT: Color = Color::WHITE;
pub(crate) const COLOR_RULE_SAT: Color = Color::WHITE;
pub(crate) const COLOR_RULE_UNSAT: Color = Color::BLACK;
pub(crate) const COLOR_CDTN_SAT: Color = Color::BLACK;
pub(crate) const COLOR_CDTN_UNSAT: Color = Color::WHITE;
pub(crate) const COLOR_CDTN_BLUE_SAT: Color = bevy::prelude::Color::Srgba(palettes::basic::BLUE);
pub(crate) const COLOR_CDTN_BLUE_UNSAT: Color = bevy::prelude::Color::Srgba(palettes::basic::NAVY);
pub(crate) const COLOR_CDTN_PURPLE_SAT: Color = Color::srgb(0.9, 0.0, 0.9);
pub(crate) const COLOR_CDTN_PURPLE_UNSAT: Color = Color::srgb(0.3, 0.0, 0.3);
pub(crate) const COLOR_CDTN_GREEN_SAT: Color = bevy::prelude::Color::Srgba(palettes::basic::LIME);
pub(crate) const COLOR_CDTN_GREEN_UNSAT: Color =
    bevy::prelude::Color::Srgba(palettes::basic::GREEN);
pub(crate) const COLOR_RULE_YELLOW_SAT: Color =
    bevy::prelude::Color::Srgba(palettes::basic::YELLOW);
pub(crate) const COLOR_RULE_YELLOW_UNSAT: Color =
    bevy::prelude::Color::Srgba(palettes::basic::OLIVE);
pub(crate) const COLOR_RULE_ORANGE_SAT: Color = bevy::prelude::Color::Srgba(palettes::css::ORANGE);
pub(crate) const COLOR_RULE_ORANGE_UNSAT: Color = bevy::prelude::Color::Srgba(palettes::css::BEIGE);
pub(crate) const COLOR_RULE_RED_SAT: Color = bevy::prelude::Color::Srgba(palettes::basic::RED);
pub(crate) const COLOR_RULE_RED_UNSAT: Color = bevy::prelude::Color::Srgba(palettes::css::CRIMSON);

pub(crate) const COLOR_SET_0: Color = Color::srgba(1.0, 0.0, 0.0, 0.5);
pub(crate) const COLOR_SET_1: Color = Color::srgba(0.0, 1.0, 0.0, 0.5);
pub(crate) const COLOR_SET_2: Color = Color::srgba(0.0, 0.0, 1.0, 0.5);
