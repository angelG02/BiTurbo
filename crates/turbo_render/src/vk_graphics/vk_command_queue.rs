use ash::vk;
use bevy_ecs::prelude::*;
//use gpu_allocator::vulkan::*;

use crate::prelude::vk_buffers::vk_command_buffer::CommandBuffer;
use crate::prelude::vk_command_pool::CommandPool;
use crate::prelude::vk_device::Device;
use crate::prelude::vk_fence::Fence;
use crate::prelude::vk_semaphore::Semaphore;
use crate::prelude::vk_swapchain::MAX_FRAMES_IN_FLIGHT;

#[derive(Resource)]
pub struct CommandQueue {
    queue: vk::Queue,
    command_buffers: Vec<CommandBuffer>,
}

impl CommandQueue {
    pub fn new(
        device: &Device,
        //allocator: Arc<Mutex<Allocator>>,
        queue: vk::Queue,
        command_pool: &CommandPool,
    ) -> Self {
        let mut command_buffers = Vec::new();
        for _ in 0..MAX_FRAMES_IN_FLIGHT {
            command_buffers.push(CommandBuffer::new(device, command_pool))
        }
        Self {
            queue,
            command_buffers,
        }
    }

    pub fn get_command_buffer(&self, current_frame: usize) -> CommandBuffer {
        self.command_buffers[current_frame].clone()
    }

    pub fn submit_command_buffer(
        &mut self,
        device: &Device,
        cmd_buffer: &CommandBuffer,
        wait_semaphores: Option<&Vec<&Semaphore>>,
        signal_semaphores: Option<&Vec<&Semaphore>>,
    ) -> Fence {
        self.submit_command_buffers(
            device,
            &vec![cmd_buffer],
            wait_semaphores,
            signal_semaphores,
        )
    }

    pub fn submit_command_buffers(
        &mut self,
        device: &Device,
        cmd_buffers: &Vec<&CommandBuffer>,
        wait_semaphores: Option<&Vec<&Semaphore>>,
        signal_semaphores: Option<&Vec<&Semaphore>>,
    ) -> Fence {
        let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];

        let mut wait_semaphores_raw = Vec::new();
        let mut signal_semaphores_raw = Vec::new();
        let mut cmd_buffers_raw = Vec::new();

        for cmd_buffer in cmd_buffers {
            cmd_buffers_raw.push(cmd_buffer.get_command_buffer());
        }

        let (wait_semaphore_count, p_wait_semaphores) = match wait_semaphores {
            Some(wait_semaphores) => {
                for wait_semaphore in wait_semaphores {
                    wait_semaphores_raw.push(*wait_semaphore.get_semaphore());
                }

                (wait_semaphores.len() as u32, wait_semaphores_raw.as_ptr())
            }
            None => (0, std::ptr::null()),
        };

        let (signal_semaphore_count, p_signal_semaphores) = match signal_semaphores {
            Some(signal_semaphores) => {
                for signal_semaphore in signal_semaphores {
                    signal_semaphores_raw.push(*signal_semaphore.get_semaphore());
                }

                (
                    signal_semaphores.len() as u32,
                    signal_semaphores_raw.as_ptr(),
                )
            }
            None => (0, std::ptr::null()),
        };

        let submit_infos = [vk::SubmitInfo {
            s_type: vk::StructureType::SUBMIT_INFO,
            p_next: std::ptr::null(),
            wait_semaphore_count: wait_semaphore_count,
            p_wait_semaphores: p_wait_semaphores,
            p_wait_dst_stage_mask: wait_stages.as_ptr(),
            command_buffer_count: cmd_buffers_raw.len() as u32,
            p_command_buffers: cmd_buffers_raw.as_ptr(),
            signal_semaphore_count: signal_semaphore_count,
            p_signal_semaphores: p_signal_semaphores,
        }];

        let fence = Fence::new(device, false);

        unsafe {
            device
                .get_device()
                .queue_submit(self.queue, &submit_infos, fence.get_fence())
                .expect("Failed to execute queue submit.");
        }

        fence
    }
}
