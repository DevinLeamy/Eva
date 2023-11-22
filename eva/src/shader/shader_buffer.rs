use encase::{internal::WriteInto, ArrayLength, ShaderSize, ShaderType};
use std::fmt::Debug;

use super::ShaderStruct;

#[derive(ShaderType, Debug, Default)]
pub struct ShaderBuffer<T: ShaderType + Debug + ShaderSize> {
    length: ArrayLength,
    #[size(runtime)]
    data: Vec<T>,
}

impl<T: ShaderType + Debug + ShaderSize> ShaderBuffer<T> {
    pub fn new() -> Self {
        ShaderBuffer {
            length: ArrayLength,
            data: Vec::new(),
        }
    }
}

impl<T: ShaderType + Debug + ShaderSize> ShaderBuffer<T> {
    pub fn extend<'a>(&mut self, data: Vec<T>) -> u32 {
        let offset = self.data.len();
        self.data.extend(data);
        offset as u32
    }

    pub fn push(&mut self, data: T) -> u32 {
        let offset = self.data.len();
        self.data.push(data);
        offset as u32
    }

    pub fn len(&self) -> u32 {
        self.data.len() as u32
    }
}

impl<T: ShaderType + Debug + ShaderSize + WriteInto> ShaderStruct for ShaderBuffer<T> {
    fn as_bytes(&self) -> Option<Vec<u8>> {
        let mut buffer = encase::StorageBuffer::new(Vec::new());
        buffer.write(self).ok()?;
        Some(buffer.into_inner())
    }
}
