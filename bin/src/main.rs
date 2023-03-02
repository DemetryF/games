use std::io::stdin;

use celestial::{sun_system::sun_system, Celestial};

fn main() {
    println!(
        "List of games: \n\
         1. Celestial (sun system)"
    );

    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("A String");
    let number = buf.trim().parse::<usize>().unwrap();

    match number {
        1 => Celestial::new(sun_system()).start(),

        _ => {
            println!("invalid argument");
        }
    }
}
