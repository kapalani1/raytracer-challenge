use rayon::prelude::*;
use raytracer::{
    canvas::Canvas, color::Color, light::PointLight, ray::Ray, sphere::Sphere, tuple::Tuple,
};

fn main() {
    let mut c = Canvas::new(500, 500);
    let wall_height = 7.;
    let origin = Tuple::point(0., 0., -5.);
    let mut s = Sphere::new();
    s.material.color = Color::new(1., 0.2, 1.);
    let light = PointLight::new(Tuple::point(-10., 10., -10.), Color::new(1., 1., 1.));

    c.pixels
        .par_iter_mut()
        .enumerate()
        .for_each(|(index, color)| {
            let row = index / c.width;
            let col = index % c.width;
            let world_y = -(row as f64) * wall_height / c.height as f64 + wall_height / 2.;
            let world_x = col as f64 * wall_height / c.width as f64 - wall_height / 2.;
            let world_point = Tuple::point(world_x, world_y, 10.);
            let direction = (world_point - origin).normalize();
            let ray = Ray::new(origin, direction);
            if let Some(hit) = ray.intersect(&s).hit() {
                let point = ray.position(hit.t);
                let normal = hit.object.normal(point);
                let eye = -ray.direction;
                *color = hit.object.material().lighting(&light, point, eye, normal);
            }
        });

    c.save_ppm("sphere.ppm");
}
