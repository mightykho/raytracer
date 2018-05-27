extern crate raytracer;
extern crate image;

use raytracer::scene::Scene;
use raytracer::point::Point;
use raytracer::geometry::Sphere;
use raytracer::color::Color;

fn main() {
    let scene = build_scene();
    let img = raytracer::render(&scene);

    img.save("test.png").unwrap();
}

fn build_scene() -> Scene {
    let geometry = vec!(
        Sphere {
            center: Point {
                x: 0.0,
                y: 0.0,
                z: -4.0,
            },
            radius: 2.0,
            color: Color {
                r: 250,
                g: 105,
                b: 120,
            },
        },
        Sphere {
            center: Point {
                x: 1.0,
                y: 0.0,
                z: -2.0,
            },
            radius: 1.0,
            color: Color {
                r: 100,
                g: 255,
                b: 180,
            },
        });

    Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        geometry: geometry
    }
}
