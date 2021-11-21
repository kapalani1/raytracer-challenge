use crate::color::Color;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            pixels: vec![vec![Color::new(0., 0., 0.); width]; height],
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        // x dimension is width (cols) and y dimension is height (rows)
        self.pixels[y][x] = color;
    }

    fn add_component_to_line(&self, line: &mut String, ppm: &mut String, component: u8) {
        let c = format!("{}", component);
        if line.len() == 0 {
            line.push_str(c.as_str());
        } else {
            // +1 for space at the start
            if c.len() + line.len() + 1 <= 70 {
                line.push(' ');
                line.push_str(c.as_str());
            } else {
                // Cannot fit component in this line. Flush and add to a new line
                ppm.push_str(line.as_str());
                ppm.push('\n');
                line.clear();
                line.push_str(c.as_str());
            }
        }
    }

    fn write_ppm(&self) -> String {
        let mut ppm = String::new();
        ppm.push_str(format!("P3\n{} {}\n255\n", self.width, self.height).as_str());
        for row in 0..self.height {
            let mut line = String::new();
            for pixel in &self.pixels[row] {
                let mut scaled_pixel = pixel * 255.;
                scaled_pixel.clamp();
                self.add_component_to_line(&mut line, &mut ppm, scaled_pixel.red.round() as u8);
                self.add_component_to_line(&mut line, &mut ppm, scaled_pixel.green.round() as u8);
                self.add_component_to_line(&mut line, &mut ppm, scaled_pixel.blue.round() as u8);
            }
            // Row over, so flush line again
            if line.len() > 0 {
                ppm.push_str(line.as_str());
                ppm.push('\n');
            }
        }
        ppm
    }

    pub fn to_ppm(&self) -> String {
        self.write_ppm()
    }

    pub fn save_ppm(&self, path: &str) {
        std::fs::write(path, self.to_ppm()).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn canvas() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        for i in 0..c.height {
            for j in 0..c.width {
                assert_eq!(c.pixels[i][j], Color::new(0., 0., 0.));
            }
        }
    }

    #[test]
    fn write_pixel() {
        let mut c = Canvas::new(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        c.write_pixel(2, 3, Color::new(1., 0., 0.));
        for x in 0..c.width {
            for y in 0..c.height {
                if x == 2 && y == 3 {
                    assert_eq!(c.pixels[y][x], Color::new(1., 0., 0.))
                } else {
                    assert_eq!(c.pixels[y][x], Color::new(0., 0., 0.))
                }
            }
        }
    }

    #[test]
    fn write_ppm() {
        let mut c = Canvas::new(5, 3);
        c.write_pixel(0, 0, Color::new(1.5, 0., 0.));
        c.write_pixel(2, 1, Color::new(0., 0.5, 0.));
        c.write_pixel(4, 2, Color::new(-0.5, 0., 1.));
        assert_eq!(
            c.to_ppm(),
            "P3\n\
            5 3\n\
            255\n\
            255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n\
            0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n\
            0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n"
        );
    }

    #[test]
    fn write_ppm_long() {
        let mut c = Canvas::new(10, 2);
        for x in 0..c.width {
            for y in 0..c.height {
                c.write_pixel(x, y, Color::new(1., 0.8, 0.6));
            }
        }
        let ppm = c.to_ppm();
        for line in ppm.split('\n') {
            assert!(line.len() <= 70);
        }
        let pixels = &ppm
            .split_inclusive('\n')
            .filter(|s| s.len() > 1)
            .collect::<Vec<&str>>()[3..];
        let pixels = pixels.into_iter().fold(String::new(), |acc, x| acc + x);
        assert_eq!(
            pixels,
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n\
        153 255 204 153 255 204 153 255 204 153 255 204 153\n\
        255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n\
        153 255 204 153 255 204 153 255 204 153 255 204 153\n"
        );
    }
}
