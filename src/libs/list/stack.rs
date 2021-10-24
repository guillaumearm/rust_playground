// use super::StackIter;
use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

mod stack_iter;
use stack_iter::StackIter;

pub type Item<T> = Rc<RefCell<T>>;

pub enum Stack<T> {
    Node { item: Item<T>, next: Box<Stack<T>> },
    TailNode,
}

use Stack::*;

impl<T> Stack<T> {
    pub fn new() -> Self {
        TailNode
    }

    pub fn push(&mut self, item: T) {
        let item = Rc::new(RefCell::new(item));
        let next = Box::new(mem::take(self));

        *self = Node { item, next };
    }

    pub fn pop(&mut self) -> Option<Item<T>> {
        let item = self.peak()?;
        let next = self.get_next_mut()?;

        *self = mem::take(next);

        return Some(item);
    }

    pub fn peak(&self) -> Option<Item<T>> {
        match self {
            TailNode => None,
            Node { item, .. } => {
                return Some(item.clone());
            }
        }
    }

    pub fn iter(&self) -> StackIter<T> {
        StackIter::new(self)
    }

    fn get_next(&self) -> Option<&Self> {
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

impl<T> Default for Stack<T> {
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

        assert_eq!(1, *stack.peak().unwrap().borrow());
        assert_eq!(1, *stack.pop().unwrap().borrow());

        assert_eq!(None, stack.peak());
        assert_eq!(None, stack.pop());
    }

    #[test]
    fn several_push_pop() {
        let mut stack = Stack::new();

        assert_eq!(None, stack.peak());

        stack.push(1);
        assert_eq!(1, *stack.peak().unwrap().borrow());

        stack.push(2);
        assert_eq!(2, *stack.peak().unwrap().borrow());

        stack.push(3);
        assert_eq!(3, *stack.peak().unwrap().borrow());

        assert_eq!(3, *stack.pop().unwrap().borrow());
        assert_eq!(2, *stack.pop().unwrap().borrow());
        assert_eq!(1, *stack.pop().unwrap().borrow());
        assert_eq!(None, stack.pop());
    }

    #[test]
    fn iterator() {
        let mut stack = Stack::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        let data: Vec<u32> = stack.iter().map(|cell| *cell.borrow()).collect();

        assert_eq!(vec![3, 2, 1], data);
    }

    #[test]
    fn two_iterator() {
        let mut stack = Stack::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        let data: Vec<u32> = stack.iter().map(|cell| *cell.borrow()).collect();
        let data2: Vec<u32> = stack.iter().map(|cell| *cell.borrow()).collect();

        assert_eq!(vec![3, 2, 1], data);
        assert_eq!(vec![3, 2, 1], data2);
    }
}
