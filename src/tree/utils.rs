/// An iterator that skips its last element.
pub struct SkipLastIterator<I: Iterator>(std::iter::Peekable<I>);

impl<I: Iterator> Iterator for SkipLastIterator<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.0.next();
        match self.0.peek() {
            Some(_) => Some(item.unwrap()),
            None => None,
        }
    }
}

pub trait SkipLast: Iterator + Sized {
    /// Creates an iterator that skips the last element.
    fn skip_last(self) -> SkipLastIterator<Self> {
        SkipLastIterator(self.peekable())
    }
}

impl<I: Iterator> SkipLast for I {}
