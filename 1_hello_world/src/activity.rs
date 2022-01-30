use std::fmt;

#[derive(Debug)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}!", self.real, self.imag)
    }
}
