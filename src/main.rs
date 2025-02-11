use minifb::WindowOptions;
use minifb::Window;

fn main() {
    let mut window = match Window::new("Nest", 256, 240, WindowOptions::default()) {
        Ok(win) => win,
        Err(err) => {
            println!("MiniFB Err: {}", err);
            return;
        }
    };
    
    window.set_target_fps(60); // Should be 40?

    loop {
        window.update();
    };
}
