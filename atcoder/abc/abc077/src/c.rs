#![allow(unused_imports)]
use std::io;
use std::cmp;


#[allow(dead_code)]
fn read<T>() -> Vec<T>
where T:
    std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace()
        .map(|s| s.trim().parse().unwrap())
        .collect()
}

#[allow(dead_code)]
fn read_one<T>() -> T
where T:
    std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}

pub trait BinarySearch<T> {
    fn lower_bound(&self, &T) -> usize;
    fn upper_bound(&self, &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                cmp::Ordering::Less => {
                    low = mid + 1;
                },
                _ => { high = mid; }
            }
        }
        low
    }

    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                cmp::Ordering::Greater => {
                    high = mid;
                },
                _ => { low = mid + 1; }
            }
        }
        low
    }
}

fn main() {
    let _n = read_one::<usize>();
    let mut top = read::<usize>();
    let mut mid = read::<usize>();
    let mut btm = read::<usize>();
    top.sort();
    mid.sort();
    btm.sort();

    let mut pat = 0;

    // for i for j => O(n^2)
    /*
    for i in btm.iter() {
        let bound_i = mid.lower_bound(&i);
        for j in &mid[0..bound_i] {
            let bound_j = top.lower_bound(&j);
            pat += bound_j;
        }
    }
    */

    // for mid[i] lower * upper => O(n log n)
    for i in mid.iter() {
        let btm_bound = btm.upper_bound(&i);
        let top_bound = top.lower_bound(&i);
        pat += (btm.len() - btm_bound) * top_bound;
    }

    println!("{}", pat);
}
