use ash::*;

use crate::prelude::vk_device::Device;
use turbo_core::util::get_binary_blob;

pub struct ShaderModule {
    shader_module: vk::ShaderModule,
    shader_stage_flags: vk::ShaderStageFlags,
}

impl ShaderModule {
    pub fn new(device: &Device, name: &str) -> Self {
        // TODO: use resource manager for shaders
        let shader_code = get_binary_blob(
            format!("C:/Projects/Hustle/bi_turbo_v0/sandbox/assets/builtin/shaders/bin/{name}.spv")
                .as_str(),
        );

        let create_info = vk::ShaderModuleCreateInfo {
            s_type: vk::StructureType::SHADER_MODULE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: vk::ShaderModuleCreateFlags::empty(),
            code_size: shader_code.len(),
            p_code: shader_code.as_ptr() as *const u32,
        };

        let shader_module = unsafe {
            device
                .get_device()
                .create_shader_module(&create_info, None)
                .expect("Failed to create Shader Module!")
        };

        let shader_stage_flags = match std::path::Path::new(&name).extension()
                                                        .expect("Failed to get shader type from file extension").to_str().unwrap() {
            "vert" | "vertex" | "vs" => vk::ShaderStageFlags::VERTEX,
            "frag" | "fragment" | "fs" => vk::ShaderStageFlags::FRAGMENT,
            "tesc" | "tessellation_control" | "tcs" => vk::ShaderStageFlags::TESSELLATION_CONTROL,
            "tese" | "tessellation_evaluation" | "tes" => vk::ShaderStageFlags::TESSELLATION_EVALUATION,
            "geom" | "geometry" | "gs" => vk::ShaderStageFlags::GEOMETRY,
            "comp" | "compute" | "cs" => vk::ShaderStageFlags::COMPUTE,
            extension => panic!("Failed to get shader type from file extension, unable to recognize \"{extension}\".")
        };

        ShaderModule {
            shader_module,
            shader_stage_flags,
        }
    }

    pub fn get_module(&self) -> &vk::ShaderModule {
        &self.shader_module
    }

    pub fn get_stage_flags(&self) -> &vk::ShaderStageFlags {
        &self.shader_stage_flags
    }

    pub fn cleanup(&mut self, device: &Device) {
        unsafe {
            device
                .get_device()
                .destroy_shader_module(self.shader_module, None);
        }
    }
}
