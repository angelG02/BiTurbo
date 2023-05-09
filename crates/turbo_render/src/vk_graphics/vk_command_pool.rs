use std::ptr;

use ash::vk;
use bevy_ecs::prelude::*;

use crate::prelude::vk_device::Device;

#[derive(Resource, Clone)]
pub struct CommandPool {
    cmd_pool: vk::CommandPool,
}

impl CommandPool {
    pub fn new(device: &Device) -> Self {
        let command_pool_create_info = vk::CommandPoolCreateInfo {
            s_type: vk::StructureType::COMMAND_POOL_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER,
            queue_family_index: device.get_queue_indices().graphics_family.unwrap(),
        };

        let cmd_pool = unsafe {
            device
                .get_device()
                .create_command_pool(&command_pool_create_info, None)
                .expect("Failed to create Command Pool.")
        };

        CommandPool { cmd_pool }
    }

    pub fn get_command_pool(&self) -> vk::CommandPool {
        self.cmd_pool
    }

    pub fn cleanup(&mut self, device: &Device) {
        unsafe {
            device
                .get_device()
                .destroy_command_pool(self.cmd_pool, None);
        }
    }
}
