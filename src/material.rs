use image;
use image::{DynamicImage, GenericImage};
use serde::{Deserialize, Deserializer};
use color::Color;
use vector::{Vector2, Vector3};
use std::fmt;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub enum Material {
    DiffuseMaterial(DiffuseMaterial),
    MetallicMaterial(MetallicMaterial)
}

#[derive(Deserialize, Debug)]
pub struct DiffuseMaterial {
    pub color: Coloration,
    pub albedo: f64
}

#[derive(Deserialize, Debug)]
pub struct MetallicMaterial {
    pub fizziness: f64,

    #[serde(default="Color::white")]
    pub color: Color
}

#[derive(Deserialize)]
pub enum Coloration {
    Color(Color),
    Texture(
        #[serde(deserialize_with="Coloration::load_texture")]
        DynamicImage)
}

impl Default for Material {
    fn default() -> Material {
        Material::DiffuseMaterial(
            DiffuseMaterial {
                color: Coloration::Color(Color::white()),
                albedo: 0.1,
            }
        )
    }
}

impl Material {
    pub fn diffuse_multiplier_color(&self, texture_coordinate: &Vector2) -> Color {
        match self {
            Material::DiffuseMaterial(ref dm) => dm.color_at(texture_coordinate),
            Material::MetallicMaterial(ref mm) => mm.color.clone()
        }
    }

    pub fn get_color(&self, texture_coordinate: &Vector2, light_power: f64, light_color: &Color) -> Color {
        match self {
            Material::DiffuseMaterial(ref dm) => dm.get_color(texture_coordinate, light_power, light_color),
            Material::MetallicMaterial(ref _mm) => Color::black()
        }
    }

    pub fn uses_texture(&self) -> bool {
        match self {
            Material::DiffuseMaterial(ref dm) => dm.uses_texture(),
            Material::MetallicMaterial(ref _mm) => false
        }
    }

    pub fn albedo(&self) -> f64 {
        match self {
            Material::DiffuseMaterial(ref dm) => dm.albedo,
            Material::MetallicMaterial(ref _mm) => 0.0
        }
    }

    pub fn scatter(&self, vec: &Vector3, normal: &Vector3) -> Vector3 {
        match self {
            Material::DiffuseMaterial(ref dm) => dm.scatter(vec, normal),
            Material::MetallicMaterial(ref mm) => mm.scatter(vec, normal)
        }
    }
}

impl DiffuseMaterial {
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

    pub fn scatter(&self, _vec: &Vector3, normal: &Vector3) -> Vector3 {
        normal.add(&Vector3::random_unit())
    }

    pub fn color_at(&self, coords: &Vector2) -> Color {
        self.color.color_at(&coords)
    }
}

impl MetallicMaterial {
    pub fn scatter(&self, vec: &Vector3, normal: &Vector3) -> Vector3 {
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
