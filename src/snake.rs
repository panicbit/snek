use crate::{Direction, Point};
use std::collections::VecDeque;
use rustbox::{RustBox, RB_BOLD, RB_NORMAL, Color};

pub struct Snake {
    position: Point,
    direction: Direction,
    old_direction: Direction,
    tail: VecDeque<Segment>,
    is_dead: bool,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            position: Point::new(10, 10),
            direction: Direction::Right,
            old_direction: Direction::Right,
            tail: VecDeque::from(vec![
                Segment::new(Point::new(9, 10), '─'),
                Segment::new(Point::new(8, 10), '─'),
                Segment::new(Point::new(7, 10), '─'),
                Segment::new(Point::new(6, 10), '─'),
                Segment::new(Point::new(5, 10), '─'),
                Segment::new(Point::new(4, 10), '─'),
            ]),
            is_dead: false,
        }
    }

    pub fn position(&self) -> &Point {
        &self.position
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }

    pub fn crawl(&mut self, grow: bool) {
        if !grow {
            self.remove_tail_segment();
        }

        self.grow_head_segment();

        match self.direction {
            Direction::Up => self.position.y -= 1,
            Direction::Down => self.position.y += 1,
            Direction::Left => self.position.x -= 1,
            Direction::Right => self.position.x += 1,
        }
    }

    fn remove_tail_segment(&mut self) {
        self.tail.pop_back();
    }

    fn grow_head_segment(&mut self) {
        use Direction::*;

        let symbol = match (self.old_direction, self.direction) {
            (Up, Up) | (Down, Down) => '│',
            (Left, Left) | (Right, Right) => '─',
            (Up, Right) | (Left, Down) => '╭',
            (Up, Left) | (Right, Down) => '╮',
            (Down, Left) | (Right, Up) => '╯',
            (Down, Right) | (Left, Up) => '╰',
            _ => '#',
        };

        let new_segment = Segment::new(self.position, symbol);
        self.tail.push_front(new_segment);
    }

    pub fn eating_itself(&self) -> bool {
        self.tail.iter().any(|segment| segment.position == self.position)
    }

    pub fn set_direction(&mut self, new_direction: Direction) {
        if self.is_dead || new_direction == self.direction.opposite(){
            return;
        }

        self.old_direction = self.direction;
        self.direction = new_direction;
    }

    pub fn kill(&mut self) {
        self.is_dead = true;
    }

    pub fn render(&self, rb: &RustBox) {
        // Tail
        for segment in &self.tail {
            if segment.position.x < 0 || segment.position.y < 0 {
                continue;
            }

            rb.print_char(
                segment.position.x as usize,
                segment.position.y as usize,
                RB_NORMAL,
                Color::Yellow,
                Color::Green,
                segment.symbol,
            );
        }

        // Head
        if self.position.x >= 0 && self.position.y >= 0 {
            let head_symbol = match self.direction {
                Direction::Up => '↑',
                Direction::Down => '↓',
                Direction::Left => '←',
                Direction::Right => '→',
            };
            // let head_symbol = 'Ö';

            let head_color = match self.is_dead {
                true => Color::Red,
                false => Color::Yellow,
            };

            rb.print_char(
                self.position.x as usize,
                self.position.y as usize,
                RB_BOLD,
                head_color,
                Color::Green,
                head_symbol,
            );
        }
    }
}

struct Segment {
    position: Point,
    symbol: char,
}

impl Segment {
    fn new(position: Point, symbol: char) -> Self {
        Self {
            position,
            symbol,
        }
    }
}
