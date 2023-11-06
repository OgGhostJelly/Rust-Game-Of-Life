use std::collections::HashMap;

pub use super::player::Player;

#[derive(Clone, Copy)]
pub struct Action<'a> {
    text: &'a str,
    action: fn(&mut Player) -> &'a str,
}

impl<'a> Action<'a> {
    pub fn new(text: &'a str, action: fn(&mut Player) -> &'a str) -> Action<'_> {
        Self { text, action }
    }

    pub fn act(&self, player: &mut Player) -> &str {
        (self.action)(player)
    }
}

impl<'a> Action<'a> {
    pub fn text(&self) -> &str {
        self.text
    }
}

#[derive(Clone)]
pub struct Event<'a> {
    text: &'a str,
    initialize: fn(&Player),
    actions: Vec<Action<'a>>,
}

impl<'a> Event<'a> {
    pub fn new(text: &'a str, initialize: fn(&Player), actions: Vec<Action<'a>>) -> Self {
        Self {
            text,
            initialize,
            actions,
        }
    }

    pub fn init(&self, player: &Player) {
        (self.initialize)(player);
    }

    pub fn act(&self, player: &mut Player, action: usize) -> &str {
        self.actions[action].act(player)
    }
}

impl<'a> Event<'a> {
    pub fn text(&self) -> &str {
        self.text
    }
    pub fn actions(&self) -> &Vec<Action<'a>> {
        &self.actions
    }
}
