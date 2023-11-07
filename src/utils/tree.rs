use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct NodePath<'a> {
    parts: &'a [&'a str],
}

impl<'a> NodePath<'a> {
    pub const fn new(parts: &'a [&str]) -> Self {
        Self { parts }
    }
}

#[macro_export]
macro_rules! const_nodepath {
    ( $( $x:expr ),+ ) => {
        {
            const P: NodePath<'_> = NodePath::new(&[$($x,)*]);
            &P
        }
    };
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

    fn get_node(&self, path: &NodePath) -> Option<&Self> {
        let mut pointer = self;

        for part in path.parts {
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
