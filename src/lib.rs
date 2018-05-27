extern crate image;

pub mod scene;
pub mod geometry;
pub mod point;
pub mod vector;
pub mod color;
pub mod ray;

use scene::Scene;
use image::*;
use geometry::{Sphere, Intersectable};
use point::Point;
use color::Color;
use ray::Ray;

pub fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);

    let black = Rgba::from_channels(0,0,0,0);

    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);

            for object in scene.geometry.iter() {
                if object.intersect(&ray) {
                    let color = Rgba::from_channels(object.color.r, object.color.g, object.color.b, 1);

                    image.put_pixel(x, y, color);
                } else {
                    image.put_pixel(x, y, black);
                }
            }
        }
    }

    image
}

#[test]
fn test_can_render_scene() {
    let geometry = vec!(
        Sphere {
            center: Point {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            radius: 1.0,
            color: Color {
                r: 100,
                g: 255,
                b: 100,
            },
        });

    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        geometry: geometry
    };

    let img: DynamicImage = render(&scene);
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());
}
