use anyhow::*;

mod game;
use game::Game;

mod snake;
use snake::Snake;

const BASE_DELAY: f32 = 250.;
const ACCELERATION_BASE: f32 = 0.95;

fn main() -> Result<()> {
    let mut game = Game::new()?;

    game.run()?;

    Ok(())
}

#[derive(PartialEq)]
enum GameAction {
    Exit,
    KeepRunning,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn is_vertical(&self) -> bool {
        match self {
            Self::Up | Self::Down => true,
            _ => false,
        }
    }

    fn opposite(&self) -> Direction {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}
