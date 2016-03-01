
use super::{Point, LatLon, Polygon};

/// A trait for projections
pub trait Projection {
    /// Projects a latitude/longitude point into map coordinates
    fn project(&self, position: &LatLon) -> Point<f64>;
    /// Unprojects a point from map coordinates to latitude/longitude
    fn unproject(&self, position: &Point<f64>) -> LatLon;

    /// Projects a polygon from latitude/longitude into map coordinates
    fn project_poly(&self, poly: &Polygon<LatLon>) -> Polygon<Point<f64>> {
        poly.points().into_iter().map(|ll| self.project(ll)).collect()
    }
    /// Unprojects a polyon from map coordinates into latitude/longitude
    fn unproject_poly(&self, poly: &Polygon<Point<f64>>) -> Polygon<LatLon> {
        poly.points().into_iter().map(|point| self.unproject(point)).collect()
    }
}
