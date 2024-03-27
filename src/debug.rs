use bevy::prelude::*;

use crate::schedule::InGameSet;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_position.after(InGameSet::EntityUpdates));
    }
}

fn print_position(query: Query<(Entity, &Transform)>) {
    for (entity, position) in query.iter() {
        info!(
            "Entity {:?} is at position {:?}",
            entity, position.translation
        );
    }
}