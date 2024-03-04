pub mod puzzle {
    use std::{fs::File, io::Read};

    use bevy::prelude::*;
    use serde::{Deserialize};
    use uuid::Uuid;
    use game_node::GameNode;
    use serde_json;

    use crate::{despawn_screen, game_node::game_node, AppState, SelectedPuzzle, TEXT_COLOR};

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
        puzzle_id: Res<SelectedPuzzle>,
    ) {
        let puzzle = load_puzzle_from_uuid(puzzle_id.uuid);

        println!("{:?}", puzzle);

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

    fn load_json_from_file(file_path: &str) -> serde_json::Result<Puzzle> {
        // Open the file
        let mut file = File::open(file_path)
            .expect("Failed to open the file");
    
        // Read the file contents into a string
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read the file");
    
        // Parse the string as JSON
        serde_json::from_str(&contents)
    }

    fn load_puzzle_from_uuid(uuid: Uuid) -> Puzzle {
        // TODO query some database of uuids to get puzzle json object
        let file_path = "assets/campaign/puzzles/puzzle1.json";

        let puzzle = match load_json_from_file(file_path) {
            Ok(data) => data,
            Err(e) => panic!("Failed to load puzzle: {:?}", e),
        };
        puzzle
    }
}