use crate::{color::Color, material::Material, ray::Ray, tuple::Tuple, world::World};
use std::{cmp::Ord, ops::Add};

pub trait Intersect {
    fn intersect<'a>(&'a self, ray: &Ray) -> IntersectionList<'a>;
    fn as_ref(&self) -> &dyn Intersect;
    fn normal(&self, point: Tuple) -> Tuple;
    fn material(&self) -> &Material;
}

#[derive(Clone)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: Box<&'a dyn Intersect>,
    pub point: Tuple,
    pub eye_vector: Tuple,
    pub normal_vector: Tuple,
    pub inside: bool,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: &'a dyn Intersect, ray: Option<&Ray>) -> Intersection<'a> {
        let ray = match ray {
            Some(r) => *r,
            None => Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 0.)),
        };

        let point = ray.position(t);
        let eye_vector = -ray.direction;
        let inside = object.normal(point).dot(&eye_vector) < 0.;
        let normal_vector = if inside {
            -object.normal(point)
        } else {
            object.normal(point)
        };

        Intersection {
            t,
            object: Box::new(object),
            point,
            eye_vector,
            normal_vector,
            inside,
        }
    }

    pub fn shade(&self, world: &World) -> Color {
        assert_eq!(world.lights.len(), 1);
        self.object.material().lighting(
            &world.lights[0],
            self.point,
            self.eye_vector,
            self.normal_vector,
        )
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
        f.write_fmt(format_args!(
            "object {:?}",
            std::ptr::addr_of!(*self.object)
        ))
    }
}

impl<'a> PartialEq for Intersection<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && std::ptr::eq(*self.object, *other.object)
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

#[derive(Debug)]
pub struct IntersectionList<'a> {
    pub intersections: Vec<Intersection<'a>>,
}

impl<'a> IntersectionList<'a> {
    pub fn new(intersections: Vec<Intersection<'a>>) -> Self {
        let mut sorted_intersections = intersections.clone();
        sorted_intersections.sort();
        IntersectionList {
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
    use crate::{light::PointLight, sphere::Sphere, world::World};

    #[test]
    pub fn intersection() {
        let s = Sphere::new(None);
        let i = Intersection::new(3.5, &s, None);
        assert_eq!(i.t, 3.5);
        assert!(std::ptr::eq(*i.object as *const _, &s as *const _));
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
        assert!(std::ptr::eq(
            *i.intersections[0].object as *const _,
            &s as *const _
        ));
        assert!(std::ptr::eq(
            *i.intersections[1].object as *const _,
            &s as *const _
        ));
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
        assert!(std::ptr::eq(
            *i[0].object as *const _,
            shape.as_ref() as *const _
        ));
        assert_eq!(i[0].point, Tuple::point(0., 0., -1.));
        assert_eq!(i[0].eye_vector, Tuple::vector(0., 0., -1.));
        assert_eq!(i[0].normal_vector, Tuple::vector(0., 0., -1.));
        assert_eq!(i[0].inside, false);

        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
        let shape = Sphere::new(None);
        let i = r.intersect(&shape);
        let i = i.hit().unwrap();
        assert_eq!(i.t, 1.);
        assert!(std::ptr::eq(
            *i.object as *const _,
            shape.as_ref() as *const _
        ));
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
}
