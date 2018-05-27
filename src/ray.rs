use vector::{Vector3, Point};
use scene::Scene;

#[derive(Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray {
    pub fn create_prime(x: f64, y: f64, scene: &Scene) -> Ray {
        assert!(scene.width > scene.height);

        let aspect_ratio = scene.width as f64 / scene.height as f64;
        let fov_adjustment = (scene.camera.fov.to_radians() / 2.0).tan();

        let sensor_x = 2.0 * x as f64 / scene.width as f64 - 1.0;
        let sensor_y = 1.0 - 2.0 * y as f64 / scene.height as f64;

        let direction = Vector3::new(sensor_x * aspect_ratio * fov_adjustment, sensor_y * fov_adjustment, -1.0);

        Ray {
            origin: scene.camera.position.clone(),
            direction: direction.normalize()
        }
    }
}
