use std::rc::{Rc, Weak};

#[derive(Clone)]
struct Node {
    left: Option<Rc<Node>>,
    right: Option<Rc<Node>>,
    parent: Weak<Node>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.right == other.right
    }
}

impl Node {
    fn new() -> Rc<Self> {
        Rc::new(Self {
            left: None,
            right: None,
            parent: Weak::new(),
        })
    }

    fn state(self: Rc<Self>) -> i8 {
        if let Some(par) = self.parent.upgrade() {
            if par.left == Some(self.clone()) {
                return -1;
            }
            if par.right == Some(self.clone()) {
                return 1;
            }
        }
        0
    }
}
