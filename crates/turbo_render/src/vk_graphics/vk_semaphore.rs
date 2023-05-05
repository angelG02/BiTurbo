use std::{ptr, sync::Arc};

use ash::vk;

use crate::prelude::*;

#[derive(Clone)]
pub struct Semaphore {
    device: Arc<vk_device::Device>,
    semaphore: vk::Semaphore,
}

impl Semaphore {
    pub fn new(device: Arc<vk_device::Device>) -> Self {
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

        Semaphore { device, semaphore }
    }

    pub fn get_semaphore(&self) -> &vk::Semaphore {
        &self.semaphore
    }
}

// This needs to be done in cleanup (onTerminate?) and not on Drop of the object
impl Drop for Semaphore {
    fn drop(&mut self) {
        unsafe {
            self.device
                .get_device()
                .destroy_semaphore(self.semaphore, None);
        }
    }
}
