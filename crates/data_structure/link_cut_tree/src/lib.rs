use std::ptr::NonNull;

#[derive(Clone, Copy, Default)]
struct Node {
    idx: usize,
    left: Option<NonNull<Self>>,
    right: Option<NonNull<Self>>,
    parent: Option<NonNull<Self>>,
}

impl Node {
    fn new(idx: usize) -> NonNull<Self> {
        NonNull::from(Box::leak(Box::new(Node {
            idx,
            ..Default::default()
        })))
    }

    unsafe fn is_root(this: NonNull<Self>) -> bool {
        unsafe { !(Self::is_left_child(this) || Self::is_right_child(this)) }
    }

    unsafe fn is_left_child(this: NonNull<Self>) -> bool {
        unsafe {
            this.as_ref()
                .parent
                .is_some_and(|ptr| ptr.as_ref().left == Some(this))
        }
    }

    unsafe fn is_right_child(this: NonNull<Self>) -> bool {
        unsafe {
            this.as_ref()
                .parent
                .is_some_and(|ptr| ptr.as_ref().right == Some(this))
        }
    }

    unsafe fn rotate(mut this: NonNull<Self>) {
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
            if let Some(mut pp) = this.as_ref().parent {
                if pp.as_ref().left == Some(parent) {
                    pp.as_mut().left = Some(this);
                }
                if pp.as_ref().right == Some(parent) {
                    pp.as_mut().right = Some(this);
                }
            }
        }
    }

    unsafe fn splay(this: NonNull<Self>) {
        unsafe {
            while !Self::is_root(this) {
                let parent = this.as_ref().parent.unwrap();
                if Self::is_root(parent) {
                    Self::rotate(this);
                } else if Self::is_left_child(this) == Self::is_left_child(parent) {
                    Self::rotate(parent);
                    Self::rotate(this);
                } else {
                    Self::rotate(this);
                    Self::rotate(this);
                }
            }
        }
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (left, right, parent) = unsafe {
            let Self {
                left,
                right,
                parent,
                ..
            } = self;
            (
                left.map(|ptr| ptr.as_ref().idx),
                right.map(|ptr| ptr.as_ref().idx),
                parent.map(|ptr| ptr.as_ref().idx),
            )
        };
        f.debug_struct("Node")
            .field("idx", &self.idx)
            .field("left", &left)
            .field("right", &right)
            .field("parent", &parent)
            .finish()
    }
}

pub struct LinkCutTree {
    node: Vec<NonNull<Node>>,
}

impl LinkCutTree {
    pub fn new(n: usize) -> Self {
        Self {
            node: (0..n).map(Node::new).collect(),
        }
    }

    fn expose(&mut self, v: usize) -> NonNull<Node> {
        let mut node = Some(self.node[v]);
        let mut right = None;
        while let Some(mut this) = node {
            unsafe {
                Node::splay(this);
                this.as_mut().right = right;
            }
            right = Some(this);
            unsafe {
                node = this.as_ref().parent;
            }
        }
        unsafe {
            Node::splay(self.node[v]);
        }
        right.unwrap()
    }

    pub fn link(&mut self, v: usize, p: usize) {
        let mut this = self.node[v];
        let mut parent = self.node[p];
        self.expose(v);
        self.expose(p);
        unsafe {
            this.as_mut().parent = Some(parent);
            parent.as_mut().right = Some(this);
        }
    }

    pub fn cut(&mut self, v: usize) {
        let mut this = self.node[v];
        self.expose(v);
        unsafe {
            if let Some(mut parent) = this.as_ref().left {
                parent.as_mut().parent = None;
            }
            this.as_mut().left = None;
        }
    }

    pub fn lca(&mut self, u: usize, v: usize) -> usize {
        self.expose(u);
        unsafe { self.expose(v).as_ref().idx }
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

impl std::fmt::Debug for LinkCutTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.node.iter().map(|ptr| unsafe { ptr.as_ref() }))
            .finish()
    }
}
