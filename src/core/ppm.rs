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
    pub fn save(&self, filepath: &str) {
        dbg!(filepath);

        println!("P3");
        println!("{} {}", self.width, self.height);
        println!("{}", Color::MAX_VALUE);
        for pixel in &self.pixels {
            println!("{pixel}");
        }
    }
}
