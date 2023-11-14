use crate::tree;

use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
pub struct Path<Item, T: IntoIterator<Item = Option<Item>>>(pub T);

impl<Item, Iter: IntoIterator<Item = Option<Item>>> IntoIterator for Path<Item, Iter> {
    type Item = Iter::Item;
    type IntoIter = Iter::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[macro_export]
macro_rules! path {
    ( $( $x:expr ),* ) => {
        Path([$(
            match $x {
                ".." => None,
                _ => Some($x),
            }
        ,)*])
    };
}

#[derive(Debug)]
pub struct Event<'a> {
    data: &'a str,
    path: Vec<&'a str>,
    children: HashMap<&'a str, Event<'a>>,
}

impl<'a> Event<'a> {
    pub fn new(data: &'a str, mut children: HashMap<&'a str, Event<'a>>) -> Self {
        fn update_path<'a>(node: &mut Event<'a>, path: &'a str) {
            node.path.push(path);

            for (key, value) in &mut node.children {
                update_path(value, path)
            }
        }

        for (key, value) in &mut children {
            update_path(value, key)
        }

        Self {
            data,
            path: Vec::new(),
            children,
        }
    }

    pub fn path(&self) -> std::iter::Rev<std::slice::Iter<'_, &str>> {
        self.path.iter().rev()
    }

    pub fn parent_path(&self) -> std::iter::Rev<std::iter::Skip<std::slice::Iter<'_, &str>>> {
        self.path.iter().skip(1).rev()
    }
}

impl<'a> tree::Node<'a, &'a str> for Event<'a> {
    fn get_child(&self, key: &'a str) -> Option<&Self> {
        self.children.get(key)
    }
}
