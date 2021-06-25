use super::{read_buffer::ReadBuffer, serializable::Serializable};

macro_rules! make_primitive_types_serializable {
    ( $ ( $t:ty ), * ) => {
        $(
            impl Serializable for $t {
                fn write_bytes(value: &Self, bytes: &mut Vec<u8>) {
                    bytes.extend_from_slice(&value.to_le_bytes());
                }

                fn read_bytes(buffer: &mut ReadBuffer) -> Option<Self> {
                    const SIZE : usize = std::mem::size_of::<$t>();

                    match buffer.read(SIZE) {
                        None => None,
                        Some(bytes) => {
                            let mut arr : [u8; SIZE] = [0; SIZE];
                            arr.copy_from_slice(bytes);

                            Some(<$t>::from_le_bytes(arr))
                        }
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

            fn read_bytes(buffer: &mut ReadBuffer) -> Option<Self> {
                match <$target>::read_bytes(buffer) {
                    None => None,
                    Some(value) => Some(value as $src)
                }
            }
        }
    }
}

make_primitive_types_serializable!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);
make_size_type_serializable!(usize, u64);
make_size_type_serializable!(isize, i64);

impl Serializable for bool {
    fn write_bytes(value: &Self, bytes: &mut Vec<u8>) {
        bytes.push(match *value {
            true => 1,
            false => 0
        });
    }

    fn read_bytes(buffer: &mut ReadBuffer) -> Option<Self> {
        match u8::read_bytes(buffer) {
            None => None,
            Some(value) => Some({
                match value {
                    0 => false,
                    _ => true
                }
            })
        }
    }
}

impl Serializable for String {
    fn write_bytes(value: &Self, bytes: &mut Vec<u8>) {
        u16::write_bytes(&(value.len() as u16), bytes);
        bytes.extend_from_slice(value.as_bytes());
    }

    fn read_bytes(buffer: &mut ReadBuffer) -> Option<Self> {
        let length = match u16::read_bytes(buffer) {
            None => return None,
            Some(value) => value as usize
        };

        match buffer.read(length) {
            None => None,
            Some(bytes) => Some(String::from_utf8_lossy(bytes).to_string())
        }
    }
}

impl<T : Serializable> Serializable for Vec<T> {
    fn write_bytes(value: &Self, bytes: &mut Vec<u8>) {
        u32::write_bytes(&(value.len() as u32), bytes);
        for item in value {
            T::write_bytes(item, bytes);
        }
    }

    fn read_bytes(buffer: &mut ReadBuffer) -> Option<Self> {
        let count = match u32::read_bytes(buffer) {
            None => return None,
            Some(value) => value as usize
        };

        let mut result = Vec::with_capacity(count);

        for _i in 0..count {
            match T::read_bytes(buffer) {
                None => return None,
                Some(item) => {
                    result.push(item);
                }
            }
        }

        Some(result)
    }
}

impl<T : Serializable, const N : usize> Serializable for [T; N] {
    fn write_bytes(value: &Self, bytes: &mut Vec<u8>) {
        for item in value {
            T::write_bytes(item, bytes);
        }
    }

    fn read_bytes(buffer: &mut ReadBuffer) -> Option<Self> {
        let mut result : [T; N] = unsafe { std::mem::zeroed() };

        for i in 0..N {
            match T::read_bytes(buffer) {
                None => return None,
                Some(item) => {
                    result[i] = item;
                }
            }
        }

        Some(result)
    }
}

impl<T : Serializable> Serializable for Option<T> {
    fn write_bytes(value: &Self, bytes: &mut Vec<u8>) {
        match value {
            None => bytes.push(0),
            Some(x) => {
                bytes.push(1);
                T::write_bytes(x, bytes);
            }
        }
    }

    fn read_bytes(buffer: &mut ReadBuffer) -> Option<Self> {
        match u8::read_bytes(buffer) {
            None => return None,
            Some(value) => Some(match value {
                0 => None,
                1 => match T::read_bytes(buffer) {
                    None => return None,
                    Some(item) => Some(item)
                },
                _ => return None,
            }),
        }
    }
}

impl<T : Serializable, E : Serializable> Serializable for Result<T, E> {
    fn write_bytes(value: &Self, bytes: &mut Vec<u8>) {
        match value {
            Ok(v) => {
                bytes.push(0);
                T::write_bytes(v, bytes);
            },
            Err(e) => {
                bytes.push(1);
                E::write_bytes(e, bytes);
            }
        }
    }

    fn read_bytes(buffer: &mut ReadBuffer) -> Option<Self> {
        match u8::read_bytes(buffer) {
            None => return None,
            Some(header) => Some(match header {
                0 => match T::read_bytes(buffer) {
                    None => return None,
                    Some(value) => Ok(value)
                },
                1 => match E::read_bytes(buffer) {
                    None => return None,
                    Some(error) => Err(error)
                },
                _ => return None
            })
        }
    }
}