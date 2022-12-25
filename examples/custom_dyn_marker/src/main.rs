use bevy::prelude::{App, Commands, Component, Query};
use bevy_ecs_markers::{
    params::{Marker, MarkerMut},
    EntityMarker, InitMarkerData,
};
use players_data::PlayersMarkerData;

mod players_data;

pub struct Players;

impl EntityMarker for Players {
    type MarkerData = PlayersMarkerData;
}

#[derive(Component)]
struct Player(u32);

fn main() {
    App::new()
        .init_markerdata::<Players>()
        .add_startup_system(setup)
        .add_system(get_selected_player)
        .run();
}

fn setup(mut commands: Commands, mut players: MarkerMut<Players>) {
    let red = commands.spawn(Player(12)).id();
    players.add(red);

    let blue = commands.spawn(Player(7)).id();
    players.add(blue);

    players.select(0);
    players.select_next();
}

fn get_selected_player(mut query: Query<&mut Player>, players: Marker<Players>) {
    if let Ok(mut player) = query.get_mut(**players) {
        player.0 = 2;
    }
}
