use std::f32::consts::PI;

use self::{cosmos_object::*, game::Game};

mod cosmos_object;
mod game;

const SUN_MASS: f32 = 6e6;

fn main() {
    let mut counter = 0;

    let mut planet = |mass: f32, mut dist: f32, name: &str| -> CosmosObject {
        let degree = 0.5 * PI * counter as f32;
        counter += 1;

        dist /= 2.0;

        let sin = degree.sin();
        let cos = degree.cos();

        let v = (SUN_MASS / dist).sqrt();

        let pos = Vec2::new(dist * cos, dist * sin);
        let speed = Vec2::new(v * -sin, v * cos);

        CosmosObject::new(mass, 1.0, pos)
            .set_name(name.to_string())
            .set_velocity(speed)
    };

    let sun = CosmosObject::new(SUN_MASS, 8.0, Vec2::new(0.0, 0.0)).set_name("Sun".into());

    let planets: Vec<CosmosObject> = vec![
        sun,
        planet(1.0, 10.0, "Mercury"),
        planet(14.0, 18.6, "Venus"),
        planet(18.0, 25.6, "Earth"),
        planet(2.0, 39.3, "Mars"),
        planet(6329.0, 134.1, "Jupiter"),
        planet(1894.0, 246.5, "Saturn"),
        planet(289.0, 482.7, "Uranus"),
        planet(341.0, 770.0, "Neptune"),
    ];

    let mut game = Game::new(planets);

    game.start();
}
