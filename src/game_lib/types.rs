use rand::{thread_rng, Rng};
use sdl2::pixels::Color;

// Types

pub struct Game<'a> {
    pub invaders: Vec<Vec<Invader<'a>>>,
    pub state: GameState,
    // ship: Ship,
}

impl<'a> Game<'a> {
    pub fn new() -> Self {
        let mut invaders = Vec::new();

        for _i in 0..5 {
            let mut inner = Vec::new();
            for _j in 0..11 {
                inner.push(Invader::new())
            }
            invaders.push(inner)
        }

        let game_state = GameState {
            counter: 0,
            horizontal_drift: 0,
            drift_direction: DriftDirection::Right,
        };

        let game = Game {
            invaders: invaders,
            state: game_state,
        };
        game
    }
}

#[derive(Debug, Clone)]
pub struct Invader<'a> {
    pub width: i32,
    pub height: i32,
    pub color: Color,
    pub kind: InvaderType<'a>,
}

impl<'a> Invader<'a> {
    fn new() -> Self {
        let mut rng = thread_rng();

        let width = 33_i32;
        let height = 24_i32;

        let r: u8 = rng.gen();
        let g: u8 = rng.gen();
        let b: u8 = rng.gen();

        let kind = InvaderType {
            fg_color: &[
                (5 + 6, 5 + 6, 21_u32, 15_u32),
                // antennae
                (5 + 6, 5 + 0, 3_u32, 3_u32),
                (5 + 24, 5 + 0, 3_u32, 3_u32),
                // antennae left
                (5 + 9, 5 + 3, 3_u32, 3_u32),
                (5 + 21, 5 + 3, 3_u32, 3_u32),
                // upper arm left
                (5 + 3, 5 + 9, 3_u32, 6_u32),
                // upper arm right
                (5 + 27, 5 + 9, 3_u32, 6_u32),
                // chaning parts
                // lower arm left
                (5 + 0, 5 + 12, 3_u32, 9_u32),
                // lower arm right
                (5 + 30, 5 + 12, 3_u32, 9_u32),
                // lower leg right
                (5 + 9, 5 + 21, 6_u32, 3_u32),
                // lower leg left
                (5 + 18, 5 + 21, 6_u32, 3_u32),
            ],
            bg_color: &[
                // eyes
                (5 + 9, 5 + 9, 3_u32, 3_u32),
                (5 + 21, 5 + 9, 3_u32, 3_u32),
                // between legs
                (5 + 9, 5 + 18, 15_u32, 3_u32),
            ],
        };

        let invader = Invader {
            width: width,
            height: height,
            color: Color::RGB(r, g, b),
            kind: kind,
        };

        invader
    }
}

#[derive(Debug, Clone)]
pub struct InvaderType<'a> {
    pub fg_color: &'a [(i32, i32, u32, u32)],
    pub bg_color: &'a [(i32, i32, u32, u32)],
}

pub struct Ship {}

pub struct GameState {
    pub counter: i32,
    pub horizontal_drift: i32,
    pub drift_direction: DriftDirection,
}

#[derive(Debug)]
pub enum GameEvent {
    FieldOccupied,
    GameTied,
    GameWon,
}
#[derive(PartialEq)]
pub enum DriftDirection {
    Left,
    Right,
}
