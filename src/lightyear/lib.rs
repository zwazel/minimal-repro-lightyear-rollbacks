use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::Duration,
};

use bevy::{ecs::system::SystemParam, prelude::*};
use lightyear::prelude::{
    client::{self, Authentication},
    server, CompressionConfig, Key, LinkConditionerConfig,
};

use super::my_shared::shared_config;

pub const CLIENT_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);
pub const SERVER_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), NETCODE_PORT);
pub const NETCODE_PORT: u16 = 4000;

#[derive(SystemParam)]
pub struct MyNetConfigControl<'w> {
    _server_config: ResMut<'w, server::ServerConfig>,
    client_config: ResMut<'w, client::ClientConfig>,
    // steam_client: ResMut<'w, SteamClientResource>,
}

impl<'w> MyNetConfigControl<'w> {
    pub(crate) fn set_to_join(&mut self) {
        let client_config = {
            println!("Setting client to join");
            let server_addr = SERVER_ADDR;
            let client_addr = CLIENT_ADDR;
            let random_client_id = 1;
            client::NetConfig::Netcode {
                auth: Authentication::Manual {
                    server_addr,
                    client_id: random_client_id,
                    private_key: Key::default(),
                    protocol_id: 0,
                },
                config: client::NetcodeConfig::default(),
                io: client::IoConfig {
                    transport: client::ClientTransport::UdpSocket(client_addr),
                    conditioner: Some(MyNetConfigControl::get_default_conditioner()),
                    compression: CompressionConfig::None,
                },
            }
        };

        *self.client_config = client::ClientConfig {
            shared: shared_config(),
            net: client_config,
            ..default()
        };
    }

    pub(crate) fn set_to_host(&mut self) {
        println!("Setting client to host");
        let net_config = client::NetConfig::Local { id: 0 };

        *self.client_config = client::ClientConfig {
            shared: shared_config(),
            net: net_config,
            ..default()
        };
    }

    pub(crate) fn get_default_conditioner() -> LinkConditionerConfig {
        LinkConditionerConfig {
            incoming_latency: Duration::from_millis(0),
            incoming_jitter: Duration::from_millis(0),
            incoming_loss: 0.0,
        }
    }
}
