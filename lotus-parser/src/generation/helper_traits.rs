use super::Wat;

pub trait ToInt {
    fn to_i32(self) -> i32;
}

pub trait ToWat {
    fn to_wat(self) -> Wat;
}

impl ToInt for i32 {
    fn to_i32(self) -> i32 {
        self
    }
}

impl ToInt for usize {
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl ToWat for Wat {
    fn to_wat(self) -> Wat {
        self
    }
}

impl ToWat for String {
    fn to_wat(self) -> Wat {
        Wat::single(self)
    }
}

impl<'a> ToWat for &'a str {
    fn to_wat(self) -> Wat {
        Wat::single(self.to_string())
    }
}

impl ToWat for usize {
    fn to_wat(self) -> Wat {
        Wat::single(self.to_string())
    }
}

impl ToWat for i32 {
    fn to_wat(self) -> Wat {
        Wat::single(self.to_string())
    }
}

impl ToWat for f32 {
    fn to_wat(self) -> Wat {
        Wat::single(self.to_string())
    }
}