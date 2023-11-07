use std::collections::HashMap;
use crate::utils::tree::{Node, NodePath};
use super::player::Player;

#[derive(Clone, Copy)]
pub struct Action<'a> {
    pub text: &'a str,
    pub event: &'a NodePath<'a>,
}

pub struct Event<'a> {
    text: &'a str,
    actions: Vec<Action<'a>>,
    initialize: fn(&mut Player) -> bool,

    parent: Option<&'a Event<'a>>,
    children: HashMap<&'a str, Event<'a>>,
}

impl<'a> Event<'a> {
    pub fn new(text: &str, actions: Vec<Action<'a>>, initialize: fn(&mut Player) -> bool, children: HashMap<&'a str, Event<'a>>) -> Self {
        let this = Self {
            text,
            actions,
            initialize,
            parent: None,
            children: children,
        };

        for (key, mut value) in children {
            value.parent = Some(&this);
        };

        this
    }
}

impl<'a> Node for Event<'a> {
    fn parent(&self) -> Option<&Self> {
        self.parent
    }

    fn children(&self) -> &HashMap<&str, Self> {
        &self.children
    }
}
/*
impl<'a> Event<'a> {
    pub fn new(text: &str, initialize: fn(&mut Player) -> bool, children: HashMap<&'a str, Event<'a>>) -> Self {
        let this = Self {
            text,
            initialize,

            parent: None,
            children,
        };

        for (key, mut value) in children {
            value.parent = Some(&this);
        };

        this
    }
}

impl<'a> Event<'a> {
    pub fn text(&self) -> &str {
        self.text
    }

    pub fn init(&self, player: &mut Player) -> bool {
        (self.initialize)(player)
    }

    pub fn parent(&self) -> Option<&Event<'_>> {
        self.parent
    }
    
    pub fn children(&self) -> &HashMap<&'a str, Event<'a>> {
        &self.children
    }
}*/