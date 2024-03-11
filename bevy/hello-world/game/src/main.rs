use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::window::{MonitorSelection, WindowPosition};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    prevent_default_event_handling: false,
                    canvas: Some("#canvas".to_string()),
                    position: WindowPosition::Centered(MonitorSelection::Current),
                    ..default()
                }),
                ..default()
            }),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, main_menu)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("/Roboto-Black.ttf");

    let title_font = title_text_style(60.0, font.clone());

    commands.spawn({
        Text2dBundle {
            text: Text::from_section("Hello world!", title_font.clone())
                .with_justify(JustifyText::Center),
            ..default()
        }
    });
}

fn title_text_style(font_size: f32, font: Handle<Font>) -> TextStyle {
    TextStyle {
        font,
        font_size,
        color: Color::WHITE,
    }
}
