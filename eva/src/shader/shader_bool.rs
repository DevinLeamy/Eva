pub type ShaderBool = u32;
pub const SHADER_TRUE: ShaderBool = 1;
pub const SHADER_FALSE: ShaderBool = 0;

pub trait IntoShaderBool {
    fn shader_bool(self) -> ShaderBool;
}

impl IntoShaderBool for bool {
    fn shader_bool(self) -> ShaderBool {
        if self {
            SHADER_TRUE
        } else {
            SHADER_FALSE
        }
    }
}
