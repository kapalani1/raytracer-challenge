use crate::{
    canvas::Canvas, color::BLACK, matrix::Matrix, ray::Ray, shape::MAX_REFLECTIONS, tuple::Tuple,
    world::World,
};
use rand::Rng;
use rayon::prelude::*;

pub struct Camera {
    hsize: usize,
    vsize: usize,
    field_of_view: f64,
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
    pub transform: Matrix,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Self {
        let half_view = (field_of_view / 2.).tan();
        let aspect = hsize as f64 / vsize as f64;
        let (half_width, half_height) = if aspect >= 1. {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };

        let pixel_size = (half_width * 2.) / hsize as f64;

        Camera {
            hsize,
            vsize,
            field_of_view,
            half_width,
            half_height,
            pixel_size,
            transform: Matrix::identity(4),
        }
    }

    pub fn project_subsample_rays(&self, x: usize, y: usize) -> Vec<Ray> {
        let mut subsamples = vec![];
        for _ in 0..10 {
            subsamples.push((
                (x as f64 + rand::thread_rng().gen_range(0_f64..1.)) * self.pixel_size,
                (y as f64 + rand::thread_rng().gen_range(0_f64..1.)) * self.pixel_size,
            ));
        }
        subsamples
            .into_iter()
            .map(|(x, y)| {
                let world_x = self.half_width - x;
                let world_y = self.half_height - y;

                let pixel = self.transform.inverse() * Tuple::point(world_x, world_y, -1.);
                let origin = self.transform.inverse() * Tuple::point(0., 0., 0.);
                let direction = (pixel - origin).normalize();

                Ray::new(origin, direction)
            })
            .collect()
    }

    pub fn project_ray(&self, x: usize, y: usize) -> Ray {
        let x_offset = (x as f64 + 0.5) * self.pixel_size;
        let y_offset = (y as f64 + 0.5) * self.pixel_size;
        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let pixel = self.transform.inverse() * Tuple::point(world_x, world_y, -1.);
        let origin = self.transform.inverse() * Tuple::point(0., 0., 0.);
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut canvas = Canvas::new(self.hsize, self.vsize);
        canvas
            .pixels
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, color)| {
                let row = index / canvas.width;
                let col = index % canvas.width;
                let ray = self.project_ray(col, row);
                *color = ray.color_at(&world, MAX_REFLECTIONS);
            });

        canvas
    }

    pub fn render_supersample(&self, world: &World) -> Canvas {
        let mut canvas = Canvas::new(self.hsize, self.vsize);
        canvas
            .pixels
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, color)| {
                let row = index / canvas.width;
                let col = index % canvas.width;
                let ray = self.project_subsample_rays(col, row);
                *color = ray
                    .iter()
                    .map(|ray| ray.color_at(&world, MAX_REFLECTIONS))
                    .fold(BLACK, |a, b| a + b)
                    * (1.0 / ray.len() as f64);
            });

        canvas
    }
}

#[cfg(test)]
mod tests {
    use crate::{color::Color, EPSILON, PI};
    use float_cmp::approx_eq;

    use super::*;
    #[test]
    fn camera() {
        let c = Camera::new(160, 120, PI / 2.);
        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
        assert_eq!(c.field_of_view, PI / 2.);
        assert_eq!(c.transform, Matrix::identity(4));
    }

    #[test]
    fn pixel_size() {
        let c = Camera::new(200, 125, PI / 2.);
        approx_eq!(f64, c.pixel_size, 0.01, epsilon = EPSILON);
        let c = Camera::new(125, 200, PI / 2.);
        approx_eq!(f64, c.pixel_size, 0.01, epsilon = EPSILON);
    }

    #[test]
    fn camera_ray() {
        let mut c = Camera::new(201, 101, PI / 2.);
        let r = c.project_ray(100, 50);
        assert_eq!(
            r,
            Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., -1.))
        );
        let r = c.project_ray(0, 0);
        assert_eq!(
            r,
            Ray::new(
                Tuple::point(0., 0., 0.),
                Tuple::vector(0.66519, 0.33259, -0.66851)
            )
        );

        c.transform = Matrix::rotation_y(PI / 4.) * &Matrix::translation(0., -2., 5.);
        let r = c.project_ray(100, 50);
        assert_eq!(
            r,
            Ray::new(
                Tuple::point(0., 2., -5.),
                Tuple::vector(2_f64.sqrt() / 2., 0., -2_f64.sqrt() / 2.)
            )
        );
    }

    #[test]
    fn render() {
        let w = World::default();
        let mut c = Camera::new(11, 11, PI / 2.);
        let from = Tuple::point(0., 0., -5.);
        let to = Tuple::point(0., 0., 0.);
        let up = Tuple::vector(0., 1., 0.);
        c.transform = Matrix::view_transform(from, to, up);
        let canvas = c.render(&w);
        assert_eq!(canvas.get_pixel(5, 5), Color::new(0.38066, 0.47583, 0.2855));
    }
}
