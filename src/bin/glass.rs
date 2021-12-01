use raytracer::{
    camera::{Camera, SuperSamplingMode},
    color::{Color},
    light::PointLight,
    material::Material,
    matrix::Matrix,
    pattern::{CheckerPattern, StripePattern},
    plane::Plane,
    sphere::Sphere,
    tuple::Tuple,
    world::World,
};

fn main() {
    let mut wall_material = Material::new();
    let mut wall_pattern = StripePattern::new(vec![
        Color::new(0.45, 0.45, 0.45),
        Color::new(0.55, 0.55, 0.55),
    ]);
    wall_pattern.set_transform(&(&Matrix::scaling(0.25, 0.25, 0.25) * &Matrix::rotation_y(1.5708)));
    wall_material.pattern = Some(wall_pattern);
    wall_material.ambient = 0.;
    wall_material.diffuse = 0.4;
    wall_material.specular = 0.;
    wall_material.reflective = 0.3;

    let mut floor_material = Material::new();
    let floor_pattern =
        CheckerPattern::new(Color::new(0.35, 0.35, 0.35), Color::new(0.65, 0.65, 0.65));
    floor_material.specular = 0.;
    floor_material.reflective = 0.;
    floor_material.pattern = Some(floor_pattern);
    let mut floor = Plane::new(Some(floor_material));
    floor.transform = Matrix::rotation_y(0.31415);

    let mut ceiling_material = Material::new();
    ceiling_material.color = Color::new(0.8, 0.8, 0.8);
    ceiling_material.ambient = 0.3;
    ceiling_material.specular = 0.;
    let mut ceiling = Plane::new(Some(ceiling_material));
    ceiling.transform = Matrix::translation(0., 5., 0.);

    let mut west_wall = Plane::new(Some(wall_material.clone()));
    west_wall.transform = &Matrix::translation(-5., 0., 0.)
        * &Matrix::rotation_z(1.5708)
        * &Matrix::rotation_y(1.5708);

    let mut east_wall = Plane::new(Some(wall_material.clone()));
    east_wall.transform = &Matrix::translation(5., 0., 0.)
        * &Matrix::rotation_z(1.5708)
        * &Matrix::rotation_y(1.5708);

    let mut north_wall = Plane::new(Some(wall_material.clone()));
    north_wall.transform = &Matrix::translation(0., 0., 5.) * &Matrix::rotation_x(1.5708);

    let mut south_wall = Plane::new(Some(wall_material.clone()));
    south_wall.transform = &Matrix::translation(0., 0., -5.) * &Matrix::rotation_x(1.5708);

    let mut sphere1_material = Material::new();
    sphere1_material.color = Color::new(0.8, 0.5, 0.3);
    sphere1_material.shininess = 50.;
    let mut sphere1 = Sphere::new(Some(sphere1_material));
    sphere1.transform = &Matrix::translation(4.6, 0.4, 1.) * &Matrix::scaling(0.4, 0.4, 0.4);

    let mut sphere2_material = Material::new();
    sphere2_material.color = Color::new(0.9, 0.4, 0.5);
    sphere2_material.shininess = 50.;
    let mut sphere2 = Sphere::new(Some(sphere2_material));
    sphere2.transform = &Matrix::translation(4.7, 0.3, 0.4) * &Matrix::scaling(0.3, 0.3, 0.3);

    let mut sphere3_material = Material::new();
    sphere3_material.color = Color::new(0.4, 0.9, 0.6);
    sphere3_material.shininess = 50.;
    let mut sphere3 = Sphere::new(Some(sphere3_material));
    sphere3.transform = &Matrix::translation(-1., 0.5, 4.5) * &Matrix::scaling(0.5, 0.5, 0.5);

    let mut sphere4_material = Material::new();
    sphere4_material.color = Color::new(0.4, 0.6, 0.9);
    sphere4_material.shininess = 50.;
    let mut sphere4 = Sphere::new(Some(sphere4_material));
    sphere4.transform = &Matrix::translation(-1.7, 0.3, 4.7) * &Matrix::scaling(0.3, 0.3, 0.3);

    let mut sphere5_material = Material::new();
    sphere5_material.color = Color::new(1., 0.3, 0.2);
    sphere5_material.specular = 0.4;
    sphere5_material.shininess = 5.;
    let mut sphere5 = Sphere::new(Some(sphere5_material));
    sphere5.transform = Matrix::translation(-0.6, 1., 0.6);

    let mut sphere6_material = Material::new();
    sphere6_material.color = Color::new(0., 0., 0.2);
    sphere6_material.ambient = 0.;
    sphere6_material.diffuse = 0.4;
    sphere6_material.specular = 0.9;
    sphere6_material.shininess = 300.;
    sphere6_material.reflective = 0.9;
    sphere6_material.transparency = 0.9;
    sphere6_material.refractive_index = 1.5;
    let mut sphere6 = Sphere::new(Some(sphere6_material));
    sphere6.transform = &Matrix::translation(0.6, 0.7, -0.6) * &Matrix::scaling(0.7, 0.7, 0.7);

    let mut sphere7_material = Material::new();
    sphere7_material.color = Color::new(0., 0.2, 0.);
    sphere7_material.ambient = 0.;
    sphere7_material.diffuse = 0.4;
    sphere7_material.specular = 0.9;
    sphere7_material.shininess = 300.;
    sphere7_material.reflective = 0.9;
    sphere7_material.transparency = 0.9;
    sphere7_material.refractive_index = 1.5;
    let mut sphere7 = Sphere::new(Some(sphere7_material));
    sphere7.transform = &Matrix::translation(-0.7, 0.5, -0.8) * &Matrix::scaling(0.5, 0.5, 0.5);

    let light = PointLight::new(Tuple::point(-4.9, 4.9, -1.), Color::new(1., 1., 1.));

    let world = World::new(
        vec![
            floor, ceiling, west_wall, east_wall, north_wall, south_wall, sphere1, sphere2,
            sphere3, sphere4, sphere5, sphere6, sphere7
        ],
        vec![light],
    );
    let mut camera = Camera::new(800, 400, 1.152, SuperSamplingMode::Stochastic);
    camera.transform = Matrix::view_transform(
        Tuple::point(-2.6, 1.5, -3.9),
        Tuple::point(-0.6, 1., -0.8),
        Tuple::vector(0., 1., 0.),
    );

    let canvas = camera.render(&world);
    canvas.save_ppm("glass_spheres.ppm");
}
