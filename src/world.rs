use crate::{
    color::Color, light::PointLight, material::Material, matrix::Matrix, sphere::Sphere,
    tuple::Tuple,
};

#[derive(Debug, Clone)]
pub struct World {
    pub objects: Vec<Sphere>,
    pub lights: Vec<PointLight>,
}

impl World {
    pub fn new(objects: Vec<Sphere>, lights: Vec<PointLight>) -> Self {
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
        s2.set_transform(&Matrix::scaling(0.5, 0.5, 0.5));

        World::new(vec![s1, s2], vec![light])
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
        println!("{:?}", w.lights[0]);
        let mut mat1 = Material::new();
        mat1.color = Color::new(0.8, 1., 0.6);
        mat1.diffuse = 0.7;
        mat1.specular = 0.2;
        let s1 = Sphere::new(Some(mat1));
        let mut s2 = Sphere::new(None);
        s2.set_transform(&Matrix::scaling(0.5, 0.5, 0.5));
        assert!(w.objects.contains(&s1));
        assert!(w.objects.contains(&s2));
    }

    #[test]
    fn ray_into_world() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let xs = r.project_into_world(&w);
        assert_eq!(xs.intersections.len(), 4);
        assert_eq!(xs.intersections[0].t, 4.);
        assert_eq!(xs.intersections[1].t, 4.5);
        assert_eq!(xs.intersections[2].t, 5.5);
        assert_eq!(xs.intersections[3].t, 6.);
    }
}
