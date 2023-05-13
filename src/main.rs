use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use std::{thread, time};

mod game_lib;
use game_lib::canvas;


// Canvas and Playing Field Size

/// Canvas width in pixels
const CANVAS_WIDTH: u32 = 500;
/// Canvas height in pixels
const CANVAS_HEIGHT: u32 = 400;

///# Main 
/// 
/// The main function contains the game loop. Most the of game's logic
/// can be found in the [game_lib::game] sub module. 
fn main() {
    let (mut canvas, mut pump_events) = canvas::init(CANVAS_WIDTH, CANVAS_HEIGHT);
    let game = canvas::game_init();

    thread::spawn(move || {});
    'game: loop {
        for event in pump_events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'game,

                _ => continue 'game,
            }
        }
        canvas::display_frame(&mut canvas, &game);
        thread::sleep(time::Duration::from_millis(500));
    }
}
