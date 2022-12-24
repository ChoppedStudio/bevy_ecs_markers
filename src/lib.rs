//! bevy_ecs_markers is a helper for bevy_ecs which adds the support to mark entites
//!
//! # Features
//! - **proc** *(default)* &mdash; re-exports procedual macros from `bevy_ecs_markers_macros`
//! - **full_bevy** &mdash; Uses full bevy engine and add helpers for it.
mod entity_marker;
mod init_markerdata;
mod marker_data;
pub mod params;

pub use entity_marker::*;
pub use init_markerdata::*;
pub use marker_data::*;

#[cfg(feature = "proc")]
pub use bevy_ecs_markers_macros::EntityMarker;
