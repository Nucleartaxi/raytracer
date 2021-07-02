use super::color;

pub fn create_test_image(v: &mut Vec<u8>, width: u32, height: u32) {
    for i in (0..(height)).rev() {
        // println!("Lines remaining: {}", i + 1); //progress indicator in case of long renders
        for j in 0..width {
            let c = color::Color::new(i as f64 / (width - 1) as f64, j as f64 / (height - 1) as f64, 0.25);
            c.write_color(v);
        }
    }
}

pub fn output_image(image: &[u8], width: u32, height: u32) {
    //for writing and opening files
    use std::path::Path;
    use std::fs::File;
    use std::io::BufWriter;

	let path = Path::new("image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(image).unwrap();
}