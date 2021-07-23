use std::cmp;

fn main() {
    let n = io::read_one::<usize>();
    let mut an = io::read::<isize>();
    an.sort();

    let q = io::read_one::<usize>();


    for _ in 0..q {
        let b = io::read_one::<isize>();
        let i = an.lower_bound(&b);
        let ans = if i == 0 {
            (b - an[i]).abs()
        } else if i > n - 1 {
            (b - an[n - 1]).abs()
        } else {
            cmp::min((b - an[i]).abs(), (b - an[i - 1]).abs())
        };

        println!("{}", ans);
    }
}

/// BinarySearch
pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}
impl<T: Ord> BinarySearch<T> for [T] {
    /// returns the minimum index s.t. array[index] >= x
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();
        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                cmp::Ordering::Less => {
                    low = mid + 1;
                }
                _ => {
                    high = mid;
                }
            }
        }
        low
    }
    /// returns the minimum index s.t. array[index] > x
    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();
        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                cmp::Ordering::Greater => {
                    high = mid;
                }
                _ => {
                    low = mid + 1;
                }
            }
        }
        low
    }
}

#[allow(unused)]
mod io {
    use std::io;
    pub fn read<T>() -> Vec<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        buf.split_whitespace()
            .map(|s| s.trim().parse().unwrap())
            .collect()
    }
    pub fn read_one<T>() -> T
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        buf.trim().parse().unwrap()
    }
}
