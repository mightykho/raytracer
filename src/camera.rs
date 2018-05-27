use vector::Point;

#[derive(Deserialize, Debug)]
pub struct Camera {
    pub fov: f64,
    pub samples: u32,
    pub diffuse: u32,
    pub position: Point
}
