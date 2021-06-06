use cargo_snippet::snippet;

#[snippet("ModInt")]
#[derive(Copy, Clone, Debug, Eq)]
pub struct ModInt {
    pub v: usize,
    modulo: usize,
}

#[snippet("ModInt")]
impl ModInt {
    fn new(v: usize, modulo: usize) -> Self {
        ModInt { v, modulo }
    }

    #[inline]
    fn pow(self, power: usize) -> Self {
        if power == 0 { return ModInt { v: 1, ..self }; }
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

#[snippet("ModInt")]
impl PartialEq for ModInt {
    fn eq(&self, other: &Self) -> bool {
        self.v == other.v
    }
}

#[snippet("ModInt")]
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

#[snippet("ModInt")]
impl std::ops::AddAssign for ModInt {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

#[snippet("ModInt")]
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

#[snippet("ModInt")]
impl std::ops::SubAssign for ModInt {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

#[snippet("ModInt")]
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

#[snippet("ModInt")]
impl std::ops::MulAssign for ModInt {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

#[snippet("ModInt")]
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

#[snippet("ModInt")]
impl std::ops::DivAssign for ModInt {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

#[snippet("ModInt")]
macro_rules! modint_impl {
    ($t:tt) => {
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
    }
}

// todo apply it for isize
#[snippet("ModInt")]
modint_impl!(usize);

#[cfg(test)]
/// verify: https://atcoder.jp/contests/abc156/submissions/23260325
mod tests {
    use super::*;

    const MOD: usize = 10usize.pow(9) + 7;

    #[test]
    fn test_modint_add_with_modint() {
        let mint = ModInt::new(1, MOD);
        let rhs = ModInt::new(MOD, MOD);

        assert_eq!((mint + rhs).v, 1);
    }

    #[test]
    fn test_modint_add_assign_with_modint() {
        let mut mint = ModInt::new(1, MOD);
        let rhs = ModInt::new(MOD, MOD);

        mint += rhs;
        assert_eq!(mint.v, 1);
    }

    #[test]
    fn test_modint_add_with_usize() {
        let mint = ModInt::new(1, MOD);
        let rhs = MOD;

        assert_eq!((mint + rhs).v, 1);
    }

    #[test]
    fn test_modint_mul_assign_with_usize() {
        let mut mint = ModInt::new(5, 9);
        mint *= 5;
        assert_eq!(mint.v, 25 % 9);
    }

    #[test]
    fn test_modint_pow() {
        let mint = ModInt::new(2, MOD);
        assert_eq!(mint.pow(2).v, 4);
        assert_eq!(mint.pow(1000000000).v, 140625001);
    }
}
