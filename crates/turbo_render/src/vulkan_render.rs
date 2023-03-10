#![allow(unused)]
use crate::vulkan_device::Device;
use ash::{self, vk};

pub struct Renderer {
    device: Device,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {
            device: Device::new(),
        }
    }

    pub fn draw_frame(&self) {}
}
