pub mod campaign {
    use bevy::{
        app::{App, Update},
        asset::AssetServer,
        ecs::{
            query::{Changed, With},
            system::{Commands, Query, Res, ResMut},
        },
        hierarchy::BuildChildren,
        input::{mouse::MouseButton, ButtonInput},
        math::Vec2,
        prelude::{Component, IntoSystemConfigs, Resource},
        render::camera::OrthographicProjection,
        sprite::{Sprite, SpriteBundle},
        state::{
            condition::in_state,
            state::{NextState, OnEnter, OnExit},
        },
        transform::components::Transform,
        ui::{
            node_bundles::{ButtonBundle, NodeBundle},
            widget::Button,
            AlignItems, Interaction, JustifyContent, Style, UiImage, Val,
        },
        utils::default,
        window::{PrimaryWindow, Window},
    };
    use serde::Deserialize;
    use uuid::Uuid;

    use crate::{
        buttons::icon_button_style,
        despawn_screen, is_mouse_over_sprite,
        logic::save_data_manager::save_data_manager::is_solved,
        puzzle_manager::PuzzleManager,
        scenes::campaign::util::{get_campaign_puzzle_position, is_unlocked, update_camera},
        texture::Texture,
        AppState, MainCamera, MousePosition, SelectedPuzzle, COLOR_CAMPAIGN_PUZZLE_LOCKED,
        COLOR_CAMPAIGN_PUZZLE_SOLVED, COLOR_CAMPAIGN_PUZZLE_UNLOCKED, TILE_NODE_SPRITE_SIZE,
    };

    // This plugin will contain a campaign (for now, just the main campaign).
    pub fn campaign_plugin(app: &mut App) {
        app.add_systems(OnEnter(AppState::Campaign), campaign_setup)
            .add_systems(
                OnExit(AppState::Campaign),
                despawn_screen::<OnCampaignScene>,
            )
            .add_systems(OnExit(AppState::Campaign), despawn_screen::<OnCampaignUI>)
            .add_systems(Update, ui_action.run_if(in_state(AppState::Campaign)))
            .add_systems(
                Update,
                puzzle_select_system.run_if(in_state(AppState::Campaign)),
            )
            .insert_resource(ClickableCampaignPuzzles::default());
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct Campaign {
        pub uuid: Uuid,
        pub name: String,
        pub width: usize,
        pub height: usize,
        pub puzzle_layout: Vec<CampaignPuzzle>,
    }

    #[derive(Default, Resource, Component)]
    struct ClickableCampaignPuzzles {
        clickable_campaign_puzzles: Vec<ClickableCampaignPuzzle>,
    }

    #[derive(Component)]
    pub struct ClickableCampaignPuzzle {
        pub campaign_puzzle: CampaignPuzzle,
        sprite: SpriteBundle,
        pub unlocked: bool,
    }

    #[derive(Deserialize, Debug, Clone, Copy)]
    pub struct CampaignPuzzle {
        pub puzzle_uuid: Uuid,
        pub pos_x: usize,
        pub pos_y: usize,
    }

    // Tag component used to tag entities added on the campaign scene
    #[derive(Component)]
    struct OnCampaignScene;

    // Tag component used to tag entities added on the UI of the campaign screen
    #[derive(Component)]
    struct OnCampaignUI;

    // All actions that can be triggered from a button click
    #[derive(Component)]
    enum CampaignButtonAction {
        ReturnToSubMenu,
    }

    fn campaign_setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut puzzle_manager: ResMut<PuzzleManager>,
        mut clickable_campaign_puzzles: ResMut<ClickableCampaignPuzzles>,
        q_window: Query<&Window, With<PrimaryWindow>>,
        mut q_camera: Query<(&mut Transform, &mut OrthographicProjection), With<MainCamera>>,
    ) {
        // TODO move all the setup to when game is loaded for performance increase
        // Get the campaign by loading it directly
        let campaign = serde_json::from_str::<Campaign>(include_str!(
            "../../../assets/campaign/campaign.json"
        ))
        .unwrap();

        // Populate the puzzle manager with the campaign puzzles
        let _ = puzzle_manager.populate_campaign();

        // Get the texture that will be used for the puzzle images
        // TODO use actual image
        let puzzle_tex = asset_server.load(Texture::Node.path());

        // Create grid of puzzle layout
        let mut puzzle_grid: Vec<Vec<Uuid>> = (0..campaign.height)
            .map(|_| vec![Uuid::nil(); campaign.width.into()])
            .collect();
        for campaign_puzzle in campaign.clone().puzzle_layout {
            puzzle_grid[campaign_puzzle.pos_y as usize][campaign_puzzle.pos_x as usize] =
                campaign_puzzle.puzzle_uuid;
        }

        for campaign_puzzle in campaign.clone().puzzle_layout {
            // Check if puzzle is completed
            let solved = is_solved(campaign_puzzle.puzzle_uuid.clone());

            // Check if puzzle is unlocked
            let unlocked = is_unlocked(puzzle_grid.clone(), campaign_puzzle);

            // Add the puzzle as a sprite
            let pos = get_campaign_puzzle_position(campaign_puzzle);
            let sprite = SpriteBundle {
                texture: puzzle_tex.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(TILE_NODE_SPRITE_SIZE, TILE_NODE_SPRITE_SIZE)),
                    color: if solved {
                        COLOR_CAMPAIGN_PUZZLE_SOLVED
                    } else if unlocked {
                        COLOR_CAMPAIGN_PUZZLE_UNLOCKED
                    } else {
                        COLOR_CAMPAIGN_PUZZLE_LOCKED
                    },
                    ..default()
                },
                transform: Transform::from_xyz(pos.x as f32, pos.y as f32, 0.),
                ..default()
            };
            commands.spawn((sprite.clone(), OnCampaignScene));

            clickable_campaign_puzzles
                .clickable_campaign_puzzles
                .push(ClickableCampaignPuzzle {
                    campaign_puzzle,
                    sprite,
                    unlocked: unlocked || solved,
                });
        }

        for (mut transform, mut projection) in q_camera.iter_mut() {
            update_camera(
                q_window.single(),
                &mut transform,
                &mut projection,
                &clickable_campaign_puzzles.clickable_campaign_puzzles,
            );
        }

        // Add a back button
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
                OnCampaignUI,
            ))
            .with_children(|parent| {
                parent.spawn((
                    ButtonBundle {
                        style: icon_button_style(),
                        image: UiImage::new(asset_server.load(Texture::BtnGoBack.path())),
                        ..Default::default()
                    },
                    CampaignButtonAction::ReturnToSubMenu,
                ));
            });
    }

    fn puzzle_select_system(
        mouse_position: Res<MousePosition>,
        clickable_campaign_puzzles: Res<ClickableCampaignPuzzles>,
        mut app_state: ResMut<NextState<AppState>>,
        mut selected_puzzle: ResMut<SelectedPuzzle>,
        mouse_button_input: Res<ButtonInput<MouseButton>>,
    ) {
        // On left click, either enter the puzzle if available, or reject if locked
        if mouse_button_input.just_pressed(MouseButton::Left) {
            for clickable_campaign_puzzle in
                clickable_campaign_puzzles.clickable_campaign_puzzles.iter()
            {
                if is_mouse_over_sprite(
                    &clickable_campaign_puzzle.sprite.sprite,
                    clickable_campaign_puzzle.sprite.transform,
                    mouse_position.position,
                ) {
                    if clickable_campaign_puzzle.unlocked {
                        selected_puzzle.uuid =
                            clickable_campaign_puzzle.campaign_puzzle.puzzle_uuid;
                        app_state.set(AppState::Puzzle);
                    }
                }
            }
        }
    }

    fn ui_action(
        interaction_query: Query<
            (&Interaction, &CampaignButtonAction),
            (Changed<Interaction>, With<Button>),
        >,
        mut app_state: ResMut<NextState<AppState>>,
    ) {
        for (interaction, ui_button_action) in &interaction_query {
            if *interaction == Interaction::Pressed {
                match ui_button_action {
                    CampaignButtonAction::ReturnToSubMenu => {
                        app_state.set(AppState::Menu);
                    }
                }
            }
        }
    }
}
