use bevy::prelude::{App, Commands, Component, Query};
use bevy_ecs_markers::{
    params::{Marker, MarkerMut},
    EntityMarker, InitMarkerData,
};

#[derive(EntityMarker)]
#[entity_marker(data_name = "CurrentPlayerMarker")]
struct CurrentPlayer;

#[derive(Component)]
struct Player(u32);

fn main() {
    App::new()
        .init_markerdata::<CurrentPlayer>()
        .add_startup_system(setup)
        .add_system(get_current_player)
        .run();
}

fn setup(mut commands: Commands, mut current: MarkerMut<CurrentPlayer>) {
    let _red = commands.spawn(Player(12)).id();

    let blue = commands.spawn(Player(7)).id();

    **current = blue; // TODO: can we make this just one deref?
}

fn get_current_player(mut query: Query<&mut Player>, current: Marker<CurrentPlayer>) {
    // TODO: can we make this just one deref?
    if let Ok(mut player) = query.get_mut(**current) {
        player.0 = 2;
    }
}
