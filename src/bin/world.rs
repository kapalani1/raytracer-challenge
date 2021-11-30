use raytracer::{
    camera::Camera, color::Color, light::PointLight, material::Material, matrix::Matrix,
    plane::Plane, shape::Shape, sphere::Sphere, tuple::Tuple, world::World, PI,
};

fn main() {
    let mut material = Material::new();
    material.color = Color::new(1., 0.9, 0.9);
    material.specular = 0.;
    let floor = Plane::new(Some(material));

    material = Material::new();
    material.color = Color::new(0.1, 1., 0.5);
    material.diffuse = 0.7;
    material.specular = 0.3;
    let mut middle = Sphere::new(Some(material));
    middle.set_transform(&Matrix::translation(-0.5, 1., 0.5));

    material = Material::new();
    material.color = Color::new(0.5, 1., 0.1);
    material.diffuse = 0.7;
    material.specular = 0.3;
    let mut right = Sphere::new(Some(material));
    right.set_transform(&(&Matrix::translation(1.5, 0.5, -0.5) * &Matrix::scaling(0.5, 0.5, 0.5)));

    material = Material::new();
    material.color = Color::new(1., 0.8, 0.1);
    material.diffuse = 0.7;
    material.specular = 0.3;
    let mut left = Sphere::new(Some(material));
    left.set_transform(
        &(&Matrix::translation(-1.5, 0.33, -0.75) * &Matrix::scaling(0.33, 0.33, 0.33)),
    );

    let light = PointLight::new(Tuple::point(-10., 10., -10.), Color::new(1., 1., 1.));

    let world = World::new(
        vec![
            Box::new(floor),
            Box::new(middle),
            Box::new(right),
            Box::new(left),
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
    canvas.save_ppm("world_plane.ppm");
}
