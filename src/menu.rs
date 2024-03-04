pub mod menu {
    use std::path::{Path, PathBuf};

    use bevy::{app::AppExit, prelude::*};
    use uuid::Uuid;

    use crate::{despawn_screen, puzzle_manager::puzzle_manager::PuzzleManager, AppState, SelectedPuzzle, HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON, TEXT_COLOR};

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
            
            // Systems to handle the puzzle select screen
            .add_systems(OnEnter(MenuState::PuzzleSelect), puzzle_select_setup)
            .add_systems(OnExit(MenuState::PuzzleSelect), despawn_screen::<OnPuzzleSelectScreen>)

            // Common systems to all menu screens that handles buttons behavior
            .add_systems(
                Update,
                (menu_action, button_system).run_if(in_state(AppState::Menu)),
            )
            
            .insert_resource(PuzzleManager::new());
    }

    // State used for the current menu screen
    #[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
    enum MenuState {
        Main,
        PuzzleSelect,
        #[default]
        Disabled,
    }

    // Tag component used to tag entities added on the main menu screen
    #[derive(Component)]
    struct OnMainMenuScreen;

    // Tag component used to tag entities added on the puzzle select screen
    #[derive(Component)]
    struct OnPuzzleSelectScreen;

    // All actions that can be triggered from a button click
    #[derive(Component)]
    enum MenuButtonAction {
        Play,
        PlayPuzzle,
        Quit,
    }

    // Tag component used to mark which puzzle is tied to a button
    #[derive(Default, Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
    struct ButtonPuzzleId {
        uuid: Uuid,
    }

    // This system handles changing all buttons color based on mouse interaction
    fn button_system(
        mut interaction_query: Query<
            (&Interaction, &mut BackgroundColor),
            (Changed<Interaction>, With<Button>),
        >,
    ) {
        for (interaction, mut color) in &mut interaction_query {
            *color = match *interaction {
                Interaction::Pressed => PRESSED_BUTTON.into(),
                Interaction::Hovered => HOVERED_BUTTON.into(),
                Interaction::None => NORMAL_BUTTON.into(),
            };
        }
    }

    // Common button settings, TODO commonize? 
    fn button_style() -> Style {
        Style {
            width: Val::Px(250.0),
            height: Val::Px(65.0),
            margin: UiRect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        }
    }

    fn button_icon_style() -> Style {
        Style {
            width: Val::Px(30.0),
            // This takes the icons out of the flexbox flow, to be positioned exactly
            position_type: PositionType::Absolute,
            // The icon will be close to the left border of the button
            left: Val::Px(10.0),
            ..default()
        }
    }

    fn button_text_style() -> TextStyle {
        TextStyle {
            font_size: 40.0,
            color: TEXT_COLOR,
            ..default()
        }
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
                        background_color: Color::CRIMSON.into(),
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
                        // - quit
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style(),
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                MenuButtonAction::Play,
                            ))
                            .with_children(|parent| {
                                let icon = asset_server.load("textures/Game Icons/right.png");
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
                                    style: button_style(),
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                MenuButtonAction::Quit,
                            ))
                            .with_children(|parent| {
                                let icon = asset_server.load("textures/Game Icons/exitRight.png");
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

    fn puzzle_select_setup(
        mut commands: Commands,
        mut puzzle_manager: ResMut<PuzzleManager> 
    ) {
        // TODO preload in other menu? if this starts to slow down...
        let _ = puzzle_manager.populate(&PathBuf::from("assets/campaign/puzzles/"));
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
                OnPuzzleSelectScreen,
            ))
            .with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::CRIMSON.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        for puzzle_id in puzzle_manager.get_puzzle_uuids() {
                            // Display the levels to select TODO scrollable and populate programmatically
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: button_style(),
                                        background_color: NORMAL_BUTTON.into(),
                                        ..default()
                                    },
                                    MenuButtonAction::PlayPuzzle,
                                    ButtonPuzzleId { uuid: puzzle_id },
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "Puzzle ".to_owned() + &puzzle_id.to_string(),
                                        button_text_style(),
                                    ));
                                });
                        }
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
    ) {
        for (interaction, menu_button_action, button_puzzle_id) in &interaction_query {
            if *interaction == Interaction::Pressed {
                match menu_button_action {
                    MenuButtonAction::Quit => {
                        app_exit_events.send(AppExit);
                    }
                    MenuButtonAction::Play => {
                        menu_state.set(MenuState::PuzzleSelect);
                    }
                    MenuButtonAction::PlayPuzzle => {
                        app_state.set(AppState::Puzzle);
                        menu_state.set(MenuState::Disabled);
                        selected_puzzle.uuid = button_puzzle_id.unwrap().uuid
                    }
                }
            }
        }
    }
}