use std::fmt;

pub fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (a, b) = pair;

    (b, a)
}

#[derive(Debug)]
pub struct Matrix(pub f32, pub f32, pub f32, pub f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "( {} {} )", self.0, self.1)?;
        writeln!(f, "( {} {} )", self.2, self.3)
    }
}

pub fn transpose(m: Matrix) -> Matrix {
    let Matrix(a, b, c, d) = m;

    Matrix(a, c, b, d)
}
