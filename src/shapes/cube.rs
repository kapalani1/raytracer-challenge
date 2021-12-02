use crate::intersection::{Intersection, IntersectionList};
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::{Object, ShapeType};
use crate::tuple::Tuple;

// A unit cube
#[derive(Debug, PartialEq)]
pub struct Cube;

impl Cube {
    pub fn new(material_opt: Option<Material>) -> Object {
        let material = match material_opt {
            Some(x) => x,
            None => Material::new(),
        };

        Object {
            transform: Matrix::identity(4),
            shape: ShapeType::Cube(Cube),
            material,
        }
    }

    fn check_axis(&self, origin: f64, direction: f64) -> (f64, f64) {
        let tmin_numerator = -1. - origin;
        let tmax_numerator = 1. - origin;

        let tmin = tmin_numerator / direction;
        let tmax = tmax_numerator / direction;

        if tmin > tmax {
            (tmax, tmin)
        } else {
            (tmin, tmax)
        }
    }

    pub fn local_intersect<'a>(
        &self,
        ray_obj_space: &Ray,
        object: &'a Object,
    ) -> IntersectionList<'a> {
        let (xtmin, xtmax) = self.check_axis(ray_obj_space.origin.x, ray_obj_space.direction.x);
        let (ytmin, ytmax) = self.check_axis(ray_obj_space.origin.y, ray_obj_space.direction.y);
        let (ztmin, ztmax) = self.check_axis(ray_obj_space.origin.z, ray_obj_space.direction.z);
        let tmin = vec![xtmin, ytmin, ztmin]
            .into_iter()
            .fold(f64::NEG_INFINITY, f64::max);
        let tmax = vec![xtmax, ytmax, ztmax]
            .into_iter()
            .fold(f64::INFINITY, f64::min);

        if tmin > tmax {
            return IntersectionList::new(vec![]);
        } else {
            IntersectionList::new(vec![
                Intersection::new(tmin, object),
                Intersection::new(tmax, object),
            ])
        }
    }

    pub fn local_normal_at(&self, object_space_point: Tuple) -> Tuple {
        let maxc = vec![
            object_space_point.x.abs(),
            object_space_point.y.abs(),
            object_space_point.z.abs(),
        ]
        .into_iter()
        .fold(f64::NEG_INFINITY, f64::max);

        if maxc == object_space_point.x.abs() {
            Tuple::vector(object_space_point.x, 0., 0.)
        } else if maxc == object_space_point.y.abs() {
            Tuple::vector(0., object_space_point.y, 0.)
        } else {
            Tuple::vector(0., 0., object_space_point.z)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersect() {
        let c = Cube::new(None);
        let r = Ray::new(Tuple::point(5., 0.5, 0.), Tuple::vector(-1., 0., 0.));
        let xs = r.intersect_object(&c);
        assert_eq!(xs.intersections[0].t, 4.);
        assert_eq!(xs.intersections[1].t, 6.);

        let r = Ray::new(Tuple::point(-5., 0.5, 0.), Tuple::vector(1., 0., 0.));
        let xs = r.intersect_object(&c);
        assert_eq!(xs.intersections[0].t, 4.);
        assert_eq!(xs.intersections[1].t, 6.);

        let r = Ray::new(Tuple::point(0.5, 5., 0.), Tuple::vector(0., -1., 0.));
        let xs = r.intersect_object(&c);
        assert_eq!(xs.intersections[0].t, 4.);
        assert_eq!(xs.intersections[1].t, 6.);

        let r = Ray::new(Tuple::point(0.5, -5., 0.), Tuple::vector(0., 1., 0.));
        let xs = r.intersect_object(&c);
        assert_eq!(xs.intersections[0].t, 4.);
        assert_eq!(xs.intersections[1].t, 6.);

        let r = Ray::new(Tuple::point(0.5, 0., 5.), Tuple::vector(0., 0., -1.));
        let xs = r.intersect_object(&c);
        assert_eq!(xs.intersections[0].t, 4.);
        assert_eq!(xs.intersections[1].t, 6.);

        let r = Ray::new(Tuple::point(0.5, 0., -5.), Tuple::vector(0., 0., 1.));
        let xs = r.intersect_object(&c);
        assert_eq!(xs.intersections[0].t, 4.);
        assert_eq!(xs.intersections[1].t, 6.);

        let r = Ray::new(Tuple::point(0., 0.5, 0.), Tuple::vector(0., 0., 1.));
        let xs = r.intersect_object(&c);
        assert_eq!(xs.intersections[0].t, -1.);
        assert_eq!(xs.intersections[1].t, 1.);
    }

    #[test]
    fn misses() {
        let c = Cube::new(None);
        let r = Ray::new(
            Tuple::point(-2., 0., 0.),
            Tuple::vector(0.2673, 0.5345, 0.8018),
        );
        let xs = r.intersect_object(&c);
        assert_eq!(xs.intersections.len(), 0);

        let r = Ray::new(
            Tuple::point(0., -2., 0.),
            Tuple::vector(0.8018, 0.2673, 0.5345),
        );
        let xs = r.intersect_object(&c);
        assert_eq!(xs.intersections.len(), 0);

        let r = Ray::new(
            Tuple::point(0., 0., -2.),
            Tuple::vector(0.5345, 0.8018, 0.2673),
        );
        let xs = r.intersect_object(&c);
        assert_eq!(xs.intersections.len(), 0);

        let r = Ray::new(Tuple::point(2., 0., 2.), Tuple::vector(0., 0., -1.));
        let xs = r.intersect_object(&c);
        assert_eq!(xs.intersections.len(), 0);

        let r = Ray::new(Tuple::point(0., 2., 2.), Tuple::vector(0., -1., 0.));
        let xs = r.intersect_object(&c);
        assert_eq!(xs.intersections.len(), 0);

        let r = Ray::new(Tuple::point(2., 2., 0.), Tuple::vector(-1., 0., 0.));
        let xs = r.intersect_object(&c);
        assert_eq!(xs.intersections.len(), 0);
    }

    #[test]
    fn normal() {
        let c = Cube::new(None);
        let p = Tuple::point(1., 0.5, -0.8);
        let normal = c.normal_at(p);
        assert_eq!(normal, Tuple::vector(1., 0., 0.));

        let p = Tuple::point(-1., -0.2, 0.9);
        let normal = c.normal_at(p);
        assert_eq!(normal, Tuple::vector(-1., 0., 0.));

        let p = Tuple::point(-0.4, 1., -0.1);
        let normal = c.normal_at(p);
        assert_eq!(normal, Tuple::vector(0., 1., 0.));

        let p = Tuple::point(0.3, -1., -0.7);
        let normal = c.normal_at(p);
        assert_eq!(normal, Tuple::vector(0., -1., 0.));

        let p = Tuple::point(-0.6, 0.3, 1.);
        let normal = c.normal_at(p);
        assert_eq!(normal, Tuple::vector(0., 0., 1.));

        let p = Tuple::point(0.4, 0.4, -1.);
        let normal = c.normal_at(p);
        assert_eq!(normal, Tuple::vector(0., 0., -1.));

        let p = Tuple::point(1., 1., 1.);
        let normal = c.normal_at(p);
        assert_eq!(normal, Tuple::vector(1., 0., 0.));

        let p = Tuple::point(-1., -1., -1.);
        let normal = c.normal_at(p);
        assert_eq!(normal, Tuple::vector(-1., 0., 0.));
    }
}
