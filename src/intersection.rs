use geometry::Object;

#[derive(Debug)]
pub struct Intersection<'a> {
    pub distance: f64,
    pub object: &'a Object,
}

impl<'a> Intersection<'a> {
    pub fn new(distance: f64, object: &Object) -> Intersection {
        Intersection {
            distance: distance,
            object: object,
        }
    }
}
