// this module contains all logic around the canvas and displaying playing field
use sdl2::{pixels::Color, rect::Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

use crate::game_lib::types::Game;

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

/// Creates the game
pub fn game_init() -> Game {
    let game = Game::new();

    game

}

pub fn display_frame(renderer: &mut Canvas<Window>, game: &Game) {

    renderer.set_draw_color(Color::RGB(0, 0, 0));
    renderer.clear();

    for row in 0..5 {
        for i in 0..11 {
            let x = i as i32 *  game.invaders[row][i].hight + (10_i32 * i as i32);
            let y = row as i32 * game.invaders[row][i].width + (10_i32 * row as i32);
            let drawing_color = game.invaders[row][i].color;
            renderer.set_draw_color(drawing_color);
            renderer.fill_rect(
                Rect::new(x + 5, y + 5, 
                    game.invaders[row][i].width as u32, 
                    game.invaders[row][i].hight as u32
                )
            ).unwrap();

        }
    } 

    renderer.present();
}

/// Clears the grid
pub fn clear() {
    println!("clearing grid");     
    
}
