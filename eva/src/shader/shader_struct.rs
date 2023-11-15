use encase::ShaderType;

pub trait ShaderStruct: ShaderType {
    fn as_bytes(&self) -> Option<Vec<u8>>;

    fn size() -> u64 {
        Self::min_size().into()
    }
}
