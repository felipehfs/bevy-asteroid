use bevy::prelude::*;

mod camera;
mod debug;
mod movement;
mod spaceship;

use debug::DebugPlugin;
use camera::CameraPlugin;
use movement::*;
use spaceship::*;

fn main() {
    let window = Some(Window {
        title: "Spaceship game".to_string(),
        ..default()
    });

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: window,
            ..default()
        }))
        // bevy built-ins
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        // user configuration
        .add_plugins(SpaceshipPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .run();
}
