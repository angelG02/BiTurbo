use ash::{self, vk};

use crate::vulkan_utils::{debug::*, util::*};
use turbo_core::prelude::trace::warn;

#[cfg(target_os = "windows")]
use ash::extensions::khr::Win32Surface;
#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
use ash::extensions::khr::XlibSurface;
#[cfg(target_os = "macos")]
use ash::extensions::mvk::MacOSSurface;

use ash::extensions::ext::DebugUtils;
use ash::extensions::khr::Surface;

use std::ffi::CString;
use std::os::raw::c_void;
use std::ptr;

const VALIDATION: ValidationInfo = ValidationInfo {
    is_enable: true,
    required_validation_layers: ["VK_LAYER_KHRONOS_validation"],
};

pub struct Device {
    _entry: ash::Entry,
    instance: ash::Instance,
    debug_utils_loader: ash::extensions::ext::DebugUtils,
    debug_messenger: vk::DebugUtilsMessengerEXT,
}

impl Device {
    pub fn new() -> Self {
        let entry = unsafe { ash::Entry::load() }.expect(
            "Could not load Vulkan Library. Please make sure you have the Vulkan SDK installed!",
        );

        let instance = Device::create_instance(&entry);
        let (debug_utils_loader, debug_messenger) = Device::setup_debug_utils(&entry, &instance);

        Device {
            _entry: entry,
            instance,
            debug_utils_loader,
            debug_messenger,
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
