use super::{Vasm, VirtualInstruction, Wat};

pub trait ToInt {
    fn to_i32(self) -> i32;
}

pub trait ToWat {
    fn to_wat(self) -> Wat;
}

pub trait ToVasm {
    fn to_vasm(self) -> Vasm;
}

pub trait ToWatVec {
    fn to_wat_vec(self) -> Vec<Wat>;
}


impl ToInt for i32 {
    fn to_i32(self) -> i32 {
        self
    }
}

impl ToInt for u32 {
    fn to_i32(self) -> i32 {
        self as i32
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

impl<'a> ToWat for &'a String {
    fn to_wat(self) -> Wat {
        Wat::single(self.as_str().to_string())
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

impl ToWat for &usize {
    fn to_wat(self) -> Wat {
        Wat::single(self.to_string())
    }
}

impl ToWat for u32 {
    fn to_wat(self) -> Wat {
        Wat::single(self.to_string())
    }
}

impl ToWat for u64 {
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

impl ToWatVec for Vec<Wat> {
    fn to_wat_vec(self) -> Vec<Wat> {
        self
    }
}

impl<T : ToWat> ToWatVec for T {
    fn to_wat_vec(self) -> Vec<Wat> {
        vec![self.to_wat()]
    }
}

impl ToVasm for Vasm {
    fn to_vasm(self) -> Vasm {
        self
    }
}

impl ToVasm for VirtualInstruction {
    fn to_vasm(self) -> Vasm {
        Vasm::undefined(vec![], vec![self])
    }
}

impl ToVasm for Vec<VirtualInstruction> {
    fn to_vasm(self) -> Vasm {
        Vasm::undefined(vec![], self)
    }
}