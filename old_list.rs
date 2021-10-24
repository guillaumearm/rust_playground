use std::cell::{Cell, RefCell};
use std::rc::Rc;

type Item = u32;

type Link = Rc<RefCell<Node>>;

#[derive(Debug, PartialEq)]
pub struct Node {
    item: Cell<Item>,
    next: Option<Link>,
    prev: Option<Link>,
}

impl Node {
    pub fn new(item: Item) -> Self {
        let item = Cell::new(item);

        Node {
            item,
            next: None,
            prev: None,
        }
    }

    pub fn clone(&self) -> Node {
        Node {
            item: self.item.clone(),
            next: self.next.clone(),
            prev: self.prev.clone(),
        }
    }

    pub fn item(&self) -> &Cell<Item> {
        &self.item
    }

    fn get_next(&self) -> Option<Link> {
        Some(self.next.as_ref()?.clone())
    }

    fn get_prev(&self) -> Option<Link> {
        Some(self.prev.as_ref()?.clone())
    }

    fn get_rc(&self) -> Option<Link> {
        let rc = self.get_next().and_then(|rc| rc.borrow().get_prev());

        if rc.is_some() {
            return rc;
        }

        return self.get_prev().and_then(|rc| rc.borrow().get_next());
    }

    fn rc(&self) -> Link {
        let rc = self.get_rc();
        if rc.is_some() {
            rc.unwrap()
        } else {
            Rc::new(RefCell::new(self.clone()))
        }
    }

    pub fn attach_next(&self, new_node: Node) -> Option<Link> {
        if new_node.prev.is_some() || self.next.is_some() {
            return None;
        }

        let left_ref = self.rc();
        let right_ref = new_node.rc();

        let mut l = left_ref.borrow_mut();
        let mut r = right_ref.borrow_mut();

        l.next = Some(right_ref.clone());
        r.prev = Some(left_ref.clone());
        // right_ref.borrow_mut().prev = Some(left_ref);

        Some(right_ref.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_node() {
        let node = Node::new(42);

        assert_eq!(42, node.item.get());
    }

    #[test]
    fn get_item() {
        let node = Node::new(42);
        let cell = node.item();

        let item = cell.get();
        assert_eq!(42, item);
    }

    #[test]
    fn set_item() {
        let node = Node::new(0);
        let cell = node.item();

        cell.set(42);
        let item = cell.get();
        assert_eq!(42, item);
    }

    #[test]
    fn attach_next() {
        let node = Node::new(0);

        assert_ne!(None, node.attach_next(Node::new(1)));

        assert_eq!(0, node.item.get());

        assert_ne!(None, node.next);
        assert_eq!(1, node.next.unwrap().borrow().item.get());
        // assert_eq!(None, node.attach_next(Node::new(2)));
    }
}
