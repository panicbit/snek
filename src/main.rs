use anyhow::*;

mod game;
use game::Game;

mod snake;
use snake::Snake;
use std::collections::VecDeque;

const FIELD_TRAVERSAL_TIME_MILLIS: f32 = 5000.;
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
pub struct Point {
    x: isize,
    y: isize,
}

impl Point {
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

struct LossyBuffer<T> {
    elements: VecDeque<T>,
}

impl<T> LossyBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            elements: VecDeque::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, element: T) {
        if self.elements.len() < self.elements.capacity() {
            self.elements.push_back(element);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop_front()
    }
}
