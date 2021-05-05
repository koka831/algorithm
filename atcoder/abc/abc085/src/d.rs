use std::io;

#[allow(dead_code)]
fn read<T>() -> Vec<T>
where T:
    std::str::FromStr,
    T::Err: std::fmt::Debug {

    let stdin = io::stdin();
    let mut buf = String::new();

    stdin.read_line(&mut buf).unwrap();

    buf.split(" ")
        .map(|s| s.trim().parse().unwrap())
        .collect()
}


fn main() {

    let (n, mut hit) = {
        let v = read::<usize>();
        (v[0], v[1] as i64)
    };

    let mut vec = Vec::new();
    for _ in 0..n {
        let v = read::<i64>();
        vec.push((v[0], true));
        vec.push((v[1], false));
    }
    vec.sort();

    let mut ans = 0;

    for i in (0..vec.len()).rev() {
        let (atk, inf) = vec[i];
        if inf {
            ans += (hit + atk -1) / atk;
            break;
        } else {
            ans += 1;
            hit -= atk;
        }

        if hit <= 0 {
            break;
        }
    }
    println!("{}", ans);
}
