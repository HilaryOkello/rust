#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        let (mut temp_r, mut temp_g, mut temp_b, mut temp_a) = (self.r, self.g, self.b, self.a);

        match first {
            r if r == self.r => temp_r = second,
            g if g == self.g => temp_g = second,
            b if b == self.b => temp_b = second,
            a if a == self.a => temp_a = second,
            _ => {}
        }

        match second {
            r if r == self.r => temp_r = first,
            g if g == self.g => temp_g = first,
            b if b == self.b => temp_b = first,
            a if a == self.a => temp_a = first,
            _ => {}
        }

        Color {
            r: temp_r,
            g: temp_g,
            b: temp_b,
            a: temp_a,
        }
    }
}