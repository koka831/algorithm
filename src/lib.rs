#![allow(dead_code)]
pub mod binary_search;
pub mod union_find;
pub mod segment_tree;

pub mod io {
    use std::io;

    pub fn read<T>() -> Vec<T>
        where T:
        std::str::FromStr,
        T::Err: std::fmt::Debug {

        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        buf.split_whitespace()
            .map(|s| s.trim().parse().unwrap())
            .collect()
    }

    pub fn read_one<T>() -> T
        where T:
        std::str::FromStr,
        T::Err: std::fmt::Debug {

        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        buf.trim().parse().unwrap()
    }
}
