use std::hash::Hash;

use crate::MarkerData;

pub trait EntityMarker: Sync + Send + Eq + Hash {
    fn new_data() -> MarkerData<Self>
    where
        Self: Sized;
}
