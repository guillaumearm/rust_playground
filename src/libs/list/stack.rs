use super::StackIter;
use std::mem;

pub type Item = u32;

pub enum Stack {
    Node { item: Item, next: Box<Stack> },
    TailNode,
}

use Stack::*;

impl Stack {
    pub fn new() -> Self {
        TailNode
    }

    pub fn push(&mut self, item: Item) {
        let next = Box::new(mem::take(self));
        *self = Node { item, next };
    }

    pub fn pop(&mut self) -> Option<Item> {
        let item = self.peak()?;
        let next = self.get_next_mut()?;

        *self = mem::take(next);

        return Some(item);
    }

    pub fn peak(&self) -> Option<Item> {
        match self {
            TailNode => None,
            Node { item, .. } => Some(*item), // copy here !
        }
    }

    pub fn iter(&self) -> StackIter {
        StackIter::new(self)
    }

    pub fn get_next(&self) -> Option<&Self> {
        match self {
            TailNode => None,
            Node { next, .. } => Some(next),
        }
    }

    fn get_next_mut(&mut self) -> Option<&mut Self> {
        match self {
            TailNode => None,
            Node { next, .. } => Some(next),
        }
    }
}

impl Default for Stack {
    fn default() -> Self {
        Stack::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_push_pop() {
        let mut stack = Stack::new();

        assert_eq!(None, stack.peak());
        assert_eq!(None, stack.pop());

        stack.push(1);

        assert_eq!(Some(1), stack.peak());
        assert_eq!(Some(1), stack.pop());

        assert_eq!(None, stack.peak());
        assert_eq!(None, stack.pop());
    }

    #[test]
    fn several_push_pop() {
        let mut stack = Stack::new();

        assert_eq!(None, stack.peak());

        stack.push(1);
        assert_eq!(Some(1), stack.peak());

        stack.push(2);
        assert_eq!(Some(2), stack.peak());

        stack.push(3);
        assert_eq!(Some(3), stack.peak());

        assert_eq!(Some(3), stack.pop());
        assert_eq!(Some(2), stack.pop());
        assert_eq!(Some(1), stack.pop());
        assert_eq!(None, stack.pop());
    }

    #[test]
    fn iterator() {
        let mut stack = Stack::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        let data: Vec<u32> = stack.iter().collect();

        assert_eq!(vec![3, 2, 1], data);
    }

    #[test]
    fn two_iterator() {
        let mut stack = Stack::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        let data: Vec<u32> = stack.iter().collect();
        let data2: Vec<u32> = stack.iter().collect();

        assert_eq!(vec![3, 2, 1], data);
        assert_eq!(vec![3, 2, 1], data2);
    }
}
