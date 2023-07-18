#![feature(test)]

extern crate test;
use test::Bencher;

use least_8::{implementation, make_list};

#[bench]
fn naive(b: &mut Bencher) {
    let l = make_list();
    b.iter(|| {
        let l8 = implementation::naive(&l);
        assert_eq!(vec![4, 5, 15, 22, 28, 31, 37, 38], l8);
    })
}

#[bench]
fn optimized(b: &mut Bencher) {
    let l = make_list();
    b.iter(|| {
        let l8 = implementation::optimized(&l);
        assert_eq!(vec![4, 5, 15, 22, 28, 31, 37, 38], l8);
    })
}

#[bench]
fn thread_optimized(b: &mut Bencher) {
    let l = make_list();
    b.iter(|| {
        let l8 = implementation::thread_optimized(&l);
        assert_eq!(vec![4, 5, 15, 22, 28, 31, 37, 38], l8);
    })
}

#[bench]
fn cheat_optimized(b: &mut Bencher) {
    let l = make_list();
    b.iter(|| {
        let l8 = implementation::cheat_optimized(&l);
        assert_eq!(vec![4, 5, 15, 22, 28, 31, 37, 38], l8);
    })
}

#[bench]
fn less_cheat_optimized(b: &mut Bencher) {
    let l = make_list();
    b.iter(|| {
        let l8 = implementation::less_cheat_optimized(&l);
        assert_eq!(vec![4, 5, 15, 22, 28, 31, 37, 38], l8);
    })
}

#[bench]
fn non_cheat_optimized(b: &mut Bencher) {
    let l = make_list();
    b.iter(|| {
        let l8 = implementation::non_cheat_optimized(&l);
        assert_eq!(vec![4, 5, 15, 22, 28, 31, 37, 38], l8);
    })
}
