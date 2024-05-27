use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub enum CompressionType {
    None,
    Lz4Fast(u32),
    Lz4(u32),
}

impl Default for CompressionType {
    fn default() -> Self {
        Self::Lz4Fast(8)
    }
}

impl CompressionType {
    pub fn compress(&self, data: &[u8]) -> Vec<u8> {
        if data.is_empty() {
            return vec![];
        }

        match self {
            CompressionType::None => data.to_vec(),
            CompressionType::Lz4Fast(speed) => lz4::block::compress(
                data,
                Some(lz4::block::CompressionMode::FAST(*speed as i32)),
                true,
            )
            .expect("Compression should work"),
            CompressionType::Lz4(compression) => lz4::block::compress(
                data,
                Some(lz4::block::CompressionMode::HIGHCOMPRESSION(
                    *compression as i32,
                )),
                true,
            )
            .expect("compression should work"),
        }
    }
}
