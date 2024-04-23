pub mod campaign {
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
        render::{camera::Camera, color::Color},
        sprite::{Sprite, SpriteBundle},
        transform::components::{GlobalTransform, Transform},
        ui::{
            node_bundles::{ButtonBundle, ImageBundle, NodeBundle},
            widget::Button,
            AlignItems, FlexDirection, Interaction, JustifyContent, Style, UiImage, Val,
        },
        utils::default,
        window::{PrimaryWindow, Window},
    };
    use serde::Deserialize;
    use uuid::Uuid;

    use crate::{
        buttons::buttons::{
            button_icon_style, icon_button_style, text_button_style, NORMAL_BUTTON,
        },
        constants::{SPRITE_SPACING, TILE_NODE_SPRITE_SIZE},
        despawn_screen,
        puzzle_manager::{self, puzzle_manager::PuzzleManager},
        texture::texture::Texture,
        util::{clicked_on_sprite, get_cursor_world_position},
        AppState, MainCamera, SelectedPuzzle,
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

    #[derive(Deserialize, Debug)]
    pub struct Campaign {
        pub uuid: Uuid,
        pub name: String,
        pub puzzle_layout: Vec<CampaignPuzzle>,
    }

    #[derive(Default, Resource, Component)]
    struct ClickableCampaignPuzzles {
        clickable_campaign_puzzles: Vec<ClickableCampaignPuzzle>,
    }

    #[derive(Component)]
    struct ClickableCampaignPuzzle {
        campaign_puzzle: CampaignPuzzle,
        sprite: SpriteBundle,
    }

    #[derive(Deserialize, Debug)]
    pub struct CampaignPuzzle {
        pub id: u16,
        pub puzzle_uuid: Uuid,
        pub pos_x: u8,
        pub pos_y: u8,
        pub unlocks: Vec<u16>,
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
        mut q_camera: Query<&mut Transform, With<MainCamera>>,
    ) {
        // Get the campaign by loading it directly
        let campaign =
            serde_json::from_str::<Campaign>(include_str!("../assets/campaign/campaign.json"))
                .unwrap();

        // Populate the puzzle manager with the campaign puzzles
        let _ = puzzle_manager.populate_campaign();

        // Get the texture that will be used for the puzzle images
        // TODO use actual image
        let puzzle_tex = asset_server.load(Texture::ClassRed.path());

        for campaign_puzzle in campaign.puzzle_layout {
            // Add the puzzle as a sprite
            let sprite = SpriteBundle {
                texture: puzzle_tex.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(TILE_NODE_SPRITE_SIZE, TILE_NODE_SPRITE_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    campaign_puzzle.pos_x as f32 * (TILE_NODE_SPRITE_SIZE + SPRITE_SPACING) as f32,
                    campaign_puzzle.pos_y as f32 * (TILE_NODE_SPRITE_SIZE + SPRITE_SPACING) as f32,
                    0.,
                ),
                ..default()
            };
            commands.spawn((sprite.clone(), OnCampaignScene));

            clickable_campaign_puzzles
                .clickable_campaign_puzzles
                .push(ClickableCampaignPuzzle {
                    campaign_puzzle,
                    sprite,
                });
        }

        // TODO move camera to center on the most main-path puzzle unlocked
        for mut transform in q_camera.iter_mut() {
            *transform = Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            };
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
        clickable_campaign_puzzles: Res<ClickableCampaignPuzzles>,
        mut app_state: ResMut<NextState<AppState>>,
        mut selected_puzzle: ResMut<SelectedPuzzle>,
        mouse_button_input: Res<ButtonInput<MouseButton>>,
        q_window: Query<&Window, With<PrimaryWindow>>,
        q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    ) {
        // Get camera info and transform, assuming exacly 1 camera entity
        let (camera, camera_transform) = q_camera.single();

        // Only one primary window, so get it from query
        let window = q_window.single();

        // Check if cursor inside window and get its position, convert to world coords, discard Z
        let world_position = get_cursor_world_position(window, camera, camera_transform);

        // On left click, either enter the puzzle if available, or reject if locked
        if mouse_button_input.just_pressed(MouseButton::Left) {
            for clickable_campaign_puzzle in
                clickable_campaign_puzzles.clickable_campaign_puzzles.iter()
            {
                if clicked_on_sprite(&clickable_campaign_puzzle.sprite, world_position) {
                    // TODO check if campaign puzzle is unlocked
                    selected_puzzle.uuid = clickable_campaign_puzzle.campaign_puzzle.puzzle_uuid;
                    app_state.set(AppState::Puzzle);
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
