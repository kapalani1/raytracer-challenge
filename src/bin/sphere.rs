use raytracer::{canvas::Canvas, color::Color, ray::Ray, sphere::Sphere, tuple::Tuple};

fn main() {
    let mut c = Canvas::new(100, 100);
    let wall_height = 7.;
    let origin = Tuple::point(0., 0., -5.);
    let s = Sphere::new();

    for row in 0.. 100 {
      for col in 0..100 {
        let world_y = -(row as f64) * wall_height / 100. + wall_height / 2.;
        let world_x = col as f64 * wall_height / 100. - wall_height / 2.;
        let world_point = Tuple::point(world_x, world_y, 10.);
        let direction = (world_point - origin).normalize();
        let ray = Ray::new(origin, direction);
        if let Some(_) = ray.intersect(&s).hit() {
          c.write_pixel(col, row, Color::new(1., 0.65, 0.0));
        }
      }
    }

    c.save_ppm("sphere.ppm");
}
