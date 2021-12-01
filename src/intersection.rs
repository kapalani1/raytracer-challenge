use std::ops::Add;

use crate::{
    color::{Color, BLACK},
    ray::Ray,
    shape::Object,
    tuple::Tuple,
    world::World,
    EPSILON,
};

// A single intersection
#[derive(Debug, Clone)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Object,
}

// A list of intersections
#[derive(Debug)]
pub struct IntersectionList<'a> {
    pub intersections: Vec<Intersection<'a>>,
}

// Contexts assosciated with an intersection
#[derive(Debug)]
pub struct IntersectionContext<'a> {
    pub t: f64,
    pub object: &'a Object,
    pub point: Tuple,
    pub eye_vector: Tuple,
    pub normal_vector: Tuple,
    pub reflect_vector: Tuple,
    pub inside: bool,
    pub over_point: Tuple,
    pub under_point: Tuple,
    pub n1: f64,
    pub n2: f64,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: &'a Object) -> Intersection<'a> {
        Self { t, object }
    }

    pub fn context(&'a self, ray: &Ray, xs: Option<&IntersectionList>) -> IntersectionContext {
        let point = ray.position(self.t);
        let eye_vector = -ray.direction;
        let inside = self.object.normal_at(point).dot(&eye_vector) < 0.;
        let normal_vector = if inside {
            -self.object.normal_at(point)
        } else {
            self.object.normal_at(point)
        };
        let over_point = point + normal_vector * EPSILON;
        let under_point = point - normal_vector * EPSILON;
        let reflect_vector = ray.direction.reflect(&normal_vector);

        let mut n1 = 0.;
        let n2 = 0.;

        if let Some(xs) = xs {
            if let Some(hit) = xs.hit() {
                let containers: Vec<&Object> = vec![];

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

        IntersectionContext {
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
            n2,
        }
    }
}

impl<'a> IntersectionContext<'a> {
    pub fn reflected_color(&self, world: &World, remaining: u8) -> Color {
        if self.object.material.reflective == 0. || remaining == 0 {
            BLACK
        } else {
            let reflect_ray = Ray::new(self.over_point, self.reflect_vector);
            reflect_ray.color_hit(world, remaining - 1) * self.object.material.reflective
        }
    }

    pub fn shade_hit(&self, world: &World, remaining: u8) -> Color {
        assert_eq!(world.lights.len(), 1);
        let in_shadow = world.is_shadowed(self.over_point);
        self.object.material.lighting(
            &world.lights[0],
            self.object,
            self.over_point,
            self.eye_vector,
            self.normal_vector,
            in_shadow,
        ) + self.reflected_color(world, remaining)
    }
}

// impl<'a> PartialEq for IntersectionContext<'a> {
//   fn eq(&self, other: &Self) -> bool {
//       self.t == other.t
//           && std::ptr::eq(self.object, other.object)
//           && self.point == other.point
//           && self.eye_vector == other.eye_vector
//           && self.normal_vector == other.normal_vector
//           && self.reflect_vector == other.reflect_vector
//           && self.inside == other.inside
//           && self.over_point == other.over_point
//           && self.under_point == other.under_point
//           && self.n1 == other.n1
//           && self.n2 == other.n2
//   }
// }

impl<'a> PartialEq for Intersection<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && std::ptr::eq(self.object, other.object)
    }
}

impl<'a> Eq for Intersection<'a> {}

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

impl<'a> IntersectionList<'a> {
    pub fn new(mut intersections: Vec<Intersection<'a>>) -> Self {
        intersections.sort();
        Self { intersections }
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
    use crate::{
        color::BLACK,
        intersection::{Intersection, IntersectionList},
        light::PointLight,
        material::Material,
        matrix::Matrix,
        plane::Plane,
        ray::Ray,
        shape::MAX_REFLECTIONS,
        sphere::Sphere,
    };

    #[test]
    fn reflection() {
        let m = Material::new();
        assert_eq!(m.reflective, 0.);

        let shape = Plane::new(None);
        let r = Ray::new(
            Tuple::point(0., 1., -1.),
            Tuple::vector(0., 2_f64.sqrt() / -2., 2_f64.sqrt() / 2.),
        );
        let i = r.intersect_object(&shape);
        assert_eq!(
            i.hit().unwrap().context(&r, None).reflect_vector,
            Tuple::vector(0., 2_f64.sqrt() / 2., 2_f64.sqrt() / 2.)
        );
    }

    #[test]
    fn reflect_color() {
        let mut w = World::default();
        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
        w.objects[1].material.ambient = 1.;
        let i = Intersection::new(1., &w.objects[1]);
        assert_eq!(
            i.context(&r, None).reflected_color(&w, MAX_REFLECTIONS),
            BLACK
        );

        let mut w = World::default();
        let mut material = Material::new();
        material.reflective = 0.5;
        let mut shape = Plane::new(Some(material));
        shape.transform = Matrix::translation(0., -1., 0.);
        w.objects.push(shape);
        let r = Ray::new(
            Tuple::point(0., 0., -3.),
            Tuple::vector(0., 2_f64.sqrt() / -2., 2_f64.sqrt() / 2.),
        );
        let i = Intersection::new(2_f64.sqrt(), w.objects.last().unwrap());
        assert_eq!(
            i.context(&r, None).reflected_color(&w, MAX_REFLECTIONS),
            Color::new(0.190332, 0.237915, 0.14274)
        );
        assert_eq!(
            i.context(&r, None).shade_hit(&w, MAX_REFLECTIONS),
            Color::new(0.876757, 0.92434, 0.82917)
        );
    }

    #[test]
    fn infinite_reflection() {
        let mut material = Material::new();
        material.reflective = 1.;
        let mut lower = Plane::new(Some(material.clone()));
        lower.transform = Matrix::translation(0., -1., 0.);

        let mut upper = Plane::new(Some(material.clone()));
        upper.transform = Matrix::translation(0., 1., 0.);
        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 1., 0.));

        let w = World::new(
            vec![lower, upper],
            vec![PointLight::new(
                Tuple::point(0., 0., 0.),
                Color::new(1., 1., 1.),
            )],
        );
        r.color_hit(&w, MAX_REFLECTIONS);
    }

    #[test]
    fn refractive_indices() {
        let mut a = Sphere::glass_new();
        a.material.refractive_index = 1.5;
        a.transform = Matrix::scaling(2., 2., 2.);

        let mut b = Sphere::glass_new();
        b.material.refractive_index = 2.;
        b.transform = Matrix::translation(0., 0., -0.25);

        let mut c = Sphere::glass_new();
        c.material.refractive_index = 2.5;
        c.transform = Matrix::translation(0., 0., 0.25);

        let r = Ray::new(Tuple::point(0., 0., -4.), Tuple::vector(0., 0., 1.));
        let xs = IntersectionList::new(vec![
            Intersection::new(2., &a),
            Intersection::new(2.75, &b),
            Intersection::new(3.25, &c),
            Intersection::new(4.75, &b),
            Intersection::new(5.25, &c),
            Intersection::new(6., &a),
        ]);

        assert_eq!(xs.intersections[0].context(&r, Some(&xs)).n1, 1.);
        // assert_eq!(xs.intersections[0].n2, 1.5);
        // assert_eq!(xs.intersections[1].n1, 1.5);
        // assert_eq!(xs.intersections[1].n1, 2.);
    }
}
