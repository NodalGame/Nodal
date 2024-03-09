pub mod puzzle {
    use std::process::exit;

    use bevy::{
        input::{mouse::MouseButtonInput, InputSystem},
        prelude::*,
        scene::ron::de,
        sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle},
    };
    use bevy_prototype_lyon::prelude::*;
    use game_node::GameNode;
    use serde::Deserialize;
    use uuid::Uuid;

    use crate::{
        despawn_screen, game_node::game_node, puzzle_manager::puzzle_manager::PuzzleManager,
        AppState, SelectedPuzzle, TEXT_COLOR,
    };

    // This plugin will contain a playable puzzle.
    pub fn puzzle_plugin(app: &mut App) {
        app.add_systems(OnEnter(AppState::Puzzle), puzzle_setup)
            .add_systems(OnExit(AppState::Puzzle), despawn_screen::<OnPuzzleScreen>)
            // .add_systems(Update, line_system.run_if(in_state(AppState::Puzzle)))
            .insert_resource(ActiveNodes::default())
            .insert_resource(Lines::default());
        // .insert_resource(CurrentLine::new());
    }

    #[derive(Deserialize, Debug)]
    pub struct Puzzle {
        pub uuid: Uuid,
        width: u8,
        height: u8,
        nodes: Vec<GameNode>,
    }

    // Tracks all nodes in the puzzle, including sprite and position
    #[derive(Default, Resource, Component, Clone)]
    struct ActiveNodes {
        active_nodes: Vec<ActiveNode>,
    }

    #[derive(Component, Clone)]
    struct ActiveNode {
        node: GameNode,
        sprite: SpriteBundle,
    }

    // Tracks all lines connecting nodes in puzzle
    #[derive(Default, Resource)]
    struct Lines {
        lines: Vec<NodeLine>,
    }

    struct NodeLine {
        start_node: ActiveNode,
        end_node: ActiveNode,
        line: shapes::Line,
    }

    // Line currently being drawn by user on the screen
    #[derive(Resource)]
    struct CurrentLine {
        start_node: ActiveNode,
        line: shapes::Line,
        dragging: bool,
    }

    // Tag component used to tag entities added on the puzzle screen
    #[derive(Component)]
    struct OnPuzzleScreen;

    fn puzzle_setup(
        mut commands: Commands,
        assert_server: Res<AssetServer>,
        puzzle_id: Res<SelectedPuzzle>,
        puzzle_manager: Res<PuzzleManager>,
        mut active_nodes: ResMut<ActiveNodes>,
    ) {
        // TODO focus the camera on the center node
        // commands.spawn(Camera2dBundle::default());

        // Get the puzzle by loading it
        let puzzle = puzzle_manager
            .load_puzzle(&puzzle_id.uuid)
            .unwrap_or_else(|| {
                println!("Failed to load puzzle with id {}", puzzle_id.uuid);
                exit(1); // TODO cause game to not crash, and do this check in menu BEFORE switching scenes
            });

        // Sort nodes by id (top to bottom, left to right)
        let mut ordered_nodes = puzzle.nodes.clone();
        ordered_nodes.sort_by(|a, b| a.id.cmp(&b.id));

        // Load the node texture and make a new material
        let texture_node = assert_server.load("textures/sprites/EmptyNode.png");

        // Create a width x height grid of nodes as sprite bundles
        let spacing = 200.0;
        let sprite_size = 100.0;
        for x in 0..puzzle.width {
            for y in 0..puzzle.height {
                let node = ordered_nodes
                    .get((x * puzzle.height + y) as usize)
                    .unwrap_or_else(|| {
                        println!("Error when adding nodes to screen, index out of range?");
                        exit(1);
                    });

                let x_pos = x as f32 * spacing;
                let y_pos = y as f32 * spacing;

                let sprite_bundle = SpriteBundle {
                    texture: texture_node.clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(sprite_size, sprite_size)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(x_pos, y_pos, 0.0),
                    ..default()
                };
                commands.spawn(sprite_bundle.clone());
                active_nodes.active_nodes.push(ActiveNode {
                    node: node.clone(),
                    sprite: sprite_bundle.clone(),
                });
            }
        }
    }

    fn line_system(
        mut commands: Commands,
        active_nodes: ResMut<ActiveNodes>,
        // mut current_line: ResMut<CurrentLine>,
        mut lines: ResMut<Lines>,
        // button_query: Query<&Interaction, With<Button>>,
        mouse_button: Res<ButtonInput<MouseButton>>,
    ) {
        // for node in active_nodes.active_nodes.iter() {
        //     if let Ok(interaction) = button_query.get(node.entity) {
        //         match interaction {
        //             Interaction::Pressed => {
        //                 println!("clicked node {}", node.node.id);
        //                 // current_line.start_node = node.clone();
        //                 // current_line.dragging = true;
        //                 // current_line.line = shapes::Line(node.entity)
        //             }
        //             Interaction::Hovered => {
        //                 // TODO change to lit up version of node?
        //                 if mouse_button.just_released(MouseButton::Left) {
        //                     println!("released node {}", node.node.id);
        //                     // lines.lines.push(NodeLine { start_node: current_line.start_node.clone(), end_node: node.clone() });
        //                     // current_line.dragging = false;
        //                 }
        //                 println!("hovered node {}", node.node.id);
        //             }
        //             Interaction::None => {

        //             }
        //         }
        //     }
        // }
    }
}
