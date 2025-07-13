use hyst_math::vectors::{Vec2f32, Vec4f32};
use smol_str::SmolStr;

use crate::background::Background;

use super::pulse::Pulse;

//File containing the options the user will need to pass when creating elements.
//Other options are internals.

#[derive(Debug, Clone)]
pub struct HystBoxOptions {
    pub bg: Background,
    pub style: SmolStr,
}

pub struct HystImageOptions {
    pub source: String,
    pub style: SmolStr,
}

pub struct HystTextOptions {
    pub content: Pulse<String>,
    pub position: Vec2f32,
    pub style: SmolStr,
    pub font_size: f32,
    pub color: Pulse<Vec4f32>,
}
