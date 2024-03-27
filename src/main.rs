use asset_loader::AssetLoaderPlugin;
use bevy::prelude::*;

mod camera;
mod debug;
mod movement;
mod spaceship;
mod asteroids;
mod asset_loader;
mod collision_detection;
mod despawn;

use asteroids::AsteroidPlugin;
use debug::DebugPlugin;
use camera::CameraPlugin;
use movement::*;
use spaceship::*;
use collision_detection::*;
use despawn::DespawnPlugin;

fn main() {
    let window = Some(Window {
        title: "Spaceship game".to_string(),
        ..default()
    });

    App::new()
        // bevy built-ins
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: window,
            ..default()
        }))
        // user configuration
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(AsteroidPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(CollisionDetectionPlugin)
        //.add_plugins(DebugPlugin)
        .add_plugins(DespawnPlugin)
        .run();
}
