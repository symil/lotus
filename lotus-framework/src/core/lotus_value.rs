use std::mem::ManuallyDrop;

use serializable::Serializable;

use crate::{LotusArray, LotusBoolean, LotusNumber, LotusObject, LotusString};

pub struct LotusValue {
    ty: LotusValueType,
    data: LotusValueData
}

#[derive(Serializable)]
pub enum LotusValueType {
    Boolean,
    Number,
    String,
    Array,
    Object
}

pub union LotusValueData {
    boolean: ManuallyDrop<LotusBoolean>,
    number: ManuallyDrop<LotusNumber>,
    string: ManuallyDrop<LotusString>,
    array: ManuallyDrop<LotusArray>,
    object: ManuallyDrop<LotusObject>
}

impl LotusValue {
    pub fn boolean(value: LotusBoolean) -> Self {
        Self {
            ty: LotusValueType::Boolean,
            data: LotusValueData { boolean: ManuallyDrop::new(value) }
        }
    }

    pub fn number(value: LotusNumber) -> Self {
        Self {
            ty: LotusValueType::Number,
            data: LotusValueData { number: ManuallyDrop::new(value) }
        }
    }

    pub fn string(value: LotusString) -> Self {
        Self {
            ty: LotusValueType::String,
            data: LotusValueData { string: ManuallyDrop::new(value) }
        }
    }

    pub fn array(value: LotusArray) -> Self {
        Self {
            ty: LotusValueType::Array,
            data: LotusValueData { array: ManuallyDrop::new(value) }
        }
    }

    pub fn object(value: LotusObject) -> Self {
        Self {
            ty: LotusValueType::Object,
            data: LotusValueData { object: ManuallyDrop::new(value) }
        }
    }

    pub fn as_boolean(&self) -> &LotusBoolean {
        unsafe { &self.data.boolean }
    }

    pub fn as_number(&self) -> &LotusNumber {
        unsafe { &self.data.number }
    }

    pub fn as_string(&self) -> &LotusString {
        unsafe { &self.data.string }
    }

    pub fn as_array(&self) -> &LotusArray {
        unsafe { &self.data.array }
    }

    pub fn as_object(&self) -> &LotusObject {
        unsafe { &self.data.object }
    }
}

impl Serializable for LotusValue {
    fn write_bytes(value: &Self, buffer: &mut serializable::WriteBuffer) {
        LotusValueType::write_bytes(&value.ty, buffer);

        unsafe {
            match value.ty {
                LotusValueType::Boolean => LotusBoolean::write_bytes(&value.data.boolean, buffer),
                LotusValueType::Number => LotusNumber::write_bytes(&value.data.number, buffer),
                LotusValueType::String => LotusString::write_bytes(&value.data.string, buffer),
                LotusValueType::Array => LotusArray::write_bytes(&value.data.array, buffer),
                LotusValueType::Object => LotusObject::write_bytes(&value.data.object, buffer),
            }
        }
    }

    fn read_bytes(buffer: &mut serializable::ReadBuffer) -> Option<Self> {
        let ty = LotusValueType::read_bytes(buffer)?;
        let data = match ty {
            LotusValueType::Boolean => LotusValueData { boolean: ManuallyDrop::new(LotusBoolean::read_bytes(buffer)?) },
            LotusValueType::Number => LotusValueData { number: ManuallyDrop::new(LotusNumber::read_bytes(buffer)?) },
            LotusValueType::String => LotusValueData { string: ManuallyDrop::new(LotusString::read_bytes(buffer)?) },
            LotusValueType::Array => LotusValueData { array: ManuallyDrop::new(LotusArray::read_bytes(buffer)?) },
            LotusValueType::Object => LotusValueData { object: ManuallyDrop::new(LotusObject::read_bytes(buffer)?) },
        };

        Some(Self { ty, data })
    }
}

impl Drop for LotusValue {
    // TODO: the operation of determining which value must be dropped should be done at compile-time
    fn drop(&mut self) {
        unsafe {
            match self.ty {
                LotusValueType::Boolean => ManuallyDrop::drop(&mut self.data.boolean),
                LotusValueType::Number => ManuallyDrop::drop(&mut self.data.number),
                LotusValueType::String => ManuallyDrop::drop(&mut self.data.string),
                LotusValueType::Array => ManuallyDrop::drop(&mut self.data.array),
                LotusValueType::Object => ManuallyDrop::drop(&mut self.data.object),
            }
        }
    }
}