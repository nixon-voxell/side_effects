use bevy::prelude::*;
use main_menu::MainMenuUiPlugin;
use matchmaking::MatchmakingUiPlugin;

pub(super) mod main_menu;
pub(super) mod matchmaking;

pub(super) struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MainMenuUiPlugin)
            .add_plugins(MatchmakingUiPlugin);
        // .add_systems(
        //     Update,
        //     (
        //         join_lobby_ui.run_if(in_state(LobbyState::None)),
        //         lobby_ui.run_if(in_state(LobbyState::Joined)),
        //     ),
        // )
        // .add_systems(Update, (connect_server_btn, join_lobby_btn, exit_lobby_btn))
    }
}
