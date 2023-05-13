use sdl2::pixels::Color;
use rand::{thread_rng, Rng};
// Types

pub struct Game {
    pub invaders: Vec<Vec<Invader>>,
    // ship: Ship,
}

impl Game {
    pub fn new() -> Self {

        let mut invaders = Vec::new();

        for _i in 0..5 {
            let mut inner = Vec::new();
            for _j in 0..11 {
                inner.push(Invader::new())
            }
            invaders.push(inner)
        };

        let game = Game {
           invaders: invaders,

        }; 
        game
    }
}


#[derive(Debug, Copy, Clone)]
pub struct Invader {
    pub width: i32,
    pub hight: i32,
    pub color: Color,
}

impl Invader {
    fn new() -> Self {
        let mut rng = thread_rng();

        let r: u8 = rng.gen();
        let g: u8 = rng.gen();
        let b: u8 = rng.gen();

        let invader = Invader {
            width: 25_i32,
            hight: 25_i32,
            color: Color::RGB(r, g, b),
        };

        invader
    }

}
pub struct Ship {

}

#[derive(Debug)]
pub enum GameEvent {
    FieldOccupied,
    GameTied,
    GameWon,
}


