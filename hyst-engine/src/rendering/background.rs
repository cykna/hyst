use hyst_math::vectors::Vec4f32;

#[derive(Debug, Clone)]
pub enum Background {
    Transparent,
    Solid(Vec4f32),
    Gradient {
        top_left: Vec4f32,
        top_right: Vec4f32,
        bottom_left: Vec4f32,
        bottom_right: Vec4f32,
    },
}
