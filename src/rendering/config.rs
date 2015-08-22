use super::Pixel;

#[derive(Debug, RustcDecodable)]
pub struct TracerConfig {
    pub resolution: Pixel,
    pub sampler: SamplerConfig,
    pub filter: FilterConfig,
    pub n_reflections: u32,
    pub n_threads: u16,
}

#[derive(Debug, RustcDecodable)]
pub enum SamplerConfig {
    Stratified {
        samples_per_pixel: u32,
        jitter: bool
    }
}

#[derive(Debug, RustcDecodable)]
pub struct FilterConfig {
    pub extent: [f64; 2],
    pub function: FilterFunctionConfig
}

#[derive(Debug, RustcDecodable)]
pub enum FilterFunctionConfig {
    Box,
    Gauss(f64)
}
