use crate::libs::list::stack::{self, Stack};

pub struct StackIter<'a, T>(Option<&'a Stack<T>>);

impl<'a, T> StackIter<'a, T> {
    pub fn new(stack: &'a Stack<T>) -> Self {
        StackIter(Some(stack))
    }
}

impl<'a, T> Iterator for StackIter<'a, T> {
    type Item = stack::Item<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let stack = self.0?;
        let item = stack.peak()?;

        self.0 = stack.get_next();

        Some(item)
    }
}
