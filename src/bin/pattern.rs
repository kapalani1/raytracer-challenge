use raytracer::{
  camera::Camera,
  color::{Color, WHITE},
  light::PointLight,
  material::Material,
  matrix::Matrix,
  pattern::{CheckerPattern, RadialGradientPattern, RingPattern, StripePattern},
  plane::Plane,
  shape::Shape,
  sphere::Sphere,
  tuple::Tuple,
  world::World,
  PI,
};

fn main() {
  let mut material = Material::new();
  let mut pattern = RingPattern::new(WHITE, Color::new(0.5, 0.5, 1.));
  pattern.perturb();
  pattern.set_transform(&Matrix::translation(-2., 0.5, 1.));
  material.pattern = Some(pattern);
  let floor = Plane::new(Some(material));

  let mut material = Material::new();
  let pattern = CheckerPattern::new(Color::new(0.1, 0.5, 0.1), WHITE);
  material.pattern = Some(pattern);
  let mut wall = Plane::new(Some(material));
  wall.set_transform(&(&Matrix::translation(0., 0., 10.) * &Matrix::rotation_x(PI / 4.)));

  let mut material = Material::new();
  let mut pattern = StripePattern::new(WHITE, Color::new(0.1, 0.1, 0.1));
  pattern.set_transform(&Matrix::scaling(0.1, 0.1, 0.1));
  pattern.perturb();
  material.pattern = Some(pattern);
  let mut middle = Sphere::new(Some(material));
  middle.set_transform(&(&Matrix::translation(-2., 0.5, 1.) * &Matrix::rotation_z(PI * 1.1)));

  material = Material::new();
  let mut pattern = StripePattern::new(Color::new(0.3, 0.8, 0.8), Color::new(0.5, 0.9, 0.9));
  pattern.set_transform(&(&Matrix::scaling(0.2, 0.2, 0.2) * &Matrix::rotation_z(PI)));
  pattern.perturb();
  material.pattern = Some(pattern);
  let mut right = Sphere::new(Some(material));
  right.set_transform(
      &(&Matrix::translation(1., 1., -2.)
          * &Matrix::scaling(1., 1., 1.)
          * &Matrix::rotation_y(PI / 6.)),
  );

  let light = PointLight::new(Tuple::point(-10., 10., -10.), Color::new(1., 1., 1.));

  let world = World::new(
      vec![
          Box::new(floor),
          Box::new(wall),
          Box::new(middle),
          Box::new(right),
      ],
      vec![light],
  );
  let mut camera = Camera::new(800, 400, PI / 3.);
  camera.transform = Matrix::view_transform(
      Tuple::point(0., 1.5, -5.),
      Tuple::point(0., 1., 0.),
      Tuple::vector(0., 1., 0.),
  );

  let canvas = camera.render(&world);
  canvas.save_ppm("world_pattern.ppm");
}
