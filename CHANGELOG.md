# 1.0.3

* Make generated MarkerData having same visibility as `EntityMarker` struct.

# 1.0.2

* Rename `#[marker]` to `#[entity_marker]` due to nameing problems

# 1.0.1

* Add `#[marker]` attribute to `#[derive(EntityMarker)]`

# 1.0.0

* Split up MarkerData in smaller and custom traits
* Add `init_markerdata` helpers
* Move `unit_index` to `ValueMarkerData` exclusively
* Restructure examples
* Add new examples
* Remove inline functions from `Marker` and `MarkerMut`, just `Deref` to the provided `MarkerData`
* Add possibility to make custom MarkerData traits
