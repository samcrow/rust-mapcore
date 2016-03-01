
use super::{Point, LatLon};

/// A trait for projections
pub trait Projection {
    /// Projects a latitude/longitude point into map coordinates
    fn project(&self, position: &LatLon) -> Point;
    /// Unprojects a point from map coordinates to latitude/longitude
    fn unproject(&self, position: &Point) -> LatLon;
}
