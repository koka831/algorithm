use std::io;


fn main() {
    let (n, m, d) = {
        let i = read::<isize>();
        (i[0], i[1], i[2])
    };

    /*
    let mut sum = m - 1;
    let mut comb = 2;
    loop {
        if comb < m {
            println!("comb: {}, m: {}", comb, m);
            let a = ((m/2) as f64).powf(comb as f64) as usize;
            let b = ((m/2 - comb) as f64).powf()
            println!("score {}: {}", m - 1, a);
            sum += a;
        } else {
            break;
        }
        comb += 1;
    }
    println!("{}", sum);
    println!("{}", sum / m);
    */
    let mut vec = Vec::new();
    for i in 1..n { for j in 1..m { for k in 1..d {
        let mut score = 0;
        if (i - j).abs() == 1 {
            score += 1;
        }
        if (j - k).abs() == 1 {
            score += 1;
        }
        vec.push(score);
    }}}
    println!("{:?}", vec);
    println!("{}", vec.into_iter().sum::<isize>());
}

fn permutation(n: usize, r: usize) -> usize {
    if n > r {
        n * permutation(n - 1, r)
    } else {
        n / r
    }
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
