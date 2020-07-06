use anyhow::*;
use rustbox::{InitOptions, RustBox, Event, Key, RB_BOLD, RB_NORMAL, Color};
use std::{thread, time::Duration, collections::BTreeSet};
use rand::Rng;
use crate::{Point, ACCELERATION_BASE, GameAction, Direction, Snake, FIELD_TRAVERSAL_TIME_MILLIS, LossyBuffer, Rect, PERCENT_WALLS, MAX_SPEED_LEVEL};

pub struct Game {
    rb: RustBox,
    snake: Snake,
    pellets: BTreeSet<Point>,
    score: usize,
    lost: bool,
    paused: bool,
    input_buffer: LossyBuffer<Key>,
    field: Rect,
    walls: BTreeSet<Point>,
}

impl Game {
    pub fn new() -> Result<Self> {
        let rb = RustBox::init(InitOptions::default())
            .context("Failed to initialize terminal")?;

        let field = Rect {
            x: 1,
            y: 1,
            width: rb.width() - 2,
            height: rb.height() - 2,
        };

        let mut game = Self {
            rb,
            snake: Snake::new(),
            pellets: BTreeSet::new(),
            score: 0,
            lost: false,
            paused: false,
            input_buffer: LossyBuffer::new(2),
            field,
            walls: BTreeSet::new(),
        };

        game.spawn_walls();
        game.spawn_pellet();

        Ok(game)
    }

    fn spawn_walls(&mut self) {
        let num_walls = self.field.area() as f32 * PERCENT_WALLS / 100.;
        let num_walls = num_walls as usize;
        let mut rng = rand::thread_rng();

        for _ in 0..num_walls {
            loop {
                let x = rng.gen_range(self.field.left(), self.field.right());
                let y = rng.gen_range(self.field.top(), self.field.bottom());
                let wall = Point::new(x, y);

                if self.walls.contains(&wall) || self.pellets.contains(&wall) {
                    continue
                }

                self.walls.insert(wall);

                break
            }
        }
    }

    fn spawn_pellet(&mut self) {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(self.field.left(), self.field.right());
        let y = rng.gen_range(self.field.top(), self.field.bottom());

        self.pellets.insert(Point::new(x, y));
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            self.render();

            let field_size = match self.snake.direction().is_vertical() {
                false => self.rb.width() as f32,
                true => self.rb.height() as f32 * 1.5,
            };
            let speed_level = self.score.min(MAX_SPEED_LEVEL) as i32;
            let delay = FIELD_TRAVERSAL_TIME_MILLIS / field_size * ACCELERATION_BASE.powi(speed_level);
            let delay = Duration::from_millis(delay as u64);
            thread::sleep(delay);

            if self.run_logic_step()? == GameAction::Exit {
                return Ok(());
            }
        }
    }

    fn render(&self) {
        self.rb.clear();

        // Border
        self.render_border();

        // Score
        self.rb.print(
            1,
            0,
            RB_BOLD,
            Color::Yellow,
            Color::Default,
            &format!("╣Score: {}╠", self.score),
        );

        // Game Over
        if self.lost {
            self.rb.print(
                2,
                1,
                RB_BOLD,
                Color::Red,
                Color::Default,
                "GAME OVER",
            );
        }

        // Pellets
        for pellet in &self.pellets {
            self.rb.print_char(
                pellet.x,
                pellet.y,
                RB_BOLD,
                Color::Default,
                Color::Red,
                ' ',
            );
        }

        // Walls
        for wall in &self.walls {
            self.rb.print_char(
                wall.x,
                wall.y,
                RB_NORMAL,
                Color::Default,
                Color::Blue,
                '#',
            );
        }

        // Snake
        self.snake.render(&self.rb);

        self.rb.present();
    }

    fn render_border(&self) {
        let draw = |x, y, symbol| self.rb.print_char(
            x,
            y,
            RB_NORMAL,
            Color::Default,
            Color::Blue,
            symbol,
        );
        let field = &self.field;

        // top and bottom
        for x in (field.left() - 1)..(field.right() + 1) {
            for &y in &[field.top() - 1, field.bottom()] {
                draw(x, y, '═');
            }
        }

        // left and right
        for y in (field.top() - 1)..(field.bottom() + 1) {
            for &x in &[field.left() - 1, field.right()] {
                draw(x, y, '║');
            }
        }

        // corners
        draw(field.left() - 1, field.top() - 1, '╔');
        draw(field.right(), field.top() - 1, '╗');
        draw(field.left() - 1, field.bottom(), '╚');
        draw(field.right(), field.bottom(), '╝');
    }

    fn run_logic_step(&mut self) -> Result<GameAction> {
        let mut next_direction = *self.snake.direction();

        self.buffer_inputs()?;

        if let Some(key) = self.input_buffer.pop() {
            match key {
                Key::Up => next_direction = Direction::Up,
                Key::Down => next_direction = Direction::Down,
                Key::Left => next_direction = Direction::Left,
                Key::Right => next_direction = Direction::Right,
                Key::Esc | Key::Char('q') => return Ok(GameAction::Exit),
                Key::Char(' ') => self.paused = !self.paused,
                _ => {}
            }
        }

        if self.paused {
            return Ok(GameAction::KeepRunning);
        }

        self.snake.set_direction(next_direction);

        if self.lost {
            return Ok(GameAction::KeepRunning);
        }

        let grow = self.pellets.remove(&self.snake.position());

        if grow {
            self.score += 1;
            self.spawn_pellet();
        }

        self.snake.crawl(grow);

        // Game Over condition
        if self.snake.eating_itself() || self.snake_outside_bounds() {
            self.snake.kill();
            self.lost = true;
        }

        Ok(GameAction::KeepRunning)
    }

    fn buffer_inputs(&mut self) -> Result<()> {
        loop {
            let event = self.rb.peek_event(Duration::from_millis(1), false)?;

            match event {
                Event::KeyEvent(key) => self.input_buffer.push(key),
                Event::NoEvent => return Ok(()),
                _ => continue,
            }
        }
    }

    fn snake_outside_bounds(&self) -> bool {
        !self.field.contains(self.snake.position()) ||
        self.walls.contains(self.snake.position())
    }
}
