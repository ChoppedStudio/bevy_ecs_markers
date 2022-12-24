use bevy_ecs::system::Resource;

/// Defines which MarkerData to use for this EntityMarker
///
/// ## Hint
/// `#[derive(EntityMarker)]` automaticly creates and links a matching MarkerData type for the type to which the derive is applied to.
pub trait EntityMarker: Sync + Send {
    type MarkerData: Resource + Default;
}
