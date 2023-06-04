use assets_manager::{loader, Asset, BoxedError};
use shaderc;
use std::borrow::Cow;

pub struct Shader {
    pub variant: String,
    pub raw_code: String,
    pub byte_code: Vec<u8>,
}

pub struct ShaderLoader;

impl loader::Loader<Shader> for ShaderLoader {
    fn load(content: Cow<[u8]>, extension: &str) -> Result<Shader, BoxedError> {
        match extension {
            "vert" => {
                let raw_code = std::str::from_utf8(&content)?;
                let compiler = shaderc::Compiler::new().unwrap();
                let byte_code = compiler
                    .compile_into_spirv(
                        raw_code,
                        shaderc::ShaderKind::Vertex,
                        "vertexShader",
                        "main",
                        None,
                    )
                    .expect("Failed to compile vertex shader!");

                Ok(Shader {
                    variant: "Vertex".into(),
                    raw_code: std::str::from_utf8(&content)?.into(),
                    byte_code: byte_code.as_binary_u8().to_vec(),
                })
            }
            "frag" => {
                let raw_code = std::str::from_utf8(&content)?;
                let compiler = shaderc::Compiler::new().unwrap();
                let byte_code = compiler
                    .compile_into_spirv(
                        raw_code,
                        shaderc::ShaderKind::Fragment,
                        "fragmentShader",
                        "main",
                        None,
                    )
                    .expect("Failed to compile fragment shader!");

                Ok(Shader {
                    variant: "Fragment".into(),
                    raw_code: std::str::from_utf8(&content)?.into(),
                    byte_code: byte_code.as_binary_u8().to_vec(),
                })
            }
            _ => Err("Shader variant not supported... for now!".into()),
        }
    }
}

impl Asset for Shader {
    const EXTENSIONS: &'static [&'static str] = &["vert", "frag", "hlsl", "glsl"];
    type Loader = ShaderLoader;
}
