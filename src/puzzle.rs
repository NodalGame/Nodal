pub mod puzzle {
    use std::{fs::File, io::Read, process::exit};

    use bevy::prelude::*;
    use serde::Deserialize;
    use uuid::Uuid;
    use game_node::GameNode;
    use serde_json;

    use crate::{despawn_screen, game_node::game_node, puzzle_manager::{self, puzzle_manager::PuzzleManager}, AppState, SelectedPuzzle, TEXT_COLOR};

    // This plugin will contain a playable puzzle. 
    pub fn puzzle_plugin(app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Puzzle), puzzle_setup)
            .add_systems(OnExit(AppState::Puzzle), despawn_screen::<OnPuzzleScreen>);
    }

    #[derive(Deserialize, Debug)]
    pub struct Puzzle {
        pub uuid: Uuid,
        width: u8,
        height: u8,
        nodes: Vec<GameNode>
    }

    // Tag component used to tag entities added on the puzzle screen
    #[derive(Component)]
    struct OnPuzzleScreen;

    fn puzzle_setup(
        mut commands: Commands,
        assert_server: Res<AssetServer>,
        puzzle_id: Res<SelectedPuzzle>,
        puzzle_manager: Res<PuzzleManager>,
    ) {
        let puzzle = puzzle_manager.load_puzzle(&puzzle_id.uuid).unwrap_or_else(|| {
            println!("Failed to load puzzle with id {}", puzzle_id.uuid);
            exit(1); // TODO cause game to not crash, and do this check in menu BEFORE switching scenes 
        });
        let texture_node = assert_server.load("textures/EmptyNode.png");

        // TODO generate nodes in grid 
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
                        parent
                            .spawn(TextBundle::from_section(
                            "Got puzzle id ".to_owned() + &puzzle.uuid.to_string(),
                            TextStyle {
                                font_size: 40.0,
                                color: TEXT_COLOR,
                                ..default()
                            }
                        ));

                        // Sort nodes by id
                        let mut ordered_nodes = puzzle.nodes.clone();
                        ordered_nodes.sort_by(|a, b| a.id.cmp(&b.id));

                        for row in 0 .. puzzle.height {
                            parent
                                .spawn(NodeBundle {
                                style: Style {
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                ..default()
                            }).with_children(|parent| {
                                parent.spawn(NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::Row,
                                        justify_content: JustifyContent::SpaceBetween,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|parent| {
                                    for col in 0 .. puzzle.width {
                                        let node = ordered_nodes.get((row * puzzle.height + col) as usize);
                                        // TODO change style based on node class
                                        println!("new node at {} {}", row, col);
                                        parent.spawn(ImageBundle {
                                            style: Style {
                                                width: Val::Px(100.0),
                                                height: Val::Px(100.0),
                                                ..default()
                                            },
                                            image: UiImage::new(texture_node.clone()),
                                            ..default()
                                        });
                                    }
                                });
                            });
                        }
                    });
                // TODO Create another bundle with a button to return to menu
            });
    }
}