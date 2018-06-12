# Rust Raytracer

This is project is my attempt to build simple renderer using rust. 
It's heavily inspired by 
[Writing a Raytracer in Rust](https://bheisler.github.io/post/writing-raytracer-in-rust-part-1/) by Brook Heisler and  [Raytracer in One Weekend](https://www.amazon.co.uk/Ray-Tracing-Weekend-Minibooks-Book-ebook/dp/B01B5AODD8) by Peter Shirley.

## Render results

* **Resolution:** 1600px ✕ 1200px
* **Samples:** 1000
* **Diffuse iterations:** 4
* **Light sources:** 2
* **Processor:** 2.9 GHz Intel Core i7
* **Render Time:** 581 seconds

![Render result](https://raw.githubusercontent.com/mightykho/raytracer/master/app/out.png)
----

* **Resolution:** 1600px ✕ 1200px
* **Samples:** 1000
* **Diffuse iterations:** 4
* **Light sources:** 0
* **Processor:** 2.9 GHz Intel Core i7
* **Render Time:** 102 seconds

![Render result](https://raw.githubusercontent.com/mightykho/raytracer/master/app/out2.png)

## Usage

    cd app; cargo run --release -- ./scenes/scene.json out.png

