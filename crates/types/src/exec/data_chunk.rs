use crate::Vector;

pub struct DataChunk {
    chunks: Vec<Vector>,
}

impl Default for DataChunk {
    fn default() -> Self {
        Self::new()
    }
}

impl DataChunk {
    pub fn new() -> Self {
        Self { chunks: Vec::new() }
    }
}
