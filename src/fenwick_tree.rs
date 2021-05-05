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
            bit: vec![0; n],
        }
    }

    fn from(v: &[usize]) -> Self {
        let mut bit = Vec::new();
        bit.extend_from_slice(v);

        FenwickTree {
            n: v.len(),
            bit,
        }
    }

    // returns sum of [0, i)
    fn sum(&self, i: usize) -> usize {
        let mut sum = 0;
        let mut i = i;
        while i > 0 {
            i -= 1;
            sum += self.bit[i];
            i &= i + 1;
        }

        sum
    }

    fn add(&mut self, i: usize, v: usize) {
        let mut i = i;
        while i < self.n {
            self.bit[i] += v;
            i |= i + 1;
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
