use std::{env, sync::mpsc::Sender};

use bevy::prelude::*;
use bevy_spacetimedb::{
    AddEventChannelAppExtensions, ReadStdbConnectedEvent, ReadStdbConnectionErrorEvent,
    ReadStdbDisconnectedEvent, StdbConnection, StdbPlugin,
};

use crate::{
    GameState,
    bindings::{
        AsteroidTableAccess, DbConnection, PlayerLocationTableAccess, PlayerTableAccess,
        RemoteTables, ShipLocationTableAccess, ShipPilotTableAccess, ShipTableAccess,
        ShipTypeTableAccess, StationTableAccess,
    },
};

pub type SpacetimeDB<'a> = Res<'a, StdbConnection<DbConnection>>;

#[derive(Event, Default)]
pub struct StaticDataLoadedEvent;

#[derive(Resource)]
pub struct StaticDataLoadedSender(Sender<StaticDataLoadedEvent>);

pub struct SpacetimeDbPlugin;

impl Plugin for SpacetimeDbPlugin {
    fn build(&self, app: &mut App) {
        let uri = env::var("SPACETIMEDB_URI")
            .unwrap_or_else(|_| "https://maincloud.spacetimedb.com".to_string());

        #[cfg(not(feature = "dev"))]
        let module_name =
            env::var("SPACETIME_DB_MODULE").unwrap_or_else(|_| "nova9-staging".to_string());
        #[cfg(feature = "dev")]
        let module_name = "nova9";

        app.add_plugins(
            StdbPlugin::default()
                .with_uri(uri)
                .with_module_name(module_name)
                .with_run_fn(DbConnection::run_threaded)
                .add_table(RemoteTables::asteroid)
                .add_table(RemoteTables::player)
                .add_table(RemoteTables::player_location)
                .add_table(RemoteTables::ship)
                .add_table(RemoteTables::ship_location)
                .add_table(RemoteTables::ship_pilot)
                .add_table(RemoteTables::ship_type)
                .add_table(RemoteTables::station),
        )
        .add_systems(OnEnter(GameState::StaticDataLoading), load_static_data)
        .add_systems(
            PreUpdate,
            (on_connected, on_connection_error, on_disconnected).chain(),
        )
        .add_systems(
            Update,
            (
                ensure_connected.run_if(in_state(GameState::WaitingForConnection)),
                on_static_data_loaded.run_if(in_state(GameState::StaticDataLoading)),
            ),
        );

        let (send, recv) = std::sync::mpsc::channel();
        app.insert_resource(StaticDataLoadedSender(send));
        app.add_event_channel::<StaticDataLoadedEvent>(recv);
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
        next_state.set(GameState::StaticDataLoading);
    }
}

fn load_static_data(stdb: SpacetimeDB, sender: Res<StaticDataLoadedSender>) {
    let sender = sender.0.clone();
    info!("Loading static data...");
    stdb.subscription_builder()
        .on_applied(move |_| {
            sender.send(StaticDataLoadedEvent {}).unwrap();
        })
        .on_error(|_, err| panic!("Static data loading error: {err}"))
        .subscribe(["SELECT * FROM ship_type"]);
}

fn on_static_data_loaded(
    mut events: EventReader<StaticDataLoadedEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for _ in events.read() {
        info!("Static data loaded successfully.");
        next_state.set(GameState::InGame);
    }
}
