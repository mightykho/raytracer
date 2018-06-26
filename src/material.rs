use image;
use image::{DynamicImage, GenericImage};
use serde::{Deserialize, Deserializer};
use color::Color;
use vector::{Vector2, Vector3};
use std::fmt;
use std::path::PathBuf;
use rand::prelude::*;

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
    pub albedo: f64,
    #[serde(default="Material::default_opacity")]
    pub opacity: f64,
    #[serde(default="Material::default_refraction_index")]
    pub refraction_index: f64,
    #[serde(default="Material::default_refraction_color")]
    pub refraction_color: Color
}

#[derive(Deserialize)]
pub enum Coloration {
    Color(Color),
    Texture(
        #[serde(deserialize_with="Coloration::load_texture")]
        DynamicImage)
}

pub enum RayBehavior {
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
            albedo: Self::default_albedo(),
            opacity: Self::default_opacity(),
            refraction_index: Self::default_refraction_index(),
            refraction_color: Self::default_refraction_color()
        }
    }
}

impl Material {
    fn default_color() -> Coloration { Coloration::Color(Color::white()) }
    fn default_fizziness() -> f64 { 0.01 }
    fn default_reflection() -> f64 { 0.0 }
    fn default_reflection_color() -> Color { Color::white() }
    fn default_albedo() -> f64 { 0.08 }
    fn default_opacity() -> f64 { 1.0 }
    fn default_refraction_index() -> f64 { 1.5 }
    fn default_refraction_color() -> Color { Color::white() }

    pub fn uses_texture(&self) -> bool {
        match self.color {
            Coloration::Color(_) => false,
            Coloration::Texture(_) => true
        }
    }

    pub fn scatter(&self, vec: &Vector3, normal: &Vector3, texture_coords: &Vector2) -> (Vector3, Color) {
        let mut rng = thread_rng();
        let rand: f64 = rng.gen();
        let mut behavior = RayBehavior::Diffuse;
        let mut output_vec = Vector3::zero();

        if rand > self.opacity {
            let rand: f64 = rng.gen();
            let vec_and_behavior = self.try_refraction(rand, vec, normal);

            output_vec = vec_and_behavior.0;
            behavior = vec_and_behavior.1;
        } else if rand < self.reflection {
            output_vec = self.reflect(vec, normal);
            behavior = RayBehavior::Reflect;
        } else {
            output_vec = normal.add(&Vector3::random_unit());
            behavior = RayBehavior::Diffuse;
        }

        (output_vec, self.diffuse_color(behavior, texture_coords))
    }

    fn schlick(&self, cosine: f64) -> f64 {
        let r = (1.0 - self.refraction_index) / (1.0 + self.refraction_index);
        let r2 = r * r;

        r2 + (1.0 - r2) * (1.0 - cosine).powi(5)
    }

    fn try_refraction(&self, rand: f64, vec: &Vector3, normal: &Vector3) -> (Vector3, RayBehavior) {
        let mut outward_normal;
        let mut cosine = -vec.dot(&normal) / vec.magnitude();
        let mut ni_over_nt = self.refraction_index;

        if vec.dot(&normal) > 0.0 {
            outward_normal = normal.neg();
            cosine = self.refraction_index * cosine;
        } else {
            outward_normal = normal.clone();
            ni_over_nt = 1.0 / ni_over_nt;
        }

        let reflection_prob = self.schlick(cosine);

        if reflection_prob > rand {
            return (self.reflect(vec, normal), RayBehavior::Reflect)
        }

        if let Some(refracted) = self.refract(vec, &outward_normal, ni_over_nt) {
            (refracted, RayBehavior::Refract)
        } else {
            (self.reflect(vec, normal), RayBehavior::Reflect)
        }
    }

    pub fn diffuse_color(&self, behavior: RayBehavior, texture_coordinate: &Vector2) -> Color {
        match behavior {
            RayBehavior::Diffuse => self.color_at(texture_coordinate),
            RayBehavior::Reflect => self.reflection_color.clone(),
            RayBehavior::Refract => self.refraction_color.clone()
        }
    }

    pub fn color_at(&self, coords: &Vector2) -> Color {
        self.color.color_at(&coords)
    }

    fn refract(&self, vec: &Vector3, normal: &Vector3, ni_over_nt: f64) -> Option<Vector3> {
        let vec = vec.normalize();
        let dt = vec.dot(normal);
        let disc = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

        if disc > 0.0 {
            Some(vec.subtract(&normal.multiply(dt)).multiply(ni_over_nt).subtract(&normal.multiply(disc.sqrt())))
        } else {
            None
        }
    }

    fn reflect(&self, vec: &Vector3, normal: &Vector3) -> Vector3 {
        vec.reflect(normal).add(&Vector3::random_unit().multiply(self.fizziness))
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
