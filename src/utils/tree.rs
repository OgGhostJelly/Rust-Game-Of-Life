use std::collections::HashMap;

// use traits and then paired with generics so functions still have sized return values

#[derive(Clone, Copy)]
pub struct NodePath<'a, const N: usize>([&'a str; N]);

impl<'a, const N: usize> NodePath<'a, N> {
    pub const fn new() -> Self {
        Self([])
    }
}

impl<'a, const N: usize> From<[&'a str; N]> for NodePath<'a, N> {
    fn from(value: [&'a str; N]) -> Self {
        Self(value)
    }
}

impl<'a, const N: usize> From<Vec<&'a str>> for NodePath<'a, N> {
    fn from(value: Vec<&'a str>) -> Self {
        Self(value)
    }
}

impl<'a, const N: usize> IntoIterator for NodePath<'a, N> {
    type Item = &'a &'a str;
    type IntoIter = std::slice::Iter<'a, &'a str>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

pub trait Node<'a>
where
    Self: Sized,
{
    fn parent(&self) -> &Option<&Self>;
    fn children(&self) -> &HashMap<&str, Self>;
    // Adds the child, and if a child by that name already exists return `Some(child)` else `None`
    fn add_child(&mut self, name: &'a str, child: Self) -> Option<Self>;
    // Removes the child and return `Some(child)`, if the child doesn't exist return `None`
    fn remove_child(&mut self, name: &str) -> Option<Self>;

    fn get_child(&self, child: &str) -> Option<&Self> {
        self.children().get(child)
    }

    fn get_node<const N: usize>(&self, path: &NodePath<N>) -> Option<&Self> {
        let mut pointer = self;

        for part in path {
            pointer = match part {
                &".." => match pointer.parent() {
                    Some(p) => p,
                    None => return None,
                },
                _ => match pointer.get_child(part) {
                    Some(c) => c,
                    None => return None,
                },
            }
        }

        Some(pointer)
    }
}
