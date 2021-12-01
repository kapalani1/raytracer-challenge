use std::any::Any;

use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::{
    IntersectionList, Object, ObjectIntersection, ObjectIntersectionList, Shape, ShapeType,
};
use crate::tuple::Tuple;

#[derive(Debug, Clone, PartialEq)]
pub struct Sphere {
    pub center: Tuple,
    pub radius: f64,
    transform: Matrix,
    material: Material,
}

impl Sphere {
    pub fn new(material: Option<Material>) -> Self {
        let material = match material {
            Some(x) => x,
            None => Material::new(),
        };

        Sphere {
            center: Tuple::point(0., 0., 0.),
            radius: 1.,
            transform: Matrix::identity(4),
            material,
        }
    }

    pub fn glass_new() -> Self {
        let mut material = Material::new();
        material.transparency = 1.;
        material.refractive_index = 1.5;
        Sphere::new(Some(material))
    }

    pub fn object_new(material_opt: Option<Material>) -> Object {
        let material = match material_opt {
            Some(x) => x,
            None => Material::new(),
        };

        Object {
            transform: Matrix::identity(4),
            shape: ShapeType::Sphere(Sphere::new(Some(material.clone()))),
            material,
        }
    }

    pub fn local_object_intersect<'a>(
        &self,
        ray_obj_space: &Ray,
        object: &'a Object,
    ) -> ObjectIntersectionList<'a> {
        let sphere_to_ray = ray_obj_space.origin - self.center;
        let a = ray_obj_space.direction.dot(&ray_obj_space.direction);
        let b = 2. * ray_obj_space.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.;
        let discriminant = b * b - 4. * a * c;

        if discriminant < 0. {
            ObjectIntersectionList::new(vec![])
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2. * a);
            let t2 = (-b + discriminant.sqrt()) / (2. * a);
            ObjectIntersectionList::new(vec![
                ObjectIntersection::new(t1, object),
                ObjectIntersection::new(t2, object),
            ])
        }
    }

    pub fn local_object_normal(&self, object_space_point: Tuple) -> Tuple {
        Tuple::vector(
            object_space_point.x,
            object_space_point.y,
            object_space_point.z,
        )
    }
}

impl Shape for Sphere {
    fn local_normal(&self, object_space_point: Tuple) -> Tuple {
        Tuple::vector(
            object_space_point.x,
            object_space_point.y,
            object_space_point.z,
        )
    }

    fn material_mut(&mut self) -> &mut Material {
        &mut self.material
    }

    fn local_intersect(&self, ray_obj_space: &Ray) -> Vec<(f64, &dyn Shape)> {
        let sphere_to_ray = ray_obj_space.origin - self.center;
        let a = ray_obj_space.direction.dot(&ray_obj_space.direction);
        let b = 2. * ray_obj_space.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.;
        let discriminant = b * b - 4. * a * c;

        if discriminant < 0. {
            vec![]
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2. * a);
            let t2 = (-b + discriminant.sqrt()) / (2. * a);
            vec![(t1, self), (t2, self)]
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn transform(&self) -> &Matrix {
        &self.transform
    }

    fn set_transform(&mut self, m: &Matrix) {
        self.transform = m.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PI;

    #[test]
    fn sphere() {
        let mut s = Sphere::object_new(None);
        assert_eq!(s.transform, Matrix::identity(4));
        assert_eq!(s.material, Material::new());
        let m = Matrix::translation(2., 3., 4.);
        s.transform = m.clone();
        assert_eq!(s.transform, m);
        let mut m = Material::new();
        m.ambient = 1.;
        s.material = m.clone();
        assert_eq!(s.material, m);
    }

    #[test]
    fn ray_sphere_intersection() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let mut s = Sphere::object_new(None);
        s.transform = Matrix::scaling(2., 2., 2.);
        let i = r.intersect_object(&s);
        assert_eq!(i.intersections.len(), 2);
        assert_eq!(i.intersections[0].t, 3.);
        assert_eq!(i.intersections[1].t, 7.);

        s.transform = Matrix::translation(5., 0., 0.);
        let i = r.intersect_object(&s);
        assert_eq!(i.intersections.len(), 0);
    }

    #[test]
    fn normal() {
        let s = Sphere::object_new(None);
        assert_eq!(
            s.normal_object(Tuple::point(1., 0., 0.)),
            Tuple::vector(1., 0., 0.)
        );
        assert_eq!(
            s.normal_object(Tuple::point(0., 1., 0.)),
            Tuple::vector(0., 1., 0.)
        );
        assert_eq!(
            s.normal_object(Tuple::point(0., 0., 1.)),
            Tuple::vector(0., 0., 1.)
        );
        let normal = s.normal_object(Tuple::point(
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
        let mut s = Sphere::object_new(None);
        s.transform = Matrix::translation(0., 1., 0.);
        assert_eq!(
            s.normal_object(Tuple::point(0., 1.70711, -0.70711)),
            Tuple::vector(0., 0.70711, -0.70711)
        );

        s.transform = &Matrix::scaling(1., 0.5, 1.) * &Matrix::rotation_z(PI / 5.);
        assert_eq!(
            s.normal_object(Tuple::point(0., 2_f64.sqrt() / 2., -2_f64.sqrt() / 2.)),
            Tuple::vector(0., 0.97014, -0.24254)
        );
    }

    #[test]
    fn glass_sphere() {
        let s = Sphere::glass_new();
        assert_eq!(s.material().transparency, 1.);
        assert_eq!(s.material().refractive_index, 1.5);
    }
}
