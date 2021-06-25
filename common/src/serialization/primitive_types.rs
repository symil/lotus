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

macro_rules! make_primitive_types_serializable {
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

macro_rules! make_size_type_serializable {
    ($src:ty, $target:ty) => {
        impl Serializable for $src {
            fn write_bytes(value: &Self, bytes: &mut Vec<u8>) {
                <$target>::write_bytes(&(*value as $target), bytes);
            }

            fn read_bytes(bytes: &[u8]) -> Option<(Self, &[u8])> {
                match <$target>::read_bytes(bytes) {
                    None => None,
                    Some((value, bytes)) => Some((value as $src, bytes))
                }
            }
        }
    }
}

make_primitive_types_serializable!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);
make_size_type_serializable!(usize, u64);
make_size_type_serializable!(isize, i64);

impl Serializable for String {
    fn write_bytes(value: &Self, bytes: &mut Vec<u8>) {
        u16::write_bytes(&(value.len() as u16), bytes);
        bytes.extend_from_slice(value.as_bytes());
    }

    fn read_bytes(bytes: &[u8]) -> Option<(Self, &[u8])> {
        let (length, bytes) = match u16::read_bytes(bytes) {
            Some((len, bytes)) => (len as usize, bytes),
            None => return None
        };

        if bytes.len() < length {
            return None;
        }

        Some((String::from_utf8_lossy(&bytes[..length]).to_string(), &bytes[length..]))
    }
}

impl<T : Serializable> Serializable for Vec<T> {
    fn write_bytes(value: &Self, bytes: &mut Vec<u8>) {
        u32::write_bytes(&(value.len() as u32), bytes);
        for item in value {
            T::write_bytes(item, bytes);
        }
    }

    fn read_bytes(bytes: &[u8]) -> Option<(Self, &[u8])> {
        let (count, bytes) = match u32::read_bytes(bytes) {
            Some((len, bytes)) => (len as usize, bytes),
            None => return None
        };

        let mut result = Vec::with_capacity(count);
        let mut current_bytes = bytes;

        for _i in 0..count {
            match T::read_bytes(current_bytes) {
                None => return None,
                Some((item, new_bytes)) => {
                    result.push(item);
                    current_bytes = new_bytes;
                }
            }
        }

        Some((result, current_bytes))
    }
}