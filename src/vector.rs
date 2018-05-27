use rand::prelude::*;

#[derive(Deserialize, Debug, Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64
}

pub type Point = Vector3;

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 {
            x: x,
            y: y,
            z: z
        }
    }

    pub fn zero() -> Vector3 {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn random_unit() -> Vector3 {
        let mut vec = Vector3 { x: 1.0, y: 1.0, z: 1.0 };
        let mut rng = thread_rng();

        while vec.norm() >= 1.0 {
            let rx: f64 = rng.gen();
            let ry: f64 = rng.gen();
            let rz: f64 = rng.gen();

            vec = Vector3 {
                x: 2.0 * rx - 1.0,
                y: 2.0 * ry - 1.0,
                z: 2.0 * rz - 1.0
            };
        } {}

        vec
    }

    pub fn normalize(&self) -> Vector3 {
        let magnitude = self.magnitude();

        Vector3 {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
        }
    }

    pub fn dot(&self, vec: &Vector3) -> f64 {
        self.x * vec.x + self.y * vec.y + self.z * vec.z
    }

    pub fn cross(&self, vec: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * vec.z - self.z * vec.y,
            y: self.z * vec.x - self.x * vec.z,
            z: self.x * vec.y - self.y * vec.x
        }
    }

    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn magnitude(&self) -> f64 {
        self.norm().sqrt()
    }

    pub fn multiply(&self, distance: f64) -> Vector3 {
        Vector3 {
            x: self.x * distance,
            y: self.y * distance,
            z: self.z * distance,
        }
    }

    pub fn subtract(&self, vec: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x - vec.x,
            y: self.y - vec.y,
            z: self.z - vec.z
        }
    }

    pub fn distance(&self, point: &Point) -> f64 {
        self.subtract(point).magnitude()
    }

    pub fn reflect(&self, normal: &Vector3) -> Vector3 {
        // r = d - 2(d . n) * n

        let n = normal.normalize();
        self.subtract(&n.multiply(2.0 * self.dot(&n)))
    }

    pub fn add(&self, vec: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x + vec.x,
            y: self.y + vec.y,
            z: self.z + vec.z
        }
    }

    pub fn neg(&self) -> Vector3 {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
