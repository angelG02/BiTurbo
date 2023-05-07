#![allow(unused)]
use std::sync::Arc;

use ash::{self, vk};
use bevy_ecs::system::{NonSend, Query, Res, ResMut, SystemState};
use bevy_ecs::world::World;

use crate::prelude::vk_device::Device;
use crate::prelude::vk_pipeline::Pipeline;
use crate::prelude::vk_render_pass::RenderPass;
use crate::prelude::vk_swapchain::SwapChain;
use turbo_app::prelude::{OnShutdown, Plugin};
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

        let max_sample_count = device.get_max_sample_count();

        let render_pass = RenderPass::new(
            Arc::new(device.clone()),
            *swapchain.get_color_format(),
            *swapchain.get_depth_format(),
            max_sample_count,
            vk::AttachmentLoadOp::CLEAR,
            vk::ImageLayout::UNDEFINED,
            vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        );

        let shaders = vec!["basic.vert", "basic.frag"];

        let pipeline = Pipeline::new(
            Arc::new(device.clone()),
            swapchain.get_extent(),
            &render_pass,
            shaders,
            vk::CullModeFlags::BACK,
            vk::TRUE,
        );

        app.insert_resource(device.clone())
            .insert_resource(swapchain.clone())
            .insert_resource(render_pass.clone())
            .insert_resource(pipeline.clone())
            .add_systems(OnShutdown, (cleanup, || {}));
    }
}

fn cleanup(world: &mut World) {
    let mut system_state: SystemState<(
        ResMut<Device>,
        ResMut<SwapChain>,
        ResMut<Pipeline>,
        ResMut<RenderPass>,
    )> = SystemState::new(world);

    let (mut device, mut swapchain, mut pipeline, mut render_pass) = system_state.get_mut(world);

    pipeline.as_mut().cleanup();
    render_pass.as_mut().cleanup();
    swapchain.as_mut().cleanup();
    device.as_mut().cleanup();
}
