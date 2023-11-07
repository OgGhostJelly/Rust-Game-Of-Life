use super::player::Player;
use crate::utils::tree::{Node, NodePath};
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct Action<'a> {
    text: &'a str,
    action: fn(&mut Player) -> &NodePath,
}

impl<'a> Action<'a> {
    pub fn new(text: &'a str, action: fn(&mut Player) -> &NodePath) -> Self {
        Self {
            text,
            action,
        }
    }

    pub fn text(&self) -> &str {
        self.text
    }

    pub fn act(&self, player: &'a mut Player) -> &NodePath {
        (self.action)(player)
    }
}

#[derive(Clone)]
pub struct Event<'a> {
    text: &'a str,
    actions: Vec<Action<'a>>,
    initialize: fn(&mut Player) -> bool,

    parent: Option<&'a Event<'a>>,
    children: HashMap<&'a str, Event<'a>>,
}

impl<'a> Event<'a> {
    pub fn new(
        text: &'a str,
        actions: Vec<Action<'a>>,
        initialize: fn(&mut Player) -> bool,
        children: HashMap<&'a str, Event<'a>>,
    ) -> Self {
        Self {
            text,
            actions,
            initialize,
            parent: None,
            children,
        }
    }
}

impl<'a> Event<'a> {
    pub fn actions(&self) -> &Vec<Action<'a>> {
        &self.actions
    }

    pub fn text(&self) -> &str {
        self.text
    }

    pub fn init(&self, player: &mut Player) -> bool {
        (self.initialize)(player)
    }

    pub fn act(&self, player: &'a mut Player, action: usize) -> Option<&NodePath> {
        match self.actions.get(action) {
            Some(value) => Some(value.act(player)),
            None => None,
        }
    }
}

impl<'a> Node<'a> for Event<'a> {
    fn parent(&self) -> &Option<&Self> {
        &self.parent
    }

    fn children(&self) -> &HashMap<&str, Self> {
        &self.children
    }

    fn add_child(&mut self, name: &'a str, child: Self) -> Option<Self> {
        if self.children.contains_key(name) {
            return Some(child);
        }

        match self.children.insert(name, child) {
            Some(_) => panic!("This code should've been unreachable"),
            None => None,
        }
    }

    fn remove_child(&mut self, name: &str) -> Option<Self> {
        self.children.remove(name)
    }
}
