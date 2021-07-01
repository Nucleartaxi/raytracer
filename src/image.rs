pub fn create_test_image(v: &mut Vec<u8>, width: u32, height: u32) {
    let mut i = 0;
    while i < height {
        let mut j = 0;
        while j < width {
            let r = i as f32 / (width - 1) as f32;
            let g = j as f32 / (height - 1) as f32;
            let b = 0.25;
            
            v.push((r * 255.999) as u8);
            v.push((g * 255.999) as u8);
            v.push((b * 255.999) as u8);
            j = j + 1;
        }
        i = i + 1;
    }
}

pub fn output_image(v: &mut Vec<u8>, width: u32, height: u32) {
    //for writing and opening files
    use std::path::Path;
    use std::fs::File;
    use std::io::BufWriter;

	let path = Path::new("target/image/image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    // writer.write_image_data(v).unwrap();
}