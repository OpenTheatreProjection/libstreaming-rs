#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_imports)]


#[cfg(feature = "enable_nvenc")]
include!(concat!(env!("OUT_DIR"), "/nvenc.rs"));

use crate::frame::{EEncoding, Frame};
use super::device;
use crate::encoder::device::EncoderSettings;

#[cfg(feature = "enable_nvenc")]
use nvidia_video_codec_sdk;
#[cfg(feature = "enable_nvenc")]
use nvidia_video_codec_sdk::sys::nvEncodeAPI::*;
#[cfg(feature = "enable_nvenc")]
use nvidia_video_codec_sdk::sys::nvEncodeAPI::_NV_ENC_DEVICE_TYPE::NV_ENC_DEVICE_TYPE_CUDA;


#[cfg(feature = "enable_nvenc")]
pub struct NvencEncoder{

}
#[cfg(feature = "enable_nvenc")]
impl NvencEncoder{
    fn ck(status: NVENCSTATUS){
        
    }

    fn create_encoder_config(settings: device::EncoderSettings) -> NV_ENC_INITIALIZE_PARAMS{
        let mut profile: NV_ENC_INITIALIZE_PARAMS = Default::default();

        profile.version = NV_ENC_INITIALIZE_PARAMS_VER;

        // TO-DO: Make this customizable
        profile.encodeGUID = match settings.codec{
            EEncoding::H264 => {NV_ENC_CODEC_H264_GUID}
            EEncoding::H265 => {NV_ENC_CODEC_HEVC_GUID}
            EEncoding::AV1 => {NV_ENC_CODEC_AV1_GUID}
            _ => {NV_ENC_H264_PROFILE_MAIN_GUID}
        };

        // TO-DO: Let this be customizable
        profile.presetGUID = NV_ENC_PRESET_P4_GUID;

        profile.encodeWidth = settings.width;
        profile.encodeHeight = settings.height;
        profile.darWidth = settings.width;
        profile.darHeight = settings.height;
        profile.frameRateNum = settings.fps;
        profile.frameRateDen = 1;
        // Allow modifications maybe?
        profile.enablePTD = 1;
        profile.set_reportSliceOffsets(0);
        profile.set_enableSubFrameWrite(0);
        profile.maxEncodeWidth = settings.width;
        profile.encodeHeight = settings.height;
        // TO-DO: Change this to allow for motion estimation
        profile.set_enableMEOnlyMode(0);
        // TO-DO: Allow output somewhere other than the CPU
        profile.set_enableOutputInVidmem(0);

        profile
    }
}

#[cfg(feature = "enable_nvenc")]
impl device::EncodeDevice for NvencEncoder{
    fn is_supported() -> bool {
        true
    }

    fn init(settings: EncoderSettings) -> Self {
        unsafe{
            let mut context: CUcontext = std::ptr::null_mut();
            let mut device: CUdevice = 0;
            let mut encoder: *mut std::ffi::c_void = std::ptr::null_mut();
            cuInit(0);
            cuDeviceGet(&mut device, 0);
            cuCtxCreate_v2(&mut context, 0,device );

            let mut encoder_func = NV_ENCODE_API_FUNCTION_LIST{
                version: NV_ENCODE_API_FUNCTION_LIST_VER,
                ..Default::default()
            };

            NvEncodeAPICreateInstance(&mut encoder_func);
            
            let mut open_encode_info = NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS{
                version: NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS_VER,
                deviceType: _NV_ENC_DEVICE_TYPE::from(NV_ENC_DEVICE_TYPE_CUDA),
                device: context as *mut _,
                reserved: std::ptr::null_mut(),
                apiVersion: 0,
                reserved1: [0u32; 253],
                reserved2: [std::ptr::null_mut(); 64],
            };
            encoder_func.nvEncOpenEncodeSessionEx.unwrap()(&mut open_encode_info, &mut encoder
             as *mut *mut _);
            
            let mut encoder_config = Self::create_encoder_config(settings.clone());
            
            encoder_func.nvEncInitializeEncoder.unwrap()(encoder, &mut encoder_config);

            let mut input_ptr: NV_ENC_INPUT_PTR = std::ptr::null_mut();
            
            let mut input_buffer_params = NV_ENC_CREATE_INPUT_BUFFER{
                version: NV_ENC_CREATE_INPUT_BUFFER_VER,
                width: settings.width,
                height: settings.height,
                memoryHeap: _NV_ENC_MEMORY_HEAP::NV_ENC_MEMORY_HEAP_AUTOSELECT,
                bufferFmt: _NV_ENC_BUFFER_FORMAT::NV_ENC_BUFFER_FORMAT_UNDEFINED,
                reserved: 0,
                inputBuffer: input_ptr,
                pSysMemBuffer: std::ptr::null_mut(),
                reserved1: [0u32;57],
                reserved2: [std::ptr::null_mut(); 63],
            };
            encoder_func.nvEncCreateInputBuffer.unwrap()(encoder, &mut input_buffer_params);
            println!("Did thing!");
        }
        Self{}
    }

    fn convert_to_nal(frame: &[u8]) -> Vec<Vec<u8>> {
        vec![]
    }

    fn encode_frame(&mut self, frame: &Frame) -> Result<Vec<u8>, String> {
        Ok(vec![])
    }

    fn destroy(&mut self) {
        todo!()
    }
}