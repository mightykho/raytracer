use color::Color;
use point::Point;
use vector::Vector3;
use ray::Ray;

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Color
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> bool;
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> bool {
        // create vector from origin to center of the sphere\
        let l: Vector3 = self.center.sub(&ray.origin);

        // find length of projection of l on direction vector
        let adj = l.dot(&ray.direction);

        // find third side of l, r.direction triangle
        let adj2 = (l.dot(&l) - (adj * adj)).sqrt();

        self.radius > adj2
    }
}
