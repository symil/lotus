use super::serializable::Serializable;

impl Serializable for bool {
    fn write_bytes(value: &Self, bytes: &mut Vec<u8>) {
        bytes.push(match *value {
            true => 1,
            false => 0
        });
    }

    fn read_bytes(bytes: &[u8]) -> Option<(Self, &[u8])> {
        if bytes.len() < 1 {
            None
        } else {
            let value = match bytes[0] {
                0 => false,
                _ => true
            };

            Some((value, &bytes[1..]))
        }
    }
}

macro_rules! make_primitive_type_serializable {
    ( $ ( $t:ty ), * ) => {
        $(
            impl Serializable for $t {
                fn write_bytes(value: &Self, bytes: &mut Vec<u8>) {
                    bytes.extend_from_slice(&value.to_le_bytes());
                }

                fn read_bytes(bytes: &[u8]) -> Option<(Self, &[u8])> {
                    const SIZE : usize = std::mem::size_of::<$t>();

                    if bytes.len() < SIZE {
                        None
                    } else {
                        let mut arr : [u8; SIZE] = [0; SIZE];
                        arr.copy_from_slice(&bytes[..SIZE]);

                        Some((<$t>::from_le_bytes(arr), &bytes[SIZE..]))
                    }
                }
            }
        )*
    }
}

make_primitive_type_serializable!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);