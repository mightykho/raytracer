use image;
use image::{DynamicImage, GenericImage};
use serde::{Deserialize, Deserializer};
use color::Color;
use vector::{Vector2, Vector3};
use std::fmt;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Material {
    #[serde(default="Material::default_color")]
    pub color: Coloration,
    #[serde(default="Material::default_reflection")]
    pub reflection: f64,
    #[serde(default="Material::default_reflection_color")]
    pub reflection_color: Color,
    #[serde(default="Material::default_fizziness")]
    pub fizziness: f64,
    #[serde(default="Material::default_albedo")]
    pub albedo: f64
}

#[derive(Deserialize)]
pub enum Coloration {
    Color(Color),
    Texture(
        #[serde(deserialize_with="Coloration::load_texture")]
        DynamicImage)
}

enum RayBehavior {
    Diffuse,
    Reflect,
    Refract
}

impl Default for Material {
    fn default() -> Self {

        Self {
            color: Self::default_color(),
            fizziness: Self::default_fizziness(),
            reflection: Self::default_reflection(),
            reflection_color: Self::default_reflection_color(),
            albedo: Self::default_albedo()
        }
    }
}

impl Material {
    fn default_color() -> Coloration { Coloration::Color(Color::white()) }
    fn default_fizziness() -> f64 { 0.01 }
    fn default_reflection() -> f64 { 0.0 }
    fn default_reflection_color() -> Color { Color::white() }
    fn default_albedo() -> f64 { 0.08 }

    pub fn get_color(&self, texture_coordinate: &Vector2, light_power: f64, light_color: &Color) -> Color {
        let light_reflected = self.albedo / ::std::f64::consts::PI;

        self.color.color_at(texture_coordinate).multiply_color(&light_color).multiply(light_power * light_reflected)
    }

    pub fn uses_texture(&self) -> bool {
        match self.color {
            Coloration::Color(_) => false,
            Coloration::Texture(_) => true
        }
    }

    pub fn scatter(&self, rand: f64, vec: &Vector3, normal: &Vector3) -> Vector3 {
        match self.detect_behavior(rand) {
            RayBehavior::Diffuse => normal.add(&Vector3::random_unit()),
            RayBehavior::Reflect => {
                vec.reflect(normal).add(&Vector3::random_unit().multiply(self.fizziness))
            },
            RayBehavior::Refract => Vector3::zero()
        }
    }

    pub fn diffuse_color(&self, rand: f64, texture_coordinate: &Vector2) -> Color {
        match self.detect_behavior(rand) {
            RayBehavior::Diffuse => self.color_at(texture_coordinate),
            RayBehavior::Reflect => self.reflection_color.clone(),
            RayBehavior::Refract => Color::white()
        }
    }

    pub fn color_at(&self, coords: &Vector2) -> Color {
        self.color.color_at(&coords)
    }

    fn detect_behavior(&self, rand: f64) -> RayBehavior {
        if rand < self.reflection {
            RayBehavior::Reflect
        // } else if 1 - rand < self.refraction {
        //     RayBehavior::Refract
        } else {
            RayBehavior::Diffuse
        }
    }
}

impl Coloration {
    fn color_at(&self, coords: &Vector2) -> Color {
        match *self {
            Coloration::Color(ref color) => { color.clone() }
            Coloration::Texture(ref texture) => {
                let x = Coloration::wrap(coords.x, texture.width());
                let y = Coloration::wrap(coords.y, texture.height());

                Color::from_rgba(texture.get_pixel(x, y))
            }
        }
    }

    pub fn load_texture<'de, D>(deserializer: D) -> Result<DynamicImage, D::Error>
        where D: Deserializer<'de>
    {
        let path = PathBuf::deserialize(deserializer)?;
        Ok(image::open(path).expect("Unable to open texture file"))
    }

    fn wrap(val: f64, bound: u32) -> u32 {
        let signed_bound = bound as i32;
        let float_coord = val * bound as f64;
        let wrapped_coord = (float_coord as i32) % signed_bound;

        if wrapped_coord < 0 {
            (wrapped_coord + signed_bound) as u32
        } else {
            wrapped_coord as u32
        }
    }
}

impl fmt::Debug for Coloration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Coloration::Color(ref c) => write!(f, "Color({:?})", c),
            Coloration::Texture(_) => write!(f, "Texture"),
        }
    }
}
