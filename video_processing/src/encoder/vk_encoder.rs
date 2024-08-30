#![allow(unused_imports)]
/*
VULKAN IS CURRENTLY BROKEN

DO NOT USE!
 */

#[cfg(feature = "enable_vulkan")]
use ash;
#[cfg(feature = "enable_vulkan")]
use ash::vk;
#[cfg(feature = "enable_vulkan")]
use ash::vk::ExtensionProperties;
#[cfg(feature = "enable_vulkan")]
use ash::vk::ImageUsageFlags;
#[cfg(feature = "enable_vulkan")]
use ash::vk::native::StdVideoEncodeH264ReferenceListsInfoFlags;
#[cfg(feature = "enable_vulkan")]
use vk_mem;
#[cfg(feature = "enable_vulkan")]
use vk_mem::Alloc;
use crate::encoder::device::{EEncoderError, EncodeDevice, EncoderSettings};
use crate::frame::Frame;
use super::vk_utils;

#[cfg(feature = "enable_vulkan")]
pub struct VkEncoder{
    entry: ash::Entry,
    instance: ash::Instance,
    device: ash::Device,
    video_device: ash::khr::video_queue::Device,
    encode_device: ash::khr::video_encode_queue::Device,
    allocator: vk_mem::Allocator,

    image: vk::Image,
    image_view: vk::ImageView,
    image_alloc: vk_mem::Allocation,

    output: vk::Buffer,
    output_alloc: vk_mem::Allocation,

    video_session: vk::VideoSessionKHR,
    video_session_params: vk::VideoSessionParametersKHR
}

#[cfg(feature = "enable_vulkan")]
impl VkEncoder{

}

#[cfg(feature = "enable_vulkan")]
impl EncodeDevice for VkEncoder{
    fn is_supported() -> bool {
        unsafe {
            let entry = ash::Entry::load()
                .expect("Unable to load Vulkan Entry!");

            let app_info = vk::ApplicationInfo::default()
                .application_name(std::ffi::CStr::from_ptr(b"ExtensionCheck".as_ptr() as *const _))
                .application_version(1)
                .engine_name(std::ffi::CStr::from_ptr(b"ExtensionCheck".as_ptr() as *const _))
                .engine_version(1)
                .api_version(vk::make_api_version(0,1,3,279));
            
            let instance_extensions =
                [];

            let create_flags = vk::InstanceCreateFlags::default();

            let layer_names = [std::ffi::CStr::from_bytes_with_nul_unchecked(
                b"VK_LAYER_KHRONOS_validation\0",
            )];
            let layers_names_raw: Vec<*const std::ffi::c_char> = layer_names
                .iter()
                .map(|raw_name| raw_name.as_ptr())
                .collect();

            let create_info = vk::InstanceCreateInfo::default()
                .application_info(&app_info)
                .enabled_layer_names(&layers_names_raw)
                .enabled_extension_names(&instance_extensions)
                .flags(create_flags);

            let instance = entry.create_instance(&create_info, None)
                .expect("Unable to create instance!");

            let devices = instance.enumerate_physical_devices()
                .expect("Unable to enumerate devices!");

            let mut is_video_supported = false;
            let mut is_ycbcr_supported = false;
            for device in devices{
                let extensions =
                    instance.enumerate_device_extension_properties(device)
                        .expect("Unable to enumerate extensions!");
                for extension in extensions{
                    if extension.extension_name_as_c_str()
                        .unwrap()
                        .eq(vk::KHR_VIDEO_ENCODE_H264_NAME){
                        is_video_supported = true;
                    }
                    if extension.extension_name_as_c_str()
                        .unwrap()
                        .eq(vk::KHR_SAMPLER_YCBCR_CONVERSION_NAME){
                        is_ycbcr_supported = true;
                    }
                }
            }
            instance.destroy_instance(None);
            is_video_supported && is_ycbcr_supported
        }
    }

    fn init(settings: EncoderSettings) -> Result<Self, EEncoderError> {
        unsafe {
            // First we must create our vulkan device
            // Create our entry
            let entry = ash::Entry::load()
                .expect("Unable to load Vulkan Entry!");
            
            let app_info = vk::ApplicationInfo::default()
                .application_name(std::ffi::CStr::from_ptr(b"ALIS".as_ptr() as *const _))
                .application_version(1)
                .engine_name(std::ffi::CStr::from_ptr(b"ALIS".as_ptr() as *const _))
                .engine_version(1)
                .api_version(vk::make_api_version(0,1,3,279));

            let instance_extensions =
                [];

            let create_flags = vk::InstanceCreateFlags::default();

            let layer_names = [std::ffi::CStr::from_bytes_with_nul_unchecked(
                b"VK_LAYER_KHRONOS_validation\0",
            )];
            let layers_names_raw: Vec<*const std::ffi::c_char> = layer_names
                .iter()
                .map(|raw_name| raw_name.as_ptr())
                .collect();

            let create_info = vk::InstanceCreateInfo::default()
                .application_info(&app_info)
                .enabled_layer_names(&layers_names_raw)
                .enabled_extension_names(&instance_extensions)
                .flags(create_flags);
            
            let instance = entry.create_instance(&create_info, None)
                .expect("Unable to create instance!");

            let devices = instance.enumerate_physical_devices()
                .expect("Unable to enumerate devices!");
            
            let mut physical_device: Option<vk::PhysicalDevice> = None;
            for device in devices{
                let extensions =
                    instance.enumerate_device_extension_properties(device)
                        .expect("Unable to enumerate extensions!");
                for extension in extensions{
                    if extension.extension_name_as_c_str()
                        .unwrap()
                        .eq(vk::KHR_VIDEO_ENCODE_H264_NAME){
                        physical_device = Some(device);                                                                                      
                    }
                }
            }
            
            let device_extensions = [
                vk::KHR_SAMPLER_YCBCR_CONVERSION_NAME.as_ptr(),
                vk::KHR_VIDEO_ENCODE_H264_NAME.as_ptr(),
                vk::KHR_VIDEO_ENCODE_QUEUE_NAME.as_ptr(),
                vk::KHR_VIDEO_QUEUE_NAME.as_ptr()
            ];
            
            let features = vk::PhysicalDeviceFeatures::default();
            
            // This is the first time we use the device so if it fails we assume we didn't fine one
            let queue_family = 
                instance.get_physical_device_queue_family_properties(physical_device
                .expect("Unable to find physical device with Video Encode!"))
                    .iter()
                    .enumerate()
                    .find_map(|(index, info)|{
                        let support_video = info.queue_flags.contains(
                            vk::QueueFlags::VIDEO_ENCODE_KHR | vk::QueueFlags::TRANSFER
                        );
                        if support_video{
                            Some(index as u32)
                        } else {
                            None
                        }
                    });
            
            let priorities = [1.0];
            let queue_info = vk::DeviceQueueCreateInfo::default()
                .queue_family_index(queue_family
                    .expect("Unable to find video encode family!"))
                .queue_priorities(&priorities);
            
            let device_create_info = vk::DeviceCreateInfo::default()
                .queue_create_infos(std::slice::from_ref(&queue_info))
                .enabled_extension_names(&device_extensions)
                .enabled_features(&features);
            
            let device = instance
                .create_device(physical_device.unwrap(), &device_create_info, None)
                .expect("Unable to create logical device!");

            let mut allocator_create_info = vk_mem::AllocatorCreateInfo::new(
                &instance, &device, physical_device.unwrap()
            );
            allocator_create_info.vulkan_api_version = vk::API_VERSION_1_3;
            let allocator = vk_mem::Allocator::new(allocator_create_info)
                .expect("Unable to create vk_mem allocator!");

            // Now that we have our device, let's start the encoder!

            let mut h264_profile = vk::VideoEncodeH264ProfileInfoKHR::default()
                .std_profile_idc(1);

            let profile = vk::VideoProfileInfoKHR::default()
                .chroma_subsampling(vk::VideoChromaSubsamplingFlagsKHR::TYPE_444)
                .luma_bit_depth(vk::VideoComponentBitDepthFlagsKHR::TYPE_8)
                .chroma_bit_depth(vk::VideoComponentBitDepthFlagsKHR::TYPE_8)
                .video_codec_operation(vk::VideoCodecOperationFlagsKHR::ENCODE_H264)
                .push_next(&mut h264_profile);

            let mut profiles = vk::VideoProfileListInfoKHR::default()
                .profiles(std::slice::from_ref(&profile));

            let encoding_instance = ash::khr::video_queue::Instance::new(&entry, &instance);
            let get_device_video_caps = encoding_instance.fp().get_physical_device_video_capabilities_khr;
            let get_device_formats = encoding_instance.fp().get_physical_device_video_format_properties_khr;

            let mut device_formats = vk::PhysicalDeviceVideoFormatInfoKHR::default()
                .image_usage(vk::ImageUsageFlags::VIDEO_ENCODE_SRC_KHR | vk::ImageUsageFlags::TRANSFER_DST)
                .push_next(&mut profiles);
            let mut property_count = 1;
            let mut video_format_properties = vk::VideoFormatPropertiesKHR::default();

            get_device_formats(physical_device.unwrap(), &device_formats, &mut property_count, &mut video_format_properties)
                .result().expect("Unable to get device formats!");

            let mut h264_caps = vk::VideoEncodeH264CapabilitiesKHR::default();

            let mut encode_caps = vk::VideoEncodeCapabilitiesKHR::default();

            let mut caps = vk::VideoCapabilitiesKHR::default()
                .push_next(&mut h264_caps)
                .push_next(&mut encode_caps);
            get_device_video_caps(physical_device.unwrap(), &profile, &mut caps)
                .result().expect("Unable to get device's video caps");

            let encode_device = ash::khr::video_encode_queue::Device::new(&instance, &device);
            let video_device = ash::khr::video_queue::Device::new(&instance, &device);
            // Create the image to interface with the encoder
            let create_info = vk_mem::AllocationCreateInfo{
                usage: vk_mem::MemoryUsage::AutoPreferDevice,
                required_flags: vk::MemoryPropertyFlags::DEVICE_LOCAL,
                ..Default::default()
            };

            let (mut image, mut image_alloc) = allocator
                .create_image(
                    &vk::ImageCreateInfo::default()
                        .extent(vk::Extent3D::default()
                            .width(settings.width)
                            .height(settings.height)
                            .depth(1))
                        .usage(ImageUsageFlags::TRANSFER_DST |
                        ImageUsageFlags::VIDEO_ENCODE_SRC_KHR)
                        //Use the supported image settings
                        .format(video_format_properties.format)
                        .image_type(video_format_properties.image_type)
                        .tiling(video_format_properties.image_tiling)
                        .samples(vk::SampleCountFlags::TYPE_1)
                        .mip_levels(1)
                        .array_layers(1)
                        .push_next(
                            &mut profiles
                        ),
                    &create_info
                ).expect("Unable to create image!");

            // Then it's image view
            let image_view_create_info = vk::ImageViewCreateInfo::default()
                .image(image)
                .format(vk::Format::G8_B8R8_2PLANE_444_UNORM)
                .components(vk::ComponentMapping::default())
                .subresource_range(vk::ImageSubresourceRange::default()
                    .aspect_mask(vk::ImageAspectFlags::COLOR)
                    .base_mip_level(0)
                    .base_array_layer(0)
                    .layer_count(vk::REMAINING_ARRAY_LAYERS)
                    .level_count(vk::REMAINING_MIP_LEVELS))
                .view_type(vk::ImageViewType::TYPE_2D);

            let image_view = device.create_image_view(
                &image_view_create_info, None
            ).expect("Unable to create image view!");

            // Next create the output to store the output of the video encoder
            let create_info = vk_mem::AllocationCreateInfo{
                usage: vk_mem::MemoryUsage::AutoPreferDevice,
                flags: vk_mem::AllocationCreateFlags::HOST_ACCESS_RANDOM,
                required_flags: vk::MemoryPropertyFlags::HOST_VISIBLE,
                ..Default::default()
            };

            // Finally the image transfer and output buffer
            let (output, mut output_alloc) = allocator
                .create_buffer(
                    &vk::BufferCreateInfo::default()
                        .size((settings.width * settings.height * 3) as u64)
                        .usage(vk::BufferUsageFlags::TRANSFER_SRC |
                            vk::BufferUsageFlags::VIDEO_ENCODE_DST_KHR)
                        .push_next(&mut profiles),
                    &create_info,
                )
                .unwrap();
            
            let image_map = allocator.map_memory(&mut output_alloc)
                .expect("Unable to map image!");
            
            let mut image_set: Vec<u8> = vec![];
            for x in 0..(settings.width*settings.height*3){
                image_set.push(255);
            }
            
            std::ptr::copy(image_set.as_ptr(), image_map, (settings.width*settings.height*4) as usize);
            
            allocator.unmap_memory(&mut output_alloc);
            

            let create_video_session = video_device.fp().create_video_session_khr;



            let video_session_create_info = vk::VideoSessionCreateInfoKHR::default()
                .queue_family_index(queue_family.unwrap())
                .video_profile(&profile)
                .max_coded_extent(vk::Extent2D::default()
                    .width(settings.width)
                    .height(settings.height))
                .picture_format(vk::Format::G8_B8R8_2PLANE_444_UNORM)
                .std_header_version(&caps.std_header_version);
            let mut video_session = vk::VideoSessionKHR::default();

            // Create the video session
            create_video_session(device.handle(), &video_session_create_info,
                                 std::ptr::null(), &mut video_session)
                .result().expect("Unable to create video session");

            let create_video_session_parameters = video_device.fp().create_video_session_parameters_khr;

            let mut h264_session = vk::VideoEncodeH264SessionParametersCreateInfoKHR::default()
                .max_std_pps_count(99)
                .max_std_sps_count(99);

            let video_session_parameters_create_info = vk::VideoSessionParametersCreateInfoKHR::default()
                .video_session(video_session)
                .flags(vk::VideoSessionParametersCreateFlagsKHR::default())
                .push_next(&mut h264_session);

            let mut video_session_params = vk::VideoSessionParametersKHR::default();

            create_video_session_parameters(device.handle(), &video_session_parameters_create_info,
            std::ptr::null(), &mut video_session_params)
                .result().expect("Unable to create video session parameters");

            // Now that video session has been created, video encoding commands can be sent to the GPU


            let command_poll_create_info = vk::CommandPoolCreateInfo::default()
                .queue_family_index(queue_family.unwrap())
                .flags(vk::CommandPoolCreateFlags::default());

            let command_pool = device.create_command_pool(&command_poll_create_info, None)
                .expect("Unable to create command pool!");

            let command_buffer_create_info = vk::CommandBufferAllocateInfo::default()
                .command_pool(command_pool)
                .command_buffer_count(1)
                .level(vk::CommandBufferLevel::PRIMARY);

            let video_queue = device.get_device_queue(queue_family.unwrap(), 0);
            
            let command_buffer = *device.allocate_command_buffers(
                &command_buffer_create_info,
            ).expect("Unable to create command buffer!").first()
                .expect("Unable to get single command buffer!");

            let command_buffer_begin = vk::CommandBufferBeginInfo::default();
            device.begin_command_buffer(command_buffer, &command_buffer_begin)
                .expect("Unable to begin command buffer!");

            let cmd_encode_video = encode_device.fp().cmd_encode_video_khr;
            
            let cmd_begin_video_coding = video_device.fp().cmd_begin_video_coding_khr;
            let cmd_end_video_coding = video_device.fp().cmd_end_video_coding_khr;

            let video_session_memory_khr = video_device.fp().bind_video_session_memory_khr;

            let video_session_memory_requirements = video_device.fp().get_video_session_memory_requirements_khr;

            let mut num_memory_requirements = 10;
            let mut memory_requirements: Vec<vk::VideoSessionMemoryRequirementsKHR> = vec![vk::VideoSessionMemoryRequirementsKHR::default(); 10];
            video_session_memory_requirements(device.handle(), video_session, &mut num_memory_requirements, memory_requirements.as_mut_ptr())
                .result().expect("Unable to get vulkan video memory requirements.");

            // ALLOCATE THE MEMORY NEEDED FOR DRIVER
            for i in 0..num_memory_requirements{
                let memory_requirement = memory_requirements[i as usize];
                println!("Memory Requirements: Size: {}, Alignment: {} Bits: {}",
                memory_requirement.memory_requirements.size, memory_requirement.memory_requirements.alignment,
                         memory_requirement.memory_requirements.memory_type_bits);

                let (mut required_buffer_alloc) = allocator
                    .allocate_memory(
                        &memory_requirement.memory_requirements,
                        &vk_mem::AllocationCreateInfo::default(),
                    )
                    .unwrap();

                video_session_memory_khr(device.handle(), video_session, 1, &vk::BindVideoSessionMemoryInfoKHR::default()
                    .memory(allocator.get_allocation_info(&required_buffer_alloc)
                        .device_memory)
                    .memory_offset(0)
                    .memory_size(memory_requirement.memory_requirements.size)
                    .memory_bind_index(i))
                    .result().expect("Unable to bind video session memory!");
            }
            let picture_resource = vk::VideoPictureResourceInfoKHR::default()
                .base_array_layer(0)
                .coded_offset(vk::Offset2D::default())
                .coded_extent(vk::Extent2D::default()
                    .width(settings.width)
                    .height(settings.height))
                .image_view_binding(image_view);
            
            let video_reference_slot = vk::VideoReferenceSlotInfoKHR::default()
                .picture_resource(&picture_resource)
                .slot_index(0);
            
            let video_coding_begin_info = vk::VideoBeginCodingInfoKHR::default()
                .video_session(video_session)
                .video_session_parameters(video_session_params);
            
            // Copy image buffer to image
            /*
            device.cmd_copy_buffer_to_image(command_buffer, output, image,
            vk::ImageLayout::VIDEO_DECODE_SRC_KHR, std::slice::from_ref(&vk::BufferImageCopy::default()
                    .buffer_offset(0)
                    .buffer_image_height(height)
                    .buffer_row_length(width)
                    .image_extent(vk::Extent3D::default()
                        .width(width)
                        .height(height)
                        .depth(1))
                    .image_offset(vk::Offset3D::default())
                    .image_subresource(vk::ImageSubresourceLayers::default()
                        .base_array_layer(0)
                        .layer_count(0)
                        .aspect_mask(vk::ImageAspectFlags::COLOR)
                        .mip_level(0))));
                        
             */


            cmd_begin_video_coding(command_buffer, &video_coding_begin_info);


            let frame_info = vk_utils::FrameInfo::new(
                0, settings.width, settings.height, vk_utils::get_std_video_h264_sequence_parameter_set(settings.width, settings.height, Some(&mut vk_utils::get_std_video_h264sequence_parameter_set_vui(30))),
                vk_utils::get_std_video_h264_picture_parameter_set(),0, true
            );

            let mut h264_info = frame_info.get_encoder_h264_frame_info().clone();

            let video_encode_info = vk::VideoEncodeInfoKHR::default()
                .dst_buffer(output)
                .dst_buffer_offset(0)
                .dst_buffer_range((settings.width * settings.height * 3) as u64)
                .preceding_externally_encoded_bytes(0)
                .src_picture_resource(picture_resource)
                .push_next(&mut h264_info);


            cmd_encode_video(command_buffer, &video_encode_info);

            let video_coding_end_info = vk::VideoEndCodingInfoKHR::default();
            cmd_end_video_coding(command_buffer, &video_coding_end_info);

            device.end_command_buffer(command_buffer)
                .expect("Unable to end command buffer!");

            let submit_info = vk::SubmitInfo::default()
                .command_buffers(std::slice::from_ref(&command_buffer));
            
            let fence_create_info = vk::FenceCreateInfo::default();
            
            let fence = device.create_fence(
                &fence_create_info, None
            ).expect("Unable to make fence!");
                
            device.queue_submit(video_queue, std::slice::from_ref(&submit_info), fence)
                .expect("Unable to submit queue!");



            Ok(VkEncoder{
                entry,
                instance,
                device,
                video_device,
                encode_device,
                allocator,

                image,
                image_view,
                image_alloc,
                output,
                output_alloc,

                video_session,
                video_session_params
            })
        }
    }

    fn convert_to_nal(frame: &[u8]) -> Vec<Vec<u8>> {
        todo!()
    }

    fn encode_frame(&mut self, frame: &Frame) -> Result<(), EEncoderError> {
        todo!();
        Ok(())
    }

    fn get_latest_packet(&mut self) -> Result<Vec<u8>, EEncoderError> {
        todo!();
        Ok(vec![])
    }

    fn flush_encoder(&mut self) {
        todo!()
    }

    fn destroy(&mut self) {
        unsafe {
            self.allocator.destroy_buffer(self.output, &mut self.output_alloc);
            self.allocator.destroy_image(self.image, &mut self.image_alloc);
            self.device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
    }
}