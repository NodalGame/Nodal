pub mod campaign {
    use bevy::{app::{App, Update}, asset::AssetServer, ecs::{component::Component, query::{Changed, With}, schedule::{common_conditions::in_state, IntoSystemConfigs, NextState, OnEnter, OnExit}, system::{Commands, Query, Res, ResMut, Resource}}, math::Vec2, render::color::Color, sprite::{Sprite, SpriteBundle}, transform::components::Transform, ui::{node_bundles::{ButtonBundle, ImageBundle, NodeBundle}, widget::Button, AlignItems, FlexDirection, Interaction, JustifyContent, Style, UiImage, Val}, utils::default};
    use serde::Deserialize;
    use uuid::Uuid;

    use crate::{buttons::buttons::{button_icon_style, icon_button_style, text_button_style, NORMAL_BUTTON}, constants::{SPRITE_SPACING, TILE_NODE_SPRITE_SIZE}, despawn_screen, puzzle_manager::{self, puzzle_manager::PuzzleManager}, texture::texture::Texture, AppState, MainCamera, SelectedPuzzle};

    // This plugin will contain a campaign (for now, just the main campaign).
    pub fn campaign_plugin(app: &mut App) {
        app.add_systems(OnEnter(AppState::Campaign), campaign_setup)
            .add_systems(OnExit(AppState::Campaign), despawn_screen::<OnCampaignScene>)
            .add_systems(Update, ui_action.run_if(in_state(AppState::Campaign)))
            .insert_resource(PuzzleManager::new()); 
    }

    #[derive(Deserialize, Debug)]
    pub struct Campaign {
        pub uuid: Uuid,
        pub name: String,
        pub puzzle_layout: Vec<CampaignPuzzle>
    }

    #[derive(Deserialize, Debug)]
    pub struct CampaignPuzzle {
        pub id: u16,
        pub puzzle_uuid: Uuid,
        pub pos_x: u8,
        pub pos_y: u8,
        pub unlocks: Vec<u16>
    }

    // Tag component used to mark which puzzle is tied to a button
    #[derive(Default, Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
    struct ButtonPuzzleId {
        uuid: Uuid,
    }

    // Tag componentn used to tag entities added on the campaign scene
    #[derive(Component)]
    struct OnCampaignScene;

    // All actions that can be triggered from a button click
    #[derive(Component)]
    enum CampaignButtonAction {
        PlayPuzzle,
        ReturnToSubMenu,
    }

    fn campaign_setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut puzzle_manager: ResMut<PuzzleManager>,
        mut q_camera: Query<&mut Transform, With<MainCamera>>,
    ) {
        // Get the campaign by loading it directly
        let campaign = serde_json::from_str::<Campaign>(include_str!("../assets/campaign/campaign.json")).unwrap();
        println!("{:?}", campaign);

        // Populate the puzzle manager with the campaign puzzles
        let _ = puzzle_manager.populate_campaign();

        // Get the texture that will be used for the puzzle images
        // TODO use actual image
        let puzzle_tex = asset_server.load(Texture::ClassRed.path());

        for campaign_puzzle in campaign.puzzle_layout {
            // Add the puzzle as a sprite
            commands.spawn((
                SpriteBundle {
                    texture: puzzle_tex.clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(TILE_NODE_SPRITE_SIZE, TILE_NODE_SPRITE_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        campaign_puzzle.pos_x as f32 * (TILE_NODE_SPRITE_SIZE + SPRITE_SPACING) as f32,
                        campaign_puzzle.pos_y as f32 * (TILE_NODE_SPRITE_SIZE + SPRITE_SPACING) as f32,
                        0.
                    ),
                    ..default()
                },
                CampaignButtonAction::PlayPuzzle,
                ButtonPuzzleId { uuid: campaign_puzzle.puzzle_uuid },
            )).insert(Button);
        }
    }

    fn ui_action(
        interaction_query: Query<
            (&Interaction, &CampaignButtonAction, Option<&ButtonPuzzleId>),
            (Changed<Interaction>, With<Button>),
        >,
        mut app_state: ResMut<NextState<AppState>>,
        mut selected_puzzle: ResMut<SelectedPuzzle>,
    ) {
        for (interaction, ui_button_action, button_puzzle_id) in &interaction_query {
            if *interaction == Interaction::Pressed {
                match ui_button_action {
                    CampaignButtonAction::ReturnToSubMenu => {
                        app_state.set(AppState::Menu);
                    }
                    CampaignButtonAction::PlayPuzzle => {
                        // TODO this doesn't work since the campaign nodes are sprites, need to detect hover/click
                        app_state.set(AppState::Puzzle);
                        selected_puzzle.uuid = button_puzzle_id.unwrap().uuid
                    }
                }
            }
        }
    }
}