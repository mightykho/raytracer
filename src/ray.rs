use point::Point;
use vector::Vector3;
use scene::Scene;

pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray {
    pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
        assert!(scene.width > scene.height);

        let aspect_ratio = scene.width as f64 / scene.height as f64;
        let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();

        // sensor goes through the pixel center
        let pixel_center_x = x as f64 + 0.5;
        let pixel_center_y = y as f64 + 0.5;

        // sensor coordinate is in range (-1.0...1.0)
        let sensor_x = 2.0 * pixel_center_x / scene.width as f64 - 1.0;
        let sensor_y = 1.0 - 2.0 * pixel_center_y / scene.height as f64;

        Ray {
            origin: Point::zero(),
            direction: Vector3 {
                x: sensor_x * aspect_ratio * fov_adjustment,
                y: sensor_y * fov_adjustment,
                z: -1.0,
            }.normalize(),
        }
    }
}
