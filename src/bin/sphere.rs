use raytracer::{
    canvas::Canvas, color::Color, light::PointLight, material::lighting, ray::Ray, sphere::Sphere,
    tuple::Tuple,
};

fn main() {
    let mut c = Canvas::new(100, 100);
    let wall_height = 7.;
    let origin = Tuple::point(0., 0., -5.);
    let mut s = Sphere::new();
    s.material.color = Color::new(1., 0.2, 1.);
    let light = PointLight::new(Tuple::point(-10., 10., -10.), Color::new(1., 1., 1.));

    for row in 0..100 {
        for col in 0..100 {
            let world_y = -(row as f64) * wall_height / 100. + wall_height / 2.;
            let world_x = col as f64 * wall_height / 100. - wall_height / 2.;
            let world_point = Tuple::point(world_x, world_y, 10.);
            let direction = (world_point - origin).normalize();
            let ray = Ray::new(origin, direction);
            if let Some(hit) = ray.intersect(&s).hit() {
                let point = ray.position(hit.t);
                let normal = hit.object.normal(point);
                let eye = -ray.direction;
                let color = lighting(hit.object.material(), &light, point, eye, normal);
                c.write_pixel(col, row, color);
            }
        }
    }

    c.save_ppm("sphere.ppm");
}
