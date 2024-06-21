pub mod scene {
    use std::process::exit;

    use bevy::{
        app::{App, Update},
        asset::AssetServer,
        ecs::{
            component::Component,
            entity::Entity,
            event::{Event, EventReader, EventWriter},
            query::{Changed, With},
            schedule::{
                common_conditions::in_state, IntoSystemConfigs, NextState, OnEnter, OnExit,
            },
            system::{Commands, Query, Res, ResMut, Resource},
        },
        hierarchy::BuildChildren,
        input::{mouse::MouseButton, ButtonInput},
        math::{Vec2, Vec3},
        render::{camera::Camera, color::Color},
        sprite::{Sprite, SpriteBundle},
        transform::components::{GlobalTransform, Transform},
        ui::{
            node_bundles::{ButtonBundle, NodeBundle},
            widget::Button,
            AlignItems, Interaction, JustifyContent, Style, UiImage, Val,
        },
        utils::HashMap,
        window::{PrimaryWindow, Window},
    };
    use tokio::runtime::Runtime;
    use tracing::error;

    use crate::{
        backend::api::api::NodalApi, buttons::icon_button_style, clicked_on_sprite, despawn_screen, get_all_satisfied_states, get_cursor_world_position, logic::puzzle::tile_placement::tile_placement::get_set_upper_left_node, node_to_position, puzzle_manager::PuzzleManager, scenes::puzzle::util::{get_color_for_set_tile, get_line_texture, get_puzzle_background_tile, get_set_tiles}, structs::{
            active::{
                active_connected_node_condition::active_connected_node_condition::ActiveConnectedNodeCondition,
                active_connected_set_rule::active_connected_set_rule::ActiveConnectedSetRule,
                active_identifier::active_identifier::ActiveIdentifier,
                active_line::active_line::ActiveLine, active_node::active_node::ActiveNode,
                active_node_condition::active_node_condition::ActiveNodeCondition,
                active_set::active_set::ActiveSet, active_set_rule::active_set_rule::ActiveSetRule,
                traits::traits::Satisfiable,
            },
            immutable::{
                connected_node_condition::connected_node_condition::ConnectedNodeCondition,
                connected_set_rule::connected_set_rule::ConnectedSetRule,
                game_node::game_node::GameNodeId, node_condition::node_condition::NodeCondition,
                set_rule::set_rule::SetRule,
            },
        }, texture::Texture, AppState, MainCamera, SelectedPuzzle, CDTN_RULE_SPRITE_SIZE, COLOR_NODE_UNSAT, INTERNAL_SPACING_X, INTERNAL_SPACING_Y, SPRITE_SPACING, STACK_CDTN_RULE_SPACING, TILE_NODE_SPRITE_SIZE, Z_LINE, Z_RULE_CDTN_NODE, Z_SET_RULE_BOX
    };

    // This plugin will contain a playable puzzle.
    pub fn puzzle_plugin(app: &mut App) {
        app.add_systems(OnEnter(AppState::Puzzle), puzzle_setup)
            .add_systems(OnExit(AppState::Puzzle), despawn_screen::<OnPuzzleScene>)
            .add_systems(OnExit(AppState::Puzzle), despawn_screen::<OnPuzzleUI>)
            .add_systems(Update, line_system.run_if(in_state(AppState::Puzzle)))
            .add_systems(Update, ui_action.run_if(in_state(AppState::Puzzle)))
            .add_event::<UpdateSatisfiedStates>()
            .add_systems(Update, update_satisfied_states_ui)
            .insert_resource(ActiveNodes::default())
            .insert_resource(ActiveSets::default())
            .insert_resource(ActiveLines::default())
            .insert_resource(CurrentLine::default());
    }

    /// A map of satisfiable entities with an active identifier to their updated satisfied state.
    pub type SatisfiedStatesMap = HashMap<ActiveIdentifier, bool>;

    #[derive(Event, Debug, Clone, Default)]
    struct UpdateSatisfiedStates(SatisfiedStatesMap);

    // Tracks all nodes in the puzzle
    #[derive(Default, Resource, Component, Clone)]
    struct ActiveNodes {
        active_nodes: Vec<ActiveNode>,
    }

    // Tracks all sets in the puzzle
    #[derive(Default, Resource, Component, Clone)]
    struct ActiveSets {
        active_sets: Vec<ActiveSet>,
    }

    // Tracks all lines connecting nodes in puzzle
    #[derive(Default, Resource)]
    struct ActiveLines {
        lines: Vec<ActiveLine>,
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
        let tex_cdtn_degree_equal = asset_server.load(Texture::CdtnDegreeEqual.path());

        // Load set rule textures
        let tex_rule_disconnected = asset_server.load(Texture::SetRuleDisconnected.path());
        let tex_rule_leaf = asset_server.load(Texture::SetRuleLeaf.path());
        let tex_rule_homomorphism = asset_server.load(Texture::SetRuleHomomorphism.path());

        // Load set rule box texture
        let tex_rule_box = asset_server.load(Texture::SetRuleBox.path());

        // Create a width x height grid of nodes as sprite bundles, accounting for background tiles
        for x in 0..puzzle.width * 2 + 1 {
            for y in 0..puzzle.height * 2 + 1 {
                // If background tile, spawn it and continue
                if x % 2 == 0 || y % 2 == 0 {
                    // TODO add back the bg tiles if necessary
                    // commands.spawn((
                    //     get_puzzle_background_tile(x, y, puzzle.width, puzzle.height, asset_server.clone()),
                    //     OnPuzzleScene,
                    // ));
                    continue;
                }

                let node = ordered_nodes
                    .get((x / 2 * puzzle.height + y / 2) as usize)
                    .unwrap_or_else(|| {
                        exit(1);
                    });

                // Spawn the node on screen
                let node_x = x as f32 * SPRITE_SPACING;
                let node_y = y as f32 * SPRITE_SPACING;

                // TODO move this to ActiveNode passing node_x and node_y
                let node_sprite = SpriteBundle {
                    texture: tex_node.clone(),
                    // TODO move getting the sprite to somewhere else so it's not duplicated
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(TILE_NODE_SPRITE_SIZE, TILE_NODE_SPRITE_SIZE)),
                        color: COLOR_NODE_UNSAT,
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(node_x, node_y, 0.0),
                    ..Default::default()
                };
                let node_sprite_id = commands
                    .spawn(node_sprite.clone())
                    .insert(OnPuzzleScene)
                    .id();

                let mut active_node_conditions: Vec<ActiveNodeCondition> = vec![];

                let mut total_cdtn_idx = 0;
                for condition in node.conditions.iter() {
                    // TODO get textures via either node_condition.rs or texture.rs
                    let condition_texture = match condition {
                        NodeCondition::BranchEqual => tex_cdtn_branch_equal.clone(),
                        NodeCondition::Leaf => tex_cdtn_leaf.clone(),
                    };

                    let condition_sprite = SpriteBundle {
                        texture: condition_texture,
                        sprite: condition.sprite().clone(),
                        transform: Transform::from_xyz(
                            node_x + TILE_NODE_SPRITE_SIZE - INTERNAL_SPACING_X,
                            node_y + TILE_NODE_SPRITE_SIZE
                                - INTERNAL_SPACING_Y
                                - total_cdtn_idx as f32
                                    * (CDTN_RULE_SPRITE_SIZE + STACK_CDTN_RULE_SPACING),
                            0.0,
                        ),
                        ..Default::default()
                    };

                    let condition_sprite_id = commands
                        .spawn(condition_sprite.clone())
                        .insert(OnPuzzleScene)
                        .id();

                    active_node_conditions.push(ActiveNodeCondition {
                        active_id: ActiveIdentifier::new(),
                        sprite: condition_sprite,
                        sprite_entity_id: condition_sprite_id,
                        satisfied: false,
                        condition: condition.clone(),
                    });

                    total_cdtn_idx += 1;
                }

                let mut active_connected_conditions: Vec<ActiveConnectedNodeCondition> = vec![];

                for con_cdtn in node.connected_conditions.iter() {
                    // TODO get textures via either connected_node_condition.rs or texture.rs
                    let con_cdtn_texture = match con_cdtn {
                        ConnectedNodeCondition::DegreeEqual(_con_cdtn) => {
                            tex_cdtn_degree_equal.clone()
                        }
                    };

                    let con_cdtn_sprite = SpriteBundle {
                        texture: con_cdtn_texture,
                        sprite: con_cdtn.sprite().clone(),
                        transform: Transform::from_xyz(
                            node_x + TILE_NODE_SPRITE_SIZE - INTERNAL_SPACING_X,
                            node_y + TILE_NODE_SPRITE_SIZE
                                - INTERNAL_SPACING_Y
                                - total_cdtn_idx as f32
                                    * (CDTN_RULE_SPRITE_SIZE + STACK_CDTN_RULE_SPACING),
                            0.0,
                        ),
                        ..Default::default()
                    };

                    let connected_condition_sprite_id = commands
                        .spawn(con_cdtn_sprite.clone())
                        .insert(OnPuzzleScene)
                        .id();

                    active_connected_conditions.push(ActiveConnectedNodeCondition {
                        active_id: ActiveIdentifier::new(),
                        condition: con_cdtn.clone(),
                        sprite: con_cdtn_sprite,
                        sprite_entity_id: connected_condition_sprite_id,
                        satisfied: false,
                    });

                    total_cdtn_idx += 1;
                }

                active_nodes.active_nodes.push(ActiveNode {
                    node: node.clone(),
                    connections: Vec::new(),
                    sprite: node_sprite,
                    satisfied: false,
                    active_id: ActiveIdentifier::new(),
                    sprite_entity_id: node_sprite_id,
                    active_conditions: active_node_conditions,
                    active_connected_conditions: active_connected_conditions,
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
                // parent.spawn((
                //     ButtonBundle {
                //         style: icon_button_style(),
                //         image: UiImage::new(asset_server.load(Texture::BtnCheckAnswer.path())),
                //         ..Default::default()
                //     },
                //     PuzzleButtonAction::CheckAnswer,
                // ));
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

        // Create map of nodes by id to the total rules using that node as reference, in case of overlapping sets by top left corner
        let mut node_to_rule_count_map: HashMap<GameNodeId, u8> = HashMap::new();

        // Map tile spaces to sets to generate
        let puzzle_sets = puzzle.clone().sets;
        for set_idx in 0..puzzle_sets.len() {
            let set = &puzzle_sets[set_idx];
            let set_tiles = get_set_tiles(set, &puzzle, asset_server.clone());
            let mut set_sprite_entity_ids: Vec<Entity> = vec![];
            for set_tile in set_tiles.clone() {
                set_sprite_entity_ids.push(commands.spawn(set_tile).insert(OnPuzzleScene).id());
            }

            // Add the set rule sprites in the upper left-most corner of the set
            let upper_left_node = get_set_upper_left_node(set, &puzzle);

            let (node_x, node_y) = node_to_position(&upper_left_node, &puzzle);

            let mut active_set_rules: Vec<ActiveSetRule> = vec![];

            // Update total rule count for sets using this as upper left node
            let mut total_rule_idx: u8 = 0;
            if let Some(existing_offset) = node_to_rule_count_map.get(&upper_left_node) {
                total_rule_idx += existing_offset;
            }

            for rule in set.rules.iter() {
                // TODO get textures via either set_rule.rs or texture.rs
                let rule_texture = match rule {
                    SetRule::Disconnected => tex_rule_disconnected.clone(),
                    SetRule::Leaf => tex_rule_leaf.clone(),
                };

                let transform_x = node_x - TILE_NODE_SPRITE_SIZE + INTERNAL_SPACING_X;
                let transform_y = node_y + TILE_NODE_SPRITE_SIZE
                    - INTERNAL_SPACING_Y
                    - total_rule_idx as f32 * (CDTN_RULE_SPRITE_SIZE + STACK_CDTN_RULE_SPACING);
                commands.spawn(SpriteBundle {
                    texture: tex_rule_box.clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(CDTN_RULE_SPRITE_SIZE, CDTN_RULE_SPRITE_SIZE)),
                        color: get_color_for_set_tile(set.clone(), puzzle_sets.clone()), // TODO assign colors to sets and match
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(transform_x, transform_y, Z_SET_RULE_BOX),
                    ..Default::default()
                }).insert(OnPuzzleScene);
                let rule_sprite = SpriteBundle {
                    texture: rule_texture,
                    sprite: rule.sprite().clone(),
                    transform: Transform::from_xyz(transform_x, transform_y, Z_RULE_CDTN_NODE),
                    ..Default::default()
                };
                let rule_sprite_id = commands
                    .spawn(rule_sprite.clone())
                    .insert(OnPuzzleScene)
                    .id();

                active_set_rules.push(ActiveSetRule {
                    active_id: ActiveIdentifier::new(),
                    sprite: rule_sprite,
                    sprite_entity_id: rule_sprite_id,
                    satisfied: false,
                    rule: rule.clone(),
                });

                total_rule_idx += 1;
            }

            let mut active_connected_set_rules: Vec<ActiveConnectedSetRule> = vec![];

            for crule in set.connected_rules.iter() {
                // TODO get textures via either connected_set_rule.rs or texture.rs
                let crule_texture = match crule {
                    ConnectedSetRule::Homomorphic(_crule) => tex_rule_homomorphism.clone(),
                };

                let transform_x = node_x - TILE_NODE_SPRITE_SIZE + INTERNAL_SPACING_X;
                let transform_y = node_y + TILE_NODE_SPRITE_SIZE
                    - INTERNAL_SPACING_Y
                    - total_rule_idx as f32 * (CDTN_RULE_SPRITE_SIZE + STACK_CDTN_RULE_SPACING);
                commands.spawn(SpriteBundle {
                    texture: tex_rule_box.clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(CDTN_RULE_SPRITE_SIZE, CDTN_RULE_SPRITE_SIZE)),
                        color: Color::BLACK, // TODO assign colors to sets and match
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(transform_x, transform_y, Z_SET_RULE_BOX),
                    ..Default::default()
                });
                let crule_sprite = SpriteBundle {
                    texture: crule_texture,
                    sprite: crule.sprite().clone(),
                    transform: Transform::from_xyz(transform_x, transform_y, Z_RULE_CDTN_NODE),
                    ..Default::default()
                };
                let crule_sprite_id = commands
                    .spawn(crule_sprite.clone())
                    .insert(OnPuzzleScene)
                    .id();

                active_connected_set_rules.push(ActiveConnectedSetRule {
                    active_id: ActiveIdentifier::new(),
                    rule: crule.clone(),
                    sprite: crule_sprite,
                    sprite_entity_id: crule_sprite_id,
                    satisfied: false,
                });

                total_rule_idx += 1;
            }

            // Update total rules using this upper left node as reference
            node_to_rule_count_map.insert(upper_left_node, total_rule_idx);

            active_sets.active_sets.push(ActiveSet {
                set: puzzle_sets[set_idx].clone(),
                satisfied: false,
                active_id: ActiveIdentifier::new(),
                active_set_rules: active_set_rules,
                active_connected_set_rules: active_connected_set_rules,
                sprites: set_tiles,
                sprite_entity_ids: set_sprite_entity_ids,
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
        active_sets: Res<ActiveSets>,
        mut current_line: ResMut<CurrentLine>,
        mut active_lines: ResMut<ActiveLines>,
        mouse_button_input: Res<ButtonInput<MouseButton>>,
        q_window: Query<&Window, With<PrimaryWindow>>,
        q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
        mut event_writer: EventWriter<UpdateSatisfiedStates>,
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

            active_nodes
                .active_nodes
                .iter_mut()
                .for_each(|active_node| {
                    if active_node.node.id == current_line.start_node_id.unwrap() {
                        opt_start_node = Some(active_node);
                    } else if clicked_on_sprite(&active_node.sprite, world_position) {
                        opt_end_node = Some(active_node);
                    }
                });

            if opt_start_node.is_none() || opt_end_node.is_none() {
                return;
            }

            let start_node = opt_start_node.unwrap();
            let end_node = opt_end_node.unwrap();

            // If start node and end node both have each other as connection, remove the line. 
            if start_node.connections.contains(&end_node.node.id) && end_node.connections.contains(&start_node.node.id) {
                if let Some(pos) = start_node.connections.iter().position(|node_id| *node_id == end_node.node.id) {
                    start_node.connections.remove(pos);
                }
                if let Some(pos) = end_node.connections.iter().position(|node_id| *node_id == start_node.node.id) {
                    end_node.connections.remove(pos);
                }
                let first_node = if start_node.node.id < end_node.node.id { start_node.clone() } else { end_node.clone() };
                let second_node = if start_node.node.id < end_node.node.id { end_node.clone() } else { start_node.clone() };
                for idx in 0..active_lines.lines.len() {
                    if active_lines.lines[idx].start_node.node.id == first_node.node.id &&
                        active_lines.lines[idx].end_node.node.id == second_node.node.id {
                            commands.entity(active_lines.lines[idx].sprite_entity_id).despawn();
                            active_lines.lines.remove(idx);
                            break;
                        }
                }
                
            // Otherwise, add a new line
            } else {
                let start_pos = start_node.sprite.transform.translation.truncate();
                let end_pos = end_node.sprite.transform.translation.truncate();

                // Get the appropriate line texture, if exists (otherwise invalid node pair)
                let line_texture = get_line_texture(start_node, end_node).unwrap_or(&Texture::Missing);

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
                        Z_LINE,
                    ),
                    ..Default::default()
                };

                // Update connections of both start and end node
                start_node.connections.push(end_node.node.id);
                end_node.connections.push(start_node.node.id);

                // Add line to the screen
                let line_entity_id = commands
                    .spawn(line_sprite.clone())
                    .insert(OnPuzzleScene)
                    .id();

                // Update list of lines, putting the smallest ID first.
                let first_node = if start_node.node.id < end_node.node.id { start_node.clone() } else { end_node.clone() };
                let second_node = if start_node.node.id < end_node.node.id { end_node.clone() } else { start_node.clone() };
                active_lines.lines.push(ActiveLine {
                    start_node: first_node.clone(),
                    end_node: second_node.clone(),
                    sprite: line_sprite.clone(),
                    active_id: ActiveIdentifier::new(),
                    sprite_entity_id: line_entity_id,
                });
            }

            // Regardless if we ended on a node or not, clear the current line
            current_line.start_node_id = None;

            // Update satisfied states given the current start and end node
            let satisfied_states =
                get_all_satisfied_states(&active_nodes.active_nodes, &active_sets.active_sets);

            // Send an event to update all the relevant states visually
            event_writer.send(UpdateSatisfiedStates(satisfied_states));
        }
    }

    /// A system for updating all (relevant) satisfiable sprites on screen when an UpdateSatisfiedStates event is sent.
    fn update_satisfied_states_ui(
        mut event_reader: EventReader<UpdateSatisfiedStates>,
        mut active_nodes: ResMut<ActiveNodes>,
        mut active_sets: ResMut<ActiveSets>,
        mut q_sprites: Query<&mut Sprite>,
    ) {
        let mut processed_update = false;
        for UpdateSatisfiedStates(satisfied_states) in event_reader.read() {
            processed_update = true;
            // Update visual state of all active nodes and conditions
            active_nodes
                .active_nodes
                .iter_mut()
                .for_each(|active_node| {
                    if satisfied_states.contains_key(&active_node.active_id) {
                        active_node.set_satisfied(satisfied_states[&active_node.active_id]);
                        if let Ok(mut sprite) = q_sprites.get_mut(active_node.sprite_entity_id) {
                            active_node.update_sprites(Vec::from([sprite.as_mut()]));
                        }
                    }
                    active_node
                        .active_conditions
                        .iter_mut()
                        .for_each(|active_condition| {
                            if satisfied_states.contains_key(&active_condition.active_id) {
                                active_condition
                                    .set_satisfied(satisfied_states[&active_condition.active_id]);
                                if let Ok(mut sprite) =
                                    q_sprites.get_mut(active_condition.sprite_entity_id)
                                {
                                    active_condition.update_sprites(Vec::from([sprite.as_mut()]));
                                }
                            }
                        });
                    active_node.active_connected_conditions.iter_mut().for_each(
                        |active_connected_condition| {
                            if satisfied_states.contains_key(&active_connected_condition.active_id)
                            {
                                active_connected_condition.set_satisfied(
                                    satisfied_states[&active_connected_condition.active_id],
                                );
                                if let Ok(mut sprite) =
                                    q_sprites.get_mut(active_connected_condition.sprite_entity_id)
                                {
                                    active_connected_condition.update_sprites(Vec::from([sprite.as_mut()]));
                                }
                            }
                        },
                    );
                });

            // Update visual state of all active set rules
            active_sets.active_sets.iter_mut().for_each(|active_set| {
                active_set
                    .active_set_rules
                    .iter_mut()
                    .for_each(|active_set_rule| {
                        if satisfied_states.contains_key(&active_set_rule.active_id) {
                            active_set_rule
                                .set_satisfied(satisfied_states[&active_set_rule.active_id]);
                            if let Ok(mut sprite) =
                                q_sprites.get_mut(active_set_rule.sprite_entity_id)
                            {
                                active_set_rule.update_sprites(Vec::from([sprite.as_mut()]));
                            }
                        }
                    });
                active_set.active_connected_set_rules.iter_mut().for_each(
                    |active_connected_set_rule| {
                        if satisfied_states.contains_key(&active_connected_set_rule.active_id) {
                            active_connected_set_rule.set_satisfied(
                                satisfied_states[&active_connected_set_rule.active_id],
                            );
                            if let Ok(mut sprite) =
                                q_sprites.get_mut(active_connected_set_rule.sprite_entity_id)
                            {
                                active_connected_set_rule.update_sprites(Vec::from([sprite.as_mut()]));
                            }
                        }
                    },
                );
            });
        }

        // If we processed any updates, then check if puzzle was solved
        // TODO move this to another function
        if processed_update {
            let mut solved = true;
            for active_node in active_nodes.active_nodes.iter() {
                if !active_node.satisfied {
                    solved = false;
                }
                for condition in &active_node.active_conditions {
                    if !condition.satisfied {
                        solved = false;
                    }
                }
                for connected_condition in &active_node.active_connected_conditions {
                    if !connected_condition.satisfied {
                        solved = false;
                    }
                }
            }
            for active_set in active_sets.active_sets.iter() {
                for active_set_rule in &active_set.active_set_rules {
                    if !active_set_rule.satisfied {
                        solved = false;
                    }
                }
                for active_connected_set_rule in &active_set.active_connected_set_rules {
                    if !active_connected_set_rule.satisfied {
                        solved = false;
                    }
                }
            }

            println!("Solved: {}", solved);
        }
    }

    fn ui_action(
        mut commands: Commands,
        interaction_query: Query<
            (&Interaction, &PuzzleButtonAction),
            (Changed<Interaction>, With<Button>),
        >,
        mut active_nodes: ResMut<ActiveNodes>,
        active_sets: Res<ActiveSets>,
        mut active_lines: ResMut<ActiveLines>,
        _q_sprites: Query<&mut Sprite>,
        mut app_state: ResMut<NextState<AppState>>,
        mut event_writer: EventWriter<UpdateSatisfiedStates>,
        api: Res<NodalApi>,
    ) {
        for (interaction, ui_button_action) in &interaction_query {
            if *interaction == Interaction::Pressed {
                match ui_button_action {
                    PuzzleButtonAction::Reset => {
                        active_nodes.active_nodes.iter_mut().for_each(|node| {
                            node.connections.clear();
                        });
                        active_lines.lines.iter_mut().for_each(|active_line| {
                            commands.entity(active_line.sprite_entity_id).despawn();
                        });
                        active_lines.lines.clear();
                        event_writer.send(UpdateSatisfiedStates(get_all_satisfied_states(
                            &active_nodes.active_nodes,
                            &active_sets.active_sets,
                        )));

                        // TODO remove this once not needed as a reference 
                        let rt = Runtime::new().unwrap();
                        rt.block_on(async {
                            match api.call_test_async().await {
                                Ok(response) => {
                                    println!("async test api got response: {}", response.response);
                                }
                                Err(err) => {
                                    error!("Error calling async test api: {}", err);
                                }
                            }
                        });

                        // match api.call_test_blocking() {
                        //     Ok(response) => {
                        //         println!("blocking test api got response: {}", response.response);
                        //     }
                        //     Err(err) => {
                        //         error!("Error calling blocking test api: {}", err);
                        //     }
                        // }
                    }
                    PuzzleButtonAction::ReturnToPreviousPage => {
                        // TODO track previous state before entering puzzle (campaign vs public level select)
                        app_state.set(AppState::Campaign);
                    }
                }
            }
        }
    }
}
