use avian2d::prelude::*;
use bevy::prelude::*;
use client::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;

use crate::shared::input::{PlayerAction, ReplicateInputBundle};
use crate::shared::player::{shared_handle_player_movement, PlayerId, PlayerMovement};
use crate::shared::FixedSet;

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_player_spawn)
            .add_systems(FixedUpdate, handle_player_movement.in_set(FixedSet::Main));
    }
}

fn handle_player_spawn(
    mut commands: Commands,
    q_predicted: Query<
        (&PlayerId, Entity, Has<Predicted>),
        (Or<(Added<Predicted>, Added<Interpolated>)>, With<Position>),
    >,
) {
    for (id, entity, is_predicted) in q_predicted.iter() {
        info!("Spawn predicted entity.");

        // Add visuals for player.
        commands.entity(entity).insert(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                rect: Some(Rect::from_center_half_size(default(), Vec2::splat(20.0))),
                ..default()
            },
            ..default()
        });

        if is_predicted {
            // Replicate input from client to server.
            commands.spawn(ReplicateInputBundle::new(*id));
        }
    }
}

/// Player movement
fn handle_player_movement(
    q_player: Query<Entity, (With<Predicted>, With<Position>)>,
    q_action_states: Query<&ActionState<PlayerAction>, With<PrePredicted>>,
    mut player_movement_evw: EventWriter<PlayerMovement>,
) {
    let Ok(action_state) = q_action_states.get_single() else {
        return;
    };

    let Ok(player_entity) = q_player.get_single() else {
        return;
    };

    shared_handle_player_movement(action_state, player_entity, &mut player_movement_evw);
}
