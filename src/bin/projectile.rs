use raytracer::{canvas::Canvas, tuple::Tuple, color::Color};

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

fn tick(env: &Environment, proj: &mut Projectile) {
    proj.position += proj.velocity;
    proj.velocity = proj.velocity + env.gravity + env.wind;
}

fn main() {
    let mut p = Projectile {
        position: Tuple::point(0., 1., 0.),
        velocity: Tuple::vector(1., 1.8, 0.).normalize() * 11.25,
    };
    let e = Environment {
        gravity: Tuple::vector(0., -0.1, 0.),
        wind: Tuple::vector(-0.01, 0., 0.),
    };

    let mut c = Canvas::new(900, 550);

    while p.position.y > 0. {
        let y = (c.height as f64 - p.position.y).round() as usize;
        let x = p.position.x.round() as usize;
        c.write_pixel(x, y, Color::new(0., 1., 0.));
        tick(&e, &mut p);
    }

    c.save_ppm("projectile.ppm");
}
