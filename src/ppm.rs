use crate::canvas::Canvas;
use std::io::{prelude::*, Result};

impl Canvas {
    fn write_ppm(&self, sink: &mut impl Write) -> Result<()> {
        writeln!(sink, "P3")?;
        writeln!(sink, "{} {}", self.width, self.height)?;
        writeln!(sink, "255")?;
        //sink.flush()?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::str::from_utf8;

    use crate::{
        canvas::Canvas,
        color::{self, Color},
    };

    #[test]
    fn test_header() {
        let c = Canvas::new(5, 3);
        let mut bytes = Vec::new();
        c.write_ppm(&mut bytes).unwrap();
        //let s: String = String::from_utf8(bytes).unwrap();

        assert_eq!(
            bytes,
            b"P3
5 3
255
"
        )
    }

    #[test]
    fn test_ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);

        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 1, c3);

        let mut bytes = Vec::new();
        c.write_ppm(&mut bytes).unwrap();
        let lines: Vec<_> = from_utf8(&bytes).unwrap().lines().collect();
        assert_eq!(lines[3], "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
    }
}
