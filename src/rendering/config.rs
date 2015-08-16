use super::Pixel;

#[derive(RustcDecodable)]
pub struct TracerConfig {
    pub resolution: Pixel,
    pub sampler: SamplerConfig,
    pub filter: FilterConfig
}

#[derive(RustcDecodable)]
pub enum SamplerConfig {
    Stratified {
        samples_per_pixel: u32,
        jitter: bool
    }
}

#[derive(RustcDecodable)]
pub struct FilterConfig {
    pub extent: [f64; 2],
    pub function: FilterFunctionConfig
}

#[derive(RustcDecodable)]
pub enum FilterFunctionConfig {
    Box,
    Gauss(f64)
}
