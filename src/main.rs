use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
};

const CURRENT_FONT_SIZE: f32 = 40.0;
const TYPED_FONT_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
const CURRENT_FONT_COLOR: Color = Color::rgb(0.0, 1.0, 0.5);
const TEXT_PADDING: Val = Val::Px(5.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .insert_resource(WordTyping {
            current: "testing".to_string(),
            next: "test2".to_string(),
            input: "".to_string(),
        })
        .add_systems(Update, (bevy::window::close_on_esc, typing, update_text))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Resource, Clone)]
struct WordTyping {
    current: String,
    next: String,
    input: String,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // text
    {
        commands.spawn((TextBundle::from_sections([
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
            top: TEXT_PADDING,
            left: TEXT_PADDING,
            ..default()
        }),));
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

fn typing(
    mut commands: Commands,
    mut word_typing: ResMut<WordTyping>,
    input: Res<Input<KeyCode>>,
    mut kb_input_events: EventReader<KeyboardInput>,
) {
    for event in kb_input_events.iter() {
        if let Some(key_code) = event.key_code {
            if event.state == ButtonState::Pressed {
                if let Some(key_char) = key_code_to_char(key_code) {
                    word_typing.input.push(key_char);
                } else if key_code == KeyCode::Back {
                    word_typing.input.pop();
                }
            }
        }
    }
}

fn update_text(word_typing: Res<WordTyping>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();

    let current_string = word_typing.current.clone();
    let input_string = word_typing.input.clone();

    let current_chars: Vec<char> = current_string.chars().collect();
    let input_chars: Vec<char> = input_string.chars().collect();

    let mut index = 0;

    for (cc, ic) in current_chars.iter().zip(input_chars.iter()) {
        if cc == ic {
            index += 1;
        } else {
            break;
        }
    }

    let mut typed = "".to_string();
    let mut remaining = word_typing.current.clone();

    if index != 0 {
        typed = current_string[..index].to_string();
        remaining = current_string[index..].to_string();
    }

    text.sections[0].value = typed;
    text.sections[1].value = remaining;
}
