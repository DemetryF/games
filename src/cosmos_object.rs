use sfml::graphics::{
    CircleShape, Font, RenderStates, RenderTarget, RenderWindow, Text, Transformable,
};

pub type Vec2 = sfml::system::Vector2f;

const POINT_COUNT: usize = 32;

pub struct CosmosObject {
    pub mass: f32,
    pub radius: f32,

    pub position: Vec2,
    pub velocity: Vec2,

    pub name: Option<String>,
}

impl CosmosObject {
    pub fn new(m: f32, r: f32, pos: Vec2) -> Self {
        Self {
            mass: m,
            radius: r,
            position: pos,
            velocity: Vec2::default(),
            name: None,
        }
    }

    pub fn interact(a: &mut CosmosObject, b: &mut CosmosObject, dt: f32) {
        let rect = a.position - b.position;
        let dist = rect.length_sq().sqrt();

        let direction = rect / dist;

        let force = a.mass * b.mass / dist.powi(2);

        let a_acceleration = force / a.mass;
        let b_acceleration = force / b.mass;

        a.velocity += direction * a_acceleration * dt;
        b.velocity += -direction * b_acceleration * dt;
    }

    pub fn render(&self, window: &RenderWindow) {
        let circle = &mut CircleShape::new(self.radius, POINT_COUNT);

        circle.set_position(self.position - Vec2::new(self.radius, self.radius));

        window.draw_circle_shape(circle, &RenderStates::default());

        if let Some(name) = self.name.as_ref() {
            let font = Font::from_file("./assets/Segoe UI.ttf").unwrap();

            let mut text = Text::new(name, &font, 10);

            text.set_position(self.position);

            window.draw_text(&text, &RenderStates::default());
        };
    }

    pub fn send_into_orbit_to(mut self, other: &Self, orbit_radius: f32, degree: f32) -> Self {
        let (sin, cos) = degree.sin_cos();

        let speed = (other.mass / orbit_radius).sqrt();

        let u = Vec2::new(cos, sin);

        self.position = other.position + u * orbit_radius;
        self.velocity = u.perpendicular() * speed;

        return self;
    }

    pub fn set_name(mut self, new_name: String) -> Self {
        self.name = Some(new_name);
        return self;
    }

    pub fn set_velocity(mut self, v: Vec2) -> Self {
        self.velocity = v;
        return self;
    }
}
