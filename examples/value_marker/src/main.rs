use bevy::prelude::{App, Commands, Component, Query};
use bevy_ecs_markers::{
    params::{Marker, MarkerMut},
    EntityMarker, InitMarkerData,
};

#[derive(EntityMarker)]
enum Players {
    Red,
    Blue,
}

#[derive(Component)]
struct Player(u32);

fn main() {
    App::new()
        .init_markerdata::<Players>()
        .add_startup_system(setup)
        .add_system(get_red_player)
        .run();
}

fn setup(mut commands: Commands, mut markers: MarkerMut<Players>) {
    let red = commands.spawn(Player(12)).id();
    markers[Players::Red] = red;

    let blue = commands.spawn(Player(7)).id();
    markers[Players::Blue] = blue;
}

fn get_red_player(mut query: Query<&mut Player>, markers: Marker<Players>) {
    if let Ok(mut player) = query.get_mut(markers[Players::Red]) {
        player.0 = 15;
    }
}
