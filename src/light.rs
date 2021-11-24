use crate::color::Color;
use crate::tuple::Tuple;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PointLight {
    pub intensity: Color,
    pub position: Tuple,
}

impl PointLight {
    pub fn new(position: Tuple, intensity: Color) -> Self {
        assert!(position.is_point());
        Self {
            intensity,
            position,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn point_light() {
        let light = PointLight::new(Tuple::point(0., 0., 0.), Color::new(1., 1., 1.));
        assert_eq!(light.position, Tuple::point(0., 0., 0.));
        assert_eq!(light.intensity, Color::new(1., 1., 1.));
    }
}
