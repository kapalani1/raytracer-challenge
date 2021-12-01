use raytracer::{
    camera::Camera,
    color::{Color, BLACK, BLUE, WHITE},
    light::PointLight,
    material::Material,
    matrix::Matrix,
    pattern::{CheckerPattern, GradientPattern},
    plane::Plane,
    shape::Shape,
    sphere::Sphere,
    tuple::Tuple,
    world::World,
    PI,
};

fn main() {
    let mut material = Material::new();
    material.reflective = 0.3;
    let pattern = CheckerPattern::new(WHITE, Color::new(0.5, 0.5, 0.5));
    material.pattern = Some(pattern);
    let floor = Plane::new(Some(material));

    material = Material::new();
    material.diffuse = 0.7;
    material.specular = 0.3;
    material.reflective = 1.;
    let pattern = GradientPattern::new(BLUE, BLACK);
    material.pattern = Some(pattern);
    let mut sphere1 = Sphere::new(Some(material));
    sphere1.set_transform(&Matrix::translation(-1.3, 1.5, -4.));

    material = Material::new();
    material.diffuse = 0.7;
    material.specular = 0.3;
    material.transparency = 0.5;
    let mut sphere2 = Sphere::new(Some(material));
    sphere2.set_transform(&Matrix::translation(0.0, 2., -6.));

    let light = PointLight::new(Tuple::point(-5., 10., -10.), Color::new(1., 1., 1.));

    let world = World::new(
        vec![Box::new(floor), Box::new(sphere1), Box::new(sphere2)],
        vec![light],
        vec![]
    );
    let mut camera = Camera::new(800, 400, PI / 1.5);
    camera.transform = Matrix::view_transform(
        Tuple::point(-1., 2., -9.),
        Tuple::point(0., 1., 0.),
        Tuple::vector(0., 1., 0.),
    );

    let canvas = camera.render_supersample(&world);
    canvas.save_ppm("mirror_spheres.ppm");
}
