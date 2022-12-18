use std::hash::Hash;

pub trait EntityMarker: Sync + Send + Eq + Hash {}
