

/// Provides projections between a sphere and the map view
pub mod projection;
/// Layers that can be drawn on the map
pub mod layer;
/// Represents a map
pub mod map;

/// Stores a latitude and longitude
#[derive(Debug,Clone,PartialEq)]
pub struct LatLon {
    /// Latitude, degrees
    pub latitude: f64,
    /// Longitude, degrees
    pub longitude: f64,
}

impl LatLon {
    /// Returns a LatLon that is dimetrically opposite from this (on the other side of the planet)
    pub fn antipode(&self) -> LatLon {
        LatLon {
            latitude: normalize_latitude(self.latitude + 180.0),
            longitude: normalize_longitude(self.longitude + 180.0),
        }
    }
}

/// Normalizes a latitude into the range [-90, 90]
pub fn normalize_latitude(latitude: f64) -> f64 {
    let radians = latitude.to_radians();
    let result = f64::atan(f64::sin(radians) / f64::abs(f64::cos(radians)));
    result.to_degrees()
}
/// Normalizes a longitude into the range [-180, 180]
pub fn normalize_longitude(longitude: f64) -> f64 {
    let radians = longitude.to_radians();
    let result = f64::atan2(f64::sin(radians), f64::cos(radians));
    result.to_degrees()
}

/// Stores a point
#[derive(Debug,Clone,PartialEq)]
pub struct Point {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
}

#[test]
fn test_normalize_lat_identity_1() {
    assert_eq!(10.0, normalize_latitude(10.0));
}
#[test]
fn test_normalize_lat_identity_2() {
    assert_eq!(90.0, normalize_latitude(90.0));
}
#[test]
fn test_normalize_lon_identity_1() {
    assert_eq!(10.0, normalize_longitude(10.0));
}
#[test]
fn test_normalize_lon_identity_2() {
    assert_eq!(90.0, normalize_longitude(90.0));
}

#[test]
fn test_antipode_north_pole() {
    let north_pole = LatLon { latitude: 90.0, longitude: 0.0 };
    let antipode = north_pole.antipode();
    assert!(close_enough(-90.0, antipode.latitude));
}
#[test]
fn test_antipode_south_pole() {
    let south_pole = LatLon { latitude: -90.0, longitude: 0.0 };
    let antipode = south_pole.antipode();
    assert!(close_enough(90.0, antipode.latitude));
}
#[test]
fn test_antipode_zero_zero() {
    let point = LatLon { latitude: 0.0, longitude: 0.0 };
    let antipode = point.antipode();
    assert!(close_enough(0.0, antipode.latitude));
    assert!(close_enough(180.0, antipode.longitude));
}
#[test]
fn test_antipode_zero_90() {
    let point = LatLon { latitude: 0.0, longitude: 90.0 };
    let antipode = point.antipode();
    assert!(close_enough(0.0, antipode.latitude));
    assert!(close_enough(-90.0, antipode.longitude));
}

#[cfg(test)]
fn close_enough(a: f64, b: f64) -> bool {
    let result = (a - b).abs() < 0.001;
    if !result {
        println!("a = {} and b = {} are not close enough", a, b);
    }
    result
}
