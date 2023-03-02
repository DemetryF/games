use std::io::stdin;

use celestial::{sun_system::sun_system, Celestial};
use snake::SnakeGame;

fn main() {
    println!(
        "List of games: \n\
         1. Celestial (sun system)\n\
         2. Snake. \n\
         Choose game: "
    );

    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("A String");
    let number = buf.trim().parse::<usize>().unwrap();

    match number {
        1 => Celestial::new(sun_system()).start(),
        2 => SnakeGame::new().start(),

        _ => {
            println!("invalid argument");
        }
    }
}
