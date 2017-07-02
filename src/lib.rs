#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_get() {
        let n = BRANCH_FACTOR.pow(3) + 1;
        let mut v = PVec::new();
        for i in 0..n {
            v = v.push(i);
        }
        println!("{:#?}", v);
        for i in 0..n {
            println!("{}:", i);
            assert_eq!(v.get(i), Some(&i));
        }
        assert_eq!(v.get(n), None);
    }

    #[test]
    fn false_get() {
        let v = PVec::new().push(0);
        for n in 0..BRANCH_FACTOR {
            let i = (n + 1) * BRANCH_FACTOR;
            println!("n: {}, i: {}", n, i);
            assert_eq!(v.get(i), None);
        }
    }
}

use std::sync::Arc;
use std::mem;

#[cfg(feature = "narrow_branching")]
const BRANCH_EXPONENT: usize = 2;
#[cfg(feature = "narrow_branching")]
const BRANCH_FACTOR: usize = 4; // 2^BRANCH_EXPONENT

#[cfg(not(feature = "narrow_branching"))]
const BRANCH_EXPONENT: usize = 5;
#[cfg(not(feature = "narrow_branching"))]
const BRANCH_FACTOR: usize = 32; // 2^BRANCH_EXPONENT

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Default)]
pub struct PVec<T> {
    root: Node<T>,
    len: usize,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Node<T> {
    Branch {
        children: [Option<Arc<PVec<T>>>; BRANCH_FACTOR],
        // depth won't ever be higher than log_{BRANCH_FACTOR}(MAX: usize)
        depth: u8,
    },
    Leaf { elements: [Option<T>; BRANCH_FACTOR], },
}

#[cfg(feature = "narrow_branching")]
macro_rules! empty_arr {
    () => {
        [None, None,
         None, None]
    }
}

#[cfg(not(feature = "narrow_branching"))]
macro_rules! empty_arr {
    () => {
        [None, None, None, None,
         None, None, None, None,
         None, None, None, None,
         None, None, None, None,
         None, None, None, None,
         None, None, None, None,
         None, None, None, None,
         None, None, None, None]
    }
}

impl<T> Default for Node<T> {
    fn default() -> Self {
        Node::Leaf { elements: empty_arr!() }
    }
}

impl<T: Clone> Clone for Node<T> {
    #[inline]
    fn clone(&self) -> Self {
        match *self {
            Node::Branch {
                ref children,
                ref depth,
            } => {
                let mut kids = empty_arr!();
                for i in 0..children.len() {
                    kids[i] = children[i].clone()
                }
                Node::Branch {
                    children: kids,
                    depth: *depth,
                }
            }
            Node::Leaf { ref elements } => {
                let mut elems = empty_arr!();
                for i in 0..elements.len() {
                    elems[i] = elements[i].clone()
                }
                Node::Leaf { elements: elems }
            }
        }
    }
}

use std::fmt::Debug;

impl<T: Clone + Debug> PVec<T> {
    #[inline]
    fn new_depth(depth: u8) -> Self {
        let node = if depth == 0 {
            Node::default()
        } else {
            let mut children = empty_arr!();
            children[0] = Some(Arc::new(Self::new_depth(depth - 1)));
            Node::Branch {
                children: children,
                depth: depth,
            }
        };
        PVec { root: node, len: 0 }
    }

    #[inline]
    pub fn new() -> Self {
        Self::new_depth(0)
    }

    // For elegance, this would be recursive (and there wouldn't be a depth field on `Branch`),
    // but I'm worried about the performance of that.
    #[inline]
    fn depth(&self) -> u8 {
        match self.root {
            Node::Branch { ref depth, .. } => *depth,
            Node::Leaf { .. } => 0,
        }
    }

    #[inline]
    pub fn push(mut self, element: T) -> Self {
        if self.len < BRANCH_FACTOR {
            match self.root {
                Node::Leaf { ref mut elements } => {
                    elements[self.len] = Some(element);
                    self.len += 1;
                }
                Node::Branch {
                    ref mut children,
                    depth,
                } => {
                    let i = self.len;
                    let old = mem::replace(&mut children[i], None);
                    match old {
                        Some(child_ref) => {
                            let child = match Arc::try_unwrap(child_ref) {
                                Ok(child) => child,
                                Err(child_ref) => (*child_ref).clone(),
                            }.push(element);
                            if child.len == BRANCH_FACTOR {
                                self.len += 1;
                            }
                            mem::replace(&mut children[i], Some(Arc::new(child)));
                        }
                        None => {
                            let child = Self::new_depth(depth - 1).push(element);
                            mem::replace(&mut children[i], Some(Arc::new(child)));
                        }
                    }
                }
            }
            self
        } else {
            let mut children = empty_arr!();
            let depth = self.depth();
            children[0] = Some(Arc::new(self));
            PVec {
                root: Node::Branch {
                    children: children,
                    depth: depth + 1,
                },
                len: 1,
            }.push(element)
        }
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        let factor = BRANCH_EXPONENT * self.depth() as usize;
        let mask = BRANCH_FACTOR - 1;
        let i = (index >> factor) & mask;
        let index = index & !(mask << factor);
        if index >= BRANCH_FACTOR.pow(self.depth() as u32 + 1) {
            None
        } else {
            match self.root {
                Node::Branch { ref children, .. } => {
                    children[i].as_ref().and_then(|c| c.get(index))
                }
                Node::Leaf { ref elements } => elements[i].as_ref(),
            }
        }
    }
}
