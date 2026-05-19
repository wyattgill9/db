use bytes::Bytes;
use zerocopy::{FromBytes, Immutable};

use crate::DataTypeKind;

pub struct FlatVector {
    dtype: DataTypeKind,
    data: Bytes,
    count: u16,
}

impl FlatVector {
    pub fn new(dtype: DataTypeKind) -> Self {
        Self {
            dtype,
            data: Bytes::new(),
            count: 0,
        }
    }

    pub fn as_view<T: Immutable + FromBytes>(&self) -> FlatVectorView<'_, T> {
        unsafe {
            FlatVectorView {
                data: FromBytes::ref_from_bytes(&self.data).unwrap_unchecked(),
                count: self.count,
            }
        }
    }
}

pub struct FlatVectorView<'a, T> {
    data: &'a [T],
    count: u16,
}
