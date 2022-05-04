use bevy::prelude::*;

pub const TEXT_COLOR: Color = Color::WHITE;
pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const SELECTED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
pub struct MenuEntity;

// Tag component used to mark wich setting is currently selected
#[derive(Component)]
pub struct SelectedOption;

// All actions that can be triggered from a button click
#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    Quit,
}

pub struct MenuOptions<'a> {
    pub title: &'a str,
    pub play_text: &'a str,
}

pub fn despawn_entities<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    query.for_each(|entity| commands.entity(entity).despawn_recursive());
}

pub fn render_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
    menu_options: MenuOptions,
) {
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
                    margin: Rect::all(Val::Px(30.0)),
                    ..default()
                },
                text: Text::with_section(
                    menu_options.title,
                    TextStyle {
                        font: font.clone(),
                        font_size: (window_height / 4.0).round(),
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
                            menu_options.play_text,
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

pub fn menu_interaction(
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
