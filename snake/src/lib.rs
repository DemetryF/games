use sfml::{
    graphics::{Color, RenderTarget, RenderWindow},
    system::Vector2i,
    window::{ContextSettings, Event, Key, Style, VideoMode},
};

mod direction;
mod fruit;
mod snake;

use crate::{direction::Direction, fruit::Fruit, snake::Snake};

pub(crate) const CELL_SIZE: i32 = 16;
pub(crate) const FIELD_SIZE: i32 = 800;
pub(crate) const CELLS_COUNT: i32 = FIELD_SIZE / CELL_SIZE;

const SNAKE_START_POS: Vector2i = Vector2i { x: 20, y: 20 };
const TITLE: &str = "Snake Game";

pub struct SnakeGame {
    snake: Snake,
    fruit: Fruit,
    window: RenderWindow,

    paused: bool,

    score: usize,

    fail: bool,
}

impl SnakeGame {
    pub fn new() -> Self {
        let snake = Snake::new(SNAKE_START_POS, Direction::Left);
        let fruit = Fruit::random();
        let mut window;

        {
            let video_mode = VideoMode::new(FIELD_SIZE as u32, FIELD_SIZE as u32, 24);

            window = RenderWindow::new(
                video_mode,
                TITLE,
                Style::default(),
                &ContextSettings::default(),
            );

            window.set_framerate_limit(30)
        }

        Self {
            snake,
            fruit,
            window,

            score: 0,
            fail: false,
            paused: false,
        }
    }

    pub fn start(&mut self) {
        while self.window.is_open() {
            self.handle_events();
            self.update();
            self.render();
        }
    }

    fn update(&mut self) {
        if !self.paused && !self.fail {
            self.snake.update();
            self.fruit_prcessing();
            self.boundary_processing();
            self.snake_collision();
        }
    }

    pub fn render(&mut self) {
        self.window.clear(Color::rgb(0, 150, 0));

        self.snake.render(&mut self.window);
        self.fruit.render(&mut self.window);

        self.window.display();
    }

    fn boundary_processing(&mut self) {
        if self.snake.head.x < 0 {
            self.snake.head.x = CELLS_COUNT;
        }

        if self.snake.head.x > CELLS_COUNT {
            self.snake.head.x = 0;
        }

        if self.snake.head.y < 0 {
            self.snake.head.y = CELLS_COUNT;
        }

        if self.snake.head.y > CELLS_COUNT {
            self.snake.head.y = 0;
        }
    }

    fn fruit_prcessing(&mut self) {
        if self.snake.head == self.fruit.pos {
            self.regenerate_fruit();
            self.snake.push_node();
            self.score += 1;
        }
    }

    fn regenerate_fruit(&mut self) {
        loop {
            self.fruit = Fruit::random();

            if !self.is_fruit_on_snake() {
                break;
            }
        }
    }

    fn is_fruit_on_snake(&self) -> bool {
        self.snake.tail.iter().any(|node| *node == self.fruit.pos)
            && self.fruit.pos != self.snake.head
    }

    fn snake_collision(&mut self) {
        if self.snake.tail.iter().any(|node| *node == self.snake.head) {
            self.fail();
        }
    }

    fn handle_events(&mut self) {
        while let Some(event) = self.window.poll_event() {
            match event {
                Event::Closed => self.window.close(),

                Event::KeyPressed { code, .. } => match code {
                    Key::Escape => self.window.close(),

                    Key::Space => self.paused = !self.paused,

                    Key::Up | Key::W if self.snake.direction != Direction::Down => {
                        self.snake.direction = Direction::Up
                    }

                    Key::Left | Key::A if self.snake.direction != Direction::Right => {
                        self.snake.direction = Direction::Left
                    }

                    Key::Down | Key::S if self.snake.direction != Direction::Up => {
                        self.snake.direction = Direction::Down
                    }

                    Key::Right | Key::D if self.snake.direction != Direction::Left => {
                        self.snake.direction = Direction::Right
                    }

                    _ => {}
                },

                _ => {}
            }
        }
    }

    fn fail(&mut self) {
        self.fail = true;
    }
}
