use super::Context;
use wgpu_canvas::{Area as CanvasArea, Item as CanvasItem, Text, Shape};

#[derive(Clone, Copy, Debug)]
pub struct Area {
    pub offset: (f32, f32),
    pub size: (f32, f32)
}

/// Trait for layouts that determine the offset and allotted sizes of its children
pub trait Layout: std::fmt::Debug {

    /// Given a list of children size requests calculate the size request for the total layout
   fn request_size(&self, ctx: &mut Context, children: Vec<SizeRequest>) -> SizeRequest;

    /// Given an allotted size and the list of chlidren size requests (which may respect the size request),
    /// calculate the actual offsets and allotted sizes for its children
    fn build(&self, ctx: &mut Context, size: (f32, f32), children: Vec<SizeRequest>) -> Vec<Area>;
}

/// Structure used to designate space to a component or drawable.
///
/// A `SizeRequest` specifies the minimum and maximum dimensions that a
/// component is able to occupy. Layout systems can use this
/// information to determine how to allocate space during rendering.
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct SizeRequest {
    min_width: f32,
    min_height: f32,
    max_width: f32,
    max_height: f32,
}
impl SizeRequest {
    /// Returns the minimum width.
    pub fn min_width(&self) -> f32 { self.min_width }

    /// Returns the minimum height.
    pub fn min_height(&self) -> f32 { self.min_height }

    /// Returns the maximum width.
    pub fn max_width(&self) -> f32 { self.max_width }

    /// Returns the maximum height.
    pub fn max_height(&self) -> f32 { self.max_height }

    /// Creates a new `SizeRequest`, panicking if min > max for either dimension.
    pub fn new(min_width: f32, min_height: f32, max_width: f32, max_height: f32) -> Self {
        if min_width > max_width { panic!("Min Width was Greater Than Max Width"); }
        if min_height > max_height { panic!("Min Height was Greater Than Max Height"); }
        SizeRequest { min_width, min_height, max_width, max_height }
    }

    /// Creates a fixed-size `SizeRequest` where min and max are equal.
    pub fn fixed(size: (f32, f32)) -> Self {
        SizeRequest { min_width: size.0, min_height: size.1, max_width: size.0, max_height: size.1 }
    }

    /// Creates a `SizeRequest` that can expand to fill all available space.
    pub fn fill() -> Self {
        SizeRequest { min_width: 0.0, min_height: 0.0, max_width: f32::MAX, max_height: f32::MAX }
    }

    /// Clamps a given size into this request's min/max bounds.
    pub fn get(&self, size: (f32, f32)) -> (f32, f32) {
        (
            self.max_width.min(self.min_width.max(size.0)),
            self.max_height.min(self.min_height.max(size.1))
        )
    }

    /// Returns a new request with both width and height increased.
    pub fn add(&self, w: f32, h: f32) -> SizeRequest {
        self.add_width(w).add_height(h)
    }

    /// Returns a new request with width increased.
    pub fn add_width(&self, w: f32) -> SizeRequest {
        SizeRequest::new(self.min_width + w, self.min_height, self.max_width + w, self.max_height)
    }

    /// Returns a new request with height increased.
    pub fn add_height(&self, h: f32) -> SizeRequest {
        SizeRequest::new(self.min_width, self.min_height + h, self.max_width, self.max_height + h)
    }

    /// Returns a new request with height decreased.
    pub fn remove_height(&self, h: f32) -> SizeRequest {
        SizeRequest::new(self.min_width, self.min_height - h, self.max_width, self.max_height - h)
    }

    /// Returns the combined maximum of two requests.
    pub fn max(&self, other: &Self) -> SizeRequest {
        SizeRequest::new(
            self.min_width.max(other.min_width),
            self.min_height.max(other.min_height),
            self.max_width.max(other.max_width),
            self.max_height.max(other.max_height)
        )
    }
}

/// A simple stack layout that overlays children on top of each other.
#[derive(Debug, Clone, Copy)]
pub struct DefaultStack;
impl Layout for DefaultStack {
    fn request_size(&self, _ctx: &mut Context, children: Vec<SizeRequest>) -> SizeRequest {
        children.into_iter().reduce(|c, o| c.max(&o)).unwrap()
    }

    fn build(&self, _ctx: &mut Context, size: (f32, f32), children: Vec<SizeRequest>) -> Vec<Area> {
        children.into_iter().map(|c| Area{offset: (0.0, 0.0), size: c.get(size)}).collect()
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Scale(pub f64);
impl Scale {
    pub fn physical(&self, x: f32) -> f32 {
        (x as f64 * self.0) as f32
    }

    pub fn logical(&self, x: f32) -> f32 {
        (x as f64 / self.0) as f32
    }
}

pub(crate) trait Scaling {
    fn scale(self, scale: &Scale) -> Self;

    fn scale_text(mut text: Text, scale: &Scale) -> Text {
        text.scale = scale.0 as f32;
        text
    }

    fn scale_shape(shape: Shape, scale: &Scale) -> Shape {
        match shape {
            Shape::Ellipse(s, size, rot) => Shape::Ellipse(scale.physical(s), Self::scale_size(size, scale), rot),
            Shape::Rectangle(s, size, rot) => Shape::Rectangle(scale.physical(s), Self::scale_size(size, scale), rot),
            Shape::RoundedRectangle(s, size, r, rot) => Shape::RoundedRectangle(
                scale.physical(s), Self::scale_size(size, scale), scale.physical(r), rot
            ),
        }
    }

    fn scale_size(size: (f32, f32), scale: &Scale) -> (f32, f32) {
        (scale.physical(size.0), scale.physical(size.1))
    }
}
impl Scaling for CanvasItem {
    fn scale(self, scale: &Scale) -> Self {
        match self {
            CanvasItem::Shape(shape, color) => CanvasItem::Shape(
                Self::scale_shape(shape, scale), color
            ),
            CanvasItem::Image(shape, image, color) => CanvasItem::Image(
                Self::scale_shape(shape, scale), image, color
            ),
            CanvasItem::Text(text) => CanvasItem::Text(Self::scale_text(text, scale))
        }
    }
}
impl Scaling for CanvasArea {
    fn scale(self, scale: &Scale) -> Self {
        CanvasArea(
            (scale.physical(self.0.0), scale.physical(self.0.1)),
            self.1.map(|(x, y, w, h)| (
                scale.physical(x), scale.physical(y),
                scale.physical(w), scale.physical(h)
            ))
        )
    }
}
