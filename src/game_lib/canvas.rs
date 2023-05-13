// this module contains all logic around the canvas and displaying playing field
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;



/// This function initializes the canvas
pub fn init(width: u32, height: u32) -> (Canvas<Window>, EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Canvas", width + 1, height + 1)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

    let event_pump = sdl_context.event_pump().unwrap();
    (canvas, event_pump)
}

/// Clears the grid
pub fn clear() {
    println!("clearing grid");

   
        
    
}
