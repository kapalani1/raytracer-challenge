use std::{any::Any, ops::Add};

use crate::{
    intersection::IntersectionContext, material::Material, matrix::Matrix, ray::Ray,
    sphere::{Sphere}, tuple::Tuple, EPSILON, plane::Plane,
};

pub const MAX_REFLECTIONS: u8 = 4;

#[derive(Debug, Clone)]
pub enum ShapeType {
    Sphere(Sphere),
    Plane(Plane),
}

#[derive(Debug, Clone)]
pub struct Object {
    pub transform: Matrix,
    pub shape: ShapeType,
    pub material: Material,
}

impl Object {
    pub fn local_object_intersect(&self, ray_obj_space: &Ray) -> ObjectIntersectionList {
        match &self.shape {
            ShapeType::Sphere(ref sphere) => sphere.local_object_intersect(ray_obj_space, self),
            ShapeType::Plane(ref plane) => plane.local_object_intersect(ray_obj_space, self),
        }
    }

    pub fn local_object_normal(&self, point: Tuple) -> Tuple {
      match &self.shape {
        ShapeType::Sphere(ref sphere) => sphere.local_object_normal(point),
        &ShapeType::Plane(ref plane) => plane.local_object_normal(point),
      }
    }

    pub fn intersect_object(&self, ray: &Ray) -> ObjectIntersectionList {
        let ray_obj_space = ray.transform(&(self.transform.inverse()));
        self.local_object_intersect(&ray_obj_space)
    }

    pub fn normal_object(&self, point: Tuple) -> Tuple {
        assert!(point.is_point());
        let object_space_point = self.transform.inverse() * point;
        let object_normal = self.local_object_normal(object_space_point);
        let mut world_normal = self.transform.inverse().transpose() * object_normal;
        world_normal.w = 0.;
        world_normal.normalize()
    }
}

pub trait Shape: Send + Sync {
    fn local_normal(&self, point: Tuple) -> Tuple;
    fn material_mut(&mut self) -> &mut Material;
    fn as_any(&self) -> &dyn Any;
    fn material(&self) -> &Material;
    fn transform(&self) -> &Matrix;
    fn set_transform(&mut self, m: &Matrix);
    fn local_intersect(&self, ray_obj_space: &Ray) -> Vec<(f64, &dyn Shape)>;

    fn normal_at(&self, point: Tuple) -> Tuple {
        assert!(point.is_point());
        let object_space_point = self.transform().inverse() * point;
        let object_normal = self.local_normal(object_space_point);
        let mut world_normal = self.transform().inverse().transpose() * object_normal;
        world_normal.w = 0.;
        world_normal.normalize()
    }

    fn intersect(&self, ray: &Ray) -> IntersectionList {
        let ray_obj_space = ray.transform(&(self.transform().inverse()));
        IntersectionList::new(
            self.local_intersect(&ray_obj_space)
                .iter()
                .map(|i| Intersection::new(i.0, i.1))
                .collect(),
        )
    }
}

impl Shape for Box<dyn Shape> {
    fn local_normal(&self, point: Tuple) -> Tuple {
        self.as_ref().local_normal(point)
    }

    fn material_mut(&mut self) -> &mut Material {
        self.as_mut().material_mut()
    }

    fn intersect(&self, ray: &Ray) -> IntersectionList {
        self.as_ref().intersect(ray)
    }

    fn as_any(&self) -> &dyn Any {
        self.as_ref().as_any()
    }

    fn material(&self) -> &Material {
        self.as_ref().material()
    }

    fn transform(&self) -> &Matrix {
        self.as_ref().transform()
    }

    fn set_transform(&mut self, m: &Matrix) {
        self.as_mut().set_transform(m);
    }

    fn local_intersect(&self, ray_obj_space: &Ray) -> Vec<(f64, &dyn Shape)> {
        self.as_ref().local_intersect(ray_obj_space)
    }
}

#[derive(Clone)]
pub struct Intersection<'a> {
    pub t: f64,
    pub shape: &'a dyn Shape,
}

#[derive(Clone, Debug)]
pub struct ObjectIntersection<'a> {
    pub t: f64,
    pub object: &'a Object,
}

impl<'a> ObjectIntersection<'a> {
    pub fn new(t: f64, object: &'a Object) -> ObjectIntersection<'a> {
        Self { t, object }
    }

    pub fn context(&'a self, ray: &Ray, xs: Option<&ObjectIntersectionList>) -> ObjectIntersectionContext {
      let point = ray.position(self.t);
      let eye_vector = -ray.direction;
      let inside = self.object.normal_object(point).dot(&eye_vector) < 0.;
      let normal_vector = if inside {
          -self.object.normal_object(point)
      } else {
          self.object.normal_object(point)
      };
      let over_point = point + normal_vector * EPSILON;
      let under_point = point - normal_vector * EPSILON;
      let reflect_vector = ray.direction.reflect(&normal_vector);

      let mut n1 = 0.;
      let mut n2 = 0.;

      if let Some(xs) = xs {
          if let Some(hit) = xs.hit() {
              let mut containers: Vec<&dyn Any> = vec![];

              for i in &xs.intersections {
                  if i == hit {
                      if containers.len() == 0 {
                          n1 = 1.;
                      } else {
                      }
                  }
              }
          }
      }

      ObjectIntersectionContext {
          t: self.t,
          object: self.object,
          point,
          eye_vector,
          normal_vector,
          reflect_vector,
          inside,
          over_point,
          under_point,
          n1,
          n2: 0.,
      }
  }


}

impl<'a> PartialEq for ObjectIntersection<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && std::ptr::eq(self.object, other.object)
    }
}

impl<'a> PartialOrd for ObjectIntersection<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.t.partial_cmp(&other.t)
    }
}

impl<'a> Ord for ObjectIntersection<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

impl<'a> Eq for ObjectIntersection<'a> {}

#[derive(Clone, Debug)]
pub struct ObjectIntersectionList<'a> {
    pub intersections: Vec<ObjectIntersection<'a>>,
}

impl<'a> ObjectIntersectionList<'a> {
    pub fn new(intersections: Vec<ObjectIntersection<'a>>) -> Self {
        let mut sorted_intersections = intersections.clone();
        sorted_intersections.sort();
        Self {
            intersections: sorted_intersections,
        }
    }

    pub fn hit(&self) -> Option<&ObjectIntersection> {
      let filtered: Vec<_> = self.intersections.iter().filter(|x| x.t > 0.).collect();
      match filtered.len() {
          0 => None,
          _ => Some(&filtered[0]),
      }
  }
}

impl<'a> Add for ObjectIntersectionList<'a> {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
      let mut sorted_intersections = self.intersections;
      let mut rhs = rhs;
      sorted_intersections.append(&mut rhs.intersections);
      sorted_intersections.sort();
      ObjectIntersectionList {
          intersections: sorted_intersections,
      }
  }
}


impl<'a> std::fmt::Debug for Intersection<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Intersection")
            .field("t", &self.t)
            .finish()
            .unwrap();
        f.write_fmt(format_args!("object {:?}", std::ptr::addr_of!(*self.shape)))
    }
}

#[derive(Debug)]
pub struct IntersectionList<'a> {
    pub intersections: Vec<Intersection<'a>>,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, shape: &'a dyn Shape) -> Intersection<'a> {
        Intersection { t, shape }
    }

    pub fn context(&'a self, ray: &Ray, xs: Option<&IntersectionList>) -> IntersectionContext {
        let point = ray.position(self.t);
        let eye_vector = -ray.direction;
        let inside = self.shape.normal_at(point).dot(&eye_vector) < 0.;
        let normal_vector = if inside {
            -self.shape.normal_at(point)
        } else {
            self.shape.normal_at(point)
        };
        let over_point = point + normal_vector * EPSILON;
        let under_point = point - normal_vector * EPSILON;
        let reflect_vector = ray.direction.reflect(&normal_vector);

        let mut n1 = 0.;
        let mut n2 = 0.;

        if let Some(xs) = xs {
            if let Some(hit) = xs.hit() {
                let mut containers: Vec<&dyn Any> = vec![];

                for i in &xs.intersections {
                    if i == hit {
                        if containers.len() == 0 {
                            n1 = 1.;
                        } else {
                            let last_shape = containers
                                .last()
                                .unwrap()
                                .downcast_ref::<&dyn Shape>()
                                .unwrap();
                            n1 = last_shape.material().refractive_index;
                        }
                    }
                }
            }
        }

        IntersectionContext {
            t: self.t,
            shape: self.shape,
            point,
            eye_vector,
            normal_vector,
            reflect_vector,
            inside,
            over_point,
            under_point,
            n1,
            n2: 0.,
        }
    }
}

impl<'a> PartialEq for Intersection<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && std::ptr::eq(self.shape.as_any(), other.shape.as_any())
    }
}

impl<'a> PartialOrd for Intersection<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.t.partial_cmp(&other.t)
    }
}

impl<'a> Ord for Intersection<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

impl<'a> Eq for Intersection<'a> {}

impl<'a> IntersectionList<'a> {
    pub fn new(intersections: Vec<Intersection<'a>>) -> Self {
        let mut sorted_intersections = intersections.clone();
        sorted_intersections.sort();
        Self {
            intersections: sorted_intersections,
        }
    }

    pub fn hit(&self) -> Option<&Intersection> {
        let filtered: Vec<_> = self.intersections.iter().filter(|x| x.t > 0.).collect();
        match filtered.len() {
            0 => None,
            _ => Some(&filtered[0]),
        }
    }
}

impl<'a> Add for IntersectionList<'a> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut sorted_intersections = self.intersections;
        let mut rhs = rhs;
        sorted_intersections.append(&mut rhs.intersections);
        sorted_intersections.sort();
        IntersectionList {
            intersections: sorted_intersections,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{color::Color, light::PointLight, matrix::Matrix, sphere::Sphere, world::World};

    #[test]
    pub fn intersection() {
        let s = Sphere::new(None);
        let i = Intersection::new(3.5, &s);
        assert_eq!(i.t, 3.5);
        assert!(std::ptr::eq(i.shape, &s));
    }

    #[test]
    pub fn intersection_list() {
        let s = Sphere::new(None);
        let i1 = Intersection::new(1., &s);
        let i2 = Intersection::new(2., &s);
        let i = IntersectionList::new(vec![i1, i2]);
        assert_eq!(i.intersections.len(), 2);
        assert_eq!(i.intersections[0].t, 1.);
        assert_eq!(i.intersections[1].t, 2.);
        assert!(std::ptr::eq(i.intersections[0].shape, &s));
        assert!(std::ptr::eq(i.intersections[1].shape, &s));
    }

    #[test]
    pub fn hit() {
        let s = Sphere::new(None);
        let i1 = Intersection::new(1., &s);
        let i2 = Intersection::new(2., &s);
        let i = IntersectionList::new(vec![i1, i2]);
        assert_eq!(i.hit(), Some(&i.intersections[0]));

        let i1 = Intersection::new(-1., &s);
        let i2 = Intersection::new(1., &s);
        let i = IntersectionList::new(vec![i1.clone(), i2.clone()]);
        assert_eq!(i.hit(), Some(&i2));

        let i1 = Intersection::new(-2., &s);
        let i2 = Intersection::new(-1., &s);
        let i = IntersectionList::new(vec![i1, i2]);
        assert_eq!(i.hit(), None);

        let i1 = Intersection::new(5., &s);
        let i2 = Intersection::new(7., &s);
        let i3 = Intersection::new(-3., &s);
        let i4 = Intersection::new(2., &s);
        let i = IntersectionList::new(vec![i1.clone(), i2.clone(), i3.clone(), i4.clone()]);
        assert_eq!(i.hit(), Some(&i4));
    }

    #[test]
    fn intersection_context() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let shape = Sphere::new(None);
        let i = r.intersect(&shape).intersections;
        let c = i[0].context(&r, None);
        assert_eq!(c.t, 4.);
        assert!(std::ptr::eq(c.shape.as_any(), shape.as_any()));
        assert_eq!(c.point, Tuple::point(0., 0., -1.));
        assert_eq!(c.eye_vector, Tuple::vector(0., 0., -1.));
        assert_eq!(c.normal_vector, Tuple::vector(0., 0., -1.));
        assert_eq!(c.inside, false);

        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
        let shape = Sphere::new(None);
        let i = r.intersect(&shape);
        let i = i.hit().unwrap();
        let c = i.context(&r, None);
        assert_eq!(c.t, 1.);
        assert!(std::ptr::eq(c.shape.as_any(), shape.as_any()));
        assert_eq!(c.point, Tuple::point(0., 0., 1.));
        assert_eq!(c.eye_vector, Tuple::vector(0., 0., -1.));
        assert_eq!(c.normal_vector, Tuple::vector(0., 0., -1.));
        assert_eq!(c.inside, true);
    }

    #[test]
    fn shade_intersection() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let shape = &w.objects[0];
        let i = r.intersect(shape);
        let i = i.hit().unwrap();
        let c = i.context(&r, None).shade_hit(&w, MAX_REFLECTIONS);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shade_inside() {
        let mut w = World::default();
        w.lights[0] = PointLight::new(Tuple::point(0., 0.25, 0.), Color::new(1., 1., 1.));
        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
        let shape = &w.objects[1];
        let i = r.intersect(shape);
        let i = i.hit().unwrap();
        let c = i.context(&r, None).shade_hit(&w, MAX_REFLECTIONS);
        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn hit_offset_point() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let mut shape = Sphere::new(None);
        shape.set_transform(&Matrix::translation(0., 0., 1.));
        let i = r.intersect(&shape);
        let hit = i.hit().unwrap();
        let c = hit.context(&r, None);
        assert!(c.over_point.z < -EPSILON / 2.);
        assert!(c.point.z > c.over_point.z);
    }

    #[test]
    fn material_shape() {
        let s = Sphere::new(None);
        assert_eq!(*s.material(), Material::new());
        let mut s = Sphere::new(None);
        s.material_mut().ambient = 1.;
        let mut m = Material::new();
        m.ambient = 1.;
        assert_eq!(*s.material(), m);
    }
}
