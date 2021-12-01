use crate::{color::Color, matrix::Matrix, shape::Object, tuple::Tuple};
use noise::{NoiseFn, Seedable, SuperSimplex};
use rand::Rng;

#[derive(Debug, Clone)]
pub enum PatternType {
    StripePattern(StripePattern),
    GradientPattern(GradientPattern),
    RingPattern(RingPattern),
    CheckerPattern(CheckerPattern),
    RadialGradientPattern(RadialGradientPattern),
    TestPattern(TestPattern),
}

#[derive(Debug, Clone)]
pub struct Pattern {
    transform: Matrix,
    perturb: Option<SuperSimplex>,
    pattern_type: PatternType,
}

impl Pattern {
    fn new(pattern_type: PatternType) -> Self {
        Self {
            transform: Matrix::identity(4),
            perturb: None,
            pattern_type,
        }
    }

    pub fn perturb(&mut self) {
        self.perturb = Some(SuperSimplex::new().set_seed(rand::thread_rng().gen::<u32>()));
    }

    fn pattern_at(&self, point: Tuple) -> Color {
        assert!(point.is_point());
        let point = match self.perturb {
            Some(simplex) => {
                let simplex = 0.15 * simplex.get([point.x, point.y, point.z]);
                Tuple::point(point.x + simplex, point.y + simplex, point.z + simplex)
            }
            None => point,
        };

        match &self.pattern_type {
            PatternType::StripePattern(stripe) => stripe.color_at(point),
            PatternType::GradientPattern(gradient) => gradient.color_at(point),
            PatternType::RingPattern(ring) => ring.color_at(point),
            PatternType::CheckerPattern(checker) => checker.color_at(point),
            PatternType::RadialGradientPattern(radial_gradient) => radial_gradient.color_at(point),
            PatternType::TestPattern(_) => Color::new(point.x, point.y, point.z),
        }
    }

    pub fn pattern_at_object(&self, object: &Object, point: Tuple) -> Color {
        let object_point = object.transform.inverse() * point;
        let pattern_point = self.transform.inverse() * object_point;
        self.pattern_at(pattern_point)
    }

    pub fn set_transform(&mut self, m: &Matrix) {
        self.transform = m.clone();
    }
}

#[derive(Debug, Clone)]
pub struct StripePattern {
    pub colors: Vec<Color>,
}

impl StripePattern {
    pub fn new(colors: Vec<Color>) -> Pattern {
        Pattern::new(PatternType::StripePattern(StripePattern { colors }))
    }

    pub fn color_at(&self, point: Tuple) -> Color {
        self.colors[point.x.floor().abs() as usize % self.colors.len()]
    }
}

#[derive(Debug, Clone)]
pub struct GradientPattern {
    pub a: Color,
    pub b: Color,
}

impl GradientPattern {
    pub fn new(a: Color, b: Color) -> Pattern {
        Pattern::new(PatternType::GradientPattern(GradientPattern { a, b }))
    }

    pub fn color_at(&self, point: Tuple) -> Color {
        self.a + (self.b - self.a) * (point.x - point.x.floor())
    }
}

#[derive(Debug, Clone)]
pub struct RingPattern {
    pub colors: Vec<Color>,
}

impl RingPattern {
    pub fn new(colors: Vec<Color>) -> Pattern {
        Pattern::new(PatternType::RingPattern(RingPattern { colors }))
    }

    pub fn color_at(&self, point: Tuple) -> Color {
        self.colors
            [(point.x * point.x + point.z * point.z).sqrt().floor() as usize % self.colors.len()]
    }
}

#[derive(Debug, Clone)]
pub struct CheckerPattern {
    pub a: Color,
    pub b: Color,
}

impl CheckerPattern {
    pub fn new(a: Color, b: Color) -> Pattern {
        Pattern::new(PatternType::CheckerPattern(CheckerPattern { a, b }))
    }

    pub fn color_at(&self, point: Tuple) -> Color {
        match (point.x.floor() + point.y.floor() + point.z.floor()) as i64 % 2 {
            0 => self.a,
            _ => self.b,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RadialGradientPattern {
    pub a: Color,
    pub b: Color,
}

impl RadialGradientPattern {
    pub fn new(a: Color, b: Color) -> Pattern {
        Pattern::new(PatternType::RadialGradientPattern(RadialGradientPattern {
            a,
            b,
        }))
    }

    pub fn color_at(&self, point: Tuple) -> Color {
        let dist = (point.x * point.x + point.z * point.z).sqrt();
        self.a + (self.b - self.a) * (dist - dist.floor())
    }
}

#[derive(Debug, Clone)]
pub struct TestPattern;

impl TestPattern {
    pub fn new() -> Pattern {
        Pattern::new(PatternType::TestPattern(TestPattern))
    }
}

#[cfg(test)]
mod tests {
    use crate::color::{BLACK, WHITE};
    use crate::material::Material;
    use crate::sphere::Sphere;

    use super::StripePattern;
    use super::*;

    #[test]
    fn stripe() {
        let pattern = StripePattern::new(vec![WHITE, BLACK]);
        assert_eq!(pattern.transform, Matrix::identity(4));

        let mut p = StripePattern::new(vec![WHITE, BLACK]);
        p.set_transform(&Matrix::translation(1., 2., 3.));
        assert_eq!(p.transform, Matrix::translation(1., 2., 3.));

        assert_eq!(pattern.pattern_at(Tuple::point(0., 0., 0.)), WHITE);
        assert_eq!(pattern.pattern_at(Tuple::point(0., 1., 0.)), WHITE);
        assert_eq!(pattern.pattern_at(Tuple::point(0., 2., 0.)), WHITE);

        assert_eq!(pattern.pattern_at(Tuple::point(0., 0., 0.)), WHITE);
        assert_eq!(pattern.pattern_at(Tuple::point(0., 0., 1.)), WHITE);
        assert_eq!(pattern.pattern_at(Tuple::point(0., 0., 2.)), WHITE);

        assert_eq!(pattern.pattern_at(Tuple::point(0., 0., 0.)), WHITE);
        assert_eq!(pattern.pattern_at(Tuple::point(0.9, 0., 0.)), WHITE);
        assert_eq!(pattern.pattern_at(Tuple::point(1., 0., 0.)), BLACK);
        assert_eq!(pattern.pattern_at(Tuple::point(-0.1, 0., 0.)), BLACK);
        assert_eq!(pattern.pattern_at(Tuple::point(-0.9, 0., 0.)), BLACK);
        assert_eq!(pattern.pattern_at(Tuple::point(-1.1, 0., 0.)), WHITE);
    }

    #[test]
    fn stripe_at() {
        let mut object = Sphere::new(None);
        object.transform = Matrix::scaling(2., 2., 2.);
        let pattern = StripePattern::new(vec![WHITE, BLACK]);
        let c = pattern.pattern_at_object(&object, Tuple::point(1.5, 0., 0.));
        assert_eq!(c, WHITE);

        let mut pattern = StripePattern::new(vec![WHITE, BLACK]);
        pattern.set_transform(&Matrix::scaling(2., 2., 2.));
        let c = pattern.pattern_at_object(&object, Tuple::point(1.5, 0., 0.));
        assert_eq!(c, WHITE);

        let mut object = Sphere::new(None);
        object.transform = Matrix::scaling(2., 2., 2.);
        let mut pattern = StripePattern::new(vec![WHITE, BLACK]);
        pattern.set_transform(&Matrix::scaling(0.5, 0.5, 0.5));
        let c = pattern.pattern_at_object(&object, Tuple::point(2.5, 0., 0.));
        assert_eq!(c, WHITE);
    }

    #[test]
    fn gradient_pattern() {
        let pattern = GradientPattern::new(WHITE, BLACK);
        assert_eq!(pattern.pattern_at(Tuple::point(0., 0., 0.)), WHITE);
        assert_eq!(
            pattern.pattern_at(Tuple::point(0.25, 1., 0.)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(0.5, 1., 0.)),
            Color::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(0.75, 1., 0.)),
            Color::new(0.25, 0.25, 0.25)
        );
    }

    #[test]
    fn ring_pattern() {
        let pattern = RingPattern::new(vec![WHITE, BLACK]);
        assert_eq!(pattern.pattern_at(Tuple::point(0., 0., 0.)), WHITE);
        assert_eq!(pattern.pattern_at(Tuple::point(1., 0., 0.)), BLACK);
        assert_eq!(pattern.pattern_at(Tuple::point(0., 0., 1.)), BLACK);
        assert_eq!(pattern.pattern_at(Tuple::point(0.708, 0., 0.708)), BLACK);
    }

    #[test]
    fn checker_pattern() {
        let pattern = CheckerPattern::new(WHITE, BLACK);
        assert_eq!(pattern.pattern_at(Tuple::point(0., 0., 0.)), WHITE);
        assert_eq!(pattern.pattern_at(Tuple::point(0.99, 0., 0.)), WHITE);
        assert_eq!(pattern.pattern_at(Tuple::point(1.01, 0., 0.)), BLACK);

        assert_eq!(pattern.pattern_at(Tuple::point(0., 0., 0.)), WHITE);
        assert_eq!(pattern.pattern_at(Tuple::point(0., 0.99, 0.)), WHITE);
        assert_eq!(pattern.pattern_at(Tuple::point(0., 1.01, 0.)), BLACK);

        assert_eq!(pattern.pattern_at(Tuple::point(0., 0., 0.)), WHITE);
        assert_eq!(pattern.pattern_at(Tuple::point(0., 0., 0.99)), WHITE);
        assert_eq!(pattern.pattern_at(Tuple::point(0., 0., 1.01)), BLACK);
    }

    #[test]
    fn test_pattern() {
        let pattern = TestPattern::new();
        let mut material = Material::new();
        material.pattern = Some(pattern.clone());
        let mut s = Sphere::new(Some(material));
        s.transform = Matrix::scaling(2., 2., 2.);
        assert_eq!(
            pattern.pattern_at_object(&s, Tuple::point(2., 3., 4.)),
            Color::new(1., 1.5, 2.)
        );

        let mut pattern = TestPattern::new();
        pattern.set_transform(&Matrix::scaling(2., 2., 2.));
        let mut material = Material::new();
        material.pattern = Some(pattern.clone());
        let s = Sphere::new(Some(material));
        assert_eq!(
            pattern.pattern_at_object(&s, Tuple::point(2., 3., 4.)),
            Color::new(1., 1.5, 2.)
        );

        let mut pattern = TestPattern::new();
        pattern.set_transform(&Matrix::scaling(2., 2., 2.));
        let mut material = Material::new();
        material.pattern = Some(pattern.clone());
        let mut s = Sphere::new(Some(material));
        s.transform = Matrix::scaling(2., 2., 2.);
        assert_eq!(
            pattern.pattern_at_object(&s, Tuple::point(2., 3., 4.)),
            Color::new(0.5, 0.75, 1.)
        );
    }
}
