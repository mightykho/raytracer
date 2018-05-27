use geometry::Sphere;

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub geometry: Vec<Sphere>,
}
