use crate::canvas::Canvas;
use std::fmt::Write as FormatWrite;
use std::io::{prelude::*, Result};

fn clamp_int(f: f64) -> u16 {
    match (f * 255_f64).round() {
        v if v < 0. => 0,
        v if v > 255. => 255,
        v => v as u16,
    }
}

impl Canvas {
    pub fn write_ppm(&self, sink: &mut impl Write) -> Result<()> {
        writeln!(sink, "P3")?;
        writeln!(sink, "{} {}", self.width, self.height)?;
        writeln!(sink, "255")?;

        for row in 0..self.height {
            let mut tokens = vec![];
            for col in 0..self.width {
                let pixel = self.pixel_at(col, row);
                tokens.push(clamp_int(pixel.red()).to_string());
                tokens.push(clamp_int(pixel.green()).to_string());
                tokens.push(clamp_int(pixel.blue()).to_string());
            }
            let mut line = String::new();
            for token in tokens {
                if line.len() + token.len() + 2 > 70 {
                    writeln!(sink, "{}", line)?;
                    line = String::new();
                }
                if line.is_empty() {
                    write!(line, "{}", token).unwrap();
                } else {
                    write!(line, " {}", token).unwrap();
                }
            }
            if !line.is_empty() {
                writeln!(sink, "{}", line)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::str::from_utf8;

    use crate::{canvas::Canvas, color::Color};

    #[test]
    fn test_header() {
        let c = Canvas::new(5, 3);
        let mut bytes = Vec::new();
        c.write_ppm(&mut bytes).unwrap();
        let lines: Vec<_> = from_utf8(&bytes).unwrap().lines().collect();
        assert_eq!(lines[0], "P3");
        assert_eq!(lines[1], "5 3");
        assert_eq!(lines[2], "255");
    }

    #[test]
    fn test_ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);

        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);

        let mut bytes = Vec::new();
        c.write_ppm(&mut bytes).unwrap();
        let lines: Vec<_> = from_utf8(&bytes).unwrap().lines().collect();
        println!("{:?}", from_utf8(&bytes).unwrap());
        assert_eq!(lines[0], "P3");
        assert_eq!(lines[1], "5 3");
        assert_eq!(lines[2], "255");
        assert_eq!(lines[3], "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
        assert_eq!(lines[4], "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0");
        assert_eq!(lines[5], "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
    }

    #[test]
    fn test_ppm_line_wrap() {
        let mut canvas = Canvas::new(10, 2);
        let c = Color::new(1., 0.8, 0.6);
        for row in 0..canvas.height {
            for col in 0..canvas.width {
                canvas.write_pixel(col, row, c);
            }
        }

        let mut bytes = Vec::new();
        canvas.write_ppm(&mut bytes).unwrap();
        let lines: Vec<_> = from_utf8(&bytes).unwrap().lines().collect();
        assert_eq!(lines[0], "P3");
        assert_eq!(lines[1], "10 2");
        assert_eq!(lines[2], "255");
        assert_eq!(
            lines[3],
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"
        );
        assert_eq!(
            lines[4],
            "153 255 204 153 255 204 153 255 204 153 255 204 153"
        );
        assert_eq!(
            lines[5],
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"
        );
        assert_eq!(
            lines[6],
            "153 255 204 153 255 204 153 255 204 153 255 204 153"
        );
    }

    #[test]
    fn test_ppm_ends_with_eol() {
        let canvas = Canvas::new(5, 3);
        let mut bytes = Vec::new();
        canvas.write_ppm(&mut bytes).unwrap();
        let last_char = from_utf8(&bytes).unwrap().chars().rev().next();
        assert_eq!(last_char, Some('\n'));
    }
}
