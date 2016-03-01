use super::{Latitude, Longitude, LatLon, Point};
use projection::Projection;

///
/// A Millier cylindrical projection implementation
///
pub struct MillerCylindricalProjection;

impl Projection for MillerCylindricalProjection {
    fn project(&self, position: &LatLon) -> Point<f64> {
        let x: f64 = position.longitude.into();
        let latitude: f64 = position.latitude.into();
        let y = (5.0 / 4.0) * f64::asinh(f64::tan((4.0 / 5.0) * latitude));
        Point { x: x, y: y }
    }

    fn unproject(&self, position: &Point<f64>) -> LatLon {
        let longitude = (5.0 / 4.0) * f64::atan(f64::sinh((4.0 / 5.0) * position.y));
        LatLon {
            latitude: Latitude(position.y),
            longitude: Longitude(longitude),
        }
    }
}
