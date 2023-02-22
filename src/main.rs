use std::f32::consts::PI;

use self::{cosmos_object::*, game::Game};

mod cosmos_object;
mod game;

const SUN_MASS: f32 = 6e6;

fn main() {
    let sun = CosmosObject::new(SUN_MASS, 8.0, Vec2::new(0.0, 0.0)).set_name("Sun".into());

    let planets: Vec<CosmosObject> = vec![
        CosmosObject::new(1.0, 1.0, Vec2::default())
            .set_name("Mercury".into())
            .send_into_orbit_to(&sun, 10.0, 1.0 * PI * 0.5),
        CosmosObject::new(14.0, 3.0, Vec2::default())
            .set_name("Venus".into())
            .send_into_orbit_to(&sun, 18.6, 2.0 * PI * 0.5),
        CosmosObject::new(18.0, 3.0, Vec2::default())
            .set_name("Earth".into())
            .send_into_orbit_to(&sun, 25.6, 3.0 * PI * 0.5),
        CosmosObject::new(2.0, 2.0, Vec2::default())
            .set_name("Mars".into())
            .send_into_orbit_to(&sun, 39.3, 4.0 * PI * 0.5),
        CosmosObject::new(6329.0, 5.0, Vec2::default())
            .set_name("Jupiter".into())
            .send_into_orbit_to(&sun, 134.1, 5.0 * PI * 0.5),
        CosmosObject::new(1894.0, 5.0, Vec2::default())
            .set_name("Saturn".into())
            .send_into_orbit_to(&sun, 246.5, 6.0 * PI * 0.5),
        CosmosObject::new(289.0, 4.0, Vec2::default())
            .set_name("Uranus".into())
            .send_into_orbit_to(&sun, 482.7, 7.0 * PI * 0.5),
        CosmosObject::new(341.0, 4.0, Vec2::default())
            .set_name("Neptune".into())
            .send_into_orbit_to(&sun, 770.0, 8.0 * PI * 0.5),
        sun,
    ];

    let mut game = Game::new(planets);

    game.start();
}
