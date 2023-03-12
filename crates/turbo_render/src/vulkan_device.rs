use ash::{self, vk};

use crate::vulkan_utils::{debug::*, util::*};
use turbo_core::prelude::trace::{info, warn};

#[cfg(target_os = "windows")]
use ash::extensions::khr::Win32Surface;
#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
use ash::extensions::khr::XlibSurface;
#[cfg(target_os = "macos")]
use ash::extensions::mvk::MacOSSurface;

use ash::extensions::ext::DebugUtils;
use ash::extensions::khr::Surface;

use std::ffi::{c_char, CString};
use std::os::raw::c_void;
use std::ptr;

struct QueueFamilyIndices {
    graphics_family: Option<u32>,
}

impl QueueFamilyIndices {
    pub fn is_complete(&self) -> bool {
        self.graphics_family.is_some()
    }
}

const VALIDATION: ValidationInfo = ValidationInfo {
    is_enable: true,
    required_validation_layers: ["VK_LAYER_KHRONOS_validation"],
};

pub struct Device {
    _entry: ash::Entry,
    instance: ash::Instance,
    debug_utils_loader: ash::extensions::ext::DebugUtils,
    debug_messenger: vk::DebugUtilsMessengerEXT,
    _physical_device: vk::PhysicalDevice,
    device: ash::Device,
    _graphics_queue: vk::Queue,
}

impl Device {
    pub fn new() -> Self {
        let entry = unsafe { ash::Entry::load() }.expect(
            "Could not load Vulkan Library. Please make sure you have the Vulkan SDK installed!",
        );

        let instance = Device::create_instance(&entry);
        let (debug_utils_loader, debug_messenger) = Device::setup_debug_utils(&entry, &instance);
        let physical_device = Device::pick_physical_device(&instance);

        let (logical_device, graphics_queue) =
            Device::create_logical_device(&instance, physical_device);

        Device {
            _entry: entry,
            instance,
            debug_utils_loader,
            debug_messenger,
            _physical_device: physical_device,
            device: logical_device,
            _graphics_queue: graphics_queue,
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

    fn pick_physical_device(instance: &ash::Instance) -> vk::PhysicalDevice {
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
                if Device::is_physical_device_suitable(instance, physical_device) {
                    if result.is_none() {
                        result = Some(physical_device)
                    }
                }
            }
        }

        // If there is no discrete GPU try and pick the integrated one
        if result.is_none() {
            for &physical_device in physical_devices.iter() {
                if Device::is_physical_device_suitable(instance, physical_device) {
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
    ) -> (ash::Device, vk::Queue) {
        let indices = Device::find_queue_family(instance, physical_device);

        let queue_priorities = [1.0_f32];
        let queue_create_info = vk::DeviceQueueCreateInfo {
            s_type: vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::DeviceQueueCreateFlags::empty(),
            queue_family_index: indices.graphics_family.unwrap(),
            p_queue_priorities: queue_priorities.as_ptr(),
            queue_count: queue_priorities.len() as u32,
        };

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
            queue_create_info_count: 1,
            p_queue_create_infos: &queue_create_info,
            enabled_layer_count: if VALIDATION.is_enable {
                enable_layer_names.len()
            } else {
                0
            } as u32,
            pp_enabled_extension_names: if VALIDATION.is_enable {
                enable_layer_names.as_ptr()
            } else {
                ptr::null()
            },
            enabled_extension_count: 0,
            pp_enabled_layer_names: ptr::null(),
            p_enabled_features: &physical_device_features,
        };

        let device: ash::Device = unsafe {
            instance
                .create_device(physical_device, &device_create_info, None)
                .expect("Failed to create logical device!")
        };

        let graphics_queue =
            unsafe { device.get_device_queue(indices.graphics_family.unwrap(), 0) };

        (device, graphics_queue)
    }

    fn is_physical_device_suitable(
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
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

        let indices = Device::find_queue_family(instance, physical_device);

        return indices.is_complete();
    }

    fn find_queue_family(
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
    ) -> QueueFamilyIndices {
        let queue_families =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

        let mut queue_family_indices = QueueFamilyIndices {
            graphics_family: None,
        };

        let mut index = 0;
        for queue_family in queue_families.iter() {
            if queue_family.queue_count > 0
                && queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
            {
                queue_family_indices.graphics_family = Some(index);
            }

            if queue_family_indices.is_complete() {
                break;
            }

            index += 1;
        }
        queue_family_indices
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            if VALIDATION.is_enable {
                self.debug_utils_loader
                    .destroy_debug_utils_messenger(self.debug_messenger, None);
            }
            self.instance.destroy_instance(None);
        }
    }
}
