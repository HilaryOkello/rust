use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanNumber {
    fn from(num: u32) -> Self {
        if num == 0 {
            return RomanNumber(vec![Nulla]);
        }

        let mut n = num;
        let mut digits = Vec::new();

        while n >= 1000 {
            digits.push(M);
            n -= 1000;
        }
        if n >= 900 {
            digits.push(C);
            digits.push(M);
            n -= 900;
        }
        while n >= 500 {
            digits.push(D);
            n -= 500;
        }
        if n >= 400 {
            digits.push(C);
            digits.push(D);
            n -= 400;
        }
        while n >= 100 {
            digits.push(C);
            n -= 100;
        }
        if n >= 90 {
            digits.push(X);
            digits.push(C);
            n -= 90;
        }
        while n >= 50 {
            digits.push(L);
            n -= 50;
        }
        if n >= 40 {
            digits.push(X);
            digits.push(L);
            n -= 40;
        }
        while n >= 10 {
            digits.push(X);
            n -= 10;
        }
        if n == 9 {
            digits.push(I);
            digits.push(X);
            n -= 9;
        }
        while n >= 5 {
            digits.push(V);
            n -= 5;
        }
        if n == 4 {
            digits.push(I);
            digits.push(V);
            n -= 4;
        }
        while n >= 1 {
            digits.push(I);
            n -= 1;
        }

        RomanNumber(digits)
    }
}