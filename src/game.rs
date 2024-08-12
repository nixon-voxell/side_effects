use bevy::prelude::*;

pub mod input;
pub mod lobby;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // Game plugins
        app.add_plugins((input::InputPlugin,));
    }
}
