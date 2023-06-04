use ash::*;

use crate::prelude::vk_device::Device;

pub struct ShaderModule {
    shader_module: vk::ShaderModule,
    shader_stage_flags: vk::ShaderStageFlags,
}

impl ShaderModule {
    pub fn new(device: &Device, shader_code: &Vec<u8>, variant: &str) -> Self {
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

        let shader_stage_flags = match variant {
            "vert" | "Vertex" | "vs" => vk::ShaderStageFlags::VERTEX,
            "frag" | "Fragment" | "fs" => vk::ShaderStageFlags::FRAGMENT,
            "tesc" | "Tessellation_Control" | "tcs" => vk::ShaderStageFlags::TESSELLATION_CONTROL,
            "tese" | "Tessellation_Evaluation" | "tes" => vk::ShaderStageFlags::TESSELLATION_EVALUATION,
            "geom" | "Geometry" | "gs" => vk::ShaderStageFlags::GEOMETRY,
            "comp" | "Compute" | "cs" => vk::ShaderStageFlags::COMPUTE,
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
