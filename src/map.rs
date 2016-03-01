use super::{Point, LatLon};
use projection::Projection;
use layer::Layer;

///
/// Represents a map view
///
/// A map uses three coordinate systems:
///
/// * Latitude/longitude
/// * Map coordinates: Map coordinates are in arbitrary units, as defined by the projection.
///   The only constraint on map coordinates is that the X coordinate increases going right
///   and the Y coordinate increases going down.
/// * Display coordinates: Display coordinates are in pixels, with the X coordinate increasing
///   going right and the Y coordinate increasing going down. The origin is located at the
///   upper left corner of the viewport.
///
/// The map's projection converts between latitude/longitude and map coordinates. The map's
/// view projection converts between map coordinates and display coordinates.
///
pub struct Map {
    /// The projection
    projection: Box<Projection>,
    /// The view projection
    view_projection: ViewProjection,
    /// The layers
    layers: Vec<Box<Layer>>,
}

impl Map {
    ///
    /// Creates a new map using a given projection
    ///
    pub fn new<P>(projection: P) -> Map where P: 'static + Projection {
        Map {
            projection: Box::new(projection),
            view_projection: ViewProjection {
                center: Point::origin(),
                zoom: 1f64,
            },
            layers: Vec::new(),
        }
    }

    ///
    /// Sets the projection that this map should use
    ///
    pub fn set_projection<P>(&mut self, projection: P) where P: 'static + Projection {
        self.projection = Box::new(projection)
    }

    ///
    /// Adds a layer to this map. The new layer will be drawn on top of the existing layers.
    ///
    pub fn add_layer<L>(&mut self, layer: L) where L: 'static + Layer {
        self.layers.push(Box::new(layer))
    }

    ///
    /// Removes all layers from this map
    ///
    pub fn clear_layers(&mut self) {
        self.layers.clear()
    }

    ///
    /// Draws this map with its upper left corner at (x, y) with the specified width and height
    ///
    pub fn draw(&self, x: i32, y: i32, width: i32, height: i32) {
        let combined = CombinedProjection::new(self.projection.as_ref(), &self.view_projection, width, height);
        for layer in self.layers.iter() {
            layer.draw(&combined, x, y, width, height);
        }
    }
}

///
/// A linear projection that maps between map coordinates and display coordinates
///
struct ViewProjection {
    /// The location, in map coordinates, where the center of the viewport is located
    center: Point<f64>,
    /// The ratio of the size of a display unit to the size of a map coordinate unit
    zoom: f64,
}

impl ViewProjection {
    /// Projects a point in map coordinates to a point in screen coordinates
    pub fn project(&self, map: &Point<f64>, viewport_width: i32, viewport_height: i32) -> Point<f64> {
        // Calculate the vector from the center point to the map point
        let mut map_vector = map.clone() - self.center.clone();
        // Scale by the zoom ratio
        map_vector = map_vector * self.zoom;
        // map_vector is now the screen position relative to the center
        // Shift it to make it relative to the corner
        map_vector = map_vector + Point { x: (viewport_width / 2) as f64, y: (viewport_height / 2) as f64 };
        map_vector
    }
    /// Unprojects a point from screen coordinates to a point in map coordinates
    pub fn unproject(&self, screen: &Point<f64>, viewport_width: i32, viewport_height: i32) -> Point<f64> {
        // Shift to make it relative to the center
        let mut map_vector = screen.clone() - Point { x: (viewport_width / 2) as f64, y: (viewport_height / 2) as f64 };
        // Scale by inverse zoom ratio
        map_vector = map_vector * (1f64 / self.zoom);
        // Make relative to center point
        map_vector = map_vector + self.center.clone();
        map_vector
    }
}

///
/// A Projection implementation for a combination of a Projection and a ViewProjection
///
struct CombinedProjection<'a, 'b> {
    /// The primary projection
    projection: &'a Projection,
    /// The view projection
    view_projection: &'b ViewProjection,
    /// The width of the viewport in pixels
    viewport_width: i32,
    /// The height of the viewport in pixels
    viewport_height: i32,
}

impl<'a, 'b> CombinedProjection<'a, 'b> {
    pub fn new(projection: &'a Projection, view_projection: &'b ViewProjection, viewport_width: i32, viewport_height: i32) -> CombinedProjection<'a, 'b> {
        CombinedProjection {
            projection: projection,
            view_projection: view_projection,
            viewport_width: viewport_width,
            viewport_height: viewport_height,
        }
    }
}

impl<'a, 'b> Projection for CombinedProjection<'a, 'b> {
    fn project(&self, position: &LatLon) -> Point<f64> {
        let map = self.projection.project(position);
        self.view_projection.project(&map, self.viewport_width, self.viewport_height)
    }
    fn unproject(&self, position: &Point<f64>) -> LatLon {
        let map = self.view_projection.unproject(position, self.viewport_width, self.viewport_height);
        self.projection.unproject(&map)
    }
}
