use bevy::prelude::*;

use crate::{
    utilities::{
        despawn_entities, TEXT_COLOR, SELECTED_BUTTON,
    },
    GameState,
};

use super::components::GameData;

#[derive(Component)]
enum LoseMenuButtonAction {
    Okay
}

#[derive(Component)]
struct LoseMenuEntity;

pub struct LosePlugin;

impl Plugin for LosePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(render_lose_menu))
            .add_system_set(
                SystemSet::on_update(GameState::GameOver)
                    .with_system(select_menu_item),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::GameOver).with_system(despawn_entities::<LoseMenuEntity>),
            );
    }
}

fn render_lose_menu(mut commands: Commands, asset_server: Res<AssetServer>, windows: Res<Windows>, game_data: Res<GameData>) {
    let window = windows.get_primary().unwrap();
    let window_height = window.height();
    let window_width = window.width();
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    let button_style = Style {
        size: Size::new(
            Val::Px((window_width / 2.0).round()),
            Val::Px((window_height / 5.0).round()),
        ),
        margin: Rect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font: font.clone(),
        font_size: (window_height / 5.5).round(),
        color: TEXT_COLOR,
    };

    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(LoseMenuEntity);

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
        .insert(LoseMenuEntity)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    margin: Rect::all(Val::Px(30.0)),
                    ..default()
                },
                text: Text::with_section(
                    "Game Over",
                    TextStyle {
                        font: font.clone(),
                        font_size: (window_height / 4.0).round(),
                        color: TEXT_COLOR,
                    },
                    Default::default(),
                ),
                ..default()
            }).insert(LoseMenuEntity);

            parent.spawn_bundle(TextBundle {
                style: Style {
                    margin: Rect::all(Val::Px(5.0)),
                    ..default()
                },
                text: Text::with_section(
                    format!("Your score: {}", game_data.score),
                    TextStyle {
                        font: font.clone(),
                        font_size: (window_height / 6.0).round(),
                        color: TEXT_COLOR,
                    },
                    Default::default(),
                ),
                ..default()
            });

            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style,
                    color: SELECTED_BUTTON.into(),
                    ..default()
                })
                .insert(LoseMenuButtonAction::Okay)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section("Okay", button_text_style, Default::default()),
                        ..default()
                    });
                });
        });
}

fn select_menu_item(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    selected_option_query: Query<&LoseMenuButtonAction>,
    mut app_state: ResMut<State<GameState>>,
) {
    let menu_action = selected_option_query.single();

    if keyboard_input.just_pressed(KeyCode::Return) {
        match menu_action {
            LoseMenuButtonAction::Okay => {
                app_state.set(GameState::MainMenu).unwrap();
                keyboard_input.clear();
            }
        }
    }
}
