use crate::color::Color;
use crate::matrix::Matrix;
use crate::shape::{IntersectionList, Object, Shape, ObjectIntersectionList};
use crate::tuple::Tuple;
use crate::world::World;
use rayon::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        assert!(origin.is_point());
        assert!(direction.is_vector());
        Ray { origin, direction }
    }

    pub fn position(&self, time: f64) -> Tuple {
        self.origin + self.direction * time
    }

    pub fn intersect<'a>(&self, object: &'a impl Shape) -> IntersectionList<'a> {
        object.intersect(&self)
    }

    pub fn intersect_object<'a>(&self, object: &'a Object) -> ObjectIntersectionList<'a> {
        object.intersect_object(&self)
    }

    pub fn project_into_world<'a>(&self, world: &'a World) -> IntersectionList<'a> {
        world
            .objects
            .par_iter()
            .map(|object| self.intersect(object))
            .reduce(|| IntersectionList::new(vec![]), |i1, i2| i1 + i2)
    }

    pub fn project_into_world_object<'a>(&self, world: &'a World) -> ObjectIntersectionList<'a> {
      world
          .test_objects
          .par_iter()
          .map(|object| self.intersect_object(object))
          .reduce(|| ObjectIntersectionList::new(vec![]), |i1, i2| i1 + i2)
  }

    pub fn color_at(&self, world: &World, remaining: u8) -> Color {
        let i = self.project_into_world(world);
        let hit = i.hit();
        match hit {
            None => Color::new(0., 0., 0.),
            Some(h) => h.context(self, Some(&i)).shade_hit(world, remaining),
        }
    }

    pub fn color_at_object(&self, world: &World, remaining: u8) -> Color {
      let i = self.project_into_world_object(world);
      let hit = i.hit();
      match hit {
          None => Color::new(0., 0., 0.),
          Some(h) => h.context(self, Some(&i)).shade_hit(world, remaining),
      }
  }

    pub fn transform(&self, transformation: &Matrix) -> Self {
        let origin = transformation * self.origin;
        let direction = transformation * self.direction;
        Ray { origin, direction }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{light::PointLight, material::Material, shape::MAX_REFLECTIONS, sphere::Sphere};

    #[test]
    fn ray() {
        let origin = Tuple::point(1., 2., 3.);
        let direction = Tuple::vector(4., 5., 6.);
        let r = Ray::new(origin, direction);
        assert_eq!(origin, r.origin);
        assert_eq!(direction, r.direction);

        let r = Ray::new(Tuple::point(2., 3., 4.), Tuple::vector(1., 0., 0.));
        assert_eq!(r.position(0.), Tuple::point(2., 3., 4.));
        assert_eq!(r.position(1.), Tuple::point(3., 3., 4.));
        assert_eq!(r.position(-1.), Tuple::point(1., 3., 4.));
        assert_eq!(r.position(2.5), Tuple::point(4.5, 3., 4.));
    }

    #[test]
    fn ray_sphere_intersect() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::object_new(None);
        let i = r.intersect_object(&s);
        assert_eq!(i.intersections.len(), 2);
        assert_eq!(i.intersections[0].t, 4.);
        assert_eq!(i.intersections[1].t, 6.);
        assert!(std::ptr::eq(i.intersections[0].object, &s));
        assert!(std::ptr::eq(i.intersections[1].object, &s));

        let r = Ray::new(Tuple::point(0., 1., -5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::object_new(None);
        let i = r.intersect_object(&s);
        assert_eq!(i.intersections.len(), 2);
        assert_eq!(i.intersections[0].t, 5.);
        assert_eq!(i.intersections[1].t, 5.);
        assert!(std::ptr::eq(i.intersections[0].object, &s));
        assert!(std::ptr::eq(i.intersections[1].object, &s));

        let r = Ray::new(Tuple::point(0., 2., -5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::object_new(None);
        let i = r.intersect_object(&s);
        assert_eq!(i.intersections.len(), 0);

        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
        let s = Sphere::object_new(None);
        let i = r.intersect_object(&s);
        assert_eq!(i.intersections.len(), 2);
        assert_eq!(i.intersections[0].t, -1.);
        assert_eq!(i.intersections[1].t, 1.);
        assert!(std::ptr::eq(i.intersections[1].object, &s));
        assert!(std::ptr::eq(i.intersections[0].object, &s));

        let r = Ray::new(Tuple::point(0., 0., 5.), Tuple::vector(0., 0., 1.));
        let s = Sphere::object_new(None);
        let i = r.intersect_object(&s);
        assert_eq!(i.intersections.len(), 2);
        assert_eq!(i.intersections[0].t, -6.);
        assert_eq!(i.intersections[1].t, -4.);
        assert!(std::ptr::eq(i.intersections[0].object, &s));
        assert!(std::ptr::eq(i.intersections[1].object, &s));
    }

    #[test]
    fn transform() {
        let r = Ray::new(Tuple::point(1., 2., 3.), Tuple::vector(0., 1., 0.));
        let m = Matrix::translation(3., 4., 5.);
        let r2 = r.transform(&m);
        assert_eq!(r2.origin, Tuple::point(4., 6., 8.));
        assert_eq!(r2.direction, Tuple::vector(0., 1., 0.));

        let m = Matrix::scaling(2., 3., 4.);
        let r2 = r.transform(&m);
        assert_eq!(r2.origin, Tuple::point(2., 6., 12.));
        assert_eq!(r2.direction, Tuple::vector(0., 3., 0.));
    }

    #[test]
    fn test_world_color() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 1., 0.));
        let c = r.color_at(&w, MAX_REFLECTIONS);
        assert_eq!(c, Color::new(0., 0., 0.));

        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let c = r.color_at(&w, MAX_REFLECTIONS);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn test_world_color_inner() {
        let light = PointLight::new(Tuple::point(-10., 10., -10.), Color::new(1., 1., 1.));
        let mut mat1 = Material::new();
        mat1.color = Color::new(0.8, 1., 0.6);
        mat1.diffuse = 0.7;
        mat1.specular = 0.2;
        mat1.ambient = 1.;
        let s1 = Sphere::object_new(Some(mat1));

        let mut mat2 = Material::new();
        mat2.ambient = 1.;
        let mut s2 = Sphere::object_new(Some(mat2));
        s2.transform = Matrix::scaling(0.5, 0.5, 0.5);

        let w = World::new(vec![], vec![light], vec![Box::new(s1), Box::new(s2)]);
        let r = Ray::new(Tuple::point(0., 0., 0.75), Tuple::vector(0., 0., -1.));
        let c = r.color_at(&w, MAX_REFLECTIONS);
        assert_eq!(c, w.objects[1].material().color);
    }
}
