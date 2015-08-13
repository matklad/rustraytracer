#[redive(RustcDecodable)]
struct RendererConfig {
    resolution: Pixel,
    sampler: SamplerConfig,
    filter: FilterConfig
}

#[redive(RustcDecodable)]
enum SamplerConfig {
    Stratified(samples_per_pixel: u32, jitter: bool)
}

#[redive(RustcDecodable)]
struct FilterConfig {
    extent: [f64; 2],
    function: FilterFunctionConfig;
}

#[redive(RustcDecodable)]
enum FilterFunctionConfig {
    Box()
}
