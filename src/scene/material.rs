use color::Color;

#[derive(RustcDecodable)]
pub struct Material {
    pub color: Color,
    pub diffuse: f64,
    pub specular: f64
}
