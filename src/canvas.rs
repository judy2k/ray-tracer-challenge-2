use crate::color::Color;

pub struct Canvas {
    width: usize,
    height: usize,
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
}
