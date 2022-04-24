use cargo_snippet::snippet;

#[snippet("FloorSum")]
/// \sum_{i = 0}^{n - 1} floor((a * i + b) / m)
pub fn floor_sum(n: i64, m: i64, a: i64, b: i64) -> i64 {
    fn rec(n: i64, m: i64, mut a: i64, mut b: i64, mut acc: i64) -> i64 {
        if a >= m {
            let q = a / m;
            acc += (n - 1) * n / 2 * q;
            a -= q * m;
        }

        if b >= m {
            let q = b / m;
            acc += n * q;
            b -= q * m;
        }

        let y = (a * n + b) / m;
        let x = y * m - b;
        if y == 0 { return acc; }

        acc += (n - (x + a - 1) / a) * y;
        let mut sub_b = a - x % a;
        if sub_b >= a { sub_b -= a; }

        rec(y, a, m, sub_b, acc)
    }

    rec(n, m, a, b, 0)
}
