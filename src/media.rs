use image::{io::Reader as ImageReader, Rgb};

fn _load_image() -> (Vec<Rgb<u8>>,u32,u32) {
    let img = ImageReader::open("japan.png").unwrap().decode().unwrap();

    let buf = img.to_rgb8().pixels().cloned().collect();
    (buf, img.height(), img.width())
}