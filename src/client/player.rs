use bevy::prelude::*;
use bevy::utils::HashMap;
use blenvy::*;
use client::*;
use leafwing_input_manager::prelude::*;
use lightyear::prelude::*;

use crate::shared::input::{MovementSet, PlayerAction, ReplicateInputBundle};
use crate::shared::player::{shared_handle_player_movement, PlayerId, PlayerMovement, SpaceShip};

use super::lobby::LobbyState;
use super::MyClientId;

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerMap>()
            .add_systems(
                Update,
                (handle_player_spawn.run_if(resource_exists::<MyClientId>),),
            )
            .add_systems(
                FixedUpdate,
                handle_player_movement.in_set(MovementSet::Input),
            )
            .add_systems(OnEnter(LobbyState::None), despawn_input);
    }
}

/// Add visuals and input for player on player spawn.
fn handle_player_spawn(
    mut commands: Commands,
    q_predicted: Query<(&PlayerId, Entity), (Added<Predicted>, Added<SpaceShip>)>,
    my_client_id: Res<MyClientId>,
    mut player_map: ResMut<PlayerMap>,
) {
    for (player_id, entity) in q_predicted.iter() {
        info!("Spawn predicted entity ({:?}).", player_id);

        // Add visuals for player.
        commands.entity(entity).insert((
            BlueprintInfo::from_path("blueprints/Player.glb"),
            SpawnBlueprint,
        ));

        let client_id = player_id.0;

        if client_id == my_client_id.0 {
            commands.entity(entity).insert(MyPlayer);
            // Replicate input from client to server.
            commands.spawn(ReplicateInputBundle::new(*player_id));
        }

        player_map.insert(client_id, entity);
    }
}

/// Handle player movement based on [`PlayerAction`].
fn handle_player_movement(
    // Handles all predicted player movements too (other clients).
    q_actions: Query<(&PlayerId, &ActionState<PlayerAction>), With<Predicted>>,
    mut player_movement_evw: EventWriter<PlayerMovement>,
    player_map: Res<PlayerMap>,
) {
    for (id, action_state) in q_actions.iter() {
        if let Some(player_entity) = player_map.get(&id.0) {
            shared_handle_player_movement(action_state, *player_entity, &mut player_movement_evw);
        }
    }
}

/// Despawn all player inputs.
fn despawn_input(
    mut commands: Commands,
    q_action_states: Query<Entity, With<ActionState<PlayerAction>>>,
) {
    for entity in q_action_states.iter() {
        commands.entity(entity).despawn();
    }
}

/// The player the the local client is controlling.
#[derive(Component)]
pub(super) struct MyPlayer;

/// Maps client id to player entity.
#[derive(Resource, Default, Debug, Deref, DerefMut)]
pub struct PlayerMap(HashMap<ClientId, Entity>);
