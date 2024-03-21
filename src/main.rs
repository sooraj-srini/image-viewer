use image::{io::Reader as ImageReader, Rgb};
use minifb::{Key, Window, WindowOptions};

// const WIDTH: usize = 640;
// const HEIGHT: usize = 360;

fn load_image() -> (Vec<Rgb<u8>>,u32,u32) {
    let img = ImageReader::open("japan.png").unwrap().decode().unwrap();

    let buf = img.to_rgb8().pixels().cloned().collect();
    (buf, img.height(), img.width())
}


fn main() {

    let (buf, height, width) = load_image();

    let mut buffer: Vec<u32> = vec![0; (width * height) as usize];

    let mut window = Window::new(
        "Test - ESC to exit",
        width as usize,
        height as usize,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    let mut count = 0;
    
    //find time window is open 
    let start_time = std::time::Instant::now();
    
    buffer.iter_mut().enumerate().for_each(|(i, x)| { *x = ((buf[i][0] as u32) << 16 | (buf[i][1] as u32)<<8 | buf[i][2] as u32) as u32});
    
    while window.is_open() && !window.is_key_down(Key::Enter) {
        
        if window.is_key_pressed(Key::R, minifb::KeyRepeat::No){
            buffer.iter_mut().enumerate().for_each(|(i, x)| {*x = ((buf[i][0] as u32)<<16)^(*x)});
        }
        if window.is_key_pressed(Key::G, minifb::KeyRepeat::No){
            buffer.iter_mut().enumerate().for_each(|(i, x)| {*x = ((buf[i][1] as u32)<<8)^(*x)});
        }
        if window.is_key_pressed(Key::B, minifb::KeyRepeat::No){
            buffer.iter_mut().enumerate().for_each(|(i, x)| {*x = ((buf[i][2] as u32))^(*x)});
        }

        count = count + 1;
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, width as usize, height as usize)
            .unwrap();
    }
    let end_time = std::time::Instant::now();
    let diff = end_time - start_time;
    println!("{count}");
    println!("{:?}", diff);
    println!("Frame rate: {}", count as f64 / diff.as_secs_f64());
}