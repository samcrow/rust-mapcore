use std::ops::{Add, Sub, Mul};
use std::iter::FromIterator;

/// Provides projections between a sphere and the map view
pub mod projection;
/// Implements a stereographic projection
pub mod stereographic;
/// Implements a simple equirectangular projection
pub mod equirectangular;
/// Layers that can be drawn on the map
pub mod layer;
/// Represents a map
pub mod map;

/// Represents a latitude, in degrees
#[derive(Debug,Copy,Clone,PartialEq,PartialOrd)]
pub struct Latitude(pub f64);
/// Represents a longitude, in degrees
#[derive(Debug,Copy,Clone,PartialEq,PartialOrd)]
pub struct Longitude(pub f64);

impl Latitude {
    /// Converts this latitude into an angle in radians
    pub fn to_radians(self) -> f64 {
        f64::to_radians(self.into())
    }
}
impl Longitude {
    /// Converts this longitude into an angle in radians
    pub fn to_radians(self) -> f64 {
        f64::to_radians(self.into())
    }
}
impl Into<f64> for Latitude {
    fn into(self) -> f64 {
        self.0
    }
}
impl Into<f64> for Longitude {
    fn into(self) -> f64 {
        self.0
    }
}
impl From<f64> for Latitude {
    fn from(value: f64) -> Self {
        Latitude(value)
    }
}
impl From<f64> for Longitude {
    fn from(value: f64) -> Self {
        Longitude(value)
    }
}
impl Sub for Latitude {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::from(self.0 - rhs.0)
    }
}
impl Sub for Longitude {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::from(self.0 - rhs.0)
    }
}
impl Add for Latitude {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::from(self.0 + rhs.0)
    }
}
impl Add for Longitude {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::from(self.0 + rhs.0)
    }
}

/// Stores a latitude and longitude
#[derive(Debug,Clone,PartialEq)]
pub struct LatLon {
    /// Latitude, degrees
    pub latitude: Latitude,
    /// Longitude, degrees
    pub longitude: Longitude,
}

impl LatLon {
    /// Returns a LatLon that is dimetrically opposite from this (on the other side of the planet)
    pub fn antipode(&self) -> LatLon {
        LatLon {
            latitude: normalize_latitude(self.latitude + Latitude(180.0)),
            longitude: normalize_longitude(self.longitude + Longitude(180.0)),
        }
    }
}

///
/// A rectangle in latitude and longitude
///
#[derive(Debug,Clone,PartialEq)]
pub struct LatLonRect {
    /// The north latitude (always >= south)
    north: Latitude,
    /// The south latitude
    south: Latitude,
    /// The east latitude (always >= west)
    east: Longitude,
    /// The west latitude
    west: Longitude,
}

impl LatLonRect {
    pub fn from_bounds(north: Latitude, south: Latitude, east: Longitude, west: Longitude) -> LatLonRect {
        LatLonRect {
            north: north,
            south: south,
            east: east,
            west: west,
        }
    }
    pub fn from_corners(northwest: &LatLon, southeast: &LatLon) -> LatLonRect {
        LatLonRect {
            north: northwest.latitude,
            south: southeast.latitude,
            east: southeast.longitude,
            west: northwest.longitude,
        }
    }
    pub fn north(&self) -> Latitude {
        self.north
    }
    pub fn south(&self) -> Latitude {
        self.south
    }
    pub fn east(&self) -> Longitude {
        self.east
    }
    pub fn west(&self) -> Longitude {
        self.west
    }
    pub fn set_north(&mut self, north: Latitude) {
        self.north = north
    }
    pub fn set_south(&mut self, south: Latitude) {
        self.south = south
    }
    pub fn set_east(&mut self, east: Longitude) {
        self.east = east
    }
    pub fn set_west(&mut self, west: Longitude) {
        self.west = west
    }
}

/// Normalizes a latitude into the range [-90, 90]
pub fn normalize_latitude(latitude: Latitude) -> Latitude {
    let radians = latitude.0.to_radians();
    let result = f64::atan(f64::sin(radians) / f64::abs(f64::cos(radians)));
    Latitude(result.to_degrees())
}
/// Normalizes a longitude into the range [-180, 180]
pub fn normalize_longitude(longitude: Longitude) -> Longitude {
    let radians = longitude.0.to_radians();
    let result = f64::atan2(f64::sin(radians), f64::cos(radians));
    Longitude(result.to_degrees())
}

/// Stores a point
#[derive(Debug,Clone,PartialEq)]
pub struct Point<N> {
    /// X coordinate
    pub x: N,
    /// Y coordinate
    pub y: N,
}

impl Point<f64> {
    /// Returns a point at (0, 0)
    pub fn origin() -> Point<f64> {
        Point { x: 0f64, y: 0f64 }
    }
}

impl<N> Add for Point<N> where N: Add<Output = N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl<N> Sub for Point<N> where N: Sub<Output = N> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl<N> Mul<N> for Point<N> where N: Mul<N, Output = N> + Clone {
    type Output = Self;
    fn mul(self, rhs: N) -> Self {
        Point {
            x: self.x * rhs.clone(),
            y: self.y * rhs,
        }
    }
}

///
/// A polygon
///
/// P is the type used to represent a point
///
pub struct Polygon<P> {
    /// The points in this quadrilateral, in clockwise order
    points: Vec<P>,
}

impl<P> Polygon<P> where P: Clone {
    pub fn new(points: &[P]) -> Polygon<P> {
        Polygon {
            points: points.iter().cloned().collect(),
        }
    }
}

impl<P> Polygon<P> {
    pub fn points<'a>(&'a self) -> &'a [P] {
        &self.points
    }
}

impl<P> FromIterator<P> for Polygon<P> {
    fn from_iter<T>(iterator: T) -> Self where T: IntoIterator<Item = P> {
        Polygon {
            points: Vec::from_iter(iterator)
        }
    }
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
