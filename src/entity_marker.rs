use bevy_ecs::system::Resource;

pub trait EntityMarker: Sync + Send {
    type MarkerData: Resource + Default;
}
