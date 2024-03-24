use crate::{color::Color, space::Tuple};

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::new(0., 0., 0.); width * height],
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.pixels[y * self.width + x] = color;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[y * self.width + x]
    }

    pub fn plot_point(&mut self, point: &Tuple, color: &Color) {
        // TODO: Write tests for this function.
        let x = point.x().round() as usize;
        let y = point.y().round() as usize;

        let xmin = isize::max(0, x as isize - 1) as usize;
        let ymin = isize::max(0, y as isize - 1) as usize;

        let xmax = usize::min(x + 1, self.width - 1);
        let ymax = usize::min(y + 1, self.height - 1);

        for px in xmin..xmax + 1 {
            for py in ymin..ymax + 1 {
                self.write_pixel(px, py, *color);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_canvas() {
        let c = Canvas::new(10, 20);

        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);

        for col in 0..c.width {
            for row in 0..c.height {
                assert_eq!(c.pixel_at(col, row), Color::new(0.0, 0.0, 0.0));
            }
        }
    }

    #[test]
    fn test_write_pixel() {
        let mut c = Canvas::new(10, 20);

        let red = Color::new(1.0, 0., 0.);

        c.write_pixel(2, 3, red);
        assert_eq!(c.pixel_at(2, 3), Color::new(1.0, 0.0, 0.0));
    }
}
