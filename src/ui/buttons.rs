pub mod buttons {
    use bevy::prelude::*;

    pub const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
    pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
    pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
    pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

    // This system handles changing all buttons color based on mouse interaction
    pub fn button_system(
        mut interaction_query: Query<
            (&Interaction, &mut BackgroundColor),
            (Changed<Interaction>, With<Button>),
        >,
    ) {
        for (interaction, mut color) in &mut interaction_query {
            *color = match *interaction {
                Interaction::Pressed => PRESSED_BUTTON.into(),
                Interaction::Hovered => HOVERED_BUTTON.into(),
                Interaction::None => NORMAL_BUTTON.into(),
            };
        }
    }

    // Style for buttons with text on them
    pub fn text_button_style() -> Style {
        Style {
            width: Val::Px(250.0),
            height: Val::Px(65.0),
            margin: UiRect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        }
    }

    // Style for buttons with icons on them
    pub fn icon_button_style() -> Style {
        Style {
            width: Val::Px(50.0),
            height: Val::Px(50.0),
            ..default()
        }
    }

    pub fn button_icon_style() -> Style {
        Style {
            width: Val::Px(30.0),
            // This takes the icons out of the flexbox flow, to be positioned exactly
            position_type: PositionType::Absolute,
            // The icon will be close to the left border of the button
            left: Val::Px(10.0),
            ..default()
        }
    }

    pub fn button_text_style() -> TextStyle {
        TextStyle {
            font_size: 40.0,
            color: TEXT_COLOR,
            ..default()
        }
    }
}
