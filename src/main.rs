use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
};

use rand::Rng;

const CURRENT_FONT_SIZE: f32 = 50.0;
const NEXT_FONT_SIZE: f32 = 35.0;
const TYPED_FONT_COLOR: Color = Color::rgb(0.2, 0.5, 0.0);
const CURRENT_FONT_COLOR: Color = Color::rgb(1.0, 0.0, 0.5);
const NEXT_FONT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const TEXT_PADDING: Val = Val::Px(240.0);
const NEXT_PADDING: Val = Val::Px(205.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .insert_resource(WordTyping {
            current: get_random_word(),
            next: get_random_word(),
            typed: 0,
        })
        .add_systems(
            Update,
            (bevy::window::close_on_esc, typing, update_text, update_next),
        )
        .add_systems(FixedUpdate, (timer))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Resource, Clone)]
struct WordTyping {
    current: String,
    next: String,
    typed: usize,
}

#[derive(Component)]
struct CurrentText;

#[derive(Component)]
struct NextText;

#[derive(Component)]
struct Durek;

#[derive(Component)]
struct Bubble;

#[derive(Component)]
struct GrowingRect;

fn get_random_word() -> String {
    let words: Vec<&str> = include_str!("words.txt").lines().collect();
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..words.len());

    words[random_index].to_string()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // images

    commands.spawn(SpriteBundle {
        texture: asset_server.load("textures/bg2.png"),
        ..Default::default()
    });

    // durek
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/durek.png"),
            transform: Transform {
                translation: Vec3::new(400.0, -200.0, 100.0),
                ..default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(400.0, 400.0)),
                ..default()
            },
            ..default()
        },
        Durek,
    ));

    // bubble
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/bubble.png"),
            transform: Transform {
                translation: Vec3::new(100.0, 0.0, 100.0),
                ..default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(450.0, 350.0)),
                ..default()
            },
            ..default()
        },
        Bubble,
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(100.0, -200.0, 200.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(1.0, 0.5, 0.5),
                custom_size: Some(Vec2 { x: 50.0, y: 150.0 }),
                ..default()
            },
            ..default()
        },
        GrowingRect,
    ));

    // text
    {
        commands.spawn((
            TextBundle::from_sections([
                TextSection::from_style(TextStyle {
                    font_size: CURRENT_FONT_SIZE,
                    color: TYPED_FONT_COLOR,
                    ..default()
                }),
                TextSection::from_style(TextStyle {
                    font_size: CURRENT_FONT_SIZE,
                    color: CURRENT_FONT_COLOR,
                    ..default()
                }),
            ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(300.0),
                left: Val::Px(550.0),
                ..default()
            }),
            CurrentText,
        ));

        commands.spawn((
            TextBundle::from_sections([TextSection::from_style(TextStyle {
                font_size: NEXT_FONT_SIZE,
                color: NEXT_FONT_COLOR,
                ..default()
            })])
            .with_style(Style {
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                top: Val::Px(265.0),
                left: Val::Px(550.0),
                ..default()
            }),
            NextText,
        ));
    }
}

fn key_code_to_char(key_code: KeyCode) -> Option<char> {
    match key_code {
        KeyCode::A => Some('a'),
        KeyCode::B => Some('b'),
        KeyCode::C => Some('c'),
        KeyCode::D => Some('d'),
        KeyCode::E => Some('e'),
        KeyCode::F => Some('f'),
        KeyCode::G => Some('g'),
        KeyCode::H => Some('h'),
        KeyCode::I => Some('i'),
        KeyCode::J => Some('j'),
        KeyCode::K => Some('k'),
        KeyCode::L => Some('l'),
        KeyCode::M => Some('m'),
        KeyCode::N => Some('n'),
        KeyCode::O => Some('o'),
        KeyCode::P => Some('p'),
        KeyCode::Q => Some('q'),
        KeyCode::R => Some('r'),
        KeyCode::S => Some('s'),
        KeyCode::T => Some('t'),
        KeyCode::U => Some('u'),
        KeyCode::V => Some('v'),
        KeyCode::W => Some('w'),
        KeyCode::X => Some('x'),
        KeyCode::Y => Some('y'),
        KeyCode::Z => Some('z'),
        _ => None,
    }
}

fn is_char_equal(char_to_compare: char, input: &str, n: usize) -> bool {
    if let Some(nth_character) = input.chars().nth(n) {
        char_to_compare == nth_character
    } else {
        false
    }
}

fn typing(
    mut commands: Commands,
    mut word_typing: ResMut<WordTyping>,
    mut kb_input_events: EventReader<KeyboardInput>,
) {
    for event in kb_input_events.iter() {
        if let Some(key_code) = event.key_code {
            if event.state == ButtonState::Pressed {
                if let Some(key_char) = key_code_to_char(key_code) {
                    if is_char_equal(key_char, &word_typing.current, word_typing.typed) {
                        word_typing.typed += 1;
                    }
                    // word_typing.input.push(key_char);
                }
            }
        }
    }
}

fn update_next(mut query: Query<&mut Text, With<NextText>>, word_typing: Res<WordTyping>) {
    let mut text = query.single_mut();
    text.sections[0].value = word_typing.next.clone();
}

fn update_text(
    mut word_typing: ResMut<WordTyping>,
    mut current_query: Query<&mut Text, &CurrentText>,
    time: Res<Time>,
    mut query: Query<(&GrowingRect, &mut Sprite, &mut Transform)>,
) {
    let mut current_text = current_query.single_mut();
    let current_string = word_typing.current.clone();
    let index = word_typing.typed;

    if !(index >= current_string.len()) {
        current_text.sections[0].value = current_string[..index].to_string();
        current_text.sections[1].value = current_string[index..].to_string();
    } else {
        word_typing.current = word_typing.next.clone();
        word_typing.next = get_random_word();
        word_typing.typed = 0;

        for (_tag, mut sprite, mut transform) in query.iter_mut() {
            if let Some(vec) = &sprite.custom_size {
                println!("The y value is: {}", vec.y);
                sprite.custom_size = Some(Vec2 {
                    x: vec.x,
                    y: vec.y + 1600.0 * time.delta_seconds(),
                });
                transform.translation.y += 1600.0 * time.delta_seconds() / 2.0;
            } else {
                println!("The value is None");
            }
        }
    }
}

fn timer(time: Res<Time>, mut query: Query<(&GrowingRect, &mut Sprite, &mut Transform)>) {
    for (_tag, mut sprite, mut transform) in query.iter_mut() {
        if let Some(vec) = &sprite.custom_size {
            println!("The y value is: {}", vec.y);

            let dt = 10.0 * time.delta_seconds();
            if vec.y >= 10.0 {
                sprite.custom_size = Some(Vec2 {
                    x: vec.x,
                    y: vec.y - dt,
                });
                transform.translation.y -= dt / 2.0;
            }
        } else {
            println!("The value is None");
        }
    }
}
