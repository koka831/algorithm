const MOD: usize = 1_000_000_007;

fn main() {
    let (n, a, b) = {
        let i = io::read::<usize>();
        (i[0], i[1], i[2])
    };

    let mint = ModInt::new(2, MOD);
    let ans = mint.pow(n) - 1 - count(n, a) - count(n, b);

    println!("{}", ans.v);
}

fn count(n: usize, r: usize) -> usize {
    let mut res = ModInt::new(1, MOD);
    for i in 0..r { res *= n - i; }
    for i in 1..=r { res /= i; }

    res.v
}

#[derive(Copy, Clone, Debug, Eq)]
pub struct ModInt {
    pub v: usize,
    modulo: usize,
}
impl ModInt {
    fn new(v: usize, modulo: usize) -> Self {
        ModInt { v, modulo }
    }
    #[inline]
    fn pow(self, power: usize) -> Self {
        if power == 0 {
            return ModInt { v: 1, ..self };
        }
        if power % 2 == 0 {
            let h = self.pow(power / 2);
            h * h
        } else {
            self * self.pow(power - 1)
        }
    }
    #[inline]
    fn inverse(self) -> Self {
        self.pow(self.modulo - 2)
    }
}
impl PartialEq for ModInt {
    fn eq(&self, other: &Self) -> bool {
        self.v == other.v
    }
}
impl std::ops::Add for ModInt {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        ModInt {
            v: (self.v + rhs.v) % self.modulo,
            ..self
        }
    }
}
impl std::ops::AddAssign for ModInt {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl std::ops::Sub for ModInt {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        ModInt {
            v: (self.v + self.modulo - rhs.v) % self.modulo,
            ..self
        }
    }
}
impl std::ops::SubAssign for ModInt {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl std::ops::Mul for ModInt {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        ModInt {
            v: (self.v * rhs.v) % self.modulo,
            ..self
        }
    }
}
impl std::ops::MulAssign for ModInt {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
impl std::ops::Div for ModInt {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        ModInt {
            v: (self.v * rhs.inverse().v) % self.modulo,
            ..self
        }
    }
}
impl std::ops::DivAssign for ModInt {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}
macro_rules! modint_impl {
    ($ t : tt ) => {
        impl PartialEq<$t> for ModInt {
            fn eq(&self, other: &$t) -> bool {
                self.v == *other
            }
        }
        impl std::ops::Add<$t> for ModInt {
            type Output = Self;
            #[inline]
            fn add(self, rhs: $t) -> Self::Output {
                ModInt {
                    v: (self.v + rhs) % self.modulo,
                    ..self
                }
            }
        }
        impl std::ops::AddAssign<$t> for ModInt {
            #[inline]
            fn add_assign(&mut self, rhs: $t) {
                *self = *self + rhs;
            }
        }
        impl std::ops::Sub<$t> for ModInt {
            type Output = Self;
            #[inline]
            fn sub(self, rhs: $t) -> Self::Output {
                ModInt {
                    v: (self.v + self.modulo - rhs) % self.modulo,
                    ..self
                }
            }
        }
        impl std::ops::SubAssign<$t> for ModInt {
            #[inline]
            fn sub_assign(&mut self, rhs: $t) {
                *self = *self - rhs;
            }
        }
        impl std::ops::Mul<$t> for ModInt {
            type Output = Self;
            #[inline]
            fn mul(self, rhs: $t) -> Self::Output {
                ModInt {
                    v: (self.v * rhs) % self.modulo,
                    ..self
                }
            }
        }
        impl std::ops::MulAssign<$t> for ModInt {
            #[inline]
            fn mul_assign(&mut self, rhs: $t) {
                *self = *self * rhs;
            }
        }
        impl std::ops::Div<$t> for ModInt {
            type Output = Self;
            #[inline]
            fn div(self, rhs: $t) -> Self::Output {
                let rhs = ModInt::new(rhs, self.modulo);
                ModInt {
                    v: (self.v * rhs.inverse().v) % self.modulo,
                    ..self
                }
            }
        }
        impl std::ops::DivAssign<$t> for ModInt {
            fn div_assign(&mut self, rhs: $t) {
                *self = *self / rhs;
            }
        }
    };
}

modint_impl!(usize);

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
