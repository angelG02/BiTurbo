use std::ptr;

use ash::vk;

use crate::prelude::vk_device::Device;

#[derive(Clone)]
pub struct Semaphore {
    semaphore: vk::Semaphore,
}

impl Semaphore {
    pub fn new(device: &Device) -> Self {
        let semaphore_create_info = vk::SemaphoreCreateInfo {
            s_type: vk::StructureType::SEMAPHORE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::SemaphoreCreateFlags::empty(),
        };

        let semaphore = unsafe {
            device
                .get_device()
                .create_semaphore(&semaphore_create_info, None)
                .expect("Failed to create Semaphore Object.")
        };

        Semaphore { semaphore }
    }

    pub fn get_semaphore(&self) -> &vk::Semaphore {
        &self.semaphore
    }

    pub fn cleanup(&mut self, device: &Device) {
        unsafe {
            device.get_device().destroy_semaphore(self.semaphore, None);
        }
    }
}
