use super::LatLonRect;
use projection::Projection;

///
/// Trait for required functionality for map layers
///
pub trait Layer {
    ///
    /// Draws this layer
    ///
    /// The provided Projection is a projection that can map between latitude/longitude and
    /// display coordinates.
    ///
    fn draw(&self, projection: &Projection, x: u32, y: u32, width: u32, height: u32);

    ///
    /// Returns a LatLonRect that bounds the items that this layer displays, or None if
    /// this layer's bounds are not known or if the layer covers the whole globe.
    ///
    fn bounds(&self) -> Option<LatLonRect>;
}
