#![allow(unused)]
use ash::{self, vk};

use crate::prelude::vk_device::Device;
use turbo_app::prelude::Plugin;

pub struct VulkanRenderer;

impl Plugin for VulkanRenderer {
    fn build(&self, app: &mut turbo_app::prelude::App) {
        let device = Device::new();
        app.world.insert_resource::<Device>(device);
    }
}
