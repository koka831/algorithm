extern crate lib;

use lib::{
    io::{read, read_one},
    binary_search::BinarySearch
};


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
