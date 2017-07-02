#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_get() {
        let mut v = PVec::new();
        for i in 0..(BRANCH_FACTOR.pow(2) + 1) {
            v = v.push(i);
        }
        println!("{:#?}", v);
        for i in 0..(BRANCH_FACTOR.pow(2) + 1) {
            println!("{}:", i);
            assert_eq!(v.get(i), Some(&i));
        }
        assert_eq!(v.get(BRANCH_FACTOR.pow(2) + 1), None);
    }

    #[test]
    fn false_get() {
        let v = PVec::new().push(0);
        for n in 0..BRANCH_FACTOR {
            println!("n: {}", n);
            assert_eq!(v.get((n + 1) * BRANCH_FACTOR), None);
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
    pub fn new() -> Self {
        PVec {
            root: Node::default(),
            len: 0,
        }
    }

    // For elegance, this would be recursive (and there wouldn't be a depth field on `Branch`),
    // but I'm worried about the performance of that.
    fn depth(&self) -> u8 {
        match self.root {
            Node::Branch { ref depth, .. } => *depth,
            Node::Leaf { .. } => 0,
        }
    }

    pub fn push(self, element: T) -> Self {
        if self.len < BRANCH_FACTOR {
            let mut new = self.clone();
            match new.root {
                Node::Leaf { ref mut elements } => {
                    elements[new.len] = Some(element);
                    new.len += 1;
                }
                Node::Branch { ref mut children, .. } => {
                    let i = self.len;
                    let old = mem::replace(&mut children[i], None);
                    match old {
                        Some(child_ref) => {
                            let child = (*child_ref).clone().push(element);
                            if child.len == BRANCH_FACTOR {
                                new.len += 1;
                            }
                            mem::replace(&mut children[i], Some(Arc::new(child)));
                        }
                        None => {
                            let child = PVec::new().push(element);
                            mem::replace(&mut children[i], Some(Arc::new(child)));
                        }
                    }
                }
            }
            new
        } else {
            let mut children = empty_arr!();
            let depth = self.depth();
            children[0] = Some(Arc::new(self));
            children[1] = Some(Arc::new(PVec::new().push(element)));
            PVec {
                root: Node::Branch {
                    children: children,
                    depth: depth + 1,
                },
                len: 1,
            }
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        let i = (index >> (BRANCH_EXPONENT * (self.depth() as usize))) & (BRANCH_FACTOR - 1);
        match self.root {
            Node::Branch { ref children, .. } => children[i].as_ref().and_then(|c| c.get(index)),
            Node::Leaf { ref elements } => elements[i].as_ref(),
        }
    }
}
