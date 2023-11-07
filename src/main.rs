mod game;
mod utils;

use game::event::{Event, Action};
use game::player::Player;
use rand::seq::SliceRandom;
use utils::tree::NodePath;
use std::collections::HashMap;

fn main() {
    let mut rng = rand::thread_rng();

    let events: HashMap<&str, Event<'_>>;

    let event = Event::new(
        "Hello, welcome to my game!",
        vec![
            Action {
                text: "",
                event: &NodePath::new([""]),
            }
        ],
        |_player| { true },
        HashMap::from([
            ("goblin", Event::new(
                "A goblin appears!", 
                vec![

                ],
                |_player| { true }, 
                HashMap::from([])
            ))
        ]),
    );

    /*events = HashMap::from([
        (
            "",
            Event::new(
                "A goblin appears and does a silly dance",
                |_player| {},
                Vec::from([Action::new("Dance with them", |_player| -> &str {
                    println!("DANCE PARTY");
                    ""
                })]),
            ),
        ),
        (
            "",
            Event::new(
                "An evil wizard appears",
                |_player| {},
                Vec::from([
                    Action::new("Give him a warm and welcoming hug", |_player| -> &str {
                        println!("he just needed affection");
                        ""
                    }),
                    Action::new("MURFDER MRUEDER MUDEURFDER (drones)", |_player| -> &str {
                        println!("thats not very pg-13");
                        ""
                    }),
                ]),
            ),
        ),
    ]);*/

    
    let mut player = Player::new(50, 3, 10);
    let mut current_event: &str = "";

    loop {
        println!("{}", events[current_event].text());

        for (index, action) in events[current_event].actions().iter().enumerate() {
            println!("({0}){1}", index + 1, action.text())
        }

        events[current_event].init(&player);

        let mut action: String = String::new();

        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line.");

        let action: usize = match action.trim().parse::<usize>() {
            Ok(v) => match v.checked_sub(1) {
                Some(v) => v,
                None => continue,
            },
            Err(_) => continue,
        };

        current_event = events[current_event].act(&mut player, action);
    }
}
