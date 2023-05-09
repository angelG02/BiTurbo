use ash::{self, vk};
use bevy_ecs::system::{NonSend, ResMut, SystemState};
use bevy_ecs::world::World;

use crate::prelude::vk_buffers::vk_image::Image;
use crate::prelude::vk_command_pool::CommandPool;
use crate::prelude::vk_command_queue::CommandQueue;
use crate::prelude::vk_device::Device;
use crate::prelude::vk_pipeline::Pipeline;
use crate::prelude::vk_render_pass::RenderPass;
use crate::prelude::vk_swapchain::SwapChain;

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

        let mut swapchain = SwapChain::new(&device, window_width as u32, window_height as u32);

        let mut render_image = Image::new(
            &device,
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
            &device,
            *swapchain.get_color_format(),
            *swapchain.get_depth_format(),
            max_sample_count,
            vk::AttachmentLoadOp::CLEAR,
            vk::ImageLayout::UNDEFINED,
            vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        );

        swapchain.build_framebuffers(&device, &render_pass, &mut render_image);

        let shaders = vec!["basic.vert", "basic.frag"];

        let pipeline = Pipeline::new(
            &device,
            swapchain.get_extent(),
            &render_pass,
            shaders,
            vk::CullModeFlags::BACK,
            vk::TRUE,
        );

        let command_pool = CommandPool::new(&device);
        let command_queue = CommandQueue::new(&device, *device.get_graphics_queue(), &command_pool);

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

    let (device, mut swapchain, pipeline, render_pass, mut command_queue) =
        system_state.get_mut(world);

    swapchain.next_image(&device);

    let mut command_buffer = command_queue.get_command_buffer(swapchain.get_current_frame());
    command_buffer.reset(&device);

    command_buffer.begin(&device, vk::CommandBufferUsageFlags::SIMULTANEOUS_USE);

    command_buffer.set_viewport(&device, swapchain.get_extent());

    command_buffer.begin_render_pass(&device, &render_pass, &swapchain);

    command_buffer.bind_graphics_pipeline(&device, &pipeline);

    command_buffer.draw(&device, 3, 1, 0, 0);

    command_buffer.end_render_pass(&device);

    command_buffer.end(&device);

    let render_finished = swapchain.render_finished_semaphore();
    let image_available = swapchain.image_available_semaphore();

    let fence = command_queue.submit_command_buffer(
        device.as_ref(),
        &command_buffer,
        Some(&vec![&image_available]),
        Some(&vec![&render_finished]),
    );

    swapchain
        .as_mut()
        .present(&device, fence, &vec![&render_finished]);
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

    command_pool.cleanup(&device);
    render_image.cleanup(&device);
    pipeline.as_mut().cleanup(&device);
    render_pass.as_mut().cleanup(&device);
    swapchain.as_mut().cleanup(&device);
    device.as_mut().cleanup();
}
