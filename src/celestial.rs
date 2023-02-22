use std::time::Instant;

use sfml::{
    graphics::{Color, RenderTarget, RenderWindow, View},
    window::{ContextSettings, Event, Key, Style, VideoMode},
};

use crate::{
    cosmos_object::Vec2,
    interact::{Collision, Gravity, Interact},
    CosmosObject,
};

const FIELD_SIZE: f32 = 1000.0;
const BITS_PER_PIXER: u32 = 24;
const TITLE: &str = "demetry's celestial";
const SCROLL_COEF: f32 = 1.7;

pub struct Celestial {
    objects: Vec<CosmosObject>,
    window: RenderWindow,

    dt: f32,
    time_coef: f32,
}

impl Celestial {
    pub fn new(objects: Vec<CosmosObject>) -> Self {
        let video_mode = VideoMode::new(FIELD_SIZE as u32, FIELD_SIZE as u32, BITS_PER_PIXER);

        let mut window = RenderWindow::new(
            video_mode,
            TITLE,
            Style::default(),
            &ContextSettings::default(),
        );

        let view = View::new(Vec2::default(), Vec2::new(FIELD_SIZE, FIELD_SIZE));
        window.set_view(&view);

        Self {
            objects,
            window,
            dt: 0.0,
            time_coef: 0.01,
        }
    }

    pub fn start(&mut self) {
        while self.window.is_open() {
            self.update();
        }
    }

    fn update(&mut self) {
        let time = Instant::now();
        self.window.clear(Color::BLACK);

        self.handle_events();
        self.process_planets();

        self.window.display();
        self.dt = time.elapsed().as_secs_f32() * self.time_coef;
    }

    fn handle_events(&mut self) {
        if let Some(event) = self.window.poll_event() {
            match event {
                Event::Closed => self.window.close(),
                Event::KeyPressed { code, .. } => {
                    if code == Key::Escape {
                        self.window.close();
                    }
                }
                Event::MouseWheelScrolled { delta, .. } => {
                    let mut view = self.window.view().to_owned();
                    let size = view.size();
                    view.set_size(size * SCROLL_COEF.powf(-delta));
                    self.window.set_view(&view);
                }
                _ => {}
            }
        }
    }

    fn process_planets(&mut self) {
        for i in 0..self.objects.len() {
            let (_, [current, right @ ..]) = self.objects.split_at_mut(i) else {
                continue;
            };

            current.render(&self.window);

            for other in right {
                Collision::interact(current, other, self.dt);
                Gravity::interact(current, other, self.dt)
            }

            current.position += current.velocity * self.dt;
        }
    }
}
