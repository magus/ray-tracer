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
pub fn v3() {}

pub fn demo() {
    let aspect_ratio = 16 / 9;
    let height = 256;
    let width = height * aspect_ratio;

    let max_value = 255;

    println!("P3");
    println!("{width} {height}");
    println!("{max_value}");

    for y in 0..height {
        eprint!("saving {}/{height}\r", y + 1);

        for x in 0..width {
            let r = (x as f64 / (width - 1) as f64) * max_value as f64;
            let g = 0;
            let b = (y as f64 / (height - 1) as f64) * 255.0 as f64;
            println!("{r} {g} {b}");
        }
    }

    eprintln!();
    eprintln!("saved");
}
