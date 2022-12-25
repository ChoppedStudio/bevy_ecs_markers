//! bevy_ecs_markers is a helper for bevy_ecs which adds the support to mark entites
//!
//! # Features
//! - **proc** *(default)* &mdash; re-exports procedual macros from `bevy_ecs_markers_macros`
//! - **full_bevy** &mdash; Uses full bevy engine and add helpers for it.
//!
//! # Getting Started
//! There are 2 standard types of [`EntityMarker`]s.
//!
//! ## Single Entity Markers
//! Single Entity Markers are markers that just hold one marked entity.
//! They can automaticly be created by deriving `EntityMarker` (as long as the feature `proc` is set) on a struct.
//!
//! ### Example
//! ```
//! #[derive(EntityMarker)]
//! pub struct CameraFollowTarget;
//!
//! fn get_single(marker: Marker<CameraFollowTarget>, example_query: Query<(), With<PossibleTarget>>) {
//!     if let Ok(target) = example_query.get_mut(**marker) {
//!         // ...
//!     }
//! }
//!
//! fn set_single(marker: MarkerMut<CameraFollowTarget>) {
//!     let id = /* ... */;
//!     **marker = id;
//! }
//! ```
//!
//! ## Value Entity Markers
//! Value Entity Markers are markers that hold an entity per value.
//! These will be automaticly generated when deriving `EntityMarker` (as long as the feature `proc` is set) on an enum.
//!
//! ### Example
//! ```
//! #[derive(EntityMarker)]
//! pub enum Players {
//!     Red,
//!     Blue,
//!     Green
//! }
//!
//! fn get_value(marker: Marker<Players>, example_query: Query<(), With<Player>>) {
//!     if let Ok(target) = example_query.get_mut(marker[Players::Red]) {
//!         // ...
//!     }
//! }
//!
//! fn set_value(marker: MarkerMut<Players>) {
//!     let id = /* ... */;
//!     marker[Players::Green] = id;
//! }
//! ```
//!
//! ## Custom Entity Markers
//! The `EntityMarker` derive from above creates a matching MarkerData struct with matching util functions.
//! When you want custom MarkerData for your EntityMarker, you can create it easily.
//!
//! ### Example
//! ```
//! pub struct DualMarker;
//! impl EntityMarker for DualMarker {
//!     type MarkerData = DualMarkerData;
//! }
//!
//! #[derive(Resource)]
//! pub struct DualMarkerData(Entity, Entity);
//! impl DualMarkerData {
//!     pub fn set(&mut self, first: Entity, second: Entity) {
//!         self.0 = first;
//!         self.1 = second;
//!     }
//!
//!     pub fn first(&self) -> Entity {
//!         self.0
//!     }
//!
//!     pub fn second(&self) -> Entity {
//!         self.1
//!     }
//! }
//!
//! impl Default for DualMarkerData {
//!     fn default() -> Self {
//!         // ...
//!     }
//! }
//! ```
mod entity_marker;
mod init_markerdata;
mod marker_data;
pub mod params;

pub use entity_marker::*;
pub use init_markerdata::*;
pub use marker_data::*;

#[cfg(feature = "proc")]
pub use bevy_ecs_markers_macros::EntityMarker;
