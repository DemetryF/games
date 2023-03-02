use std::ops::Deref;

use rand::random;
use sfml::{
    graphics::{
        CircleShape, Color, RenderStates, RenderTarget, RenderWindow, Shape, Transformable,
    },
    system::{Vector2f, Vector2i},
};

use crate::{CELLS_COUNT, CELL_SIZE};

#[derive(Debug)]
pub struct Fruit {
    pub pos: Vector2i,
}

impl Fruit {
    pub fn random() -> Self {
        let pos = (Vector2f::new(rand::random(), random()) * CELLS_COUNT as f32).as_other();

        Self { pos }
    }

    pub fn render(&self, window: &mut RenderWindow) {
        let mut circle = CircleShape::new((CELL_SIZE / 2) as f32, 10);
        circle.set_position((self.pos * CELL_SIZE).as_other());
        circle.set_fill_color(Color::RED);
        window.draw_circle_shape(&circle, &RenderStates::default());
    }
}

impl Deref for Fruit {
    type Target = Vector2i;

    fn deref(&self) -> &Self::Target {
        &self.pos
    }
}
