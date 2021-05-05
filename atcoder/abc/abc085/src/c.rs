use std::io;

#[allow(dead_code)]
fn read<T>() -> Vec<T>
where T:
    std::str::FromStr,
    T::Err: std::fmt::Debug {

    let stdin = io::stdin();
    let mut buf = String::new();

    stdin.read_line(&mut buf).unwrap();
    buf.split(' ')
        .map(|s| s.trim().parse().unwrap())
        .collect()
}


fn main() {
    let (n, v) = {
        let input = read::<usize>();
        (input[0], input[1])
    };

    for j in 0..2001 {
        if j > n || 10_000 * j > v {
            continue;
        }

        for k in 0..4001 {
            if n < j + k {
                continue;
            }

            let sum = 10_000 * j + 5000 * k + (n - j - k) * 1000;
            if sum == v {
                println!("{} {} {}", j, k, n - j - k);
                return;
            }
        }
    }

    println!("-1 -1 -1");
}
