use raytracer::{
    camera::Camera,
    color::{Color, BLACK, WHITE},
    light::PointLight,
    material::Material,
    matrix::Matrix,
    pattern::{CheckerPattern, GradientPattern, RingPattern, StripePattern},
    plane::Plane,
    shape::Shape,
    sphere::Sphere,
    tuple::Tuple,
    world::World,
    PI,
};

fn main() {
    let mut material = Material::new();
    let pattern = CheckerPattern::new(WHITE, Color::new(0.5, 0.5, 0.5));
    material.pattern = Some(pattern);
    let floor = Plane::new(Some(material));

    let mut material = Material::new();
    let mut pattern = RingPattern::new(vec![
        Color::new(0.5, 0.5, 0.5),
        WHITE,
        Color::new(0.7, 0.6, 0.7),
    ]);
    pattern.set_transform(&Matrix::shearing(1., 1., 0., 0., 0., 0.));
    material.pattern = Some(pattern);
    let mut wall = Plane::new(Some(material));
    wall.set_transform(&(&Matrix::translation(0., 0., 5.) * &Matrix::rotation_x(PI / 2.)));

    let mut material = Material::new();
    let mut pattern = StripePattern::new(vec![
        Color::new(0.5, 0.5, 0.5),
        WHITE,
        Color::new(0.7, 0.6, 0.7),
    ]);
    pattern.set_transform(&Matrix::scaling(0.35, 0.35, 0.35));
    material.diffuse = 0.7;
    material.specular = 0.3;
    material.pattern = Some(pattern);
    let mut sphere1 = Sphere::new(Some(material));
    sphere1.set_transform(&(&Matrix::translation(3., 1.5, -4.) * &Matrix::scaling(1.5, 1.5, 1.5)));

    material = Material::new();
    let mut pattern = RingPattern::new(vec![WHITE, Color::new(0.7, 0.6, 0.7)]);
    pattern.set_transform(&Matrix::scaling(0.2, 0.2, 0.2));
    material.diffuse = 0.7;
    material.specular = 0.3;
    material.pattern = Some(pattern);
    let mut sphere2 = Sphere::new(Some(material));
    sphere2.set_transform(
        &(&Matrix::translation(-3., 1.5, -4.)
            * &Matrix::rotation_x(PI / 2.)
            * &Matrix::scaling(1.5, 1.5, 1.5)),
    );

    material = Material::new();
    let mut pattern = GradientPattern::new(Color::new(0.7, 0.6, 0.7), BLACK);
    pattern.set_transform(&(&Matrix::translation(-1., 0., 0.) * &Matrix::scaling(2., 1., 1.)));
    material.diffuse = 0.7;
    material.specular = 0.3;
    material.pattern = Some(pattern);
    let mut sphere3 = Sphere::new(Some(material));
    sphere3
        .set_transform(&(&Matrix::translation(0., 1., -7.) * &Matrix::scaling(0.33, 0.33, 0.33)));

    let light = PointLight::new(Tuple::point(-7., 10., -10.), Color::new(1., 1., 1.));

    let world = World::new(
        vec![
            Box::new(floor),
            Box::new(wall),
            Box::new(sphere1),
            Box::new(sphere2),
            Box::new(sphere3),
        ],
        vec![light],
    );
    let mut camera = Camera::new(800, 400, PI / 1.5);
    camera.transform = Matrix::view_transform(
        Tuple::point(-1., 2., -9.),
        Tuple::point(0., 1., 0.),
        Tuple::vector(0., 1., 0.),
    );

    let canvas = camera.render_supersample(&world);
    canvas.save_ppm("world_pattern.ppm");
}
