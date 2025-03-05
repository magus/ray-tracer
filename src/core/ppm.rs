use std::io::Write;

use crate::core::Color;

/// https://en.wikipedia.org/wiki/Netpbm
/// Portable PixMap (P3)
///
/// "P3" means this is a RGB color image in ASCII
///
/// "3 2" is the width and height of the image in pixels
///
/// "255" is the maximum value for each color
///
/// This, up through the "255" line below are the header.
///
/// Everything after that is the image data: RGB triplets.
///
/// In order: red, green, blue, yellow, purple, white and black.
///
/// Another kind of line which _may_ appear in a ppm file is a comment line, which will start with a #. If a line starts with a #, the rest of that line, up to the newline character, is a comment.
///
/// ```ppm
/// P3
/// 3 2
/// 255
/// 255   0   0
///   0 255   0
///   0   0 255
/// 255 255   0
/// 255 0   255
/// 255 255 255
///   0   0   0
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct V3 {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

impl V3 {
    pub async fn save(&self, filepath: &str) -> Result<(), std::io::Error> {
        let tmp_filepath = format!("{filepath}.tmp");

        let file = std::fs::File::create(&tmp_filepath)?;
        let mut writer = std::io::BufWriter::new(file);

        writeln!(writer, "P3")?;
        writeln!(writer, "{} {}", self.width, self.height)?;
        writeln!(writer, "{}", Color::MAX_VALUE)?;

        for pixel in &self.pixels {
            writeln!(writer, "{pixel}")?;
        }

        writer.flush()?;

        // rename tmp to target filepath for fast atomic operation
        std::fs::rename(tmp_filepath, filepath)?;

        Ok(())
    }
}
