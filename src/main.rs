#![warn(
    array_into_iter,
    bare_trait_objects,
    ellipsis_inclusive_range_patterns,
    non_fmt_panics,
    rust_2021_incompatible_closure_captures,
    rust_2021_incompatible_or_patterns,
    rust_2021_prefixes_incompatible_syntax,
    rust_2021_prelude_collisions
)]

use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::{event::Event, mouse::MouseState};
use std::{thread, time};

mod game_lib;
use game_lib::{
    canvas, game,
    types::{GameEvent},
};

// Canvas and Playing Field Size

/// Canvas width in pixels
const CANVAS_WIDTH: u32 = 300;
/// Canvas height in pixels
const CANVAS_HEIGHT: u32 = 300;

/// Number of colums

///# Main 
/// 
/// The main function contains the game loop. Most the of game's logic
/// can be found in the [game_lib::game] sub module. 
fn main() {
    let (mut canvas, mut pump_events) = canvas::init(CANVAS_WIDTH, CANVAS_HEIGHT);

    thread::spawn(move || {});
    'game: loop {
        let mouse_status = MouseState::new(&pump_events);
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

        thread::sleep(time::Duration::from_millis(500));
    }
}
