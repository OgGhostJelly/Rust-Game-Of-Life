use std::collections::HashMap;

// create FastNodePath and NodePath for Array and Vec respectively

#[derive(Clone, Copy)]
pub struct NodePath<'a, const SIZE: usize> {
    parts: [&'a str; SIZE]
}

impl<'a, const SIZE: usize> NodePath<'a, SIZE> {
    pub const fn new(parts: [&str; SIZE]) -> Self {
        Self {
            parts,
        }
    }
}

pub trait Node where Self: Sized {
    fn parent(&self) -> Option<&Self>;
    fn children(&self) -> &HashMap<&str, Self>;

    fn get_child(&self, child: &str) -> Option<&Self> {
        self.children().get(child)
    }

    fn get_node<const SIZE: usize>(&self, path: &NodePath<SIZE>) -> Option<&Self> {
        let pointer: &Self = self;
        
        for part in path.parts {
            pointer = match part {
                ".." => match pointer.parent() {
                    Some(p) => p,
                    None => return None,
                }
                _ => match pointer.get_child(part) {
                    Some(c) => c,
                    None => return None,
                }
            }
        }

        Some(pointer)
    }
}