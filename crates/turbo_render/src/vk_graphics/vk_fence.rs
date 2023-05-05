use std::{ptr, sync::Arc};

use ash::vk;

use crate::prelude::vk_device::Device;

#[derive(Clone)]
pub struct Fence {
    device: Arc<Device>,
    fence: vk::Fence,
}

impl Fence {
    pub fn new(device: Arc<Device>, signaled: bool) -> Self {
        let create_flags: vk::FenceCreateFlags = if signaled {
            vk::FenceCreateFlags::SIGNALED
        } else {
            vk::FenceCreateFlags::default()
        };

        let fence_create_info = vk::FenceCreateInfo {
            s_type: vk::StructureType::FENCE_CREATE_INFO,
            p_next: ptr::null(),
            flags: create_flags,
        };

        let fence = unsafe {
            device
                .get_device()
                .create_fence(&fence_create_info, None)
                .expect("Failed to create Semaphore Object.")
        };

        Fence { device, fence }
    }

    pub fn get_fence(&self) -> vk::Fence {
        self.fence
    }

    pub fn is_completed(&self) -> bool {
        unsafe {
            self.device
                .get_device()
                .get_fence_status(self.fence)
                .expect("Failed to get Fence status.")
        }
    }

    pub fn wait(&self) {
        unsafe {
            self.device
                .get_device()
                .wait_for_fences(&[self.fence], true, std::u64::MAX)
                .expect("Failed to wait for Fence.");
        }
    }

    pub fn reset(&self) {
        unsafe {
            self.device
                .get_device()
                .reset_fences(&[self.fence])
                .expect("Failed to reset Fence.");
        }
    }

    pub fn cleanup(&mut self) {
        unsafe {
            self.device.get_device().destroy_fence(self.fence, None);
        }
    }
}
