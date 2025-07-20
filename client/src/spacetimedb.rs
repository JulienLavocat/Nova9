use bevy::prelude::*;
use bevy_spacetimedb::{
    ReadStdbConnectedEvent, ReadStdbConnectionErrorEvent, ReadStdbDisconnectedEvent,
    StdbConnectedEvent, StdbConnection, StdbConnectionErrorEvent, StdbDisconnectedEvent,
    StdbPlugin, tables,
};

use crate::{
    GameState,
    bindings::{DbConnection, StationsTableAccess},
};

pub struct SpacetimeDbPlugin;

pub type SpacetimeDB<'a> = Res<'a, StdbConnection<DbConnection>>;

impl Plugin for SpacetimeDbPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            StdbPlugin::default()
                .with_connection(|send_connected, send_disconnected, send_error, _| {
                    let conn = DbConnection::builder()
                        .with_uri("https://stdb.jlavocat.eu")
                        .with_module_name("nova9")
                        .with_light_mode(true)
                        .on_connect(move |_, _, _| {
                            send_connected.send(StdbConnectedEvent {}).unwrap();
                        })
                        .on_disconnect(move |_, err| {
                            send_disconnected
                                .send(StdbDisconnectedEvent { err })
                                .unwrap();
                        })
                        .on_connect_error(move |_, err| {
                            send_error.send(StdbConnectionErrorEvent { err }).unwrap();
                        })
                        .build()
                        .unwrap();

                    conn.run_threaded();

                    conn
                })
                .with_events(|plugin, app, db, _| {
                    tables!(stations);
                }),
        )
        .add_systems(
            PreUpdate,
            (on_connected, on_connection_error, on_disconnected).chain(),
        )
        .add_systems(
            Update,
            ensure_connected.run_if(in_state(GameState::WaitingForConnection)),
        );
    }
}

fn on_connected(mut events: ReadStdbConnectedEvent, stdb: SpacetimeDB) {
    for _ in events.read() {
        info!(
            "Connected to SpacetimeDB with identity: {}",
            stdb.identity()
        );
    }
}

fn on_connection_error(mut events: ReadStdbConnectionErrorEvent) {
    for event in events.read() {
        error!("Error connecting to SpacetimeDB: {:?}", event.err);
    }
}

fn on_disconnected(mut events: ReadStdbDisconnectedEvent) {
    for event in events.read() {
        warn!("Disconnected from SpacetimeDB: {:?}", event.err);
    }
}

fn ensure_connected(stdb: SpacetimeDB, mut next_state: ResMut<NextState<GameState>>) {
    if stdb.is_active() {
        next_state.set(GameState::InGame);
    }
}
