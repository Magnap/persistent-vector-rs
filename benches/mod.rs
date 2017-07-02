#![feature(test)]

extern crate persistent_vector;
extern crate test;

use persistent_vector::PVec;
use test::Bencher;

const p: u32 = 5;

#[bench]
fn bench_push_get_pvec(b: &mut Bencher) {
    let n = (10 as usize).pow(p);
    b.iter(|| {
        let mut v = PVec::new();
        for i in 0..n {
            v = v.push(i);
        }
        for i in 0..n {
            assert_eq!(v.get(i), Some(&i));
        }
        assert_eq!(v.get(n), None);
    })
}

#[bench]
fn bench_push_get_vec(b: &mut Bencher) {
    let n = (10 as usize).pow(p);
    b.iter(|| {
        let mut v = Vec::new();
        for i in 0..n {
            v.push(i);
        }
        for i in 0..n {
            assert_eq!(v.get(i), Some(&i));
        }
        assert_eq!(v.get(n), None);
    })
}

#[bench]
fn bench_push_pvec(b: &mut Bencher) {
    let n = (10 as usize).pow(p);
    b.iter(|| {
        let mut v = PVec::new();
        for i in 0..n {
            v = v.push(i);
        }
        assert_eq!(v.get(n), None);
    })
}

#[bench]
fn bench_push_vec(b: &mut Bencher) {
    let n = (10 as usize).pow(p);
    b.iter(|| {
        let mut v = Vec::new();
        for i in 0..n {
            v.push(i);
        }
        assert_eq!(v.get(n), None);
    })
}

#[bench]
fn bench_get_pvec(b: &mut Bencher) {
    let n = (10 as usize).pow(p);
    let mut v = PVec::new();
    for i in 0..n {
        v = v.push(i);
    }
    b.iter(|| {
        for i in 0..n {
            assert_eq!(v.get(i), Some(&i));
        }
        assert_eq!(v.get(n), None);
    })
}

#[bench]
fn bench_get_vec(b: &mut Bencher) {
    let n = (10 as usize).pow(p);
    let mut v = Vec::new();
    for i in 0..n {
        v.push(i);
    }
    b.iter(|| {
        for i in 0..n {
            assert_eq!(v.get(i), Some(&i));
        }
        assert_eq!(v.get(n), None);
    })
}


#[bench]
fn bench_clone_pvec(b: &mut Bencher) {
    let n = (10 as usize).pow(p);
    let mut v = PVec::new();
    for i in 0..n {
        v = v.push(i);
    }
    b.iter(|| { let vs = v.clone(); })
}

#[bench]
fn bench_clone_vec(b: &mut Bencher) {
    let n = (10 as usize).pow(p);
    let mut v = Vec::new();
    for i in 0..n {
        v.push(i);
    }
    b.iter(|| { let vs = v.clone(); })
}
