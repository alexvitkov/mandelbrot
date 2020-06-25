// For reading and opening files
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

pub fn save(width: usize, height: usize, colordata: &[u8], path: &str) {
    let path = Path::new(path);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width as u32, height as u32); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(colordata).unwrap(); // Save
}
