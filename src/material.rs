use float_cmp::approx_eq;

use crate::{color::Color, light::PointLight, tuple::Tuple};

#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn new() -> Self {
        Self {
            color: Color::new(1., 1., 1.),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.,
        }
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
            && approx_eq!(f64, self.ambient, other.ambient, epsilon = 0.00001)
            && approx_eq!(f64, self.diffuse, other.diffuse, epsilon = 0.00001)
            && approx_eq!(f64, self.specular, other.specular, epsilon = 0.00001)
            && approx_eq!(f64, self.shininess, other.shininess, epsilon = 0.00001)
    }
}

pub fn lighting(
    material: &Material,
    light: &PointLight,
    point: Tuple,
    eye_vector: Tuple,
    normal_vector: Tuple,
) -> Color {
    let effective_color = material.color * light.intensity;
    let light_vector = (light.position - point).normalize();
    let ambient = effective_color * material.ambient;
    let light_dot_normal = light_vector.dot(&normal_vector);
    let mut diffuse = Color::new(0., 0., 0.);
    let mut specular = Color::new(0., 0., 0.);

    if light_dot_normal >= 0. {
        diffuse = effective_color * material.diffuse * light_dot_normal;

        let reflect_vector = -light_vector.reflect(&normal_vector);
        let reflect_dot_eye = reflect_vector.dot(&eye_vector);

        if reflect_dot_eye > 0. {
            let factor = reflect_dot_eye.powf(material.shininess);
            specular = light.intensity * material.specular * factor;
        }
    }

    ambient + diffuse + specular
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_lighting() {
        let m = Material::new();
        let position = Tuple::point(0., 0., 0.);

        let eye_vector = Tuple::vector(0., 0., -1.);
        let normal_vector = Tuple::vector(0., 0., -1.);
        let light = PointLight::new(Tuple::point(0., 0., -10.), Color::new(1., 1., 1.));
        let result = lighting(&m, &light, position, eye_vector, normal_vector);
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));

        let eye_vector = Tuple::vector(0., 2_f64.sqrt() / 2., -2_f64.sqrt() / 2.);
        let normal_vector = Tuple::vector(0., 0., -1.);
        let light = PointLight::new(Tuple::point(0., 0., -10.), Color::new(1., 1., 1.));
        let result = lighting(&m, &light, position, eye_vector, normal_vector);
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));

        let eye_vector = Tuple::vector(0., 0., -1.);
        let normal_vector = Tuple::vector(0., 0., -1.);
        let light = PointLight::new(Tuple::point(0., 10., -10.), Color::new(1., 1., 1.));
        let result = lighting(&m, &light, position, eye_vector, normal_vector);
        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));

        let eye_vector = Tuple::vector(0., -2_f64.sqrt() / 2., -2_f64.sqrt() / 2.);
        let normal_vector = Tuple::vector(0., 0., -1.);
        let light = PointLight::new(Tuple::point(0., 10., -10.), Color::new(1., 1., 1.));
        let result = lighting(&m, &light, position, eye_vector, normal_vector);
        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));

        let eye_vector = Tuple::vector(0., 0., -1.);
        let normal_vector = Tuple::vector(0., 0., -1.);
        let light = PointLight::new(Tuple::point(0., 0., 10.), Color::new(1., 1., 1.));
        let result = lighting(&m, &light, position, eye_vector, normal_vector);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
}
