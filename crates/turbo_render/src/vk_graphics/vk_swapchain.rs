use std::sync::Arc;

use ash::{self, vk};

use crate::prelude::vk_buffers::vk_image::Image;
use crate::prelude::vk_device::Device;
use crate::prelude::vk_fence::Fence;
use crate::prelude::vk_semaphore::Semaphore;

use bevy_ecs::prelude::*;

pub const MAX_FRAMES_IN_FLIGHT: usize = 3;

#[derive(Resource, Clone)]
pub struct SwapChain {
    device: Arc<Device>,

    swapchain_loader: ash::extensions::khr::Swapchain,
    swapchain: vk::SwapchainKHR,
    color_format: vk::Format,
    depth_format: vk::Format,
    swapchain_extent: vk::Extent2D,

    depth_image: Option<Image>,
    frame_buffers: Vec<vk::Framebuffer>,

    swapchain_images: Vec<vk::Image>,
    swapchain_image_views: Vec<vk::ImageView>,

    image_available_semaphores: Vec<Semaphore>,
    render_finished_semaphores: Vec<Semaphore>,
    in_flight_fences: [Fence; MAX_FRAMES_IN_FLIGHT],

    current_frame: usize,
    current_image: u32,
}

impl SwapChain {
    pub fn new(device: Arc<Device>, width: u32, height: u32) -> SwapChain {
        let swapchain_support = device.get_swapchain_support();

        let surface_format = SwapChain::choose_swapchain_format(&swapchain_support.formats);
        let present_mode =
            SwapChain::choose_swapchian_present_mode(&swapchain_support.present_modes);
        let extent =
            SwapChain::choose_swapchain_extent(&swapchain_support.capabilities, width, height);

        let image_count = swapchain_support.capabilities.min_image_count + 1;
        let image_count = if swapchain_support.capabilities.max_image_count > 0 {
            image_count.min(swapchain_support.capabilities.max_image_count)
        } else {
            image_count
        };

        let queue_family = device.get_queue_indices();
        let (image_sharing_mode, queue_family_index_count, queue_family_indices) =
            if queue_family.graphics_family != queue_family.present_family {
                (
                    vk::SharingMode::CONCURRENT,
                    2,
                    vec![
                        queue_family.graphics_family.unwrap(),
                        queue_family.present_family.unwrap(),
                    ],
                )
            } else {
                (vk::SharingMode::EXCLUSIVE, 0, vec![])
            };

        let swapchain_create_info = vk::SwapchainCreateInfoKHR {
            s_type: vk::StructureType::SWAPCHAIN_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            flags: vk::SwapchainCreateFlagsKHR::empty(),
            surface: device.get_surface_details().surface,
            min_image_count: image_count,
            image_color_space: surface_format.color_space,
            image_format: surface_format.format,
            image_extent: extent,
            image_usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
            image_sharing_mode,
            image_array_layers: 1,
            queue_family_index_count,
            p_queue_family_indices: queue_family_indices.as_ptr(),
            pre_transform: swapchain_support.capabilities.current_transform,
            composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
            present_mode,
            clipped: vk::TRUE,
            old_swapchain: vk::SwapchainKHR::null(),
        };

        let swapchain_loader =
            ash::extensions::khr::Swapchain::new(device.get_instance(), device.get_device());
        let swapchain = unsafe {
            swapchain_loader
                .create_swapchain(&swapchain_create_info, None)
                .expect("Failed to create Swap Chain!")
        };

        let swapchain_images = unsafe {
            swapchain_loader
                .get_swapchain_images(swapchain)
                .expect("Failed to get Swap Chain images!")
        };

        let swapchain_image_views =
            SwapChain::create_image_views(device.clone(), surface_format.format, &swapchain_images);

        let mut image_available_semaphores = Vec::new();
        let mut render_finished_semaphores = Vec::new();
        let mut in_flight_fences = Vec::new();

        for _ in 0..MAX_FRAMES_IN_FLIGHT {
            image_available_semaphores.push(Semaphore::new(device.clone()));
            render_finished_semaphores.push(Semaphore::new(device.clone()));
            in_flight_fences.push(Fence::new(device.clone(), true));
        }

        let depth_format = SwapChain::get_optimal_depth_format(
            device.get_instance(),
            device.get_physical_device(),
        );

        Self {
            device: device.clone(),
            swapchain_loader,
            swapchain,
            color_format: surface_format.format,
            depth_format,
            swapchain_extent: extent,
            depth_image: None,
            frame_buffers: Vec::new(),
            swapchain_images,
            swapchain_image_views,
            image_available_semaphores,
            render_finished_semaphores,
            in_flight_fences: in_flight_fences.try_into().unwrap_or_else(|_| panic!("")),
            current_frame: 0,
            current_image: 0,
        }
    }

    fn choose_swapchain_format(
        availble_formats: &Vec<vk::SurfaceFormatKHR>,
    ) -> vk::SurfaceFormatKHR {
        for format in availble_formats.iter() {
            if format.format == vk::Format::B8G8R8A8_SRGB
                && format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
            {
                return format.clone();
            }
        }
        return availble_formats.first().unwrap().clone();
    }

    fn choose_swapchian_present_mode(
        available_present_modes: &Vec<vk::PresentModeKHR>,
    ) -> vk::PresentModeKHR {
        for &present_mode in available_present_modes.iter() {
            if present_mode == vk::PresentModeKHR::MAILBOX {
                return present_mode;
            }
        }

        return vk::PresentModeKHR::FIFO;
    }

    fn choose_swapchain_extent(
        capabilities: &vk::SurfaceCapabilitiesKHR,
        width: u32,
        height: u32,
    ) -> vk::Extent2D {
        if capabilities.current_extent.width != u32::MAX {
            capabilities.current_extent
        } else {
            use num::clamp;

            vk::Extent2D {
                width: clamp(
                    width,
                    capabilities.min_image_extent.width,
                    capabilities.max_image_extent.width,
                ),
                height: clamp(
                    height,
                    capabilities.min_image_extent.height,
                    capabilities.max_image_extent.height,
                ),
            }
        }
    }

    fn create_image_views(
        device: Arc<Device>,
        surface_format: vk::Format,
        images: &Vec<vk::Image>,
    ) -> Vec<vk::ImageView> {
        let mut swapchain_image_views = vec![];

        for &image in images.iter() {
            swapchain_image_views.push(Image::create_image_view(
                device.clone(),
                image,
                surface_format,
            ));
        }

        swapchain_image_views
    }

    fn get_optimal_depth_format(
        instance: &ash::Instance,
        physical_device: &vk::PhysicalDevice,
    ) -> vk::Format {
        SwapChain::find_supported_format(
            instance,
            physical_device,
            &[
                vk::Format::D32_SFLOAT,
                vk::Format::D32_SFLOAT_S8_UINT,
                vk::Format::D24_UNORM_S8_UINT,
            ],
            vk::ImageTiling::OPTIMAL,
            vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT,
        )
    }

    fn find_supported_format(
        instance: &ash::Instance,
        physical_device: &vk::PhysicalDevice,
        candidate_formats: &[vk::Format],
        tiling: vk::ImageTiling,
        features: vk::FormatFeatureFlags,
    ) -> vk::Format {
        for &format in candidate_formats.iter() {
            let format_properties =
                unsafe { instance.get_physical_device_format_properties(*physical_device, format) };

            if tiling == vk::ImageTiling::LINEAR
                && format_properties.linear_tiling_features.contains(features)
            {
                return format.clone();
            } else if tiling == vk::ImageTiling::OPTIMAL
                && format_properties.optimal_tiling_features.contains(features)
            {
                return format.clone();
            }
        }

        panic!("Failed to find supported format.")
    }

    pub fn get_framebuffers(&self) -> &Vec<vk::Framebuffer> {
        &self.frame_buffers
    }

    pub fn get_color_format(&self) -> &vk::Format {
        &self.color_format
    }

    pub fn get_depth_format(&self) -> &vk::Format {
        &self.depth_format
    }

    pub fn get_swapchain(&self) -> &vk::SwapchainKHR {
        &self.swapchain
    }

    pub fn get_extent(&self) -> &vk::Extent2D {
        &self.swapchain_extent
    }

    pub fn get_depth_image(&self) -> &Option<Image> {
        &self.depth_image
    }

    pub fn get_current_framebuffer(&self) -> &vk::Framebuffer {
        &self.frame_buffers[self.current_image as usize]
    }

    pub fn get_framebuffer_count(&self) -> usize {
        self.frame_buffers.len()
    }

    pub fn next_image(&mut self) {
        self.in_flight_fences[self.current_frame].wait();

        self.current_image = unsafe {
            self.swapchain_loader
                .acquire_next_image(
                    self.swapchain,
                    std::u64::MAX,
                    *self.image_available_semaphores[self.current_frame].get_semaphore(),
                    vk::Fence::null(),
                )
                .expect("Failed to acquire next image!")
                .0
        };

        self.in_flight_fences[self.current_frame].reset();
    }

    pub fn get_current_image(&self) -> u32 {
        self.current_image
    }

    pub fn image_available_semaphore(&self) -> &Semaphore {
        &self.image_available_semaphores[self.current_frame]
    }

    pub fn render_finished_semaphore(&self) -> &Semaphore {
        &self.render_finished_semaphores[self.current_frame]
    }

    pub fn present(&mut self, fence: &Fence, wait_semaphores: &Vec<&Semaphore>) {
        let mut wait_semaphores_raw = Vec::new();
        for wait_semaphore in wait_semaphores {
            wait_semaphores_raw.push(*wait_semaphore.get_semaphore());
        }

        let swapchains = [self.swapchain];
        let present_info = vk::PresentInfoKHR {
            s_type: vk::StructureType::PRESENT_INFO_KHR,
            p_next: std::ptr::null(),
            wait_semaphore_count: 1,
            p_wait_semaphores: wait_semaphores_raw.as_ptr(),
            swapchain_count: 1,
            p_swapchains: swapchains.as_ptr(),
            p_image_indices: &self.current_image,
            p_results: std::ptr::null_mut(),
        };

        unsafe {
            self.swapchain_loader
                .queue_present(*self.device.get_present_queue(), &present_info)
                .expect("Failed to execute present queue!");
        }

        self.in_flight_fences[self.current_frame] = fence.clone();
        self.current_frame = (self.current_frame + 1) % MAX_FRAMES_IN_FLIGHT;
    }

    pub fn get_swapchain_images(&self) -> &Vec<vk::Image> {
        &self.swapchain_images
    }
}

// This needs to be done in cleanup (onTerminate?) and not on Drop of the object
impl Drop for SwapChain {
    fn drop(&mut self) {
        unsafe {
            for &imageview in self.swapchain_image_views.iter() {
                self.device.get_device().destroy_image_view(imageview, None);
            }

            self.swapchain_loader
                .destroy_swapchain(self.swapchain, None);
        }
    }
}
