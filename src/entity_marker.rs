use crate::MarkerData;

pub trait EntityMarker: Sync + Send {
    const LENGTH: usize;

    fn new_data() -> MarkerData<Self>
    where
        [(); Self::LENGTH]:,
        Self: Sized;

    fn unit_index(&self) -> usize;
}
