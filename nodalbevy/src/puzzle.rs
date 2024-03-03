pub mod puzzle {
    use bevy::prelude::*;

    use crate::{despawn_screen, AppState, SelectedPuzzle, TEXT_COLOR};

    // This plugin will contain a playable puzzle. 
    pub fn puzzle_plugin(app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Puzzle), puzzle_setup)
            .add_systems(OnExit(AppState::Puzzle), despawn_screen::<OnPuzzleScreen>);
    }

    // Tag component used to tag entities added on the puzzle screen
    #[derive(Component)]
    struct OnPuzzleScreen;

    fn puzzle_setup(
        mut commands: Commands,
        puzzle_id: Res<SelectedPuzzle>,
    ) {


        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        // center children
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                },
                OnPuzzleScreen,
            ))
            .with_children(|parent| {
                // First create a `NodeBundle` for centering what we want to display
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            // This will display its children in a column, from top to bottom
                            flex_direction: FlexDirection::Column,
                            // `align_items` will align children on the cross axis. Here the main axis is
                            // vertical (column), so the cross axis is horizontal. This will center the
                            // children
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::BLACK.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        // Add button to go to puzzle screen (TODO generate list of these to scroll through)
                        parent.spawn(TextBundle::from_section(
                            "Got puzzle id ".to_owned() + &puzzle_id.uuid.to_string(),
                            TextStyle {
                                font_size: 40.0,
                                color: TEXT_COLOR,
                                ..default()
                            }
                        ));
                    });
            });
    }
}