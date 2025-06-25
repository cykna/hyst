use smol_str::SmolStr;

use crate::background::Background;

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
