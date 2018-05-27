use vector::Vector3;
use vector::Point;
use color::Color;

#[derive(Deserialize, Debug)]
pub enum Light {
    DirectionalLight(DirectionalLight),
    SphericalLight(SphericalLight)
}

#[derive(Deserialize, Debug)]
pub struct SphericalLight {
    pub position: Point,
    pub intensity: f64,

    #[serde(default="Color::white")]
    pub color: Color,

    #[serde(default="default_cast_shadow")]
    pub cast_shadow: bool,
}

#[derive(Deserialize, Debug)]
pub struct DirectionalLight {
    pub direction: Vector3,
    pub intensity: f64,

    #[serde(default="Color::white")]
    pub color: Color,

    #[serde(default="default_cast_shadow")]
    pub cast_shadow: bool,
}

fn default_cast_shadow() -> bool { true }

impl Light {
    pub fn cast_shadow(&self) -> bool {
        match *self {
            Light::DirectionalLight(ref d) => d.cast_shadow,
            Light::SphericalLight(ref s) => s.cast_shadow,
       }
    }

    pub fn color(&self) -> &Color {
        match *self {
            Light::DirectionalLight(ref d) => &d.color,
            Light::SphericalLight(ref s) => &s.color,
       }
    }

    pub fn distance(&self, point: &Point) -> f64 {
        match *self {
            Light::DirectionalLight(ref _d) => ::std::f64::INFINITY,
            Light::SphericalLight(ref s) => s.position.distance(point)
       }
    }

    pub fn direction_vector(&self, point: &Point) -> Vector3 {
        match *self {
            Light::DirectionalLight(ref d) => d.direction.clone(),
            Light::SphericalLight(ref s) => point.subtract(&s.position)
       }
    }

    pub fn relative_intensity(&self, point: &Point) -> f64 {
        match *self {
            Light::DirectionalLight(ref d) => d.intensity,
            Light::SphericalLight(ref s) => {
                let distance2 = s.position.subtract(point).norm();

                s.intensity / (4.0 * ::std::f64::consts::PI * distance2)
            }
       }
    }
}
