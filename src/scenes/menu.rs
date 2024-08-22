pub mod menu {
    use bevy::{app::AppExit, prelude::*};
    use uuid::Uuid;

    use crate::{
        backend::api::api::NodalApi,
        buttons::{
            button_icon_style, button_text_style, text_button_style, NORMAL_BUTTON, TEXT_COLOR,
        },
        despawn_screen,
        texture::Texture,
        AppState, SelectedPuzzle,
    };

    // This plugin manages the menu, with 5 different screens:
    // - a main menu with "New Game", "Settings", "Quit"
    // - a settings menu with two submenus and a back button
    // - two settings screen with a setting that can be set and a back button
    pub fn menu_plugin(app: &mut App) {
        app
            // At start, the menu is not enabled. This will be changed in `menu_setup` when
            // entering the `GameState::Menu` state.
            // Current screen in the menu is handled by an independent state from `GameState`
            .init_state::<MenuState>()
            .add_systems(OnEnter(AppState::Menu), menu_setup)
            // Systems to handle the main menu screen
            .add_systems(OnEnter(MenuState::Main), main_menu_setup)
            .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
            // Systems to handle the submenu screen (campaign, community, daily challenge, etc)
            .add_systems(OnEnter(MenuState::SubMenu), submenu_setup)
            .add_systems(
                OnExit(MenuState::SubMenu),
                despawn_screen::<OnSubMenuScreen>,
            )
            // Systems to handle the puzzle select screen
            // .add_systems(OnEnter(MenuState::PuzzleSelect), puzzle_select_setup)
            .add_systems(
                OnExit(MenuState::PuzzleSelect),
                despawn_screen::<OnPuzzleSelectScreen>,
            )
            // Common systems to all menu screens that handles buttons behavior
            .add_systems(Update, menu_action.run_if(in_state(AppState::Menu)));
    }

    // State used for the current menu screen
    #[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
    enum MenuState {
        Main,
        SubMenu,
        PuzzleSelect,
        #[default]
        Disabled,
    }

    // Tag component used to tag entities added on the main menu screen
    #[derive(Component)]
    struct OnMainMenuScreen;

    // Tag component used to tag entities added on the submenu screen
    #[derive(Component)]
    struct OnSubMenuScreen;

    // Tag component used to tag entities added on the puzzle select screen
    #[derive(Component)]
    struct OnPuzzleSelectScreen;

    // All actions that can be triggered from a button click
    #[derive(Component)]
    enum MenuButtonAction {
        Campaign,
        SubMenu,
        PlayPuzzle,
        Quit,
    }

    // Tag component used to mark which puzzle is tied to a button
    #[derive(Default, Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
    struct ButtonPuzzleId {
        uuid: Uuid,
    }

    fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
        menu_state.set(MenuState::Main);
    }

    fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                },
                OnMainMenuScreen,
            ))
            .with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::BLACK.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        // Display the game name
                        parent.spawn(
                            TextBundle::from_section(
                                "Bevy Game Menu UI",
                                TextStyle {
                                    font_size: 80.0,
                                    color: TEXT_COLOR,
                                    ..default()
                                },
                            )
                            .with_style(Style {
                                margin: UiRect::all(Val::Px(50.0)),
                                ..default()
                            }),
                        );

                        // Display buttons for each action available from the main menu:
                        // - new game
                        // - sign in with itch
                        // - quit
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: text_button_style(),
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                MenuButtonAction::SubMenu,
                            ))
                            .with_children(|parent| {
                                let icon = asset_server.load(Texture::Node.path());
                                parent.spawn(ImageBundle {
                                    style: button_icon_style(),
                                    image: UiImage::new(icon),
                                    ..default()
                                });
                                parent.spawn(TextBundle::from_section(
                                    "New Game",
                                    button_text_style(),
                                ));
                            });
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: text_button_style(),
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                MenuButtonAction::Quit,
                            ))
                            .with_children(|parent| {
                                let icon = asset_server.load(Texture::BtnGoBack.path());
                                parent.spawn(ImageBundle {
                                    style: button_icon_style(),
                                    image: UiImage::new(icon),
                                    ..default()
                                });
                                parent.spawn(TextBundle::from_section("Quit", button_text_style()));
                            });
                    });
            });
    }

    fn submenu_setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..Default::default()
                },
                OnSubMenuScreen,
            ))
            .with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::BLACK.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        // Campaign
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: text_button_style(),
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                MenuButtonAction::Campaign,
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    "Campaign",
                                    button_text_style(),
                                ));
                            });
                        // TODO browse levels, daily challenge
                    });
            });
    }

    fn menu_action(
        interaction_query: Query<
            (&Interaction, &MenuButtonAction, Option<&ButtonPuzzleId>),
            (Changed<Interaction>, With<Button>),
        >,
        mut app_exit_events: EventWriter<AppExit>,
        mut menu_state: ResMut<NextState<MenuState>>,
        mut app_state: ResMut<NextState<AppState>>,
        mut selected_puzzle: ResMut<SelectedPuzzle>,
        _api: Res<NodalApi>,
    ) {
        for (interaction, menu_button_action, button_puzzle_id) in &interaction_query {
            if *interaction == Interaction::Pressed {
                match menu_button_action {
                    MenuButtonAction::Quit => {
                        app_exit_events.send(AppExit::Success);
                    }
                    MenuButtonAction::SubMenu => {
                        menu_state.set(MenuState::SubMenu);
                    }
                    MenuButtonAction::Campaign => {
                        app_state.set(AppState::Campaign);
                        menu_state.set(MenuState::Disabled);
                    }
                    MenuButtonAction::PlayPuzzle => {
                        app_state.set(AppState::Campaign);
                        menu_state.set(MenuState::Disabled);
                        selected_puzzle.uuid = button_puzzle_id.unwrap().uuid
                    }
                }
            }
        }
    }
}
