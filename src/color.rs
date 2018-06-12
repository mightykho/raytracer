use image::{Rgba, Pixel};

#[derive(Deserialize, Debug, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

const GAMMA: f64 = 2.2;

fn gamma_encode(linear: f64) -> f64 {
    linear.powf(1.0 / GAMMA)
}

fn gamma_decode(encoded: f64) -> f64 {
    encoded.powf(GAMMA)
}

impl Color {
    pub fn white() -> Color {
        Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }

    pub fn black() -> Color {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    pub fn multiply_color(&self, col: &Color) -> Color {
        Color {
            r: self.r * col.r,
            g: self.g * col.g,
            b: self.b * col.b,
        }
    }

    pub fn add_color(&self, col: &Color) -> Color {
        Color {
            r: self.r + col.r,
            g: self.g + col.g,
            b: self.b + col.b,
        }
    }

    pub fn multiply(&self, mul: f64) -> Color {
        Color {
            r: self.r * mul,
            g: self.g * mul,
            b: self.b * mul,
        }
    }

    pub fn divide(&self, div: f64) -> Color {
        Color {
            r: self.r / div,
            g: self.g / div,
            b: self.b / div,
        }
    }

    pub fn clamp(&self) -> Color {
        Color {
            r: self.r.min(1.0).max(0.0),
            g: self.g.min(1.0).max(0.0),
            b: self.b.min(1.0).max(0.0),
        }
    }

    pub fn lerp(start: &Color, end: &Color, t: f64) -> Color {
        start.multiply(1.0 - t).add_color(&end.multiply(t))
    }

    pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba::from_channels((gamma_encode(self.r) * 255.0) as u8,
                            (gamma_encode(self.g) * 255.0) as u8,
                            (gamma_encode(self.b) * 255.0) as u8,
                            255)
    }

    pub fn from_rgba(rgba: Rgba<u8>) -> Color {
        Color {
            r: gamma_decode((rgba.data[0] as f64) / 255.0),
            g: gamma_decode((rgba.data[1] as f64) / 255.0),
            b: gamma_decode((rgba.data[2] as f64) / 255.0),
        }
    }
}
