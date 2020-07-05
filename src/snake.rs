use crate::{Direction, Pos};
use std::collections::VecDeque;
use rustbox::{RustBox, RB_BOLD, RB_NORMAL, Color};

pub struct Snake {
    position: Pos,
    direction: Direction,
    tail: VecDeque<Pos>,
    is_dead: bool,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            position: Pos::new(10, 10),
            direction: Direction::Right,
            tail: VecDeque::from(vec![
                Pos::new(9, 10),
                Pos::new(8, 10),
                Pos::new(7, 10),
                Pos::new(6, 10),
                Pos::new(5, 10),
                Pos::new(4, 10),
            ]),
            is_dead: false,
        }
    }

    pub fn position(&self) -> &Pos {
        &self.position
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }

    pub fn crawl(&mut self, grow: bool) {
        if !grow {
            self.tail.pop_back();
        }

        self.tail.push_front(self.position);

        match self.direction {
            Direction::Up => self.position.y -= 1,
            Direction::Down => self.position.y += 1,
            Direction::Left => self.position.x -= 1,
            Direction::Right => self.position.x += 1,
        }
    }

    pub fn eating_itself(&self) -> bool {
        self.tail.contains(&self.position)
    }

    pub fn set_direction(&mut self, direction: Direction) {
        if self.is_dead || direction == self.direction.opposite(){
            return;
        }

        self.direction = direction;
    }

    pub fn kill(&mut self) {
        self.is_dead = true;
    }

    pub fn render(&self, rb: &RustBox) {
        // Tail
        for segment in &self.tail {
            if segment.x < 0 || segment.y < 0 {
                continue;
            }

            rb.print_char(
                segment.x as usize,
                segment.y as usize,
                RB_NORMAL,
                Color::Green,
                Color::Default,
                'o'
            );
        }

        // Head
        if self.position.x >= 0 && self.position.y >= 0 {
            let head_symbol = match self.direction {
                Direction::Up => '⮉',
                Direction::Down => '⮋',
                Direction::Left => '⮈',
                Direction::Right => '⮊',
            };

            let head_color = match self.is_dead {
                true => Color::Red,
                false => Color::Green,
            };

            rb.print_char(
                self.position.x as usize,
                self.position.y as usize,
                RB_BOLD,
                head_color,
                Color::Default,
                head_symbol,
            );
        }
    }
}
