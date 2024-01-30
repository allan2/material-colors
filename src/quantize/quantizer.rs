use ahash::HashMap;
use indexmap::IndexMap;

use crate::utils::color::Argb;

pub trait Quantizer {
    fn quantize(
        &mut self,
        pixels: &[Argb],
        max_colors: i32,
        return_input_pixel_to_cluster_pixel: Option<bool>,
    ) -> QuantizerResult;
}

pub struct QuantizerResult {
    pub color_to_count: IndexMap<Argb, u32>,
    pub input_pixel_to_cluster_pixel: HashMap<Argb, Argb>,
}
