use std::{any::Any, ops::Add};

use crate::{
    color::Color, material::Material, matrix::Matrix, ray::Ray, tuple::Tuple, world::World, EPSILON,
};

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
        let ray_obj_space = ray.transform(&self.transform().inverse());
        IntersectionList::new(
            self.local_intersect(&ray_obj_space)
                .iter()
                .map(|i| Intersection::new(i.0, i.1, Some(ray)))
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
    pub point: Tuple,
    pub eye_vector: Tuple,
    pub normal_vector: Tuple,
    pub inside: bool,
    pub over_point: Tuple,
}

pub struct IntersectionList<'a> {
    pub intersections: Vec<Intersection<'a>>,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, shape: &'a dyn Shape, ray: Option<&Ray>) -> Intersection<'a> {
        let ray = match ray {
            Some(r) => *r,
            None => Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 0.)),
        };

        let point = ray.position(t);
        let eye_vector = -ray.direction;
        let inside = shape.normal_at(point).dot(&eye_vector) < 0.;
        let normal_vector = if inside {
            -shape.normal_at(point)
        } else {
            shape.normal_at(point)
        };
        let over_point = point + normal_vector * EPSILON;

        Intersection {
            t,
            shape,
            point,
            eye_vector,
            normal_vector,
            inside,
            over_point,
        }
    }

    pub fn shade(&self, world: &World) -> Color {
        assert_eq!(world.lights.len(), 1);
        let in_shadow = world.is_shadowed(self.over_point);
        self.shape.material().lighting(
            &world.lights[0],
            self.over_point,
            self.eye_vector,
            self.normal_vector,
            in_shadow,
        )
    }
}

impl<'a> PartialEq for Intersection<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t
            && std::ptr::eq(self.shape, other.shape)
            && self.point == other.point
            && self.eye_vector == other.eye_vector
            && self.normal_vector == other.normal_vector
            && self.over_point == other.over_point
            && self.inside == other.inside
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

impl<'a> std::fmt::Debug for Intersection<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Intersection")
            .field("t", &self.t)
            .field("point", &self.point)
            .field("eye_vector", &self.eye_vector)
            .field("normal_vector", &self.normal_vector)
            .field("inside", &self.inside)
            .finish()
            .unwrap();
        f.write_fmt(format_args!("object {:?}", std::ptr::addr_of!(*self.shape)))
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
        let i = Intersection::new(3.5, &s, None);
        assert_eq!(i.t, 3.5);
        assert!(std::ptr::eq(i.shape, &s));
    }

    #[test]
    pub fn intersection_list() {
        let s = Sphere::new(None);
        let i1 = Intersection::new(1., &s, None);
        let i2 = Intersection::new(2., &s, None);
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
        let i1 = Intersection::new(1., &s, None);
        let i2 = Intersection::new(2., &s, None);
        let i = IntersectionList::new(vec![i1, i2]);
        assert_eq!(i.hit(), Some(&i.intersections[0]));

        let i1 = Intersection::new(-1., &s, None);
        let i2 = Intersection::new(1., &s, None);
        let i = IntersectionList::new(vec![i1.clone(), i2.clone()]);
        assert_eq!(i.hit(), Some(&i2));

        let i1 = Intersection::new(-2., &s, None);
        let i2 = Intersection::new(-1., &s, None);
        let i = IntersectionList::new(vec![i1, i2]);
        assert_eq!(i.hit(), None);

        let i1 = Intersection::new(5., &s, None);
        let i2 = Intersection::new(7., &s, None);
        let i3 = Intersection::new(-3., &s, None);
        let i4 = Intersection::new(2., &s, None);
        let i = IntersectionList::new(vec![i1.clone(), i2.clone(), i3.clone(), i4.clone()]);
        assert_eq!(i.hit(), Some(&i4));
    }

    #[test]
    fn intersection_context() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let shape = Sphere::new(None);
        let i = r.intersect(&shape).intersections;
        assert_eq!(i[0].t, 4.);
        assert!(std::ptr::eq(i[0].shape.as_any(), shape.as_any()));
        assert_eq!(i[0].point, Tuple::point(0., 0., -1.));
        assert_eq!(i[0].eye_vector, Tuple::vector(0., 0., -1.));
        assert_eq!(i[0].normal_vector, Tuple::vector(0., 0., -1.));
        assert_eq!(i[0].inside, false);

        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
        let shape = Sphere::new(None);
        let i = r.intersect(&shape);
        let i = i.hit().unwrap();
        assert_eq!(i.t, 1.);
        assert!(std::ptr::eq(i.shape.as_any(), shape.as_any()));
        assert_eq!(i.point, Tuple::point(0., 0., 1.));
        assert_eq!(i.eye_vector, Tuple::vector(0., 0., -1.));
        assert_eq!(i.normal_vector, Tuple::vector(0., 0., -1.));
        assert_eq!(i.inside, true);
    }

    #[test]
    fn shade_intersection() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let shape = &w.objects[0];
        let i = r.intersect(shape);
        let i = i.hit().unwrap();
        assert_eq!(i.shade(&w), Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shade_inside() {
        let mut w = World::default();
        w.lights[0] = PointLight::new(Tuple::point(0., 0.25, 0.), Color::new(1., 1., 1.));
        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
        let shape = &w.objects[1];
        let i = r.intersect(shape);
        let i = i.hit().unwrap();
        assert_eq!(i.shade(&w), Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn hit_offset_point() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let mut shape = Sphere::new(None);
        shape.set_transform(&Matrix::translation(0., 0., 1.));
        let i = r.intersect(&shape);
        let hit = i.hit().unwrap();
        assert!(hit.over_point.z < -EPSILON / 2.);
        assert!(hit.point.z > hit.over_point.z);
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
