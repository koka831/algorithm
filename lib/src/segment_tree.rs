use cargo_snippet::snippet;

/// SegmentTree
#[snippet("SegmentTree")]
pub struct SegmentTree<T, F> {
    // number of elements
    n: usize,
    value: Vec<T>,
    // identity element
    id: T,
    // functor
    f: F
}

#[snippet("SegmentTree")]
impl<T, F> SegmentTree<T, F>
where
    T: Clone,
    F: Fn(&T, &T) -> T {

    pub fn new(n: usize, id: T, f: F) -> Self {
        let mut capacity = 1;
        while capacity <= n { capacity *= 2; }

        SegmentTree {
            n: capacity,
            value: vec![id.clone(); 2 * capacity],
            id,
            f,
        }
    }

    // construct `SegmentTree` from an array
    pub fn from(v: &[T], id: T, f: F) -> Self {
        let n = v.len();
        let mut vs = vec![id.clone(); n];
        vs.extend_from_slice(v);

        for i in (1..n).rev() {
            vs[i] = f(&vs[i * 2], &vs[i * 2 + 1]);
        }

        SegmentTree {
            n,
            value: vs,
            id,
            f,
        }
    }

    #[inline]
    pub fn get(&self, i: usize) -> T {
        self.value[i + self.n].clone()
    }

    // update the i-th value with `x`
    pub fn update(&mut self, i: usize, x: T) {
        let mut i = i;
        i += self.n;
        self.value[i] = x;

        while { i /= 2; i > 0 } {
            self.value[i] = (self.f)(&self.value[i * 2], &self.value[i * 2 + 1]);
        }
    }

    // apply the F to the elements in [l, r).
    pub fn apply(&mut self, l: usize, r: usize) -> T {
        assert!(r < self.n);

        let (mut x, mut y) = (self.id.clone(), self.id.clone());
        let (mut l, mut r) = (l + self.n, r + self.n);

        while l < r {
            if l % 2 != 0 {
                x = (self.f)(&x, &self.value[l]);
                l += 1;
            }

            if r % 2 != 0 {
                r -= 1;
                y = (self.f)(&self.value[r], &y);
            }

            l /= 2;
            r /= 2;
        }

        (self.f)(&x, &y)
    }
}
