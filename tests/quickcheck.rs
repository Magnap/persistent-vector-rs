#![feature(plugin)]
#![plugin(quickcheck_macros)]

extern crate quickcheck;
extern crate persistent_vector;

use persistent_vector::*;
use quickcheck::{Arbitrary, Gen, TestResult};

#[derive(Debug, Clone)]
enum Action<T> {
    Push(T),
    Get(u8),
    Len,
}

impl<T: Arbitrary> Arbitrary for Action<T> {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let x: u8 = g.gen();
        let x = x % 3;
        match x {
            0 => Action::Push(Arbitrary::arbitrary(g)),
            1 => Action::Get(g.gen()),
            2 => Action::Len,
            _ => panic!("math is broken"),
        }
    }
}

#[quickcheck]
fn vec_equivalence(actions: Vec<Action<usize>>) -> TestResult {
    let mut v = Vec::new();
    let mut v_res = Vec::new();
    for a in &actions {
        match *a {
            Action::Push(x) => v.push(x),
            Action::Get(i) => {
                let x = v.get(i as usize);
                let x = match x {
                    Some(r) => Some(*r),
                    None => None,
                };
                v_res.push(x)
            }
            Action::Len => v_res.push(Some(v.len())),
        }
    }
    let mut pv = PVec::new();
    let mut pv_res = Vec::new();
    for a in &actions {
        match *a {
            Action::Push(x) => pv = pv.push(x),
            Action::Get(i) => {
                let x = pv.get(i as usize);
                let x = match x {
                    Some(r) => Some(*r),
                    None => None,
                };
                pv_res.push(x)
            }
            Action::Len => pv_res.push(Some(pv.len())),
        }
    }
    println!("Test complete\n");
    if v_res != pv_res {
        println!("{:?}", actions);
        println!("{:?}", v);
        println!("{:#?}", pv);
        println!("{:?} vs {:?}", v_res, pv_res);
        return TestResult::failed();
    }
    if v_res.iter().filter(|x| x.is_some()).count() == 0 {
        TestResult::discard()
    } else {
        TestResult::passed()
    }
}
