use std::io;


fn main() {
    let q = read_one::<usize>();
    for _ in 0..q {
        let _ = read_one::<usize>();
        let mut an = read::<usize>();
        an.sort();
        let mut flg = false;
        loop {
            if an.iter().all(|&a| a == 1) || an.len() == 0 { flg = true; break; }
            if an.len() == 4 {
                an.dedup();
                if an.len() == 2 || an.len() == 1 { flg = true; break; }
                else { break; }
            }

            let x = an.last().unwrap();
            let qn = prime_factor(*x);
            let mut cnt = 0;
            for q in qn.iter() {
                for i in 0..an.len() {
                    if q == &an[i] { an[i] = 1; cnt += 1; break; }
                }
            }
            // println!("cnt: {}, an: {:?}, qn: {:?}", cnt, an, qn);
            if cnt != qn.len() { break; }
        }

        if flg { println!("YES"); }
        else { println!("NO"); }
    }
}

fn divide(n: usize) -> Option<usize> {
    println!("{}", n);
    if n <= 1 { return None; }
    for i in 2..n + 1 {
        if n % i == 0 {
            return Some(i);
        }
    } 
    return None;
}

fn prime_factor(n: usize) -> Vec<usize> {
    let mut x = n;
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(x);
    while let Some(f) = divide(x) {
        x /= f;
        vec.push(f);
    }
    return vec;
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
