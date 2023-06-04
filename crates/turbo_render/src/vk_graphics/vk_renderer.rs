use ash::{self, vk};
use bevy_ecs::schedule::IntoSystemConfigs;
use bevy_ecs::system::{Res, ResMut, SystemState};
use bevy_ecs::world::World;

use turbo_core::event::Event;
use turbo_core::trace::info;

use crate::prelude::vk_command_pool::CommandPool;
use crate::prelude::vk_command_queue::CommandQueue;
use crate::prelude::vk_device::Device;
use crate::prelude::vk_pipeline::Pipeline;
use crate::prelude::vk_swapchain::SwapChain;

use turbo_app::prelude::{OnMainRender, OnShutdown, Plugin};
use turbo_window::window::Window;

use crate::shader::Shader;
use assets_manager::AssetCache;

pub struct VulkanRendererPlugin;

impl Plugin for VulkanRendererPlugin {
    fn build(&self, app: &mut turbo_app::prelude::App) {
        let mut window = app.world.get_non_send_resource_mut::<Window>().unwrap();
        let extent = window.get_extent();

        if extent.0 == 0 || extent.1 == 0 || !window.get_glfw_window().is_focused() {
            window.get_glfw_window_mut().glfw.wait_events();
        }

        let device = Device::new(window.get_glfw_window());

        let (window_width, window_height) = window.get_glfw_window().get_framebuffer_size();

        let mut swapchain = SwapChain::new(&device, window_width as u32, window_height as u32);

        swapchain.build_framebuffers(&device);

        let cache = app.world.get_resource::<AssetCache>().unwrap();

        let vert_shader = cache
            .load::<Shader>("builtin/shaders/src/basicVert")
            .expect("Could not load basic vertex shader!");

        let frag_shader = cache
            .load::<Shader>("builtin/shaders/src/basicFrag")
            .expect("Could not load basic vertex shader!");

        let pipeline = Pipeline::new(
            &device,
            swapchain.get_extent(),
            &swapchain.get_render_pass(),
            vec![vert_shader, frag_shader],
            vk::CullModeFlags::BACK,
            vk::TRUE,
        );

        let command_pool = CommandPool::new(&device);
        let command_queue = CommandQueue::new(&device, *device.get_graphics_queue(), &command_pool);

        app.insert_resource(device)
            .insert_resource(swapchain)
            .insert_resource(pipeline)
            .insert_resource(command_pool)
            .insert_resource(command_queue)
            .add_systems(
                OnMainRender,
                (recreate_swapchain, recreate_pipeline, render_frame).chain(),
            )
            .add_systems(OnShutdown, (cleanup, || {}));
    }
}

fn render_frame(world: &mut World) {
    let mut system_state: SystemState<(
        ResMut<Device>,
        ResMut<SwapChain>,
        ResMut<Pipeline>,
        ResMut<CommandQueue>,
    )> = SystemState::new(world);

    let (device, mut swapchain, pipeline, mut command_queue) = system_state.get_mut(world);

    swapchain.next_image(&device);

    let mut command_buffer = command_queue.get_command_buffer(swapchain.get_current_frame());
    command_buffer.reset(&device);

    command_buffer.begin(&device, vk::CommandBufferUsageFlags::SIMULTANEOUS_USE);

    command_buffer.set_viewport(&device, swapchain.get_extent());

    command_buffer.begin_render_pass(&device, &swapchain);

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

fn recreate_swapchain(world: &mut World) {
    let mut extent = (0, 0);

    let event = world.get_resource_mut::<Event>();
    if let Some(mut event) = event {
        if let Event::WindowResize(width, height) = *event {
            extent.0 = width as u32;
            extent.1 = height as u32;

            info!("{:?}", extent);
            *event = Event::Handled;
        } else {
            return;
        }
    } else {
        return;
    }
    let mut system_state: SystemState<(ResMut<Device>, ResMut<SwapChain>)> =
        SystemState::new(world);

    let (device, mut swapchain) = system_state.get_mut(world);

    device.wait_idle();

    swapchain.cleanup(&device);

    //SwapChain::new(device, width, height, Some(old_swapchain))

    let new_swapchain = SwapChain::recreate_swapchain(&device, extent.0, extent.1);

    world.insert_resource(new_swapchain);
}

fn recreate_pipeline(world: &mut World) {
    let mut system_state: SystemState<(
        Res<Device>,
        Res<SwapChain>,
        Res<AssetCache>,
        ResMut<Pipeline>,
        Option<ResMut<Event>>,
    )> = SystemState::new(world);

    let (device, swapchain, cache, mut pipeline, event) = system_state.get_mut(world);

    if let Some(mut event) = event {
        if let Event::KeyPressed(glfw::Key::R, 0) = *event {
            *event = Event::Handled;
        } else {
            return;
        }
    } else {
        return;
    }
    device.wait_idle();
    pipeline.cleanup(&device);
    let vert_shader = cache
        .get_cached::<Shader>("builtin/shaders/src/basicVert")
        .expect("Could not load basic vertex shader!");

    let frag_shader = cache
        .get_cached::<Shader>("builtin/shaders/src/basicFrag")
        .expect("Could not load basic vertex shader!");

    let pipeline = Pipeline::new(
        &device,
        swapchain.get_extent(),
        &swapchain.get_render_pass(),
        vec![vert_shader, frag_shader],
        vk::CullModeFlags::BACK,
        vk::TRUE,
    );

    world.insert_resource::<Pipeline>(pipeline);
}

fn cleanup(world: &mut World) {
    let mut system_state: SystemState<(
        ResMut<Device>,
        ResMut<SwapChain>,
        ResMut<Pipeline>,
        ResMut<CommandPool>,
    )> = SystemState::new(world);

    let (mut device, mut swapchain, mut pipeline, mut command_pool) = system_state.get_mut(world);

    device.wait_idle();

    command_pool.cleanup(&device);
    pipeline.as_mut().cleanup(&device);
    swapchain.as_mut().cleanup(&device);
    device.as_mut().cleanup();
}
