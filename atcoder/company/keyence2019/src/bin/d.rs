use std::io;
use std::cmp;


static MOD : usize = (1e9 as usize) + 7;

// (i, j) candidates: max(Ai, Bj)
// (i, j) : Ai == Bj
fn main() {
    let (n, m) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let an = read::<usize>();
    let bm = read::<usize>();

    // check dedup
    let mut a_c = an.clone();
    let mut b_c = bm.clone();
    a_c.sort();
    b_c.sort();
    a_c.dedup();
    b_c.dedup();

    if a_c.len() != an.len() || b_c.len() != bm.len() {
        println!("0");
        return;
    }

    let mut fixed = Vec::new();
    let mut fixed_n = vec![0; n];
    let mut fixed_m = vec![0; m];

    for i in 0..n { for j in 0..m {
        if an[i] == bm[j] {
            fixed.push((i, j));
            fixed_n[i] = 1;
            fixed_m[j] = 1;
        }
    }}

    let mut upper = Vec::new();

    for i in 0..n { for j in 0..m {
        if fixed.contains(&(i, j)) {
            upper.push(an[i]);
        } else {
            upper.push(cmp::min(an[i] - fixed_n[i], bm[j] - fixed_m[j]));
        }
    }}

    upper.sort();

    let mut ans: usize = 1;
    let mut prev = 0;
    let mut cnt: usize = 1;
    for i in 0..n * m {
        if i + 1 > upper[i] { println!("0"); return; }

        if upper[i] != prev {
            prev = upper[i];
            ans = ans * cnt % MOD;
            cnt = 1;
        } else { cnt += 1; }
    }

    println!("{}", ans);
}


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
