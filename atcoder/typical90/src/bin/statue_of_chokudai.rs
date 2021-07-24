use std::f64::consts::PI;

fn main() {
    let t = io::read_one::<f64>();
    let (l, x, y) = {
        let i = io::read::<f64>();
        (i[0], i[1], i[2])
    };

    let q = io::read_one::<usize>();
    for _ in 0..q {
        let e = io::read_one::<f64>();
        let theta = e / t * 2f64 * PI;
        let y_e = -l / 2f64 * theta.sin();
        let z_e = l / 2f64 * (1f64 - theta.cos());
        let dist = ((y - y_e) * (y - y_e) + x * x).sqrt();
        let h = (z_e / dist).atan() * 180f64 / PI;
        println!("{}", h);
    }
}

#[allow(unused)]
mod io {
    use std::io;
    pub fn read<T>() -> Vec<T>
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
    pub fn read_one<T>() -> T
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        buf.trim().parse().unwrap()
    }
}
