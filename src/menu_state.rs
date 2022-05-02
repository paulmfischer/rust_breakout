use bevy::{app::AppExit, prelude::*};

use crate::utilities::despawn_entities;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainMenu,
    InGame,
    // Paused,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(setup_menu))
            .add_system_set(
                SystemSet::on_update(GameState::MainMenu)
                    .with_system(menu_interaction)
                    .with_system(select_menu_item),
            )
            .add_system_set(
                SystemSet::on_pause(GameState::MainMenu)
                    .with_system(despawn_entities::<MenuEntity>),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::MainMenu).with_system(despawn_entities::<MenuEntity>),
            );
    }
}

const TEXT_COLOR: Color = Color::WHITE;
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const SELECTED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

// Tag component used to mark wich setting is currently selected
#[derive(Component)]
struct SelectedOption;

// All actions that can be triggered from a button click
#[derive(Component)]
enum MenuButtonAction {
    Play,
    Quit,
}

#[derive(Component)]
struct MenuEntity;

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    let button_style = Style {
        size: Size::new(Val::Px(380.0), Val::Px(110.0)),
        margin: Rect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font: font.clone(),
        font_size: 100.0,
        color: TEXT_COLOR,
    };

    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(MenuEntity);

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: Rect::all(Val::Auto),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::DARK_GRAY.into(),
            ..default()
        })
        .insert(MenuEntity)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    margin: Rect::all(Val::Px(50.0)),
                    ..default()
                },
                text: Text::with_section(
                    "Breakout!!",
                    TextStyle {
                        font: font.clone(),
                        font_size: 180.0,
                        color: TEXT_COLOR,
                    },
                    Default::default(),
                ),
                ..default()
            });

            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.clone(),
                    color: SELECTED_BUTTON.into(),
                    ..default()
                })
                .insert(MenuButtonAction::Play)
                .insert(SelectedOption)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Start",
                            button_text_style.clone(),
                            Default::default(),
                        ),
                        ..default()
                    });
                });

            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style,
                    color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .insert(MenuButtonAction::Quit)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section("Quit", button_text_style, Default::default()),
                        ..default()
                    });
                });
        });
}

fn menu_interaction(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut buttons_query: Query<(Entity, &mut UiColor, Option<&SelectedOption>), With<Button>>,
) {
    if keyboard_input.just_pressed(KeyCode::Up) {
        let mut removed_selected = false;
        for (button_entity, mut button_color, selected_option) in buttons_query.iter_mut() {
            if removed_selected {
                commands.entity(button_entity).insert(SelectedOption);
                *button_color = SELECTED_BUTTON.into();
                return;
            }

            if let Some(_) = selected_option {
                *button_color = NORMAL_BUTTON.into();
                commands.entity(button_entity).remove::<SelectedOption>();
                removed_selected = true;
            }
        }
    }

    if keyboard_input.just_pressed(KeyCode::Down) {
        let mut removed_selected = false;
        for (button_entity, mut button_color, selected_option) in buttons_query.iter_mut() {
            if removed_selected {
                commands.entity(button_entity).insert(SelectedOption);
                *button_color = SELECTED_BUTTON.into();
                return;
            }

            if let Some(_) = selected_option {
                *button_color = NORMAL_BUTTON.into();
                commands.entity(button_entity).remove::<SelectedOption>();
                removed_selected = true;
            }
        }
    }
}

fn select_menu_item(
    keyboard_input: Res<Input<KeyCode>>,
    selected_option_query: Query<&MenuButtonAction, With<SelectedOption>>,
    mut exit: EventWriter<AppExit>,
    mut app_state: ResMut<State<GameState>>,
) {
    let menu_action = selected_option_query.single();

    if keyboard_input.just_pressed(KeyCode::Return) {
        match menu_action {
            MenuButtonAction::Play => {
                app_state.push(GameState::InGame).unwrap();
            }
            MenuButtonAction::Quit => {
                exit.send(AppExit);
            }
        }
    }
}
