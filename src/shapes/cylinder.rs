use crate::intersection::{Intersection, IntersectionList};
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::{Object, ShapeType};
use crate::tuple::Tuple;

// A unit cube
#[derive(Debug, PartialEq)]
pub struct Cylinder;

impl Cylinder {
    pub fn new(material_opt: Option<Material>) -> Object {
        let material = match material_opt {
            Some(x) => x,
            None => Material::new(),
        };

        Object {
            transform: Matrix::identity(4),
            shape: ShapeType::Cylinder(Cylinder),
            material,
        }
    }

    pub fn local_intersect<'a>(
        &self,
        ray_obj_space: &Ray,
        object: &'a Object,
    ) -> IntersectionList<'a> {
        return IntersectionList::new(vec![]);
    }

    pub fn local_normal_at(&self, object_space_point: Tuple) -> Tuple {
        Tuple::vector(1., 0., 0.)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersect() {}

    #[test]
    fn misses() {}

    #[test]
    fn normal() {}
}
