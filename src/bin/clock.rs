use raytracer::{canvas::Canvas, color::Color, matrix::Matrix, tuple::Tuple, PI};

fn main() {
    let mut c = Canvas::new(500, 500);
    let radius = 3. / 8. * c.width as f64;
    let points = vec![Tuple::point(0., 1., 0.); 12];

    let translation = Matrix::translation(c.width as f64 / 2., c.height as f64 / 2., 0.);
    let scaling = Matrix::scaling(radius, radius, 0.);

    let points: Vec<_> = points
        .iter()
        .enumerate()
        .map(|(i, x)| &translation * &scaling * &Matrix::rotation_z(PI * 2. * i as f64 / 12.) * *x)
        .collect();

    for point in points {
        c.write_pixel(
            point.x.round() as usize,
            point.y.round() as usize,
            Color::new(1., 1., 0.),
        );
    }

    c.save_ppm("clock.ppm");
}
