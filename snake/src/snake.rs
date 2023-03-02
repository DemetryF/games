use sfml::{
    graphics::{Color, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable},
    system::Vector2i,
};

use crate::{direction::Direction, CELL_SIZE};

pub struct Snake {
    pub head: Vector2i,
    pub tail: Vec<Vector2i>,

    pub direction: Direction,
}

impl Snake {
    pub fn new(head: Vector2i, direction: Direction) -> Self {
        Self {
            head,
            direction,
            tail: vec![
                Vector2i::default(),
                Vector2i::default(),
                Vector2i::default(),
            ],
        }
    }

    pub fn update(&mut self) {
        self.tail = self
            .tail
            .clone()
            .iter_mut()
            .enumerate()
            .map(|(i, _)| {
                if i == 0 {
                    self.head
                } else {
                    self.tail.get(i - 1).unwrap().to_owned()
                }
            })
            .collect();

        self.head += self.direction.into();
    }

    pub fn render(&self, window: &mut RenderWindow) {
        let mut rect = RectangleShape::new();

        rect.set_position((self.head * CELL_SIZE).as_other());
        rect.set_size((Vector2i::new(1, 1) * CELL_SIZE).as_other());
        rect.set_fill_color(Color::YELLOW);

        window.draw(&rect);

        for point in self.tail.iter() {
            rect.set_position((point.to_owned() * CELL_SIZE).as_other());
            rect.set_size((Vector2i::new(1, 1) * CELL_SIZE).as_other());

            window.draw(&rect);
        }
    }

    pub fn push_node(&mut self) {
        let last = *self.tail.last().unwrap();
        self.tail.push(last);
    }
}
