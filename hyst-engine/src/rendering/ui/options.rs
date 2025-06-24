use hyst_math::Rect;
use smol_str::SmolStr;

use crate::{background::Background, meshes::SizeMethod};

//File containing the options the user will need to pass when creating elements.
//Other options are internals.

#[derive(Debug, Clone)]
pub struct HystBoxOptions {
    pub bg: Background,
    pub rect: Rect,
    pub size_method: SizeMethod,
    pub styles: Vec<SmolStr>,
}

pub struct HystImageOptions {
    pub source: String,
    pub rect: Rect,
    pub styles: Vec<SmolStr>,
    pub size_method: SizeMethod,
}
