use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc, usize};

use super::{write_buffer::WriteBuffer, read_buffer::ReadBuffer, serializable::Serializable};

macro_rules! make_primitive_types_serializable {
    ( $ ( $t:ty ), * ) => {
        $(
            impl Serializable for $t {
                fn write_bytes(value: &Self, buffer: &mut WriteBuffer) {
                    buffer.write(&value.to_le_bytes());
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
            fn write_bytes(value: &Self, buffer: &mut WriteBuffer) {
                <$target>::write_bytes(&(*value as $target), buffer);
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

make_primitive_types_serializable!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);
make_size_type_serializable!(usize, u64);
make_size_type_serializable!(isize, i64);

impl Serializable for bool {
    fn write_bytes(value: &Self, buffer: &mut WriteBuffer) {
        buffer.write_byte(match *value {
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
    fn write_bytes(value: &Self, buffer: &mut WriteBuffer) {
        u16::write_bytes(&(value.len() as u16), buffer);
        buffer.write(value.as_bytes());
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
    fn write_bytes(value: &Self, buffer: &mut WriteBuffer) {
        u32::write_bytes(&(value.len() as u32), buffer);
        for item in value {
            T::write_bytes(item, buffer);
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
    fn write_bytes(value: &Self, buffer: &mut WriteBuffer) {
        for item in value {
            T::write_bytes(item, buffer);
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
    fn write_bytes(value: &Self, buffer: &mut WriteBuffer) {
        match value {
            None => buffer.write_byte(0),
            Some(value) => {
                buffer.write_byte(1);
                T::write_bytes(value, buffer);
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
    fn write_bytes(value: &Self, buffer: &mut WriteBuffer) {
        match value {
            Ok(v) => {
                buffer.write_byte(0);
                T::write_bytes(v, buffer);
            },
            Err(e) => {
                buffer.write_byte(1);
                E::write_bytes(e, buffer);
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

impl<T : Serializable> Serializable for RefCell<T> {
    fn write_bytes(value: &Self, buffer: &mut WriteBuffer) {
        T::write_bytes(unsafe { value.as_ptr().as_ref().unwrap() }, buffer);
    }

    fn read_bytes(buffer: &mut ReadBuffer) -> Option<Self> {
        match T::read_bytes(buffer) {
            Some(value) => Some(RefCell::new(value)),
            None => None
        }
    }
}

impl<T : Serializable + 'static> Serializable for Rc<T> {
    fn write_bytes(value: &Self, buffer: &mut WriteBuffer) {
        let addr = Rc::as_ptr(value) as usize;

        usize::write_bytes(&addr, buffer);

        if buffer.register(addr) {
            T::write_bytes(value, buffer);
        } else {
            panic!("attempt to serialize a cycle of Rc<{}>", std::any::type_name::<T>());
        }
    }

    fn read_bytes(buffer: &mut ReadBuffer) -> Option<Self> {
        match T::read_bytes(buffer) {
            Some(value) => Some(Rc::new(value)),
            None => None
        }
    }
}

impl<K : Serializable + Eq + Hash, V : Serializable> Serializable for HashMap<K, V> {
    fn write_bytes(value: &Self, buffer: &mut WriteBuffer) {
        usize::write_bytes(&value.len(), buffer);

        for (key, value) in value.iter() {
            K::write_bytes(key, buffer);
            V::write_bytes(value, buffer);
        }
    }

    fn read_bytes(buffer: &mut ReadBuffer) -> Option<Self> {
        let size = match usize::read_bytes(buffer) {
            Some(size) => size,
            None => return None
        };
        let mut hashmap = HashMap::with_capacity(size);

        for _i in 0..size {
            let key = match K::read_bytes(buffer) {
                Some(key) => key,
                None => return None
            };
            let value = match V::read_bytes(buffer) {
                Some(value) => value,
                None => return None
            };

            hashmap.insert(key, value);
        }

        Some(hashmap)
    }
}