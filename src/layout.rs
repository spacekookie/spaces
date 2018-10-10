//! A generic layout module
//!
//! Given some screen cordinates and block sizes, it will
//! layout a collection of items in a given shape.
//! It copies the strings that are required (for now) but
//! can be optimised further in the future.

pub mod rhombus;

/// Specify the shape that should be aproximated
pub enum Shape {
    /// A simple block
    Block,
    /// Like a block but _cool_
    Rhombus,
    /// A circle, optionally filled
    Circle(bool),
}

pub struct Layout {
    shape: Shape,
    offset: (i64, i64),
}

pub struct Block {
    text: String,
    position: (i64, i64),
    size: (i64, i64),
}

pub trait LayoutGenerator {
    fn rel_position(index: u64) -> (i64, i64);
}

impl Layout {
    /// Create a new Layout
    pub fn new(shape: Shape, offset: (i64, i64)) -> Self {
        Layout { shape, offset }
    }

    pub fn layout(&self, items: Vec<String>) -> Vec<Block> {
        match self.shape {
            _ => unimplemented!(),
        }
    }
}
