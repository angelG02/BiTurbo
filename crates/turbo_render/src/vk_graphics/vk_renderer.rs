#![allow(unused)]
use std::sync::Arc;

use ash::{self, vk};
use bevy_ecs::system::{NonSend, Query, Res, SystemState};

use crate::prelude::vk_device::Device;
use crate::prelude::vk_swapchain::SwapChain;
use turbo_app::prelude::Plugin;
use turbo_window::window::Window;

pub struct VulkanRendererPlugin;

impl Plugin for VulkanRendererPlugin {
    fn build(&self, app: &mut turbo_app::prelude::App) {
        let mut system_state: SystemState<Option<NonSend<Window>>> =
            SystemState::new(&mut app.world);

        let window = system_state.get(&app.world).unwrap();

        let device = Device::new(window.get_glfw_window());

        let (window_width, window_height) = window.get_glfw_window().get_framebuffer_size();
        let swapchain = SwapChain::new(
            Arc::new(device.clone()),
            window_width as u32,
            window_height as u32,
        );
        app.insert_resource(device.clone())
            .insert_resource(swapchain.clone());
    }
}
