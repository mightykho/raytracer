#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate scoped_threadpool;
extern crate image;
extern crate rand;
extern crate num_cpus;

pub mod scene;
pub mod geometry;
pub mod point;
pub mod vector;
pub mod color;
pub mod material;
pub mod ray;
pub mod intersection;
pub mod light;
pub mod camera;

use rand::prelude::*;
use scoped_threadpool::Pool;
use scene::Scene;
use image::*;
use ray::Ray;
use color::Color;

pub fn render(scene: &Scene) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut image = ImageBuffer::new(scene.width, scene.height);
    let threads_count = num_cpus::get() as u32;
    let mut pool = Pool::new(threads_count);
    let pixels_count = scene.width * scene.height;
    println!("Using {} threads", threads_count);
    print!("Progress: 0%");

    let start_time = std::time::Instant::now();

    pool.scoped(|scope| {
        for (i, (x, y, pixel)) in image.enumerate_pixels_mut().enumerate() {
            scope.execute(move || {
                let mut rng = thread_rng();
                let mut color = Color::black();

                let progress = ((i as f64 / pixels_count as f64) * 100.0) as u8;

                print!("\rProgress: {}%", progress);

                for _n in 0..scene.camera.samples {
                    let rx: f64 = rng.gen();
                    let ry: f64 = rng.gen();
                    let ray = Ray::create_prime(rx + x as f64, ry + y as f64, scene);


                    color = color.add_color(&scene.get_color(&ray, scene.camera.diffuse));
                }

                *pixel = color.divide(scene.camera.samples as f64).clamp().to_rgba();
            });
        }
    });

    print!("\rProgress 100%\n");
    println!("Rendered in {} seconds", start_time.elapsed().as_secs());

    image
}
