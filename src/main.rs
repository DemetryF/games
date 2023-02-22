use std::f32::consts::FRAC_PI_2;

use self::{celestial::Celestial, cosmos_object::*};

mod celestial;
mod cosmos_object;
mod interact;

fn main() {
    let sun = CosmosObject {
        mass: 6e6,
        radius: 8.0,
        name: Some("Sun".into()),
        ..CosmosObject::default()
    };

    let planets = vec![
        CosmosObject {
            mass: 1.0,
            radius: 1.0,
            name: Some("Mercury".into()),
            ..CosmosObject::default()
        }
        .orbit(&sun, 10.0, 1. * FRAC_PI_2),
        CosmosObject {
            mass: 14.0,
            radius: 3.0,
            name: Some("Venus".into()),
            ..CosmosObject::default()
        }
        .orbit(&sun, 18.6, 2. * FRAC_PI_2),
        CosmosObject {
            mass: 18.0,
            radius: 3.0,
            name: Some("Earth".into()),
            ..CosmosObject::default()
        }
        .orbit(&sun, 25.6, 3. * FRAC_PI_2),
        CosmosObject {
            mass: 2.0,
            radius: 2.0,
            name: Some("Mars".into()),
            ..CosmosObject::default()
        }
        .orbit(&sun, 39.3, 4. * FRAC_PI_2),
        CosmosObject {
            mass: 6329.0,
            radius: 5.0,
            name: Some("Jupiter".into()),
            ..CosmosObject::default()
        }
        .orbit(&sun, 134.1, 5. * FRAC_PI_2),
        CosmosObject {
            mass: 1894.0,
            radius: 5.0,
            name: Some("Saturn".into()),
            ..CosmosObject::default()
        }
        .orbit(&sun, 246.5, 6. * FRAC_PI_2),
        CosmosObject {
            mass: 289.0,
            radius: 4.0,
            name: Some("Uranus".into()),
            ..CosmosObject::default()
        }
        .orbit(&sun, 482.7, 7. * FRAC_PI_2),
        CosmosObject {
            mass: 341.0,
            radius: 4.0,
            name: Some("Neptune".into()),
            ..CosmosObject::default()
        }
        .orbit(&sun, 482.7, 8. * FRAC_PI_2),
        sun,
    ];

    let mut game = Celestial::new(planets);

    game.start();
}
