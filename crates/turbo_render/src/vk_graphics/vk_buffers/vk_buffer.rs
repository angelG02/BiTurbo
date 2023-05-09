use std::{
    ffi::c_void,
    sync::{Arc, Mutex},
};

use ash::vk;

use crate::prelude::vk_device::Device;
use gpu_allocator::{vulkan::*, *};

pub struct Buffer {
    allocator: Arc<Mutex<Allocator>>,
    buffer: vk::Buffer,
    allocation: Option<Allocation>,
    size: vk::DeviceSize,

    name: String,
}

impl Buffer {
    pub fn new(
        name: &'static str,
        device: &Device,
        allocator: Arc<Mutex<Allocator>>,
        size: vk::DeviceSize,
        usage: vk::BufferUsageFlags,
        required_memory_properties: vk::MemoryPropertyFlags,
        alignment: Option<vk::DeviceSize>,
    ) -> Self {
        let buffer_create_info = vk::BufferCreateInfo {
            s_type: vk::StructureType::BUFFER_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: vk::BufferCreateFlags::empty(),
            size,
            usage,
            sharing_mode: vk::SharingMode::EXCLUSIVE,
            queue_family_index_count: 0,
            p_queue_family_indices: std::ptr::null(),
        };

        let buffer = unsafe {
            device
                .get_device()
                .create_buffer(&buffer_create_info, None)
                .expect("Failed to create Vertex Buffer")
        };

        let mut mem_requirements =
            unsafe { device.get_device().get_buffer_memory_requirements(buffer) };
        if let Some(alignment) = alignment {
            mem_requirements.alignment = alignment;
        }

        let location = if required_memory_properties.contains(vk::MemoryPropertyFlags::DEVICE_LOCAL)
        {
            MemoryLocation::GpuOnly
        } else {
            MemoryLocation::CpuToGpu
        };

        let allocation = allocator
            .as_ref()
            .lock()
            .unwrap()
            .allocate(&AllocationCreateDesc {
                name: "VkBuffer",
                requirements: mem_requirements,
                location: location,
                linear: true,
                allocation_scheme: AllocationScheme::GpuAllocatorManaged,
            })
            .unwrap();

        unsafe {
            device
                .get_device()
                .bind_buffer_memory(buffer, allocation.memory(), allocation.offset())
                .expect("Failed to bind Buffer.");
        }

        let mut buffer = Buffer {
            allocator,
            buffer,
            allocation: Some(allocation),
            size,
            name: String::from(name),
        };

        buffer.set_debug_name(device, name);
        buffer
    }

    pub fn find_memory_type(
        type_filter: u32,
        required_properties: vk::MemoryPropertyFlags,
        mem_properties: vk::PhysicalDeviceMemoryProperties,
    ) -> u32 {
        for (i, memory_type) in mem_properties.memory_types.iter().enumerate() {
            if (type_filter & (1 << i)) > 0
                && memory_type.property_flags.contains(required_properties)
            {
                return i as u32;
            }
        }

        panic!("Failed to find suitable memory type.")
    }

    pub fn map(&self) -> *mut c_void {
        unsafe {
            self.allocation
                .as_ref()
                .unwrap()
                .mapped_ptr()
                .unwrap()
                .as_mut()
        }
    }

    pub fn unmap(&self) {}

    pub fn get_buffer(&self) -> vk::Buffer {
        self.buffer
    }

    pub fn get_size(&self) -> vk::DeviceSize {
        self.size
    }

    pub fn get_device_address(&self, device: &Device) -> vk::DeviceAddress {
        let info = vk::BufferDeviceAddressInfo {
            s_type: vk::StructureType::BUFFER_DEVICE_ADDRESS_INFO,
            buffer: self.buffer,
            ..Default::default()
        };

        unsafe {
            device
                .get_buffer_device_address_loader()
                .get_buffer_device_address(&info)
        }
    }

    pub fn set_debug_name(&mut self, device: &Device, name: &'static str) {
        let handle: u64 = unsafe { std::mem::transmute(self.buffer) };

        let info = vk::DebugUtilsObjectNameInfoEXT::builder()
            .object_type(vk::ObjectType::BUFFER)
            .object_handle(handle)
            .object_name(&std::ffi::CString::new(name).unwrap())
            .build();

        unsafe {
            device
                .get_debug_utils_loader()
                .set_debug_utils_object_name(device.get_device().handle(), &info)
                .expect("Failed to set debug name.");
        }

        self.name = String::from(name);
    }

    pub fn cleanup(&mut self, device: &Device) {
        unsafe {
            if let Some(allocation) = self.allocation.take() {
                self.allocator
                    .as_ref()
                    .lock()
                    .unwrap()
                    .free(allocation)
                    .unwrap();
            }

            device.get_device().destroy_buffer(self.buffer, None);
        }
    }
}
