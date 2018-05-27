extern crate raytracer;
extern crate serde;
extern crate serde_json;
extern crate image;

#[macro_use]
extern crate clap;

use std::fs::File;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let scene_path = matches.value_of("scene").unwrap();
    let scene_file = File::open(scene_path).expect("File not found");

    let image_path = matches.value_of("output").unwrap();

    let scene = serde_json::from_reader(scene_file).unwrap();

    let img = raytracer::render(&scene);

    img.save(image_path).unwrap();
}
