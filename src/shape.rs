use crate::{
    intersection::IntersectionList,
    material::Material,
    matrix::Matrix,
    ray::Ray,
    shapes::Plane,
    shapes::{
        Cylinder,
        Cube, Sphere,
    },
    tuple::Tuple,
};

pub const MAX_REFLECTIONS: u8 = 5;
pub const MAX_REFRACTIONS: u8 = 5;

#[derive(Debug, PartialEq)]
pub enum ShapeType {
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
    Cylinder(Cylinder),
}

#[derive(Debug, PartialEq)]
pub struct Object {
    pub transform: Matrix,
    pub shape: ShapeType,
    pub material: Material,
}

impl Object {
    fn local_intersect(&self, ray_obj_space: &Ray) -> IntersectionList {
        match &self.shape {
            ShapeType::Sphere(ref sphere) => sphere.local_intersect(ray_obj_space, self),
            ShapeType::Plane(ref plane) => plane.local_intersect(ray_obj_space, self),
            ShapeType::Cube(ref cube) => cube.local_intersect(ray_obj_space, self),
            ShapeType::Cylinder(ref cylinder) => cylinder.local_intersect(ray_obj_space, self),
        }
    }

    fn local_normal_at(&self, point: Tuple) -> Tuple {
        match &self.shape {
            ShapeType::Sphere(ref sphere) => sphere.local_normal_at(point),
            ShapeType::Plane(ref plane) => plane.local_normal_at(point),
            ShapeType::Cube(ref cube) => cube.local_normal_at(point),
            ShapeType::Cylinder(ref cylinder) => cylinder.local_normal_at(point),
        }
    }

    pub fn intersect(&self, ray: &Ray) -> IntersectionList {
        let ray_obj_space = ray.transform(&(self.transform.inverse()));
        self.local_intersect(&ray_obj_space)
    }

    pub fn normal_at(&self, point: Tuple) -> Tuple {
        assert!(point.is_point());
        let object_space_point = self.transform.inverse() * point;
        let object_normal = self.local_normal_at(object_space_point);
        let mut world_normal = self.transform.inverse().transpose() * object_normal;
        world_normal.w = 0.;
        world_normal.normalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        color::Color, intersection::Intersection, light::PointLight, matrix::Matrix,
        shapes::Sphere, world::World, EPSILON,
    };

    #[test]
    pub fn intersection() {
        let s = Sphere::new(None);
        let i = Intersection::new(3.5, &s);
        assert_eq!(i.t, 3.5);
        assert!(std::ptr::eq(i.object, &s));
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
        assert!(std::ptr::eq(i.intersections[0].object, &s));
        assert!(std::ptr::eq(i.intersections[1].object, &s));
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
        let i = r.intersect_object(&shape).intersections;
        let c = i[0].context(&r, None);
        assert_eq!(c.t, 4.);
        assert!(std::ptr::eq(c.object, &shape));
        assert_eq!(c.point, Tuple::point(0., 0., -1.));
        assert_eq!(c.eye_vector, Tuple::vector(0., 0., -1.));
        assert_eq!(c.normal_vector, Tuple::vector(0., 0., -1.));
        assert_eq!(c.inside, false);

        let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
        let shape = Sphere::new(None);
        let i = r.intersect_object(&shape);
        let i = i.hit().unwrap();
        let c = i.context(&r, None);
        assert_eq!(c.t, 1.);
        assert!(std::ptr::eq(c.object, &shape));
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
        let i = r.intersect_object(shape);
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
        let i = r.intersect_object(shape);
        let i = i.hit().unwrap();
        let c = i.context(&r, None).shade_hit(&w, MAX_REFLECTIONS);
        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn hit_offset_point() {
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let mut shape = Sphere::new(None);
        shape.transform = Matrix::translation(0., 0., 1.);
        let i = r.intersect_object(&shape);
        let hit = i.hit().unwrap();
        let c = hit.context(&r, None);
        assert!(c.over_point.z < -EPSILON / 2.);
        assert!(c.point.z > c.over_point.z);
        assert!(c.under_point.z > EPSILON / 2.);
        assert!(c.point.z < c.under_point.z);
    }

    #[test]
    fn material_shape() {
        let s = Sphere::new(None);
        assert_eq!(s.material, Material::new());
        let mut s = Sphere::new(None);
        s.material.ambient = 1.;
        let mut m = Material::new();
        m.ambient = 1.;
        assert_eq!(s.material, m);
    }
}
