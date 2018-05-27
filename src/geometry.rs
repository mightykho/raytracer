use material::Material;
use vector::{Vector2, Vector3, Point};
use ray::Ray;

#[derive(Deserialize, Debug)]
pub enum Object {
    Sphere(Sphere),
    Plane(Plane)
}

#[derive(Deserialize, Debug)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,

    #[serde(default)]
    pub material: Material,
}

#[derive(Deserialize, Debug)]
pub struct Plane {
    pub origin: Point,
    pub normal: Vector3,

    #[serde(default)]
    pub material: Material,
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
    fn surface_normal(&self, point: &Point) -> Vector3;
    fn texture_coords(&self, point: &Point) -> Vector2;
}

impl Object {
    pub fn material(&self) -> &Material {
        match *self {
            Object::Plane(ref p) => &p.material,
            Object::Sphere(ref s) => &s.material,
       }
    }
}

impl Intersectable for Object {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        match *self {
            Object::Plane(ref p) => p.intersect(ray),
            Object::Sphere(ref s) => s.intersect(ray),
        }
    }

    fn surface_normal(&self, point: &Point) -> Vector3 {
        match *self {
            Object::Plane(ref p) => p.surface_normal(point),
            Object::Sphere(ref s) => s.surface_normal(point)
       }
    }

    fn texture_coords(&self, point: &Point) -> Vector2 {
        match *self {
            Object::Plane(ref p) => p.texture_coords(point),
            Object::Sphere(ref s) => s.texture_coords(point)
       }
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let normal = &self.normal;
        let denom = normal.dot(&ray.direction);

        if denom > 1e-6 {
            let vec = self.origin.subtract(&ray.origin);

            let distance = vec.dot(&normal) / denom;

            if distance >= 0.0 {
                Some(distance)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn surface_normal(&self, _: &Point) -> Vector3 {
        Vector3 {
            x: -self.normal.x,
            y: -self.normal.y,
            z: -self.normal.z
        }
    }

    fn texture_coords(&self, point: &Point) -> Vector2 {
        let hit_vec = point.subtract(&self.origin);

        let mut x_axis = self.normal.cross(&Vector3::new(0.0, 0.0, 1.0));

        if x_axis.magnitude() == 0.0 {
            x_axis = self.normal.cross(&Vector3::new(0.0, 1.0, 0.0));
        }

        let y_axis = self.normal.cross(&x_axis);

        Vector2 {
            x: hit_vec.dot(&x_axis),
            y: hit_vec.dot(&y_axis)
        }
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        // create vector from origin to center of the sphere
        let l: Vector3 = self.center.subtract(&ray.origin);
        let l_norm = l.norm();

        if l_norm > 100000.0 {
            // ignore very far distances
            return None;
        }

        // find length of projection of l on direction vector
        let adj = l.dot(&ray.direction.normalize());

        // find third side of l, adj triangle
        let adj2 = (l_norm - (adj * adj)).sqrt();

        if self.radius < adj2 {
            return None;
        }

        // find thid side of adj2, radius triangle
        let adj3 = (self.radius * self.radius - adj2 * adj2).sqrt();

        // there are two possible options here
        let d1 = adj + adj3;
        let d2 = adj - adj3;

        if d1 < 0.0 && d2 < 0.0 {
            return None;
        } else {
            let distance = if d2 > d1 { d1 } else { d2 };

            if distance < 0.001 {
                return None;
            } else {
                Some(distance)
            }
        }
    }

    fn surface_normal(&self, point: &Point) -> Vector3 {
        point.subtract(&self.center).normalize()
    }

    fn texture_coords(&self, point: &Point) -> Vector2 {
        let hit_vec = point.subtract(&self.center);

        Vector2 {
            x: (1.0 + hit_vec.z.atan2(hit_vec.x) / ::std::f64::consts::PI) * 0.5,
            y: (hit_vec.y / self.radius).acos()  / ::std::f64::consts::PI,
        }
    }
}
