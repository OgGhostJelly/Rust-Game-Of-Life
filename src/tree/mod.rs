//! Utilities for a tree data structure.

mod test;
mod utils;

use self::utils::SkipLast;
use std;


pub struct ArenaNode<T> {
    data: T,
    parent: Option<usize>,
    children: Vec<usize>,
}

pub struct ArenaTree<T> {
    nodes: Vec<ArenaNode<T>>,
}

impl<T> ArenaNode<T> {
    fn get_child<'a>(&'a self, tree: &'a ArenaTree<T>, key: usize) -> Option<&Self> {
        tree.nodes.get(key)
    }

    fn get_node<'a, P: IntoIterator<Item = usize>>(&'a self, tree: &'a ArenaTree<T>, path: P) -> Option<&Self> {
        let mut node = self;

        for key in path {
            node = match node.get_child(tree, key) {
                Some(child) => child,
                None => return None,
            }
        }

        Some(node)
    }
}


// Implement arena tree
// OneWayPath & TwoWayPath?
// OneWayNode & TwoWayNode?

/*
/// The path to a node in a tree, simply a wrapper around any type that implements `IntoIterator`.
///
/// you can use the iterator directly but `Path` makes it more clear what the iterator is suppose to be used for.
#[derive(Clone, Copy)]
pub struct Path<T: IntoIterator>(pub T);

impl<'a, T: IntoIterator> IntoIterator for Path<T> {
    type Item = T::Item;
    type IntoIter = T::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// Represents a node in a tree.
pub trait Node<'a, K> {
    /// Returns the child at `key` in this node.
    fn get_child(&self, key: K) -> Option<&Self>;

    /// Returns a node using a path or `None` if no node is at that path.
    fn get_node<P: IntoIterator<Item = K>>(&self, path: P) -> Option<&Self> {
        let mut node = self;

        for key in path {
            node = match node.get_child(key) {
                Some(child) => child,
                None => return None,
            }
        }

        Some(node)
    }
}

/// For types that have children that can be publicly accessed.
pub trait AccessibleChildren<'a, T: IntoIterator<Item = &'a Self>>
where
    Self: Sized + 'a,
{
    /// Returns an immutable iterator over this node's children.
    fn children(&'a self) -> T;

    /// Calls `f` on each of this node's descendants.
    fn descendants<F: Fn(&Self)>(&'a self, f: &F) {
        for child in self.children().into_iter() {
            (f)(child);
            child.descendants(f);
        }
    }
}

/// For types that have children that can be publicly modified.
pub trait AccessibleChildrenMut<'a, T: IntoIterator<Item = &'a mut Self>>
where
    Self: Sized + 'a,
{
    /// Returns a mutable iterator over this node's children.
    fn children_mut(&'a mut self) -> T;

    /// Calls `f` on each of this node's descendants.
    fn descendants_mut<F: Fn(&mut Self)>(&'a mut self, f: &F) {
        for child in self.children_mut().into_iter() {
            (f)(child);
            child.descendants_mut(f);
        }
    }
}

/// For types that know the path to itself.
pub trait Pathed<'a, P: Iterator> {
    /// Returns the path to this node.
    fn path(&'a self) -> P;

    /// Returns the path to this node's parent.
    fn parent_path(&'a self) -> utils::SkipLastIterator<P> {
        self.path().skip_last()
    }
}

/// An implementation of `Node` where each node knows the path to itself. As of now, children cannot be removed once added.
#[derive(Clone, Debug)]
pub struct PathedNode<T> {
    pub value: T,
    path: Vec<usize>,
    children: Vec<Self>,
}

impl<'a, T> Node<'a, usize> for PathedNode<T> {
    fn get_child(&self, key: usize) -> Option<&Self> {
        self.children.get(key)
    }
}

impl<'a, T> AccessibleChildren<'a, &'a Vec<Self>> for PathedNode<T> {
    fn children(&'a self) -> &'a Vec<Self> {
        &self.children
    }
}

impl<'a, T> AccessibleChildrenMut<'a, &'a mut Vec<Self>> for PathedNode<T> {
    fn children_mut(&'a mut self) -> &'a mut Vec<Self> {
        &mut self.children
    }
}

impl<'a, T> Pathed<'a, std::iter::Rev<std::slice::Iter<'a, usize>>> for PathedNode<T> {
    fn path(&'a self) -> std::iter::Rev<std::slice::Iter<'a, usize>> {
        self.path.iter().rev()
    }
}

impl<'a, T> PathedNode<T> {
    // Returns a new instance of `PathedNode`
    pub fn new(value: T) -> Self {
        Self {
            value,
            path: Vec::new(),
            children: Vec::new(),
        }
    }

    // Returns a new instance of `PathedNode` with the added children.
    pub fn create(value: T, children: Vec<Self>) -> Self {
        let mut this = Self::new(value);
        for child in children {
            this.add_child(child)
        }
        this
    }

    // Adds the child into the node.
    pub fn add_child(&mut self, mut child: Self) {
        let idx = self.children.len();

        child.path.push(idx);

        child.descendants_mut(&|x| x.path.push(idx));

        self.children.push(child)
    }

    /*// Returns a `&Vec` of this node's children.
    pub fn children(&self) -> &Vec<Self> {
        &self.children
    }

    // Returns a `&mut Vec` of this node's children.
    pub fn children_mut(&mut self) -> &mut Vec<Self> {
        &mut self.children
    }

    // Calls `f` on each of this node's descendants.
    pub fn descendants<F: Fn(&Self)>(&self, f: &F) {
        for child in &self.children {
            (f)(child);
            child.descendants(f);
        }
    }

    // Calls `f` on each of this node's descendants.
    pub fn descendants_mut<F: Fn(&mut Self)>(&mut self, f: &F) {
        for child in &mut self.children {
            (f)(child);
            child.descendants_mut(f);
        }
    }

    // Returns the path to this node.
    pub fn path(&self) -> std::iter::Rev<std::slice::Iter<'_, usize>> {
        self.path.iter().rev()
    }

    // Returns the path to this node's parent.
    //
    // Same as `ascend_path(1)`
    pub fn parent_path(&self) -> std::iter::Rev<std::iter::Skip<std::slice::Iter<'_, usize>>> {
        self.ascend_path(1)
    }

    // Returns the path to the `n`th ancestor to this node.
    //
    // If `n`th is larger than the amount of ancestors, the path will refer to the root.
    pub fn ascend_path(&self, n: usize) -> std::iter::Rev<std::iter::Skip<std::slice::Iter<'_, usize>>> {
        self.path.iter().skip(n).rev()
    }*/
}
*/