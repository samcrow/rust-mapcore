use projection::Projection;
use layer::Layer;

use std::rc::Rc;
use std::cell::RefCell;

/// Represents a map view
pub struct Map {
    /// The projection
    projection: Box<Projection>,
    /// The layers
    layers: Vec<Rc<RefCell<Layer>>>,
}

impl Map {
    pub fn new<P>(projection: P) -> Map where P: 'static + Projection {
        Map {
            projection: Box::new(projection),
            layers: Vec::new(),
        }
    }

    pub fn draw(&self, )
}
