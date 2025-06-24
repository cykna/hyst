use crate::vectors::Vec4f32;

pub type Rgba = Vec4f32;

impl Rgba {
    pub const WHITE: Rgba = Rgba::new(1.0, 1.0, 1.0, 1.0);
    pub const BLACK: Rgba = Rgba::new(0.0, 0.0, 0.0, 1.0);
    pub const TRANSPARENT: Rgba = Rgba::new(0.0, 0.0, 0.0, 0.0);
    pub const RED: Rgba = Rgba::new(1.0, 0.0, 0.0, 1.0);
    pub const GREEN: Rgba = Rgba::new(0.0, 1.0, 0.0, 1.0);
    pub const BLUE: Rgba = Rgba::new(0.0, 0.0, 1.0, 1.0);

    pub const fn inverse(&self) -> Self {
        Self::new(
            Self::WHITE.x() - self.x(),
            Self::WHITE.y() - self.y(),
            Self::WHITE.z() - self.z(),
            self.w(),
        )
    }
}
