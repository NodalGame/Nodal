pub mod puzzle {
    use std::borrow::{Borrow, BorrowMut};
    use std::f32::consts::PI;
    use std::process::exit;

    use crate::buttons::buttons::icon_button_style;
    use crate::game_node::game_node::{GameNode, NodeClass};
    use crate::game_set::game_set::GameSet;
    use crate::node_condition::node_condition::NodeCondition;
    use crate::{texture::texture::Texture, MainCamera};
    use bevy::prelude::*;
    use bevy::window::PrimaryWindow;
    use serde::Deserialize;
    use std::collections::{HashMap, HashSet, VecDeque};
    use std::hash::{Hash, Hasher};
    use uuid::Uuid;

    use crate::{
        despawn_screen, puzzle_manager::puzzle_manager::PuzzleManager, AppState, SelectedPuzzle,
    };

    // This plugin will contain a playable puzzle.
    pub fn puzzle_plugin(app: &mut App) {
        app.add_systems(OnEnter(AppState::Puzzle), puzzle_setup)
            .add_systems(OnExit(AppState::Puzzle), despawn_screen::<OnPuzzleScene>)
            .add_systems(Update, line_system.run_if(in_state(AppState::Puzzle)))
            .add_systems(Update, ui_action.run_if(in_state(AppState::Puzzle)))
            .insert_resource(ActiveNodes::default())
            .insert_resource(ActiveLines::default())
            .insert_resource(CurrentLine::default());
    }

    #[derive(Deserialize, Debug)]
    pub struct Puzzle {
        pub uuid: Uuid,
        width: u8,
        height: u8,
        nodes: Vec<GameNode>,
        sets: Vec<GameSet>,
    }

    // Tracks all nodes in the puzzle, including sprite and position
    #[derive(Default, Resource, Component, Clone)]
    struct ActiveNodes {
        active_nodes: Vec<ActiveNode>,
    }

    // TODO ActiveNode should get its own file/module
    #[derive(Component, Clone)]
    pub struct ActiveNode {
        pub node: GameNode,
        pub connections: Vec<u16>,
        sprite: SpriteBundle,
    }

    impl ActiveNode {
        pub fn class_connection_pass(&self, active_nodes: Vec<&ActiveNode>) -> bool {
            if self.node.conditions.contains(&NodeCondition::Universal) {
                println!("Node is universal, passes check.");
                return true;
            }

            for connection in &self.connections {
                let connected_node = active_nodes.iter().find(|node| node.node.id == *connection);
                if connected_node.is_none() {
                    println!("Could not find connected node with id {}", connection);
                    return false;
                }
                if connected_node.unwrap().node.class != self.node.class
                    && !connected_node
                        .unwrap()
                        .node
                        .conditions
                        .contains(&NodeCondition::Universal)
                {
                    println!("Connected node is not of same class and isn't universal, fails check.");
                    return false;
                }
            }

            return true;
        }

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
        ReturnToMenu,
    }

    const TILE_NODE_SPRITE_SIZE: f32 = 100.0;
    const CDTN_RULE_SPRITE_SIZE: f32 = 45.0;
    const INTERNAL_SPACING_X: f32 = 25.0;
    const INTERNAL_SPACING_Y: f32 = 25.0;
    const STACK_CDTN_RULE_SPACING: f32 = 10.0;
    const SPRITE_SPACING: f32 = 100.0;

    fn puzzle_setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        puzzle_id: Res<SelectedPuzzle>,
        puzzle_manager: Res<PuzzleManager>,
        mut active_nodes: ResMut<ActiveNodes>,
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

        // Sort nodes by id (top to bottom, left to right)
        let mut ordered_nodes = puzzle.nodes.clone();
        ordered_nodes.sort_by(|a, b| a.id.cmp(&b.id));

        // TODO load only when needed, then cache in map to re-access in the screen spawning loop
        // Load node textures
        let tex_node_red = asset_server.load(Texture::ClassRed.path());
        let tex_node_blue = asset_server.load(Texture::ClassBlue.path());
        let tex_node_yellow = asset_server.load(Texture::ClassYellow.path());

        // Load condition textures
        let tex_cdtn_branch_equal = asset_server.load(Texture::CdtnBranchEqual.path());
        let tex_cdtn_leaf = asset_server.load(Texture::CdtnLeaf.path());
        let tex_cdtn_linked = asset_server.load(Texture::CdtnLinked.path());
        let tex_cdtn_universal = asset_server.load(Texture::CdtnUniversal.path());

        // Create a width x height grid of nodes as sprite bundles, accounting for background tiles
        for x in 0..puzzle.width * 2 + 1 {
            for y in 0..puzzle.height * 2 + 1 {
                // If background tile, spawn it and continue
                if x % 2 == 0 || y % 2 == 0 {
                    commands.spawn(get_bg_tile(
                        x,
                        y,
                        puzzle.width,
                        puzzle.height,
                        asset_server.clone(),
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

                let node_texture = match node.class {
                    NodeClass::Red => tex_node_red.clone(),
                    NodeClass::Blue => tex_node_blue.clone(),
                    NodeClass::Yellow => tex_node_yellow.clone(),
                    _ => {
                        println!("Error when adding nodes to screen, invalid class?");
                        exit(1);
                    }
                };

                let node_sprite = SpriteBundle {
                    texture: node_texture,
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(TILE_NODE_SPRITE_SIZE, TILE_NODE_SPRITE_SIZE)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(node_x, node_y, 0.0),
                    ..default()
                };
                commands.spawn(node_sprite.clone()).insert(OnPuzzleScene);

                // Spawn its conditions on the screen
                for (cdtn_idx, condition) in node.conditions.iter().enumerate() {
                    // TODO get textures via either node_condition.rs or texture.rs
                    let condition_texture = match condition {
                        NodeCondition::BranchEqual => tex_cdtn_branch_equal.clone(),
                        NodeCondition::Leaf => tex_cdtn_leaf.clone(),
                        NodeCondition::Linked => tex_cdtn_linked.clone(),
                        NodeCondition::Universal => tex_cdtn_universal.clone(),
                        _ => {
                            println!("Error when adding nodes to screen, invalid condition?");
                            exit(1);
                        }
                    };

                    let condition_sprite = SpriteBundle {
                        texture: condition_texture,
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(
                                CDTN_RULE_SPRITE_SIZE,
                                CDTN_RULE_SPRITE_SIZE,
                            )),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(
                            node_x + TILE_NODE_SPRITE_SIZE - INTERNAL_SPACING_X,
                            node_y + TILE_NODE_SPRITE_SIZE
                                - INTERNAL_SPACING_Y
                                - cdtn_idx as f32
                                    * (CDTN_RULE_SPRITE_SIZE + STACK_CDTN_RULE_SPACING),
                            0.0,
                        ),
                        ..default()
                    };
                    commands
                        .spawn(condition_sprite.clone())
                        .insert(OnPuzzleScene);
                }

                active_nodes.active_nodes.push(ActiveNode {
                    node: node.clone(),
                    connections: Vec::new(),
                    sprite: node_sprite.clone(),
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
                ..default()
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
                        ..default()
                    },
                    ..default()
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
                    PuzzleButtonAction::ReturnToMenu,
                ));
            });
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
        let world_position = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
            .unwrap_or_else(|| {
                // TODO fix this it causes game to crash when you hover out of it
                println!("Failed to get cursor position");
                exit(1);
            });

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
                ..default()
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

    fn clicked_on_sprite(sprite: &SpriteBundle, cursor: Vec2) -> bool {
        let node_pos = sprite.transform.translation.truncate();
        let distance = cursor.distance(node_pos);
        // Assuming the sprite size is a good proxy for click detection radius
        if distance
            < sprite
                .sprite
                .custom_size
                .unwrap_or_else(|| {
                    // TODO fix this
                    println!("Failed to get sprite size");
                    exit(1);
                })
                .x
                / 2.0
        {
            return true;
        }
        false
    }

    fn ui_action(
        interaction_query: Query<
            (&Interaction, &PuzzleButtonAction),
            (Changed<Interaction>, With<Button>),
        >,
        active_nodes: Res<ActiveNodes>,
    ) {
        for (interaction, ui_button_action) in &interaction_query {
            if *interaction == Interaction::Pressed {
                match ui_button_action {
                    PuzzleButtonAction::CheckAnswer => {
                        let solved = check_answer(&active_nodes);
                        println!("Puzzle solved: {}", solved);
                    }
                    PuzzleButtonAction::Reset => {
                        println!("Clear lines button pressed");
                    }
                    PuzzleButtonAction::ReturnToMenu => {
                        println!("Go back button pressed");
                    }
                }
            }
        }
    }

    /// Returns a background tile as a sprite bundle.
    ///
    /// # Parameters
    ///
    /// - `x_pos`: The x position of the tile in the grid (0..width*2+1)
    /// - `y_pos`: The y position of the tile in the grid (0..height*2+1)
    /// - `width`: The width of the puzzle.
    /// - `height`: The height of the puzzle.
    /// - `sprite_size`: The size of the sprite in pixels.
    /// - `spacing`: The spacing between each sprite in the grid.
    ///
    /// # Returns
    ///
    /// A sprite bundle representing the background tile at the given position.
    fn get_bg_tile(x: u8, y: u8, width: u8, height: u8, asset_server: AssetServer) -> SpriteBundle {
        // Load background textures
        let bg_side_left: Handle<Image> = asset_server.load(Texture::BgTileSideLeft.path());
        let bg_side_right: Handle<Image> = asset_server.load(Texture::BgTileSideRight.path());
        let bg_side_bottom: Handle<Image> = asset_server.load(Texture::BgTileSideBottom.path());
        let bg_side_top: Handle<Image> = asset_server.load(Texture::BgTileSideTop.path());
        let bg_bottom_left: Handle<Image> = asset_server.load(Texture::BgTileBottomLeft.path());
        let bg_bottom_right: Handle<Image> = asset_server.load(Texture::BgTileBottomRight.path());
        let bg_top_left: Handle<Image> = asset_server.load(Texture::BgTileTopLeft.path());
        let bg_top_right: Handle<Image> = asset_server.load(Texture::BgTileTopRight.path());
        let bg_between_horizontal: Handle<Image> =
            asset_server.load(Texture::BgTileBetweenHorizontal.path());
        let bg_between_vertical: Handle<Image> =
            asset_server.load(Texture::BgTileBetweenVertical.path());
        let bg_between_cross: Handle<Image> = asset_server.load(Texture::BgTileBetweenCross.path());

        let transform =
            Transform::from_xyz(x as f32 * SPRITE_SPACING, y as f32 * SPRITE_SPACING, 0.0);
        let sprite = Sprite {
            custom_size: Some(Vec2::new(TILE_NODE_SPRITE_SIZE, TILE_NODE_SPRITE_SIZE)),
            ..Default::default()
        };

        // Bottom left corner
        if x == 0 && y == 0 {
            SpriteBundle {
                texture: bg_bottom_left,
                sprite,
                transform,
                ..default()
            }
        } else if x == 0 {
            // Top left corner
            if y == height * 2 {
                SpriteBundle {
                    texture: bg_top_left,
                    sprite,
                    transform,
                    ..default()
                }
            } else {
                // Left side
                SpriteBundle {
                    texture: bg_side_left,
                    sprite,
                    transform,
                    ..default()
                }
            }
        } else if y == 0 {
            // Bottom right corner
            if x == width * 2 {
                SpriteBundle {
                    texture: bg_bottom_right,
                    sprite,
                    transform,
                    ..default()
                }
            } else {
                // Bottom side
                SpriteBundle {
                    texture: bg_side_bottom,
                    sprite,
                    transform,
                    ..default()
                }
            }
        } else {
            // Top right corner
            if x == width * 2 && y == height * 2 {
                SpriteBundle {
                    texture: bg_top_right,
                    sprite,
                    transform,
                    ..default()
                }
            } else if x == width * 2 {
                // Right side
                SpriteBundle {
                    texture: bg_side_right,
                    sprite,
                    transform,
                    ..default()
                }
            } else if y == height * 2 {
                // Top side
                SpriteBundle {
                    texture: bg_side_top,
                    sprite,
                    transform,
                    ..default()
                }
            } else {
                if x % 2 == 0 && y % 2 == 0 {
                    // Between cross
                    SpriteBundle {
                        texture: bg_between_cross,
                        sprite,
                        transform,
                        ..default()
                    }
                } else if x % 2 == 0 {
                    // Between horizontal
                    SpriteBundle {
                        texture: bg_between_horizontal,
                        sprite,
                        transform,
                        ..default()
                    }
                } else {
                    // Between vertical
                    SpriteBundle {
                        texture: bg_between_vertical,
                        sprite,
                        transform,
                        ..default()
                    }
                }
            }
        }
    }

    fn get_line_texture(start_node: ActiveNode, end_node: ActiveNode) -> Option<&'static Texture> {
        let start_pos = start_node.sprite.transform.translation.truncate();
        let end_pos = end_node.sprite.transform.translation.truncate();
        let direction = end_pos - start_pos;
        let distance = direction.length();
        let angle = direction.y.atan2(direction.x);

        // Determine if line is valid connection between adjacent nodes
        if distance > SPRITE_SPACING + TILE_NODE_SPRITE_SIZE && (angle == 0.0 || angle == PI / 2.0)
        {
            return None;
        } else if distance > (2.0 * (SPRITE_SPACING + TILE_NODE_SPRITE_SIZE).powi(2)).sqrt() {
            return None;
        }

        if angle.abs() == 0.0 || angle.abs() == PI {
            Some(&Texture::LineHorizontal)
        } else if angle.abs() == PI / 2.0 {
            Some(&Texture::LineVertical)
        } else if angle == PI / 4.0 || angle == -3.0 * PI / 4.0 {
            Some(&Texture::LineDiagonalBottomLeftTopRight)
        } else {
            Some(&Texture::LineDiagonalTopLeftBottomRight)
        }
    }

    /// Checks if the puzzle is solved.
    fn check_answer(active_nodes: &Res<ActiveNodes>) -> bool {
        // First verify that every node of the same class is connected to each other
        let mut nodes_by_class: HashMap<NodeClass, Vec<ActiveNode>> = HashMap::new();
        for node in active_nodes.active_nodes.iter() {
            nodes_by_class
                .entry(node.node.class.clone())
                .or_default()
                .push(node.clone());
        }

        // Check connectivity of each node class
        for (class, nodes_in_class) in nodes_by_class {
            let mut visited: HashSet<u16> = HashSet::new();
            let mut queue: VecDeque<u16> = VecDeque::new();

            if nodes_in_class.is_empty() {
                continue;
            }

            queue.push_back(nodes_in_class.get(0).unwrap().node.id);
            visited.insert(nodes_in_class.get(0).unwrap().node.id);

            while queue.len() > 0 {
                let curr_node_id = queue.pop_front().unwrap_or_else(|| {
                    // TODO handle this better
                    exit(1);
                });
                let curr_node = active_nodes
                    .active_nodes
                    .iter()
                    .find(|node| node.node.id == curr_node_id)
                    .unwrap();
                for connection in curr_node.connections.iter() {
                    if !visited.contains(connection) {
                        visited.insert(*connection);
                        queue.push_back(*connection);
                    }
                }
            }

            // Check that all nodes in the class are connected to each other
            for node in &nodes_in_class {
                if !visited
                    .iter()
                    .any(|visited_node| *visited_node == node.node.id)
                {
                    return false;
                }
            }
        }

        let active_node_refs: Vec<&ActiveNode> = active_nodes.active_nodes.iter().collect();
        for node in &active_node_refs {
            // Check that the node is immediately connected to only nodes of same class (unless it is universal)
            if !node.class_connection_pass(active_node_refs.clone()) {
                println!("Node connected to nodes which aren't of same class");
                return false;
            }

            // Check failed node conditions
            let failed_conditions = node.get_failed_conditions(active_node_refs.clone());
            if !failed_conditions.is_empty() {
                println!("Node failed conditions: {:?}", failed_conditions);
                return false;
            }
        }
        return true;
    }
}
