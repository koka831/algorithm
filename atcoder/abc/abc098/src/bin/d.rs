use std::io;


fn main() {
    // 隣り合う数のうち, xorが全て1になるもののペア数
    // 0は挟んでもok
    let n = read_one::<usize>();
    let a = read::<usize>();

    let mut cnt = 0;
    for i in 0..(n - 1) {
        cnt += 1; // for a_i^a_i
        let mut j = i + 1;
        if a[i] ^ a[j] == a[i] + a[j] {
            println!("{} ^ {} = {}", a[i], a[j], a[i]^a[j]);
            cnt += 1;
        }

        let mut flg = true;
        while flg {
            if j < n - 1  { j += 1; } else { break; }

            if a[i] ^ a[j] == a[i..j].iter().sum::<usize>() {
                cnt += 1;
                println!("{} ^ {} = {}", a[i], a[j], a[i]^a[j]);
            }
        }
    }
    println!("{}", cnt + 1);
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
