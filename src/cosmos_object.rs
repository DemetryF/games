use sfml::graphics::{
    CircleShape, Font, RenderStates, RenderTarget, RenderWindow, Text, Transformable,
};

pub type Vec2 = sfml::system::Vector2f;

const POINT_COUNT: usize = 32;

pub struct CosmosObject {
    pub m: f32,
    pub r: f32,

    pub pos: Vec2,
    pub v: Vec2,

    pub name: Option<String>,
}

impl CosmosObject {
    pub fn new(m: f32, r: f32, pos: Vec2) -> Self {
        Self {
            m,
            r,
            pos,
            v: Vec2::default(),
            name: None,
        }
    }

    pub fn influence(&mut self, other: &Self, dt: f32) {
        let rect = other.pos - self.pos;
        let dist = rect.length_sq().sqrt();

        let u = rect / dist;
        let acc = other.m / dist.powi(2);

        self.v += u * acc * dt;
    }

    pub fn render(&self, window: &RenderWindow) {
        let circle = &mut CircleShape::new(self.r, POINT_COUNT);

        circle.set_position(self.pos - Vec2::new(self.r, self.r));

        window.draw_circle_shape(circle, &RenderStates::default());

        if let Some(name) = self.name.as_ref() {
            let font = Font::from_file("./assets/Segoe UI.ttf").unwrap();

            let mut text = Text::new(name, &font, 10);

            text.set_position(self.pos);

            window.draw_text(&text, &RenderStates::default());
        };
    }

    pub fn send_into_orbit_to(mut self, other: &Self, orbit_radius: f32, degree: f32) -> Self {
        let (sin, cos) = degree.sin_cos();

        let speed = (other.m / orbit_radius).sqrt();

        let u = Vec2::new(cos, sin);

        self.pos = other.pos + u * orbit_radius;
        self.v = u.perpendicular() * speed;

        return self;
    }

    pub fn set_name(mut self, new_name: String) -> Self {
        self.name = Some(new_name);
        return self;
    }

    pub fn set_velocity(mut self, v: Vec2) -> Self {
        self.v = v;
        return self;
    }
}
