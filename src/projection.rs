
use super::{Point, LatLon, normalize_latitude, normalize_longitude};

/// A trait for projections
pub trait Projection {
    /// Projects a latitude/longitude point into map coordinates
    fn project(&self, position: &LatLon) -> Point;
    /// Unprojects a point from map coordinates to latitude/longitude
    fn unproject(&self, position: &Point) -> LatLon;
}

/// A stereographic projection around a projection point
pub struct StereographicProjection {
    /// The projection point
    projection_point: LatLon,
}

impl StereographicProjection {
    pub fn new(projection_point: LatLon) -> StereographicProjection {
        StereographicProjection {
            projection_point: projection_point,
        }
    }

    /// Returns the projection point of this projection
    pub fn projection_point(&self) -> LatLon {
        self.projection_point.clone()
    }
    /// Sets the projection point
    pub fn set_projection_point(&mut self, point: LatLon) {
        self.projection_point = point;
    }
}

impl Projection for StereographicProjection {
    fn project(&self, position: &LatLon) -> Point {
        // Calculate a position relative to the projection point
        let zenith_radians = (position.latitude - self.projection_point.latitude).to_radians();
        let azimuth_radians = (position.longitude - self.projection_point.longitude).to_radians();
        // Project
        let r = f64::sin(zenith_radians) / (1.0 - f64::cos(zenith_radians));
        let theta = azimuth_radians;
        // Convert to rectangular coordinates
        let x = r * theta.cos();
        let y = r * theta.sin();

        Point { x: x, y: y }
    }
    fn unproject(&self, position: &Point) -> LatLon {
        // Convert to polar coordinates
        let r = f64::hypot(position.x, position.y);
        let theta = f64::atan2(position.y, position.x);
        // Unproject
        let zenith_radians = 2.0 * f64::atan(1.0 / r);
        let azimuth_radians = theta;
        // Convert to lat/lon
        let latitude = normalize_latitude(zenith_radians.to_degrees() + self.projection_point.latitude);
        let longitude = normalize_longitude(azimuth_radians.to_degrees() + self.projection_point.longitude);
        LatLon {
            latitude: latitude,
            longitude: longitude,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::{LatLon, Point};

    #[test]
    fn test_stereographic_identity_1() {
        let center = LatLon { latitude: 0.0, longitude: 0.0 };
        let stereo = StereographicProjection::new(center.clone());
        assert_eq!(center, stereo.projection_point());

        let ll = LatLon { latitude: 47.6609, longitude: -122.2816 };
        let projected = stereo.project(&ll);
        let unprojected = stereo.unproject(&projected);

        println!("Stereographic: {:?} => {:?} => {:?}", ll, projected, unprojected);
        assert_eq!(ll, unprojected);
    }
    #[test]
    fn test_stereographic_identity_2() {
        let center = LatLon { latitude: 47.6609, longitude: -122.2816 };
        let stereo = StereographicProjection::new(center.clone());
        assert_eq!(center, stereo.projection_point());

        let ll = LatLon { latitude: 37.4096, longitude: -122.299 };
        let projected = stereo.project(&ll);
        let unprojected = stereo.unproject(&projected);

        println!("Stereographic: {:?} => {:?} => {:?}", ll, projected, unprojected);
        assert_eq!(ll, unprojected);
    }
    #[test]
    fn test_stereographic_identity_3() {
        let center = LatLon { latitude: 47.6609, longitude: -122.2816 };
        let stereo = StereographicProjection::new(center.clone());
        assert_eq!(center, stereo.projection_point());

        let ll = LatLon { latitude: 37.4096, longitude: 122.299 };
        let projected = stereo.project(&ll);
        let unprojected = stereo.unproject(&projected);

        println!("Stereographic: {:?} => {:?} => {:?}", ll, projected, unprojected);
        assert_eq!(ll, unprojected);
    }
    #[test]
    fn test_stereographic_antipode() {
        let center = LatLon { latitude: 47.6609, longitude: -122.2816 };
        let antipode = center.antipode();
        let stereo = StereographicProjection::new(center.clone());
        let projected = stereo.project(&antipode);
        assert_eq!(projected, Point { x: 0.0, y: 0.0 });
    }
}
