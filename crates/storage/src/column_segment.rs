use types::DataTypeKind;

use crate::ColumnSegmentStatistics;

#[derive(Debug)]
pub struct ColumnSegment {
    data: Vec<u8>,
    dtype: DataTypeKind,
    stats: ColumnSegmentStatistics,
}

impl ColumnSegment {
    pub fn new(dtype: DataTypeKind) -> Self {
        Self {
            data: Vec::new(),
            dtype, 
            stats: ColumnSegmentStatistics::new(),
        }
    }

    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }

    pub fn push_val(&mut self, bytes: &[u8]) {
        self.stats.update(bytes, self.dtype);
        self.data.extend_from_slice(bytes);
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn as_f32_slice(&self) -> f32 {
        0.0
    }
}
