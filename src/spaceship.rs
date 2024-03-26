use bevy::prelude::*;

use super::movement::Velocity;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0); 
const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0); 

#[derive(Bundle)]
struct SpaceshipBundle {
    velocity: Velocity,
    model: SceneBundle,
}

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_spaceship);
    }
}

fn spawn_spaceship(mut commands: Commands, asset_server: Res<AssetServer>) {
    let model_handle: Handle<Scene> = asset_server.load("Spaceship.glb#Scene0");

    commands.spawn(SpaceshipBundle{
        velocity: Velocity{
            value: STARTING_VELOCITY
        }      ,
        model: SceneBundle{
            scene: model_handle,
            transform: Transform::from_translation(STARTING_TRANSLATION),
            ..default()
        }
    });
}


