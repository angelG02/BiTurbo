use ash::{self, vk};
use glfw::Window;

use crate::prelude::vk_utils::{debug::*, util::*};
use turbo_core::prelude::trace::{info, warn};

#[cfg(target_os = "windows")]
use ash::extensions::khr::Win32Surface;
#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
use ash::extensions::khr::XlibSurface;
#[cfg(target_os = "macos")]
use ash::extensions::mvk::MacOSSurface;

use ash::extensions::ext::DebugUtils;
use ash::extensions::khr::Surface;

use std::collections::HashSet;
use std::ffi::{c_char, CString};
use std::os::raw::c_void;
use std::ptr;

use bevy_ecs::prelude::*;

use super::vk_utils::platforms;

#[derive(Clone)]
pub struct QueueFamilyIndices {
    pub graphics_family: Option<u32>,
    pub present_family: Option<u32>,
}

#[derive(Clone)]
pub struct SurfaceDetails {
    pub surface_loader: ash::extensions::khr::Surface,
    pub surface: vk::SurfaceKHR,
}

impl QueueFamilyIndices {
    pub fn new() -> QueueFamilyIndices {
        Self {
            graphics_family: None,
            present_family: None,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.graphics_family.is_some() && self.present_family.is_some()
    }
}

struct DeviceExtensions {
    pub names: [&'static str; 1],
}

impl DeviceExtensions {
    pub fn get_raw_names() -> [*const c_char; 1] {
        [ash::extensions::khr::Swapchain::name().as_ptr()]
    }
}

pub struct SwapchainSupportDetail {
    pub capabilities: vk::SurfaceCapabilitiesKHR,
    pub formats: Vec<vk::SurfaceFormatKHR>,
    pub present_modes: Vec<vk::PresentModeKHR>,
}

const VALIDATION: ValidationInfo = ValidationInfo {
    is_enable: true,
    required_validation_layers: ["VK_LAYER_KHRONOS_validation"],
};

const DEVICE_EXT: DeviceExtensions = DeviceExtensions {
    names: ["VK_KHR_swapchain"],
};

#[derive(Resource, Clone)]
pub struct Device {
    _entry: ash::Entry,
    device: ash::Device,
    instance: ash::Instance,
    surface_details: SurfaceDetails,
    debug_utils_loader: ash::extensions::ext::DebugUtils,
    debug_messenger: vk::DebugUtilsMessengerEXT,
    physical_device: vk::PhysicalDevice,
    queue_indices: QueueFamilyIndices,
    graphics_queue: vk::Queue,
    present_queue: vk::Queue,
    buffer_device_address_loader: ash::extensions::khr::BufferDeviceAddress,
}

impl Device {
    pub fn new(window: &Window) -> Self {
        let entry = unsafe { ash::Entry::load() }.expect(
            "Could not load Vulkan Library. Please make sure you have the Vulkan SDK installed!",
        );

        let instance = Device::create_instance(&entry);
        let (debug_utils_loader, debug_messenger) = Device::setup_debug_utils(&entry, &instance);
        let surface_details = Device::create_surface(&entry, &instance, &window);
        let physical_device = Device::pick_physical_device(&instance, &surface_details);

        let (logical_device, family_indices) =
            Device::create_logical_device(&instance, physical_device, &surface_details);

        let graphics_queue =
            unsafe { logical_device.get_device_queue(family_indices.graphics_family.unwrap(), 0) };
        let present_queue =
            unsafe { logical_device.get_device_queue(family_indices.present_family.unwrap(), 0) };

        let buffer_device_address_loader =
            ash::extensions::khr::BufferDeviceAddress::new(&instance, &logical_device);

        Device {
            _entry: entry,
            instance,
            surface_details,
            debug_utils_loader,
            debug_messenger,
            physical_device,
            device: logical_device,
            queue_indices: family_indices,
            graphics_queue,
            present_queue,
            buffer_device_address_loader,
        }
    }

    fn create_instance(entry: &ash::Entry) -> ash::Instance {
        if VALIDATION.is_enable && Device::check_validation_layer_support(entry) == false {
            panic!("Validation layers requested, but not available!");
        }

        let app_name = CString::new("V8 BiTurbo").unwrap();
        let engine_name = CString::new("BiTurbo").unwrap();

        let app_info = vk::ApplicationInfo {
            s_type: vk::StructureType::APPLICATION_INFO,
            p_next: ptr::null(),
            p_application_name: app_name.as_ptr(),
            application_version: 1,
            p_engine_name: engine_name.as_ptr(),
            engine_version: 1,
            api_version: vk::make_api_version(0, 1, 3, 239),
        };

        let debug_utils_create_info = populate_debug_messenger_create_info();

        let extension_names = Device::enumerate_extension_names();

        let required_validation_layer_raw_names: Vec<CString> = VALIDATION
            .required_validation_layers
            .iter()
            .map(|layer_name| CString::new(*layer_name).unwrap())
            .collect();

        let enable_layer_names: Vec<*const i8> = required_validation_layer_raw_names
            .iter()
            .map(|layer_name| layer_name.as_ptr())
            .collect();

        let create_info = vk::InstanceCreateInfo {
            s_type: vk::StructureType::INSTANCE_CREATE_INFO,
            p_next: if VALIDATION.is_enable {
                &debug_utils_create_info as *const vk::DebugUtilsMessengerCreateInfoEXT
                    as *const c_void
            } else {
                ptr::null()
            },
            flags: vk::InstanceCreateFlags::empty(),
            p_application_info: &app_info,
            pp_enabled_layer_names: if VALIDATION.is_enable {
                enable_layer_names.as_ptr()
            } else {
                ptr::null()
            },
            enabled_layer_count: if VALIDATION.is_enable {
                enable_layer_names.len() as u32
            } else {
                0
            },
            pp_enabled_extension_names: extension_names.as_ptr(),
            enabled_extension_count: extension_names.len() as u32,
        };

        let instance: ash::Instance = unsafe {
            entry
                .create_instance(&create_info, None)
                .expect("Failed to create instance!")
        };

        instance
    }

    fn create_surface(
        entry: &ash::Entry,
        instance: &ash::Instance,
        window: &Window,
    ) -> SurfaceDetails {
        let surface = unsafe {
            platforms::create_surface(entry, instance, window).expect("Failed to create surface!")
        };

        let surface_loader = ash::extensions::khr::Surface::new(entry, instance);

        SurfaceDetails {
            surface_loader,
            surface,
        }
    }

    #[cfg(all(windows))]
    fn enumerate_extension_names() -> Vec<*const i8> {
        vec![
            Surface::name().as_ptr(),
            Win32Surface::name().as_ptr(),
            DebugUtils::name().as_ptr(),
        ]
    }

    fn check_validation_layer_support(entry: &ash::Entry) -> bool {
        // If validation layers are supported return true

        let layer_props = entry
            .enumerate_instance_layer_properties()
            .expect("Failed To Enumerate Instance Layer Properties!");

        if layer_props.len() <= 0 {
            warn!("No available layers.");
            return false;
        }

        for required_layer_name in VALIDATION.required_validation_layers.iter() {
            let mut is_layer_found = false;

            for layer_prop in layer_props.iter() {
                let test_layer_name = vk_to_string(&layer_prop.layer_name);
                if (*required_layer_name) == test_layer_name {
                    is_layer_found = true;
                    break;
                }
            }

            if is_layer_found == false {
                return false;
            }
        }

        true
    }

    fn setup_debug_utils(
        entry: &ash::Entry,
        instance: &ash::Instance,
    ) -> (ash::extensions::ext::DebugUtils, vk::DebugUtilsMessengerEXT) {
        let debug_utils_loader = ash::extensions::ext::DebugUtils::new(entry, instance);

        if VALIDATION.is_enable == false {
            (debug_utils_loader, vk::DebugUtilsMessengerEXT::null())
        } else {
            let messenger_ci = populate_debug_messenger_create_info();

            let utils_messenger = unsafe {
                debug_utils_loader
                    .create_debug_utils_messenger(&messenger_ci, None)
                    .expect("Debug Utils Callback")
            };

            (debug_utils_loader, utils_messenger)
        }
    }

    fn pick_physical_device(
        instance: &ash::Instance,
        surface_details: &SurfaceDetails,
    ) -> vk::PhysicalDevice {
        let physical_devices = unsafe {
            instance
                .enumerate_physical_devices()
                .expect("Failed to enumerate physical devices")
        };

        info!(
            "{} device(s) (GPU) found with Vulkan support and picked:",
            physical_devices.len()
        );

        // Enumerate over the available devices and try and pick the discrete GPU first
        let mut result = None;
        for &physical_device in physical_devices.iter() {
            let device_props = unsafe { instance.get_physical_device_properties(physical_device) };
            if let vk::PhysicalDeviceType::DISCRETE_GPU = device_props.device_type {
                if Device::is_physical_device_suitable(instance, physical_device, surface_details) {
                    if result.is_none() {
                        result = Some(physical_device)
                    }
                }
            }
        }

        // If there is no discrete GPU try and pick the integrated one
        if result.is_none() {
            for &physical_device in physical_devices.iter() {
                if Device::is_physical_device_suitable(instance, physical_device, surface_details) {
                    if result.is_none() {
                        result = Some(physical_device)
                    }
                }
            }
        }

        match result {
            None => panic!("Failed to find a suitable GPU!"),
            Some(physical_device) => physical_device,
        }
    }

    fn create_logical_device(
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
        surface_details: &SurfaceDetails,
    ) -> (ash::Device, QueueFamilyIndices) {
        let indices = Device::find_queue_family(instance, physical_device, surface_details);

        let mut unique_queue_families = HashSet::new();
        unique_queue_families.insert(indices.graphics_family.unwrap());
        unique_queue_families.insert(indices.present_family.unwrap());

        let queue_priorities = [1.0_f32];
        let mut queue_create_infos = vec![];
        for &queue_family in unique_queue_families.iter() {
            let queue_create_info = vk::DeviceQueueCreateInfo {
                s_type: vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
                p_next: ptr::null(),
                flags: vk::DeviceQueueCreateFlags::empty(),
                queue_family_index: queue_family,
                p_queue_priorities: queue_priorities.as_ptr(),
                queue_count: queue_priorities.len() as u32,
            };
            queue_create_infos.push(queue_create_info);
        }

        let physical_device_features = vk::PhysicalDeviceFeatures {
            ..Default::default() // default will not enable any features
        };

        let required_validation_layer_raw_names: Vec<CString> = VALIDATION
            .required_validation_layers
            .iter()
            .map(|layer_name| CString::new(*layer_name).unwrap())
            .collect();

        let enable_layer_names: Vec<*const c_char> = required_validation_layer_raw_names
            .iter()
            .map(|layer_name| layer_name.as_ptr())
            .collect();

        let device_create_info = vk::DeviceCreateInfo {
            s_type: vk::StructureType::DEVICE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::DeviceCreateFlags::empty(),
            queue_create_info_count: queue_create_infos.len() as u32,
            p_queue_create_infos: queue_create_infos.as_ptr(),
            enabled_layer_count: if VALIDATION.is_enable {
                enable_layer_names.len()
            } else {
                0
            } as u32,
            pp_enabled_layer_names: if VALIDATION.is_enable {
                enable_layer_names.as_ptr()
            } else {
                ptr::null()
            },
            enabled_extension_count: DEVICE_EXT.names.len() as u32,
            pp_enabled_extension_names: DeviceExtensions::get_raw_names().as_ptr(),
            p_enabled_features: &physical_device_features,
        };

        let device: ash::Device = unsafe {
            instance
                .create_device(physical_device, &device_create_info, None)
                .expect("Failed to create logical device!")
        };

        (device, indices)
    }

    fn is_physical_device_suitable(
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
        surface_details: &SurfaceDetails,
    ) -> bool {
        let device_props = unsafe { instance.get_physical_device_properties(physical_device) };

        let device_features = unsafe { instance.get_physical_device_features(physical_device) };

        let device_queue_families =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

        let device_type = match device_props.device_type {
            vk::PhysicalDeviceType::CPU => "CPU",
            vk::PhysicalDeviceType::INTEGRATED_GPU => "Integrated GPU",
            vk::PhysicalDeviceType::DISCRETE_GPU => "Discrete GPU",
            vk::PhysicalDeviceType::VIRTUAL_GPU => "Virtual GPU",
            vk::PhysicalDeviceType::OTHER => "Unknown",
            _ => panic!(),
        };

        let device_name = vk_to_string(&device_props.device_name);

        info!(
            "\tDevice Name: {}, id: {}, type: {}",
            device_name, device_props.device_id, device_type
        );

        info!(
            "\tSupported Queue Families: {}",
            device_queue_families.len()
        );
        info!("\tQueue Count | Graphics, Compute, Transfer, Sparse Binding");

        for queue_family in device_queue_families.iter() {
            let is_graphics_support = if queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
            {
                "supported"
            } else {
                "unsupported"
            };
            let is_compute_support = if queue_family.queue_flags.contains(vk::QueueFlags::COMPUTE) {
                "supported"
            } else {
                "unsupported"
            };
            let is_transfer_support = if queue_family.queue_flags.contains(vk::QueueFlags::TRANSFER)
            {
                "supported"
            } else {
                "unsupported"
            };
            let is_sparse_support = if queue_family
                .queue_flags
                .contains(vk::QueueFlags::SPARSE_BINDING)
            {
                "supported"
            } else {
                "unsupported"
            };

            info!(
                "\t{}\t    | {},  {},  {},  {}",
                queue_family.queue_count,
                is_graphics_support,
                is_compute_support,
                is_transfer_support,
                is_sparse_support
            );
        }

        info!(
            "\tGeometry Shader support: {}",
            if device_features.geometry_shader == 1 {
                "Supported"
            } else {
                "Unsupported"
            }
        );

        let indices = Device::find_queue_family(instance, physical_device, surface_details);
        let is_queue_family_supported = indices.is_complete();

        let is_extension_supported =
            Device::check_device_extension_suppot(instance, physical_device);
        let is_swapchain_supported = if is_extension_supported {
            let swapchain_support =
                Device::query_swapchain_support(physical_device, surface_details);
            !swapchain_support.formats.is_empty() && !swapchain_support.present_modes.is_empty()
        } else {
            false
        };

        return is_queue_family_supported && is_extension_supported && is_swapchain_supported;
    }

    fn find_queue_family(
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
        surface_details: &SurfaceDetails,
    ) -> QueueFamilyIndices {
        let queue_families =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

        let mut queue_family_indices = QueueFamilyIndices::new();

        let mut index = 0;
        for queue_family in queue_families.iter() {
            if queue_family.queue_count > 0
                && queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
            {
                queue_family_indices.graphics_family = Some(index);
            }

            let is_present_supported = unsafe {
                surface_details
                    .surface_loader
                    .get_physical_device_surface_support(
                        physical_device,
                        index as u32,
                        surface_details.surface,
                    )
                    .unwrap()
            };

            if queue_family.queue_count > 0 && is_present_supported {
                queue_family_indices.present_family = Some(index);
            }

            if queue_family_indices.is_complete() {
                break;
            }

            index += 1;
        }
        queue_family_indices
    }

    fn check_device_extension_suppot(
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
    ) -> bool {
        let available_ext = unsafe {
            instance
                .enumerate_device_extension_properties(physical_device)
                .expect("Failed to get device extension properties!")
        };

        let mut available_extension_names = vec![];

        //info!("Available Device Extensions:");
        for extension in available_ext.iter() {
            let ext_name = vk_to_string(&extension.extension_name);
            //info!("Name: {}, Version: {}", ext_name, extension.spec_version);
            available_extension_names.push(ext_name);
        }

        let mut required_extensions = HashSet::new();
        for extension in DEVICE_EXT.names.iter() {
            required_extensions.insert(extension.to_string());
        }

        for extension_name in available_extension_names.iter() {
            required_extensions.remove(extension_name);
        }

        return required_extensions.is_empty();
    }

    fn query_swapchain_support(
        physical_device: vk::PhysicalDevice,
        surface_details: &SurfaceDetails,
    ) -> SwapchainSupportDetail {
        unsafe {
            let capabilities = surface_details
                .surface_loader
                .get_physical_device_surface_capabilities(physical_device, surface_details.surface)
                .expect("Failed to query for surface capabilities!");
            let formats = surface_details
                .surface_loader
                .get_physical_device_surface_formats(physical_device, surface_details.surface)
                .expect("Failed to query for surface formats!");
            let present_modes = surface_details
                .surface_loader
                .get_physical_device_surface_present_modes(physical_device, surface_details.surface)
                .expect("Failed to query for surface present modes");

            SwapchainSupportDetail {
                capabilities,
                formats,
                present_modes,
            }
        }
    }

    pub fn get_swapchain_support(&self) -> SwapchainSupportDetail {
        Device::query_swapchain_support(self.physical_device, &self.surface_details)
    }

    pub fn get_instance(&self) -> &ash::Instance {
        &self.instance
    }

    pub fn get_physical_device(&self) -> &vk::PhysicalDevice {
        &self.physical_device
    }

    pub fn get_device(&self) -> &ash::Device {
        &self.device
    }

    pub fn get_surface_details(&self) -> &SurfaceDetails {
        &self.surface_details
    }

    pub fn get_present_queue(&self) -> &vk::Queue {
        &self.present_queue
    }

    pub fn get_graphics_queue(&self) -> &vk::Queue {
        &self.graphics_queue
    }

    pub fn get_queue_indices(&self) -> &QueueFamilyIndices {
        &self.queue_indices
    }

    pub fn get_buffer_device_address_loader(&self) -> &ash::extensions::khr::BufferDeviceAddress {
        &self.buffer_device_address_loader
    }

    pub fn get_debug_utils_loader(&self) -> &DebugUtils {
        &self.debug_utils_loader
    }

    pub fn cleanup(&mut self) {
        unsafe {
            self.surface_details
                .surface_loader
                .destroy_surface(self.surface_details.surface, None);
            self.device.destroy_device(None);

            if VALIDATION.is_enable {
                self.debug_utils_loader
                    .destroy_debug_utils_messenger(self.debug_messenger, None);
            }
            self.instance.destroy_instance(None);
        }
    }

    pub fn get_max_sample_count(&self) -> vk::SampleCountFlags {
        let physical_device_properties = unsafe {
            self.instance
                .get_physical_device_properties(self.physical_device)
        };

        let count = std::cmp::min(
            physical_device_properties
                .limits
                .framebuffer_color_sample_counts,
            physical_device_properties
                .limits
                .framebuffer_depth_sample_counts,
        );

        if count.contains(vk::SampleCountFlags::TYPE_64) {
            return vk::SampleCountFlags::TYPE_64;
        }
        if count.contains(vk::SampleCountFlags::TYPE_32) {
            return vk::SampleCountFlags::TYPE_32;
        }
        if count.contains(vk::SampleCountFlags::TYPE_16) {
            return vk::SampleCountFlags::TYPE_16;
        }
        if count.contains(vk::SampleCountFlags::TYPE_8) {
            return vk::SampleCountFlags::TYPE_8;
        }
        if count.contains(vk::SampleCountFlags::TYPE_4) {
            return vk::SampleCountFlags::TYPE_4;
        }
        if count.contains(vk::SampleCountFlags::TYPE_2) {
            return vk::SampleCountFlags::TYPE_2;
        }

        vk::SampleCountFlags::TYPE_1
    }

    pub fn get_physical_device_mem_properties(&self) -> vk::PhysicalDeviceMemoryProperties {
        unsafe {
            self.instance
                .get_physical_device_memory_properties(self.physical_device)
        }
    }
}
