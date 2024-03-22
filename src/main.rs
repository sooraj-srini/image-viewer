mod geometry;
mod lighting;
mod media;
mod object;
use lighting::*;
use minifb::{Key, Window, WindowOptions};
use object::*;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

fn main() {
    let sphere = Sphere::new(
        [500., 600., 60.],
        60.,
    );

    let buffer: Vec<u32> = vec![0; (WIDTH * HEIGHT) as usize];
    let mut screen = Screen {
        width: WIDTH,
        height: HEIGHT,
        buffer: buffer,
    };

    let camera = ParallelLight{angle: 0.}; 

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH as usize,
        HEIGHT as usize,
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

    while window.is_open() && !window.is_key_down(Key::Enter) {

        camera.render(&sphere, &mut screen);

        count = count + 1;
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&(screen.buffer), WIDTH as usize, HEIGHT as usize)
            .unwrap();
    }
    let end_time = std::time::Instant::now();
    let diff = end_time - start_time;
    println!("{count}");
    println!("{:?}", diff);
    println!("Frame rate: {}", count as f64 / diff.as_secs_f64());
}
