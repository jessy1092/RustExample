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

pub struct List(pub Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }

            write!(f, "{}: {}", count, v)?;
        }

        write!(f, "]")
    }
}

pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RGM ({}, {}, {} 0x{:02X}{:02X}{:02X})",
            self.red, self.green, self.blue, self.red, self.green, self.blue,
        )
    }
}
