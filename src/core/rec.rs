use crate::core::point::Point;

/// A rectangle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Rec<T = f32> {
    /// X coordinate of the top-left corner.
    pub x: T,

    /// Y coordinate of the top-left corner.
    pub y: T,

    /// Width of the rectangle.
    pub width: T,

    /// Height of the rectangle.
    pub height: T,
}

impl Rec<f32> {
    /// Returns true if the given [`Point`] is contained in the [`Rec`].
    ///
    /// [`Point`]: struct.Point.html
    /// [`Rec`]: struct.Rec.html
    pub fn contains(&self, point: Point) -> bool {
        self.x <= point.x
            && point.x <= self.x + self.width
            && self.y <= point.y
            && point.y <= self.y + self.height
    }
}

impl std::ops::Mul<f32> for Rec<u32> {
    type Output = Self;

    fn mul(self, scale: f32) -> Self {
        Self {
            x: (self.x as f32 * scale).round() as u32,
            y: (self.y as f32 * scale).round() as u32,
            width: (self.width as f32 * scale).round() as u32,
            height: (self.height as f32 * scale).round() as u32,
        }
    }
}

