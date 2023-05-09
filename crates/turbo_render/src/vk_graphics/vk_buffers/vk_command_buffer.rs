use std::sync::Arc;

use ash::vk;
//use gpu_allocator::vulkan::*;

use crate::prelude::vk_command_pool::CommandPool;
use crate::prelude::vk_device::Device;
use crate::prelude::vk_pipeline::Pipeline;
use crate::prelude::vk_render_pass::RenderPass;
use crate::prelude::vk_swapchain::SwapChain;

#[derive(Clone)]
pub struct CommandBuffer {
    //_allocator: Arc<Mutex<Allocator>>,
    cmd_buffer: vk::CommandBuffer,

    pipeline: Option<Arc<Pipeline>>,
}

impl CommandBuffer {
    pub fn new(
        device: &Device,
        //allocator: Arc<Mutex<Allocator>>,
        command_pool: &CommandPool,
    ) -> Self {
        let command_buffer_allocate_info = vk::CommandBufferAllocateInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
            p_next: std::ptr::null(),
            command_buffer_count: 1,
            command_pool: command_pool.get_command_pool(),
            level: vk::CommandBufferLevel::PRIMARY,
        };

        let command_buffers = unsafe {
            device
                .get_device()
                .allocate_command_buffers(&command_buffer_allocate_info)
                .expect("Failed to allocate Command Buffers.")
        };

        Self {
            //_allocator: allocator,
            cmd_buffer: command_buffers[0],
            pipeline: None,
        }
    }

    pub fn get_command_buffer(&self) -> vk::CommandBuffer {
        self.cmd_buffer
    }

    pub fn reset(&mut self, device: &Device) {
        unsafe {
            self.pipeline = None;

            device
                .get_device()
                .reset_command_buffer(self.cmd_buffer, vk::CommandBufferResetFlags::empty())
                .expect("Failed to execute queue reset!");
        }
    }

    pub fn begin(&self, device: &Device, flags: vk::CommandBufferUsageFlags) {
        let command_buffer_begin_info = vk::CommandBufferBeginInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
            p_next: std::ptr::null(),
            p_inheritance_info: std::ptr::null(),
            flags,
        };

        unsafe {
            device
                .get_device()
                .begin_command_buffer(self.cmd_buffer, &command_buffer_begin_info)
                .expect("Failed to begin Command Buffer!");
        }
    }

    pub fn end(&self, device: &Device) {
        unsafe {
            device
                .get_device()
                .end_command_buffer(self.cmd_buffer)
                .expect("Failed to end Command Buffer!");
        }
    }

    pub fn set_scissor(&self, scissor: vk::Rect2D, device: &Device) {
        unsafe {
            device
                .get_device()
                .cmd_set_scissor(self.cmd_buffer, 0, &[scissor]);
        }
    }

    pub fn set_viewport(&self, device: &Device, extent: &vk::Extent2D) {
        let viewports = [vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: extent.width as f32,
            height: extent.height as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        }];

        let scissors = [vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: *extent,
        }];

        unsafe {
            device
                .get_device()
                .cmd_set_viewport(self.cmd_buffer, 0, &viewports);

            device
                .get_device()
                .cmd_set_scissor(self.cmd_buffer, 0, &scissors);
        }
    }

    pub fn begin_render_pass(
        &self,
        device: &Device,
        render_pass: &RenderPass,
        swapchain: &SwapChain,
    ) {
        let clear_values = [
            vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [0.0, 0.0, 0.0, 1.0],
                },
            },
            vk::ClearValue {
                depth_stencil: vk::ClearDepthStencilValue {
                    depth: 1.0,
                    stencil: 0,
                },
            },
        ];

        let render_pass_begin_info = vk::RenderPassBeginInfo {
            s_type: vk::StructureType::RENDER_PASS_BEGIN_INFO,
            p_next: std::ptr::null(),
            render_pass: render_pass.get_render_pass(),
            framebuffer: *swapchain.get_current_framebuffer(),
            render_area: vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: *swapchain.get_extent(),
            },
            clear_value_count: clear_values.len() as u32,
            p_clear_values: clear_values.as_ptr(),
        };

        unsafe {
            device.get_device().cmd_begin_render_pass(
                self.cmd_buffer,
                &render_pass_begin_info,
                vk::SubpassContents::INLINE,
            );
        }
    }

    pub fn end_render_pass(&self, device: &Device) {
        unsafe {
            device.get_device().cmd_end_render_pass(self.cmd_buffer);
        }
    }

    pub fn bind_graphics_pipeline(&mut self, device: &Device, pipeline: &Pipeline) {
        unsafe {
            device.get_device().cmd_bind_pipeline(
                self.cmd_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                *pipeline.get_pipeline(),
            );
        }
    }

    pub fn draw(
        &self,
        device: &Device,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        unsafe {
            device.get_device().cmd_draw(
                self.cmd_buffer,
                vertex_count,
                instance_count,
                first_vertex,
                first_instance,
            );
        }
    }
}
