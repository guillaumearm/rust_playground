use super::stack::{self, Stack};

pub struct StackIter<'a>(Option<&'a Stack>);

impl<'a> StackIter<'a> {
    pub fn new(stack: &'a Stack) -> Self {
        StackIter(Some(stack))
    }
}

impl<'a> Iterator for StackIter<'a> {
    type Item = stack::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let stack = self.0?;
        let item = stack.peak()?;

        self.0 = stack.get_next();

        Some(item)
    }
}
