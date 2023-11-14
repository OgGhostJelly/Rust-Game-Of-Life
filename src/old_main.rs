use rand::Rng;
use std::{io, str::FromStr};
mod tree;

enum Class {
    Knight, // Strong
    Rogue,  // Fast
    Mage,   // Smart
}

struct GameState {
    class: Class,
}

impl GameState {
    fn new(class: Class) -> Self {
        Self { class: class }
    }
}

struct Action<'a> {
    text: &'a str,
    event: fn(&mut GameState) -> Event<'a>,
    condition: fn(&GameState) -> bool,
}

struct Event<'a> {
    text: &'a str,
    actions: &'a [Action<'a>],
}

impl<'a> Event<'a> {
    fn act(self, mut state: GameState) -> (Event<'a>, GameState) {
        loop {
            println!("{}", self.text);

            for (index, action) in self
                .actions
                .iter()
                .filter(|x| (x.condition)(&state))
                .enumerate()
            {
                println!("({0}) {1}", index + 1, action.text)
            }

            let choice: usize = match input_parse::<usize>() {
                Ok(v) => match v.checked_sub(1) {
                    Some(v) => v,
                    None => continue,
                },
                Err(_) => continue,
            };

            let action = match self.actions.get(choice) {
                Some(action) => action,
                None => continue,
            };

            break ((action.event)(&mut state), state);
        }
    }
}

fn main() {
    const ROOT: Event = Event {
        text: "A magical wizard appears!",
        actions: &[
            Action {
                text: "Attack",
                event: |state| {
                    match state.class {
                    Class::Rogue => if rand::thread_rng().gen_bool(0.9) {
                        Event {
                            text: "You sneak up behind the magical wizard and perform your special maneuver. The wizard goes down without a sound.",
                            actions: &[],
                        }
                    } else {
                        Event {
                            text: "The wizard casts foresight and predicts your attack. Narrowly dodging your poison dagger.",
                            actions: &[
                                Action {
                                    text: "Try again",
                                    event: |_| if rand::thread_rng().gen_bool(0.9) {
                                        Event {
                                            text: "You swing your dagger once more, and you land a critical hit. The wizard coughs up blood and falls to the ground, motionless.",
                                            actions: &[],
                                        }
                                    } else {
                                        Event {
                                            text: "You swing your dagger once more... and fail spectacularly. Just so you know this ending has a 1% chance of happening.",
                                            actions: &[],
                                        }
                                    },
                                    condition: |_| true,
                                },
                                Action {
                                    text: "Escape",
                                    event: |_| if rand::thread_rng().gen_bool(0.5) {
                                        Event {
                                            text: "You successfully fled the battle.",
                                            actions: &[],
                                        }
                                    } else {
                                        Event {
                                            text: "You try to run but the magical wizard casts a fireball spell before you can escape.",
                                            actions: &[],
                                        }
                                    },
                                    condition: |_| true,
                                }
                            ],
                        }
                    },
                    Class::Mage => Event {
                        text: "You cast a spell on the wizard... but he's magic proof. Guess you should've expected that.",
                        actions: &[

                        ]
                    },
                    Class::Knight => if rand::thread_rng().gen_bool(0.1) {
                        Event {
                            text: "You pounce on the magical wizard before they can realize what's happening. The magical wizard crumbles to dust.",
                            actions: &[],
                        }
                    } else {
                        Event {
                            text: "You try to attack the magical wizard but they're too fast and all your attacks miss!",
                            actions: &[
                                
                            ],
                        }
                    }
                }
                },
                condition: |_| true,
            },
            Action {
                text: "Befriend",
                event: |_| {
                    Event {
                    text: "The magical wizard says he shall only join you if you prove strong enough. How will you convince the magical wizard to join your party?",
                    actions: &[
                        Action {
                            text: "Duel",
                            event: |_| Event {
                                text: "Nothing here...",
                                actions: &[],
                            },
                            condition: |_| true,
                        },
                        Action {
                            text: "Outsmart",
                            event: |_| Event {
                                text: "Nothing here...",
                                actions: &[],
                            },
                            condition: |_| true,
                        },
                        Action {
                            text: "Charm",
                            event: |_| Event {
                                text: "Nothing here...",
                                actions: &[],
                            },
                            condition: |_| true,
                        },
                    ],
                }
                },
                condition: |_| true,
            },
            Action {
                text: "Escape",
                event: |_| Event {
                    text: "you disgust me",
                    actions: &[
                        Action {
                            text: "SERIOUSLY!?!?!?",
                            event: |_| ROOT,
                            condition: |_| true,
                        },
                        Action {
                            text: "I SPENT SO LONG TIRING AWAY TO GIVE YOU THIS AMAZING BATTLE!!!",
                            event: |_| ROOT,
                            condition: |_| true,
                        },
                        Action {
                            text: "AND YOU DECIDE!!!",
                            event: |_| ROOT,
                            condition: |_| true,
                        },
                        Action {
                            text: "TO SKIP IT?!?!?!?!?",
                            event: |_| ROOT,
                            condition: |_| true,
                        },
                        Action {
                            text: "NO. GO BACK. TRY AGAIN!",
                            event: |_| ROOT,
                            condition: |_| true,
                        },
                    ],
                },
                condition: |_| true,
            },
        ],
    };

    let mut event = ROOT;

    // This code is appalling. I am so sorry mr torvald
    let class = loop {
        println!("What is your class...");
        println!("(1) Knight");
        println!("(2) Rogue");
        println!("(3) Mage");

        match input_parse::<String>() {
            Ok(v) => {
                break match &*v {
                    "1" => Class::Knight,
                    "2" => Class::Rogue,
                    "3" => Class::Mage,
                    _ => continue,
                }
            }
            Err(_) => continue,
        }
    };

    let mut state: GameState = GameState::new(class);

    loop {
        (event, state) = event.act(state);
    }
}

fn input_parse<T: FromStr>() -> Result<T, T::Err> {
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    input.trim().parse::<T>()
}
