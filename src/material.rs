use crate::{
    color::Color, light::PointLight, pattern::Pattern, shape::Object, tuple::Tuple, EPSILON,
};
use float_cmp::approx_eq;

#[derive(Debug, Clone)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub reflective: f64,
    pub transparency: f64,
    pub refractive_index: f64,
    pub pattern: Option<Pattern>,
}

impl Material {
    pub fn new() -> Self {
        Self {
            color: Color::new(1., 1., 1.),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.,
            reflective: 0.,
            transparency: 0.,
            refractive_index: 1.,
            pattern: None,
        }
    }

    pub fn lighting(
        &self,
        light: &PointLight,
        object: &Object,
        point: Tuple,
        eye_vector: Tuple,
        normal_vector: Tuple,
        in_shadow: bool,
    ) -> Color {
        assert!(point.is_point());
        assert!(eye_vector.is_vector());
        assert!(normal_vector.is_vector());

        let color = match self.pattern {
            None => self.color,
            Some(ref pattern) => pattern.pattern_at_object(object, point),
        };

        // Haddamard multiplication of material and light
        let effective_color = color * light.intensity;
        // Direction to light source
        let light_vector = (light.position - point).normalize();
        // Constant ambient contribution
        let ambient = effective_color * self.ambient;
        // If light is in front this quantity is positive else negative
        let light_dot_normal = light_vector.dot(&normal_vector);

        let mut diffuse = Color::new(0., 0., 0.);
        let mut specular = Color::new(0., 0., 0.);

        if !in_shadow && light_dot_normal >= 0. {
            // Diffuse contribution depends on angle between light and point
            diffuse = effective_color * self.diffuse * light_dot_normal;

            let reflect_vector = -light_vector.reflect(&normal_vector);
            let reflect_dot_eye = reflect_vector.dot(&eye_vector);

            if reflect_dot_eye > 0. {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }

        ambient + diffuse + specular
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
            && approx_eq!(f64, self.ambient, other.ambient, epsilon = EPSILON)
            && approx_eq!(f64, self.diffuse, other.diffuse, epsilon = EPSILON)
            && approx_eq!(f64, self.specular, other.specular, epsilon = EPSILON)
            && approx_eq!(f64, self.shininess, other.shininess, epsilon = EPSILON)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        color::{BLACK, WHITE},
        pattern::StripePattern,
        shapes::Sphere,
    };

    use super::*;
    #[test]
    pub fn test_lighting() {
        let m = Material::new();
        let position = Tuple::point(0., 0., 0.);

        let eye_vector = Tuple::vector(0., 0., -1.);
        let normal_vector = Tuple::vector(0., 0., -1.);
        let light = PointLight::new(Tuple::point(0., 0., -10.), Color::new(1., 1., 1.));
        let result = m.lighting(
            &light,
            &Sphere::new(None),
            position,
            eye_vector,
            normal_vector,
            false,
        );
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));

        let eye_vector = Tuple::vector(0., 2_f64.sqrt() / 2., -2_f64.sqrt() / 2.);
        let normal_vector = Tuple::vector(0., 0., -1.);
        let light = PointLight::new(Tuple::point(0., 0., -10.), Color::new(1., 1., 1.));
        let result = m.lighting(
            &light,
            &Sphere::new(None),
            position,
            eye_vector,
            normal_vector,
            false,
        );
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));

        let eye_vector = Tuple::vector(0., 0., -1.);
        let normal_vector = Tuple::vector(0., 0., -1.);
        let light = PointLight::new(Tuple::point(0., 10., -10.), Color::new(1., 1., 1.));
        let result = m.lighting(
            &light,
            &Sphere::new(None),
            position,
            eye_vector,
            normal_vector,
            false,
        );
        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));

        let eye_vector = Tuple::vector(0., -2_f64.sqrt() / 2., -2_f64.sqrt() / 2.);
        let normal_vector = Tuple::vector(0., 0., -1.);
        let light = PointLight::new(Tuple::point(0., 10., -10.), Color::new(1., 1., 1.));
        let result = m.lighting(
            &light,
            &Sphere::new(None),
            position,
            eye_vector,
            normal_vector,
            false,
        );
        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));

        let eye_vector = Tuple::vector(0., 0., -1.);
        let normal_vector = Tuple::vector(0., 0., -1.);
        let light = PointLight::new(Tuple::point(0., 0., 10.), Color::new(1., 1., 1.));
        let result = m.lighting(
            &light,
            &Sphere::new(None),
            position,
            eye_vector,
            normal_vector,
            false,
        );
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));

        let eye_vector = Tuple::vector(0., 0., -1.);
        let normal_vector = Tuple::vector(0., 0., -1.);
        let light = PointLight::new(Tuple::point(0., 0., -10.), Color::new(1., 1., 1.));
        let result = m.lighting(
            &light,
            &Sphere::new(None),
            position,
            eye_vector,
            normal_vector,
            true,
        );
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn test_pattern() {
        let mut m = Material::new();
        m.pattern = Some(StripePattern::new(vec![WHITE, BLACK]));
        m.ambient = 1.;
        m.diffuse = 0.;
        m.specular = 0.;

        let eye_vector = Tuple::vector(0., 0., -1.);
        let normal_vector = Tuple::vector(0., 0., -1.);
        let light = PointLight::new(Tuple::point(0., 0., -10.), Color::new(1., 1., 1.));
        let result = m.lighting(
            &light,
            &Sphere::new(None),
            Tuple::point(0.9, 0., 0.),
            eye_vector,
            normal_vector,
            false,
        );
        assert_eq!(result, Color::new(1., 1., 1.));
        let result = m.lighting(
            &light,
            &Sphere::new(None),
            Tuple::point(1.1, 0., 0.),
            eye_vector,
            normal_vector,
            false,
        );
        assert_eq!(result, Color::new(0., 0., 0.));
    }
}
