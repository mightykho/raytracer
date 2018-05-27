use vector::Vector3;

pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn zero() -> Point {
        Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn sub(&self, point: &Point) -> Vector3  {
        Vector3 {
            x: self.x - point.x,
            y: self.y - point.y,
            z: self.z - point.z
        }
    }
}

