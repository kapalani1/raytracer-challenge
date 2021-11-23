use crate::intersection::{Intersect, Intersection, IntersectionList};
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::tuple::Tuple;

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Tuple,
    pub radius: f64,
    pub transform: Matrix,
    pub material: Material,
}

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            center: Tuple::point(0., 0., 0.),
            radius: 1.,
            transform: Matrix::identity(4),
            material: Material::new(),
        }
    }

    pub fn set_transform(&mut self, m: &Matrix) {
        self.transform = m.clone();
    }
}

impl Intersect for Sphere {
    fn intersect(&self, ray: &Ray) -> IntersectionList {
        let ray = ray.transform(&self.transform.inverse());
        let sphere_to_ray = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2. * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.;
        let discriminant = b * b - 4. * a * c;

        if discriminant < 0. {
            IntersectionList::new(vec![])
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2. * a);
            let t2 = (-b + discriminant.sqrt()) / (2. * a);
            IntersectionList::new(vec![
                Intersection::new(t1, self),
                Intersection::new(t2, self),
            ])
        }
    }

    fn as_ref(&self) -> &dyn Intersect {
        self
    }

    fn normal(&self, point: Tuple) -> Tuple {
        let object_space_point = self.transform.inverse() * point;
        let object_normal = Tuple::vector(
            object_space_point.x,
            object_space_point.y,
            object_space_point.z,
        );
        let mut world_normal = self.transform.inverse().transpose() * object_normal;
        world_normal.w = 0.;
        world_normal.normalize()
    }

    fn material(&self) -> &Material {
        &self.material
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sphere() {
        let mut s = Sphere::new();
        assert_eq!(s.transform, Matrix::identity(4));
        assert_eq!(s.material, Material::new());
        let m = Matrix::translation(2., 3., 4.);
        s.set_transform(&m);
        assert_eq!(s.transform, m);
        let mut m = Material::new();
        m.ambient = 1.;
        s.material = m.clone();
        assert_eq!(s.material, m);
    }

    #[test]
    fn ray_sphere_intersection() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let mut s = Sphere::new();
        s.set_transform(&Matrix::scaling(2., 2., 2.));
        let i = r.intersect(&s);
        assert_eq!(i.intersections.len(), 2);
        assert_eq!(i.intersections[0].t, 3.);
        assert_eq!(i.intersections[1].t, 7.);

        s.set_transform(&Matrix::translation(5., 0., 0.));
        let i = r.intersect(&s);
        assert_eq!(i.intersections.len(), 0);
    }

    #[test]
    fn normal() {
        let s = Sphere::new();
        assert_eq!(
            s.normal(Tuple::point(1., 0., 0.)),
            Tuple::vector(1., 0., 0.)
        );
        assert_eq!(
            s.normal(Tuple::point(0., 1., 0.)),
            Tuple::vector(0., 1., 0.)
        );
        assert_eq!(
            s.normal(Tuple::point(0., 0., 1.)),
            Tuple::vector(0., 0., 1.)
        );
        let normal = s.normal(Tuple::point(
            3_f64.sqrt() / 3.,
            3_f64.sqrt() / 3.,
            3_f64.sqrt() / 3.,
        ));
        assert_eq!(
            normal,
            Tuple::vector(3_f64.sqrt() / 3., 3_f64.sqrt() / 3., 3_f64.sqrt() / 3.)
        );
        assert_eq!(normal.normalize(), normal);
    }

    #[test]
    fn normal_translated() {
        let mut s = Sphere::new();
        s.set_transform(&Matrix::translation(0., 1., 0.));
        assert_eq!(
            s.normal(Tuple::point(0., 1.70711, -0.70711)),
            Tuple::vector(0., 0.70711, -0.70711)
        );

        s.set_transform(
            &(Matrix::scaling(1., 0.5, 1.) * &Matrix::rotation_z(std::f64::consts::PI / 5.)),
        );
        assert_eq!(
            s.normal(Tuple::point(0., 2_f64.sqrt() / 2., -2_f64.sqrt() / 2.)),
            Tuple::vector(0., 0.97014, -0.24254)
        );
    }
}
