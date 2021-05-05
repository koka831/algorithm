use std::io;

fn main() {
    const MOD: usize = 1_000_000_007;
    let (n, k) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let mut an = read::<isize>();
    an.sort();

    let m = Modulo::new(n, MOD);
    let mut ans: isize = 0;
    for i in 0..n {
        if i >= k - 1 {
            ans += an[i] * (m.comb(i, k - 1) % MOD) as isize;
        }

        if n - i >= k {
            ans -= an[i] * (m.comb(n - i - 1, k - 1) % MOD) as isize;
        }
    }
    println!("{}", ans % MOD as isize);
}

pub struct Modulo {
    fact: Vec<usize>,
    inv_fact: Vec<usize>,
    modulo: usize,
}

impl Modulo {
    pub fn new(n: usize, modulo: usize) -> Self {
        let mut fact = vec![0; n + 1];
        let mut inv = vec![0; n + 1];
        let mut inv_fact = vec![0; n + 1];
        inv[1] = 1;
        for i in 2..n + 1 {
            inv[i] = inv[modulo % i] * (modulo - modulo / i) % modulo;
        }
        fact[0] = 1;
        inv_fact[0] = 1;
        for i in 0..n {
            fact[i + 1] = fact[i] * (i + 1) % modulo;
        }
        for i in 0..n {
            inv_fact[i + 1] = inv_fact[i] * inv[i + 1] % modulo;
        }
        Modulo {
            fact: fact,
            inv_fact: inv_fact,
            modulo: modulo,
        }
    }

    pub fn comb(&self, n: usize, r: usize) -> usize {
        if n < r {
            return 0;
        }
        self.fact[n] * self.inv_fact[r] % self.modulo * self.inv_fact[n - r] % self.modulo
    }
}

#[allow(dead_code)]
fn read<T>() -> Vec<T>
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

#[allow(dead_code)]
fn read_one<T>() -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}
