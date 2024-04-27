pub mod puzzle {
    use core::hash::{Hash, Hasher};
    use std::process::exit;

    use bevy::{
        app::{App, Update},
        asset::AssetServer,
        ecs::{
            component::Component,
            query::{Changed, With},
            schedule::{
                common_conditions::in_state, IntoSystemConfigs, NextState, OnEnter, OnExit,
            },
            system::{Commands, Query, Res, ResMut, Resource},
        },
        hierarchy::BuildChildren,
        input::{mouse::MouseButton, ButtonInput},
        math::{Vec2, Vec3},
        render::camera::Camera,
        sprite::{Sprite, SpriteBundle},
        transform::components::{GlobalTransform, Transform},
        ui::{
            node_bundles::{ButtonBundle, NodeBundle},
            widget::Button,
            AlignItems, Interaction, JustifyContent, Style, UiImage, Val,
        },
        window::{PrimaryWindow, Window},
    };
    use serde::Deserialize;
    use uuid::Uuid;

    use crate::{
        buttons::icon_button_style,
        check_answer, clicked_on_sprite, despawn_screen, get_bg_tile, get_cursor_world_position,
        get_line_texture, get_set_tiles, get_set_upper_left_node,
        objects::{
            connected_set_rule::connected_set_rule::ConnectedSetRule,
            game_node::game_node::GameNode,
            game_set::game_set::GameSet,
            node_condition::node_condition::NodeCondition,
            set_rule::set_rule::SetRule,
        },
        puzzle_manager::PuzzleManager,
        texture::Texture,
        AppState, MainCamera, SelectedPuzzle, CDTN_RULE_SPRITE_SIZE, INTERNAL_SPACING_X,
        INTERNAL_SPACING_Y, SPRITE_SPACING, STACK_CDTN_RULE_SPACING, TILE_NODE_SPRITE_SIZE,
    };

    // This plugin will contain a playable puzzle.
    pub fn puzzle_plugin(app: &mut App) {
        app.add_systems(OnEnter(AppState::Puzzle), puzzle_setup)
            .add_systems(OnExit(AppState::Puzzle), despawn_screen::<OnPuzzleScene>)
            .add_systems(OnExit(AppState::Puzzle), despawn_screen::<OnPuzzleUI>)
            .add_systems(Update, line_system.run_if(in_state(AppState::Puzzle)))
            .add_systems(Update, ui_action.run_if(in_state(AppState::Puzzle)))
            .insert_resource(ActiveNodes::default())
            .insert_resource(ActiveSets::default())
            .insert_resource(ActiveLines::default())
            .insert_resource(CurrentLine::default());
    }

    #[derive(Deserialize, Debug)]
    pub struct Puzzle {
        pub uuid: Uuid,
        pub width: u8,
        pub height: u8,
        pub nodes: Vec<GameNode>,
        pub sets: Vec<GameSet>,
    }

    // Tracks all nodes in the puzzle
    #[derive(Default, Resource, Component, Clone)]
    struct ActiveNodes {
        active_nodes: Vec<ActiveNode>,
    }

    // TODO ActiveNode should get its own file/module
    #[derive(Component, Clone)]
    pub struct ActiveNode {
        pub node: GameNode,
        pub connections: Vec<u16>,
        pub sprite: SpriteBundle,
        // TODO will use in the proactive sat checks per puzzle update
        pub satisfied: bool,
    }

    // Tracks all sets in the puzzle
    #[derive(Default, Resource, Component, Clone)]
    struct ActiveSets {
        active_sets: Vec<ActiveSet>,
    }

    // TODO ActiveSet should get its own file/module
    #[derive(Component, Clone)]
    pub struct ActiveSet {
        pub set: GameSet,
        pub satisfied: bool,
    }

    impl ActiveNode {
        // TODO this will need to be updated to take set rules as well
        pub fn get_failed_conditions(&self, active_nodes: Vec<&ActiveNode>) -> Vec<NodeCondition> {
            self.node
                .conditions
                .iter()
                .filter(|c| !c.is_satisfied(self, &active_nodes))
                .cloned()
                .collect()
        }
    }

    impl PartialEq for ActiveNode {
        fn eq(&self, other: &Self) -> bool {
            self.node == other.node && self.connections == other.connections
            // Note: SpriteBundle is not compared
        }
    }

    impl Eq for ActiveNode {}

    impl Hash for ActiveNode {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.node.hash(state);
            self.connections.hash(state);
            // Note: SpriteBundle is not hashed
        }
    }

    // Tracks all lines connecting nodes in puzzle
    #[derive(Default, Resource)]
    struct ActiveLines {
        lines: Vec<ActiveLine>,
    }

    struct ActiveLine {
        // TODO remove start_node end_node since they are very likely not needed, only need ref to the sprites
        start_node: ActiveNode,
        end_node: ActiveNode,
        sprite: SpriteBundle,
    }

    // Start node of the line currently being drawn by user on the screen
    #[derive(Default, Resource)]
    struct CurrentLine {
        start_node_id: Option<u16>,
    }

    // Tag component used to tag entities added on the puzzle scene
    #[derive(Component)]
    struct OnPuzzleScene;

    // Tag component used to tag entities added on the UI of the puzzle screen
    #[derive(Component)]
    struct OnPuzzleUI;

    // All actions that can be triggered from a button click
    #[derive(Component)]
    enum PuzzleButtonAction {
        CheckAnswer,
        Reset,
        ReturnToPreviousPage,
    }

    fn puzzle_setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        puzzle_id: Res<SelectedPuzzle>,
        puzzle_manager: Res<PuzzleManager>,
        mut active_nodes: ResMut<ActiveNodes>,
        mut active_sets: ResMut<ActiveSets>,
        // Query to get camera transform
        mut q_camera: Query<&mut Transform, With<MainCamera>>,
    ) {
        // Get the puzzle by loading it
        let puzzle = puzzle_manager
            .load_puzzle(&puzzle_id.uuid)
            .unwrap_or_else(|| {
                println!("Failed to load puzzle with id {}", puzzle_id.uuid);
                exit(1); // TODO cause game to not crash, and do this check in menu BEFORE switching scenes
            });

        // Sort nodes by id (bottom to top, left to right)
        let mut ordered_nodes = puzzle.nodes.clone();
        ordered_nodes.sort_by(|a, b| a.id.cmp(&b.id));

        // TODO load only when needed, then cache in map to re-access in the screen spawning loop
        // Load node textures
        let tex_node = asset_server.load(Texture::Node.path());

        // Load condition textures
        let tex_cdtn_branch_equal = asset_server.load(Texture::CdtnBranchEqual.path());
        let tex_cdtn_leaf = asset_server.load(Texture::CdtnLeaf.path());

        // Load set rule textures
        let tex_rule_connected = asset_server.load(Texture::SetRuleConnected.path());
        let tex_rule_homomorphic = asset_server.load(Texture::SetRuleHomomorphic.path());

        // Create a width x height grid of nodes as sprite bundles, accounting for background tiles
        for x in 0..puzzle.width * 2 + 1 {
            for y in 0..puzzle.height * 2 + 1 {
                // If background tile, spawn it and continue
                if x % 2 == 0 || y % 2 == 0 {
                    commands.spawn((
                        get_bg_tile(x, y, puzzle.width, puzzle.height, asset_server.clone()),
                        OnPuzzleScene,
                    ));
                    continue;
                }

                let node = ordered_nodes
                    .get((x / 2 * puzzle.height + y / 2) as usize)
                    .unwrap_or_else(|| {
                        println!("Error when adding nodes to screen, index out of range?");
                        exit(1);
                    });

                // Spawn the node on screen
                let node_x = x as f32 * SPRITE_SPACING;
                let node_y = y as f32 * SPRITE_SPACING;

                let node_sprite = SpriteBundle {
                    texture: tex_node,
                    // TODO move getting the sprite to somewhere else so it's not duplicated
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(TILE_NODE_SPRITE_SIZE, TILE_NODE_SPRITE_SIZE)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(node_x, node_y, 0.0),
                    ..Default::default()
                };
                commands.spawn(node_sprite.clone()).insert(OnPuzzleScene);

                // Spawn its conditions on the screen
                for (cdtn_idx, condition) in node.conditions.iter().enumerate() {
                    // TODO get textures via either node_condition.rs or texture.rs
                    let condition_texture = match condition {
                        NodeCondition::BranchEqual => tex_cdtn_branch_equal.clone(),
                        NodeCondition::Leaf => tex_cdtn_leaf.clone(),
                        _ => {
                            println!("Error when adding nodes to screen, invalid condition?");
                            exit(1);
                        }
                    };

                    let condition_sprite = SpriteBundle {
                        texture: condition_texture,
                        sprite: condition.sprite().clone(),
                        transform: Transform::from_xyz(
                            node_x + TILE_NODE_SPRITE_SIZE - INTERNAL_SPACING_X,
                            node_y + TILE_NODE_SPRITE_SIZE
                                - INTERNAL_SPACING_Y
                                - cdtn_idx as f32
                                    * (CDTN_RULE_SPRITE_SIZE + STACK_CDTN_RULE_SPACING),
                            0.0,
                        ),
                        ..Default::default()
                    };
                    commands
                        .spawn(condition_sprite.clone())
                        .insert(OnPuzzleScene);
                }

                active_nodes.active_nodes.push(ActiveNode {
                    node: node.clone(),
                    connections: Vec::new(),
                    sprite: node_sprite.clone(),
                    satisfied: false,
                });
            }
        }

        // Get camera transform
        for mut transform in q_camera.iter_mut() {
            // Move it to center of puzzle
            *transform = Transform {
                translation: Vec3::new(
                    puzzle.width as f32 * SPRITE_SPACING,
                    puzzle.height as f32 * SPRITE_SPACING,
                    0.0,
                ),
                ..Default::default()
            };
        }

        // Add a back button, check answer button, and restart button
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::End,
                        justify_content: JustifyContent::End,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                OnPuzzleUI,
            ))
            .with_children(|parent| {
                parent.spawn((
                    ButtonBundle {
                        style: icon_button_style(),
                        image: UiImage::new(asset_server.load(Texture::BtnCheckAnswer.path())),
                        ..Default::default()
                    },
                    PuzzleButtonAction::CheckAnswer,
                ));
                parent.spawn((
                    ButtonBundle {
                        style: icon_button_style(),
                        image: UiImage::new(asset_server.load(Texture::BtnClearLines.path())),
                        ..Default::default()
                    },
                    PuzzleButtonAction::Reset,
                ));
                parent.spawn((
                    ButtonBundle {
                        style: icon_button_style(),
                        image: UiImage::new(asset_server.load(Texture::BtnGoBack.path())),
                        ..Default::default()
                    },
                    PuzzleButtonAction::ReturnToPreviousPage,
                ));
            });

        // Map tile spaces to sets to generate
        for set_idx in 0..puzzle.sets.len() {
            let set = &puzzle.sets[set_idx];
            for set_tile in get_set_tiles(set, &puzzle, asset_server.clone()) {
                commands.spawn((set_tile, OnPuzzleScene));
            }

            // Add the set rule sprites in the upper left-most corner of the set
            let upper_left_node = get_set_upper_left_node(set, &puzzle);
            let node_x = (upper_left_node / puzzle.height as u16) as f32 * SPRITE_SPACING;
            let node_y = (upper_left_node % puzzle.height as u16) as f32 * SPRITE_SPACING;

            let mut total_rule_idx = 0;
            for rule in set.rules.iter() {
                // TODO get textures via either set_rule.rs or texture.rs
                let rule_texture = match rule {
                    SetRule::Connected => tex_rule_connected.clone(),
                    SetRule::Unconnected => todo!(),
                    SetRule::Disconnected => todo!(),
                    SetRule::Cycle => todo!(),
                    SetRule::NoCycle => todo!(),
                    SetRule::Scope => todo!(),
                };

                let rule_sprite = SpriteBundle {
                    texture: rule_texture,
                    sprite: rule.sprite().clone(),
                    transform: Transform::from_xyz(
                        node_x - TILE_NODE_SPRITE_SIZE,
                        node_y + TILE_NODE_SPRITE_SIZE
                            - INTERNAL_SPACING_Y
                            - total_rule_idx as f32
                                * (CDTN_RULE_SPRITE_SIZE + STACK_CDTN_RULE_SPACING),
                        0.0,
                    ),
                    ..Default::default()
                };
                commands.spawn(rule_sprite.clone()).insert(OnPuzzleScene);

                total_rule_idx += 1;
            }
            for crule in set.connected_rules.iter() {
                // TODO get textures via either connected_set_rule.rs or texture.rs
                let crule_texture = match crule {
                    ConnectedSetRule::Homomorphism(crule) => tex_rule_homomorphic.clone(),
                };

                let crule_sprite = SpriteBundle {
                    texture: crule_texture,
                    sprite: crule.sprite().clone(),
                    transform: Transform::from_xyz(
                        node_x - TILE_NODE_SPRITE_SIZE,
                        node_y + TILE_NODE_SPRITE_SIZE
                            - INTERNAL_SPACING_Y
                            - total_rule_idx as f32
                                * (CDTN_RULE_SPRITE_SIZE + STACK_CDTN_RULE_SPACING),
                        0.0,
                    ),
                    ..Default::default()
                };
                commands.spawn(crule_sprite.clone()).insert(OnPuzzleScene);

                total_rule_idx += 1;
            }
            active_sets.active_sets.push(ActiveSet {
                set: puzzle.sets[set_idx].clone(),
                satisfied: false,
            });
        }
    }

    /// A system for handling lines added to the puzzle.
    ///
    /// # Parameters
    ///
    /// - `commands`: Bevy's command system, used to spawn new entities.
    /// - `active_nodes`: Resource containing all active nodes in the puzzle.
    /// - `current_line`: Resource containing the current line being drawn by the user.
    /// - `lines`: Resource containing all lines in the puzzle.
    /// - `mouse_button_input`: Bevy's mouse button input system, used to check if the left mouse button is pressed.
    /// - `q_window`: Bevy's query system, used to get the window, so we can read current cursor position.
    /// - `q_camera`: Bevy's query system, used to get the camera transform to also read current cursor position.
    /// - `asset_server`: Bevy's asset server, used to load textures.
    fn line_system(
        mut commands: Commands,
        mut active_nodes: ResMut<ActiveNodes>,
        mut current_line: ResMut<CurrentLine>,
        mut lines: ResMut<ActiveLines>,
        mouse_button_input: Res<ButtonInput<MouseButton>>,
        q_window: Query<&Window, With<PrimaryWindow>>,
        q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
        asset_server: Res<AssetServer>,
    ) {
        // Get camera info and transform, assuming exacly 1 camera entity
        let (camera, camera_transform) = q_camera.single();

        // Only one primary window, so get it from query
        let window = q_window.single();

        // Check if cursor inside window and get its position, convert to world coords, discard Z
        let world_position = get_cursor_world_position(window, camera, camera_transform);

        // On left click, start new line on a clicked node, if exists
        if mouse_button_input.just_pressed(MouseButton::Left) {
            for active_node in active_nodes.active_nodes.iter() {
                if clicked_on_sprite(&active_node.sprite, world_position) {
                    current_line.start_node_id = Some(active_node.clone().node.id);
                }
            }
        // If left click release, end the line on the released node, if exists
        } else if mouse_button_input.just_released(MouseButton::Left) {
            // If start node is not set, return
            if !current_line.start_node_id.is_some() {
                return;
            }

            // Grab the start node and end node objects from the active_nodes as an iter_mut
            let mut opt_start_node: Option<&mut ActiveNode> = None;
            let mut opt_end_node: Option<&mut ActiveNode> = None;

            for active_node in active_nodes.active_nodes.iter_mut() {
                if active_node.node.id == current_line.start_node_id.unwrap() {
                    opt_start_node = Some(active_node);
                } else if clicked_on_sprite(&active_node.sprite, world_position) {
                    opt_end_node = Some(active_node);
                }
            }

            if opt_start_node.is_none() || opt_end_node.is_none() {
                return;
            }

            let start_node = opt_start_node.unwrap();
            let end_node = opt_end_node.unwrap();

            let start_pos = start_node.clone().sprite.transform.translation.truncate();
            let end_pos = end_node.sprite.transform.translation.truncate();

            // Get the appropriate line texture, if exists (otherwise invalid node pair)
            let line_texture =
                get_line_texture(start_node.clone(), end_node.clone()).unwrap_or(&Texture::Missing);

            if *line_texture == Texture::Missing {
                return;
            }

            let line_sprite = SpriteBundle {
                texture: asset_server.load(line_texture.path()),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(TILE_NODE_SPRITE_SIZE, TILE_NODE_SPRITE_SIZE)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(
                    (end_pos.x + start_pos.x) / 2.0,
                    (end_pos.y + start_pos.y) / 2.0,
                    0.0,
                ),
                ..Default::default()
            };

            // Update list of lines
            lines.lines.push(ActiveLine {
                start_node: start_node.clone(),
                end_node: end_node.clone(),
                sprite: line_sprite.clone(),
            });

            // Update connections of both start and end node
            start_node.connections.push(end_node.node.id);
            end_node.connections.push(start_node.node.id);

            // Add line to the screen
            commands.spawn(line_sprite).insert(OnPuzzleScene);

            // Regardless if we ended on a node or not, clear the current line
            current_line.start_node_id = None;
        }
    }

    fn ui_action(
        interaction_query: Query<
            (&Interaction, &PuzzleButtonAction),
            (Changed<Interaction>, With<Button>),
        >,
        active_nodes: Res<ActiveNodes>,
        active_sets: Res<ActiveSets>,
        mut app_state: ResMut<NextState<AppState>>,
    ) {
        for (interaction, ui_button_action) in &interaction_query {
            if *interaction == Interaction::Pressed {
                match ui_button_action {
                    PuzzleButtonAction::CheckAnswer => {
                        let solved = check_answer(
                            active_nodes.active_nodes.iter().collect(),
                            active_sets.active_sets.iter().collect(),
                        );
                        println!("Puzzle solved: {}", solved);
                    }
                    PuzzleButtonAction::Reset => {
                        println!("Clear lines button pressed");
                    }
                    PuzzleButtonAction::ReturnToPreviousPage => {
                        println!("Go back button pressed");
                        // TODO track previous state before entering puzzle (campaign vs public level select)
                        app_state.set(AppState::Campaign);
                    }
                }
            }
        }
    }
}
