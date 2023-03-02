use sfml::system::Vector2i;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl From<Direction> for Vector2i {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => Vector2i::new(0, -1),
            Direction::Down => Vector2i::new(0, 1),
            Direction::Left => Vector2i::new(-1, 0),
            Direction::Right => Vector2i::new(1, 0),
        }
    }
}
