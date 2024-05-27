use std::ptr::NonNull;

#[derive(Clone, Copy, Default)]
struct Node {
    left: Option<NonNull<Self>>,
    right: Option<NonNull<Self>>,
    parent: Option<NonNull<Self>>,
}

impl Node {
    fn new() -> NonNull<Self> {
        NonNull::from(Box::leak(Box::default()))
    }

    fn is_root(this: NonNull<Self>) -> bool {
        !(Self::is_left_child(this) || Self::is_right_child(this))
    }

    fn is_left_child(this: NonNull<Self>) -> bool {
        unsafe {
            this.as_ref()
                .parent
                .is_some_and(|ptr| ptr.as_ref().left == Some(this))
        }
    }

    fn is_right_child(this: NonNull<Self>) -> bool {
        unsafe {
            this.as_ref()
                .parent
                .is_some_and(|ptr| ptr.as_ref().right == Some(this))
        }
    }

    fn rotate(mut this: NonNull<Self>) {
        unsafe {
            let mut parent = this.as_ref().parent.unwrap();
            if let Some(mut mid) = if Self::is_left_child(this) {
                let mid = this.as_ref().right;
                parent.as_mut().left = mid;
                this.as_mut().right = Some(parent);
                mid
            } else {
                let mid = this.as_ref().left;
                parent.as_mut().right = mid;
                this.as_mut().left = Some(parent);
                mid
            } {
                mid.as_mut().parent = Some(parent);
            }
            this.as_mut().parent = parent.as_ref().parent;
            parent.as_mut().parent = Some(this);
            if let Some(mut pp) = parent.as_ref().parent {
                if pp.as_ref().left == Some(parent) {
                    pp.as_mut().left = Some(this);
                }
                if pp.as_ref().right == Some(parent) {
                    pp.as_mut().right = Some(this);
                }
            }
        }
    }

    fn splay(mut this: NonNull<Self>) {
        while !Self::is_root(this) {
            let parent = unsafe { this.as_ref().parent.unwrap() };
            if Self::is_root(parent) {
                Self::rotate(this);
            } else if Self::is_left_child(this) ^ Self::is_left_child(parent) {
                Self::rotate(this);
                Self::rotate(this);
            } else {
                Self::rotate(parent);
                Self::rotate(this);
            }
        }
    }
}

pub struct LinkCutTree {
    node: Vec<NonNull<Node>>,
}

impl LinkCutTree {
    pub fn new(n: usize) -> Self {
        Self {
            node: vec![Node::new(); n],
        }
    }
}

impl Drop for LinkCutTree {
    fn drop(&mut self) {
        for node in &self.node {
            unsafe {
                drop(Box::from_raw(node.as_ptr()));
            }
        }
    }
}
