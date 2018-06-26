use geometry::{Object, Intersectable};
use light::Light;
use vector::{Vector2, Vector3, Point};
use intersection::Intersection;
use ray::Ray;
use color::Color;
use camera::Camera;

#[derive(Deserialize, Debug)]
pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub camera: Camera,
    pub geometry: Vec<Object>,
    pub lights: Vec<Light>,
}

impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.geometry.iter()
            .filter_map(|o| o.intersect(ray).map(|d| Intersection::new(d, o)))
            .min_by(|i1, i2| {
                i1.distance.partial_cmp(&i2.distance).unwrap()
            })
    }

    pub fn get_color(&self, ray: &Ray, diffuse_depth: u32) -> Color {
        let mut color = Color::black();

        if diffuse_depth == 0 {
            return color;
        }

        // TODO: Move that to scene config
        let bg_start_color = Color { r: 1.0, g: 1.0 , b: 1.0 };
        let bg_end_color = Color { r: 0.5, g: 0.7 , b: 1.0 };

        match self.trace(ray) {
            Some(intersection) => {

                let object = intersection.object;
                let material = object.material();
                let hit_point = ray.origin.add(&ray.direction.multiply(intersection.distance));
                let surface_normal = object.surface_normal(&hit_point);

                let mut texture_coords = Vector2 { x: 0.0, y: 0.0 };
                if material.uses_texture() {
                    texture_coords = object.texture_coords(&hit_point);
                }

                let light_reflected = material.albedo / ::std::f64::consts::PI;
                let light_color = self.light_color_at_hit_point(&hit_point, &surface_normal)
                    .multiply(light_reflected);

                let (diffuse_vector, main_color) = material.scatter(&ray.direction, &surface_normal, &texture_coords);

                let diffuse_ray = Ray {
                    origin: hit_point.clone(),
                    direction: diffuse_vector.normalize()
                };

                let diffuse_color = self.get_color(&diffuse_ray, diffuse_depth - 1)
                    .multiply(1.0 - material.albedo);

                let highlighted_color = main_color.multiply_color(&light_color);

                color = main_color.multiply_color(&diffuse_color).add_color(&highlighted_color);
            },

            None => {
                let t = 0.5 * (ray.direction.y + 1.0);
                color = Color::lerp(&bg_start_color, &bg_end_color, t);
            }
        }

        color
    }

    fn light_color_at_hit_point(&self, hit_point: &Point, normal: &Vector3) -> Color {
        let mut color = Color::black();

        for light in self.lights.iter() {
            let direction_to_light = light.direction_vector(&hit_point).normalize().neg();

            let in_light = if light.cast_shadow() {
                let shadow_ray = Ray {
                    origin: hit_point.add(&normal.multiply(1e-13)), // add tiny shadow bias to remove artifacts
                    direction: direction_to_light.clone(),
                };

                let shadow_intersection = self.trace(&shadow_ray);

                shadow_intersection.is_none() || shadow_intersection.unwrap().distance > light.distance(&hit_point)
            } else {
                true
            };

            let light_intensity = if in_light { light.relative_intensity(&hit_point) } else { 0.0 };
            let light_power = normal.dot(&direction_to_light).max(0.0) * light_intensity;

            color = color.add_color(&light.color().multiply(light_power));
        }

        color
    }
}
