use cargo_snippet::snippet;

#[snippet("FenwickTree")]
#[derive(Debug)]
struct FenwickTree {
    n: usize,
    bit: Vec<usize>,
}

#[snippet("FenwickTree")]
impl FenwickTree {
    fn new(n: usize) -> Self {
        FenwickTree {
            n,
            bit: vec![0; n + 1],
        }
    }

    fn from(v: &[usize]) -> Self {
        let n = v.len();
        let mut t = Self::new(n);
        for i in 0..n { t.add(i, v[i]); }

        t
    }

    // returns sum of [0, i)
    fn sum(&self, i: usize) -> usize {
        let mut sum = 0;
        let mut x = i;
        while x > 0 {
            x -= 1;
            sum += self.bit[x];
            x &= x + 1;
        }

        sum
    }

    fn add(&mut self, i: usize, v: usize) {
        let mut x = i;
        while x < self.n {
            self.bit[x] += v;
            x |= x + 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fenwick_tree() {
        let n = 10;
        let mut acc = vec![0; n + 1];
        let mut bit = FenwickTree::new(n);

        for i in 0..n {
            acc[i + 1] = acc[i] + i;
            bit.add(i, i);
        }

        for i in 0..n {
            assert_eq!(bit.sum(i), acc[i]);
        }
    }
}
