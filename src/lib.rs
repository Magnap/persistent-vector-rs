#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_empty() {
        let v = PVec::<u32>::new();
        println!("{:?}", v);
    }
}

use std::sync::Arc;

#[cfg(feature = "narrow_branching")]
//const BRANCH_EXPONENT: usize = 2;
const BRANCH_FACTOR: usize = 4; // 2^BRANCH_EXPONENT
#[cfg(not(feature = "narrow_branching"))]
//const BRANCH_EXPONENT: usize = 5;
const BRANCH_FACTOR: usize = 32; // 2^BRANCH_EXPONENT

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Default)]
pub struct PVec<T> {
    root: Node<T>,
    len: usize,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Node<T> {
    Branch { children: [Option<Arc<PVec<T>>>; BRANCH_FACTOR], },
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

impl<T> PVec<T> {
    pub fn new() -> Self {
        PVec {
            root: Node::default(),
            len: 0,
        }
    }
}
