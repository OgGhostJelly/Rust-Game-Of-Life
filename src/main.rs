mod game;
mod utils;

use game::event::{Action, Event};
use game::player::Player;
//use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::io;
use utils::tree::NodePath;

use crate::utils::tree::Node;

fn main() {
    //let mut rng = rand::thread_rng();

    let mut event = &Event::new(
        "Hello, welcome to my game!",
        vec![
            Action::new("", |_| const_nodepath!("")),
            /*Action {
                text: "",
                action: |_player| { const P: NodePath<'_> = NodePath::new(&[""]); &P }
            }*/
        ],
        |_player| true,
        HashMap::from([(
            "goblin",
            Event::new(
                "A goblin appears!",
                vec![],
                |_player| true,
                HashMap::from([]),
            ),
        )]),
    );

    let mut player = Player::new(50, 3, 10);

    loop {
        // If event is not ok, continue
        if !event.init(&mut player) {
            continue;
        }

        // Print text and player actions
        println!("{}", event.text());

        for (index, action) in event.actions().iter().enumerate() {
            println!("({0}){1}", index + 1, action.text())
        }

        // Get player input
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

        event = match event.get_node(match event.act(&mut player, action) {
            Some(v) => v,
            None => continue,
        }) {
            Some(v) => v,
            None => continue,
        }
    }
}
