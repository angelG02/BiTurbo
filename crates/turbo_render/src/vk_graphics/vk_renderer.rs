#![allow(unused)]
use ash::{self, vk};

use crate::prelude::vk_device::Device;
use turbo_app::prelude::Plugin;
use turbo_window::window::Window;

pub struct VulkanRendererPlugin;

impl Plugin for VulkanRendererPlugin {
    fn build(&self, app: &mut turbo_app::prelude::App) {
        let window = app.world.get_non_send_resource::<Window>().unwrap();

        let device = Device::new(window.get_glfw_window());
        app.world.insert_resource::<Device>(device.clone());
    }
}
