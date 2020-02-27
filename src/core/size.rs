use std::ops::Add;
use winit::dpi::PhysicalSize;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size<T = f32> {
    width: T,
    height: T,
}

impl<T> Size<T> {
    pub fn new(width: T, height: T) -> Self {
        Size { width, height }
    }
}

// impl <T> From<Size<T>> for (u16, u16) where T: From<u16> {
//     fn from(s: Size<T>) -> Self {
//         (s.width.into(), s.height.into())
//     }
// }
// impl <T> Into<(u16, u16)> for Size<T> where T: Into<u16> {
//     fn into(self) -> (u16, u16) {
//         (self.width.into(), self.height.into())
//     }
// }

// impl <P> From<winit::dpi::PhysicalSize<P>> for Size where P: Into<f32> {
//     fn from(p: PhysicalSize<P>) -> Self {
//         Size::new(p.width.into(), p.height.into())
//     }
// }

impl From<[u16; 2]> for Size {
    fn from(s: [u16; 2]) -> Self {
        Size::new(s[0].into(), s[1].into())
    }
}

impl<T> std::ops::Add for Size<T>
    where
        T: std::ops::Add<Output=T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Size::new(self.width + rhs.width, self.height + rhs.height)
    }
}

impl From<winit::dpi::PhysicalSize<u32>> for Size {
    fn from(ps: PhysicalSize<u32>) -> Self {
        Size::new(ps.width as f32, ps.height as f32)
    }
}

impl From<(u16, u16)> for Size {
    fn from(s: (u16, u16)) -> Self {
        Size::new(s.0.into(), s.1.into())
    }
}

impl From<Size> for (u16, u16) {
    fn from(s: Size<f32>) -> Self {
        (s.width as u16, s.height as u16)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_physical_size() {
        let ps = winit::dpi::PhysicalSize::new(100 as u32, 200 as u32);

        let size: Size = ps.into();

        assert_eq!(size.width, 100.0)
    }

    #[test]
    fn to_tuple() {
        let s = Size::new(100.0 as f32, 200.0 as f32);

        let (w, h): (u16, u16) = s.into();

        assert_eq!(w, 100);
    }
}