use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use bevy::{prelude::*, render::view::RenderLayers};
use lightyear::prelude::client::*;
use lightyear::prelude::*;

use crate::server::SERVER_ADDR;
use crate::shared::shared_config;

mod lobby;

const CLIENT_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 4000);

pub struct ClientPlugin {
    pub port_offset: u16,
}

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        info!("Adding `ClientPlugin`.");
        // Lightyear plugins
        app.add_plugins(ClientPlugins::new(client_config(self.port_offset)));

        // Server-specific logic.
        app.add_plugins(lobby::LobbyPlugin)
            .add_systems(Startup, spawn_game_camera);
    }
}

/// Create the lightyear [`ClientConfig`].
fn client_config(port_offset: u16) -> ClientConfig {
    // Authentication is where you specify how the client should connect to the server
    // This is where you provide the server address.
    let auth = Authentication::Manual {
        server_addr: SERVER_ADDR,
        client_id: rand::random(),
        private_key: Key::default(),
        protocol_id: 0,
    };

    let mut client_addr = CLIENT_ADDR;
    client_addr.set_port(CLIENT_ADDR.port() + port_offset);

    // The IoConfig will specify the transport to use.
    let io = IoConfig {
        // the address specified here is the client_address, because we open a UDP socket on the client
        transport: ClientTransport::UdpSocket(client_addr),
        ..default()
    };
    // The NetConfig specifies how we establish a connection with the server.
    // We can use either Steam (in which case we will use steam sockets and there is no need to specify
    // our own io) or Netcode (in which case we need to specify our own io).
    let net_config = NetConfig::Netcode {
        auth,
        io,
        config: NetcodeConfig::default(),
    };
    ClientConfig {
        // part of the config needs to be shared between the client and server
        shared: shared_config(),
        net: net_config,
        ..default()
    }
}

/// Spawn camera for game rendering (render layer 0).
fn spawn_game_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Game Camera"),
        Camera2dBundle {
            camera: Camera {
                clear_color: ClearColorConfig::None,
                ..default()
            },
            ..default()
        },
        RenderLayers::layer(0),
    ));

    // TODO: Remove this.
    commands.connect_client();
}
