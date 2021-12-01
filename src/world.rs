use crate::{
    color::Color, light::PointLight, material::Material, matrix::Matrix, ray::Ray, shape::Object,
    sphere::Sphere, tuple::Tuple,
};

pub struct World {
    pub objects: Vec<Object>,
    pub lights: Vec<PointLight>,
}

impl World {
    pub fn new(objects: Vec<Object>, lights: Vec<PointLight>) -> Self {
        World { objects, lights }
    }

    pub fn default() -> Self {
        let light = PointLight::new(Tuple::point(-10., 10., -10.), Color::new(1., 1., 1.));
        let mut mat1 = Material::new();
        mat1.color = Color::new(0.8, 1., 0.6);
        mat1.diffuse = 0.7;
        mat1.specular = 0.2;
        let s1 = Sphere::new(Some(mat1));

        let mut s2 = Sphere::new(None);
        s2.transform = Matrix::scaling(0.5, 0.5, 0.5);

        World::new(vec![s1, s2], vec![light])
    }

    pub fn is_shadowed(&self, point: Tuple) -> bool {
        assert!(point.is_point());
        assert_eq!(self.lights.len(), 1);
        let v = self.lights[0].position - point;
        let distance = v.magnitude();
        let direction = v.normalize();

        let r = Ray::new(point, direction);
        let i = r.intersect_world(&self);
        let hit = i.hit();
        match hit {
            Some(h) => {
                if h.t < distance {
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ray::Ray;

    use super::*;
    #[test]
    fn default_world() {
        let w = World::default();
        assert_eq!(
            w.lights[0],
            PointLight::new(Tuple::point(-10., 10., -10.), Color::new(1., 1., 1.))
        );
        let mut mat1 = Material::new();
        mat1.color = Color::new(0.8, 1., 0.6);
        mat1.diffuse = 0.7;
        mat1.specular = 0.2;
        let s1 = Sphere::new(Some(mat1));
        let mut s2 = Sphere::new(None);
        s2.transform = Matrix::scaling(0.5, 0.5, 0.5);
        assert_eq!(w.objects[0], s1);
        assert_eq!(w.objects[1], s2);
    }

    #[test]
    fn ray_into_world() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let xs = r.intersect_world(&w);
        assert_eq!(xs.intersections.len(), 4);
        assert_eq!(xs.intersections[0].t, 4.);
        assert_eq!(xs.intersections[1].t, 4.5);
        assert_eq!(xs.intersections[2].t, 5.5);
        assert_eq!(xs.intersections[3].t, 6.);
    }

    #[test]
    fn shadows() {
        let w = World::default();
        let p = Tuple::point(0., 10., 0.);
        assert!(!w.is_shadowed(p));
        let p = Tuple::point(10., -10., 10.);
        assert!(w.is_shadowed(p));
        let p = Tuple::point(-20., -20., -20.);
        assert!(!w.is_shadowed(p));
        let p = Tuple::point(-2., 2., 2.);
        assert!(!w.is_shadowed(p));
    }
}
