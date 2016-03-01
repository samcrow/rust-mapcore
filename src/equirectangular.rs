use super::{Latitude, Longitude, LatLon, Point};
use projection::Projection;

///
/// A simple plate carrée equirectangular projection
///
pub struct EquirectangularProjection;

impl Projection for EquirectangularProjection {
    fn project(&self, position: &LatLon) -> Point {
        let x: f64 = position.longitude.into();
        let y: f64 = position.latitude.into();
        Point { x: x, y: y }
    }

    fn unproject(&self, position: &Point) -> LatLon {
        LatLon {
            latitude: Latitude(position.y),
            longitude: Longitude(position.x),
        }
    }
}
