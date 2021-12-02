use raytracer::{
    camera::{Camera, SuperSamplingMode},
    color::{Color, BLACK, BLUE, GREEN, RED, WHITE},
    light::PointLight,
    material::Material,
    matrix::Matrix,
    pattern::{CheckerPattern, GradientPattern},
    shapes::Cube,
    shapes::Plane,
    tuple::Tuple,
    world::World,
    PI,
};

fn main() {
    let mut material = Material::new();
    material.reflective = 0.;
    let pattern = CheckerPattern::new(WHITE, Color::new(0.5, 0.5, 0.5));
    material.pattern = Some(pattern);
    let floor = Plane::new(Some(material.clone()));

    let mut left_wall = Plane::new(Some(material.clone()));
    left_wall.transform = &Matrix::translation(-15., 0., 0.) * &Matrix::rotation_z(PI / 2.);

    let mut right_wall = Plane::new(Some(material.clone()));
    right_wall.transform = &Matrix::translation(0., 0., 15.) * &Matrix::rotation_x(PI / 2.);

    material = Material::new();
    material.diffuse = 0.7;
    material.specular = 0.3;
    material.reflective = 0.05;
    let mut pattern = GradientPattern::new(BLUE, BLACK);
    pattern.set_transform(&(&Matrix::translation(-1., 0., 0.) * &Matrix::scaling(2., 1., 1.)));
    material.pattern = Some(pattern);
    let mut cube1 = Cube::new(Some(material.clone()));
    cube1.transform = &Matrix::translation(0., 2., 0.) * &Matrix::scaling(2., 2., 2.);

    let mut pattern = GradientPattern::new(RED, BLACK);
    pattern.set_transform(&(&Matrix::translation(-1., 0., 0.) * &Matrix::scaling(2., 1., 1.)));
    material.pattern = Some(pattern);
    let mut cube2 = Cube::new(Some(material.clone()));
    cube2.transform = &Matrix::translation(0., 5., 0.) * &Matrix::scaling(1., 1., 1.);

    let mut pattern = GradientPattern::new(GREEN, BLACK);
    pattern.set_transform(&(&Matrix::translation(-1., 0., 0.) * &Matrix::scaling(2., 1., 1.)));
    material.pattern = Some(pattern);
    let mut cube3 = Cube::new(Some(material.clone()));
    cube3.transform = &Matrix::translation(0., 6.5, 0.) * &Matrix::scaling(0.5, 0.5, 0.5);

    let light = PointLight::new(Tuple::point(-5., 10., -10.), Color::new(1., 1., 1.));

    let world = World::new(
        vec![floor, left_wall, right_wall, cube1, cube2, cube3],
        vec![light],
    );
    let mut camera = Camera::new(800, 400, PI / 1.9, SuperSamplingMode::Stochastic);
    camera.transform = Matrix::view_transform(
        Tuple::point(5., 2.5, -7.5),
        Tuple::point(1.5, 3., 0.),
        Tuple::vector(0., 1., 0.),
    );

    let canvas = camera.render(&world);
    canvas.save_ppm("cubes.ppm");
}
