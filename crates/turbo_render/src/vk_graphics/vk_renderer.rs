#![allow(unused)]
use std::sync::{Arc, Mutex};

use ash::{self, vk};
use bevy_ecs::system::{NonSend, Query, Res, ResMut, SystemState};
use bevy_ecs::world::World;

use crate::prelude::vk_buffers::vk_command_buffer::CommandBuffer;
use crate::prelude::vk_buffers::vk_image::Image;
use crate::prelude::vk_command_pool::CommandPool;
use crate::prelude::vk_command_queue::CommandQueue;
use crate::prelude::vk_device::Device;
use crate::prelude::vk_pipeline::Pipeline;
use crate::prelude::vk_render_pass::RenderPass;
use crate::prelude::vk_swapchain::{SwapChain, MAX_FRAMES_IN_FLIGHT};

use turbo_app::prelude::{OnMainRender, OnShutdown, Plugin};
use turbo_window::window::Window;

pub struct VulkanRendererPlugin;

impl Plugin for VulkanRendererPlugin {
    fn build(&self, app: &mut turbo_app::prelude::App) {
        let mut system_state: SystemState<Option<NonSend<Window>>> =
            SystemState::new(&mut app.world);

        let window = system_state.get(&app.world).unwrap();

        let device = Device::new(window.get_glfw_window());

        let (window_width, window_height) = window.get_glfw_window().get_framebuffer_size();
        let max_sample_count = device.get_max_sample_count();

        let mut swapchain = SwapChain::new(
            Arc::new(device.clone()),
            window_width as u32,
            window_height as u32,
        );

        let mut render_image = Image::new(
            Arc::new(device.clone()),
            swapchain.get_extent().width,
            swapchain.get_extent().height,
            1,
            *swapchain.get_color_format(),
            max_sample_count,
            vk::ImageTiling::OPTIMAL,
            vk::ImageUsageFlags::TRANSIENT_ATTACHMENT | vk::ImageUsageFlags::COLOR_ATTACHMENT,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            &device.get_physical_device_mem_properties(),
        );

        let render_pass = RenderPass::new(
            Arc::new(device.clone()),
            *swapchain.get_color_format(),
            *swapchain.get_depth_format(),
            max_sample_count,
            vk::AttachmentLoadOp::CLEAR,
            vk::ImageLayout::UNDEFINED,
            vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        );

        swapchain.build_framebuffers(&render_pass, &mut render_image);

        let shaders = vec!["basic.vert", "basic.frag"];

        let pipeline = Pipeline::new(
            Arc::new(device.clone()),
            swapchain.get_extent(),
            &render_pass,
            shaders,
            vk::CullModeFlags::BACK,
            vk::TRUE,
        );

        let command_pool = CommandPool::new(Arc::new(device.clone()));
        let mut command_queue = CommandQueue::new(
            Arc::new(device.clone()),
            *device.get_graphics_queue(),
            Arc::new(command_pool.clone()),
        );

        app.insert_resource(device.clone())
            .insert_resource(swapchain.clone())
            .insert_resource(render_image.clone())
            .insert_resource(render_pass.clone())
            .insert_resource(pipeline.clone())
            .insert_resource(command_pool.clone())
            .insert_resource(command_queue)
            .add_systems(OnMainRender, (render_frame, || {}))
            .add_systems(OnShutdown, (cleanup, || {}));
    }
}

fn render_frame(world: &mut World) {
    let mut system_state: SystemState<(
        ResMut<Device>,
        ResMut<SwapChain>,
        ResMut<Pipeline>,
        ResMut<RenderPass>,
        ResMut<CommandQueue>,
    )> = SystemState::new(world);

    let (mut device, mut swapchain, mut pipeline, mut render_pass, mut command_queue) =
        system_state.get_mut(world);

    swapchain.next_image();

    let mut command_buffer = command_queue.get_command_buffer(swapchain.get_current_frame());
    command_buffer.reset();

    command_buffer.begin(vk::CommandBufferUsageFlags::SIMULTANEOUS_USE);

    command_buffer.set_viewport(swapchain.get_extent());

    command_buffer.begin_render_pass(&render_pass, &swapchain);

    command_buffer.bind_graphics_pipeline(Arc::new(pipeline.clone()));

    command_buffer.draw(3, 1, 0, 0);

    command_buffer.end_render_pass();

    command_buffer.end();

    let render_finished = swapchain.render_finished_semaphore();
    let image_available = swapchain.image_available_semaphore();

    let fence = command_queue.submit_command_buffer(
        device.as_ref(),
        Arc::new(Mutex::new(command_buffer.clone())),
        Some(&vec![&image_available]),
        Some(&vec![&render_finished]),
    );

    swapchain.as_mut().present(fence, &vec![&render_finished]);
}

fn cleanup(world: &mut World) {
    let mut system_state: SystemState<(
        ResMut<Device>,
        ResMut<SwapChain>,
        ResMut<Pipeline>,
        ResMut<RenderPass>,
        ResMut<Image>,
        ResMut<CommandPool>,
    )> = SystemState::new(world);

    let (
        mut device,
        mut swapchain,
        mut pipeline,
        mut render_pass,
        mut render_image,
        mut command_pool,
    ) = system_state.get_mut(world);

    device.wait_idle();

    command_pool.cleanup();
    render_image.cleanup();
    pipeline.as_mut().cleanup();
    render_pass.as_mut().cleanup();
    swapchain.as_mut().cleanup();
    device.as_mut().cleanup();
}
