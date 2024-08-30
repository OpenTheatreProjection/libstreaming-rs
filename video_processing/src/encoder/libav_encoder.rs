#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::encoder::device::{EEncoderError, EncoderSettings};
use crate::encoder::device::EEncoderError::*;
use crate::frame::{EFrameType, EPixelFormat, Frame, EHardwareAcceleration};
use super::device;

#[cfg(feature = "enable_ffmpeg")]
include!(concat!(env!("OUT_DIR"), "/ffmpeg.rs"));

#[cfg(feature = "enable_ffmpeg")]
pub struct LibavEncoder{
    pub encoder: *const AVCodec,
    pub context: *mut AVCodecContext,
    pub settings: EncoderSettings,
    pub img_frame: *mut AVFrame,
    pub out_packet: *mut AVPacket
}

#[cfg(feature = "enable_ffmpeg")]
impl LibavEncoder{
    pub fn is_valid(&self) -> Result<(),()>{
        if self.encoder != std::ptr::null_mut(){
            return Ok(())
        }
        Err(())
    }
}

#[cfg(feature = "enable_ffmpeg")]
impl device::EncodeDevice for LibavEncoder{
    fn is_supported() -> bool {
        true
    }

    fn init(settings: EncoderSettings) -> Result<Self, EEncoderError> {
        unsafe {

            let mut encoder_name = String::from(match settings.hardware_acceleration{
                EHardwareAcceleration::CPU => {"libx264\0"}
                EHardwareAcceleration::NVIDIA => {"h264_nvenc\0"}
                EHardwareAcceleration::AMD => {"h264_amf\0"}
                EHardwareAcceleration::RASPBERRYPI => {"h264_v4l2m2m\0"}
                _ => {"libx264\0"}
            });
            encoder_name.make_ascii_lowercase();

            let encoder =
                avcodec_find_encoder_by_name(
                    encoder_name.as_ptr() as *const _
                );
            if encoder == std::ptr::null(){
                return Err(NoCodecSupport)
            }
            
            let context = avcodec_alloc_context3(encoder);
            if context == std::ptr::null_mut() {
                return Err(CouldntCreateContext)
            }

            (*context).bit_rate = settings.bitrate;
            (*context).width = settings.width as i32;
            (*context).height = settings.height as i32;
            (*context).time_base = AVRational{num: 1, den: settings.fps as i32};
            (*context).framerate = AVRational{num: settings.fps as i32, den: 1};
            (*context).gop_size = 10;
            (*context).max_b_frames = 0;
            (*context).pix_fmt =
                match settings.pixel_format{
                    EPixelFormat::RGB8 => {AVPixelFormat_AV_PIX_FMT_RGB8}
                    EPixelFormat::YUV420 => { AVPixelFormat_AV_PIX_FMT_YUV420P }
                    _ => { AVPixelFormat_AV_PIX_FMT_RGB8 }};
            
            if avcodec_open2(context, encoder, std::ptr::null_mut()) < 0{
                return Err(CouldntCreateEncoder)
            }

            av_opt_set((*context).priv_data, String::from("tune\0").to_ascii_lowercase().as_ptr() as *const _,
                       String::from("zerolatency\0").to_ascii_lowercase().as_ptr() as *const _, 0);

            av_opt_set((*context).priv_data, String::from("preset\0").to_ascii_lowercase().as_ptr() as *const _,
                       String::from("ultrafast\0").to_ascii_lowercase().as_ptr() as *const _, 0);


            let img_frame = av_frame_alloc();
            let out_packet = av_packet_alloc();

            Ok(Self {
                encoder,
                context,
                settings,
                img_frame,
                out_packet
            })
        }
    }

    fn convert_to_nal(frame: &[u8]) -> Vec<Vec<u8>> {
        todo!()
    }

    fn encode_frame(&mut self, frame: &Frame) -> Result<(), EEncoderError> {
        unsafe {

            (*self.img_frame).format = (match frame.pixel_format{
                EPixelFormat::RGB8 => {Some(AVPixelFormat_AV_PIX_FMT_RGB8)}
                EPixelFormat::YUV420 => { Some(AVPixelFormat_AV_PIX_FMT_YUV420P) }
                _ => {
                    None
                }
            }).expect("Invalid Pixel Format!");

            (*self.img_frame).width = frame.width as i32;
            (*self.img_frame).height = frame.height as i32;

            if av_frame_get_buffer(self.img_frame, 0) < 0 {
                return Err(CouldntReceiveFrame)
            }

            if av_frame_make_writable(self.img_frame) < 0 {
                return Err(CouldntGetFrameBuffer)
            }

            match frame.frame_type {
                EFrameType::CPU => {
                    // Work with the CPU frames
                    match frame.pixel_format {
                        EPixelFormat::RGB8 => { ( * self.img_frame).data[0].copy_from(frame.data[0].as_ptr(),
                                                                                      (( * self.img_frame).linesize[0] * frame.height as i32) as usize); }
                        EPixelFormat::YUV420 => {
                            ( * self.img_frame).data[0].copy_from(frame.data[0].as_ptr(),
                                                                  (frame.width * frame.height) as usize);
                            ( *self.img_frame).data[1].copy_from(frame.data[1].as_ptr(),
                                                                 (frame.width / 2 * frame.height / 2) as usize);
                            ( * self.img_frame).data[2].copy_from(frame.data[2].as_ptr(),
                                                                  (frame.width / 2 * frame.height / 2) as usize);

                        }
                        _ => {
                            return Err(UnsupportedImageFormat);
                        }
                    }
                }
                _ => {
                    return Err(UnsupportedImageDevice);
                }
            }

            (*self.img_frame).pts = frame.index as i64;

            if avcodec_send_frame(self.context, self.img_frame) < 0 {
                // TO-DO: Identify actual errors VS random codes
                //return Err(CouldntEncodeFrame)
            }
            Ok(())
        }
    }

    fn get_latest_packet(&mut self) -> Result<Vec<u8>, EEncoderError> {
        unsafe {
            if avcodec_receive_packet(self.context, self.out_packet) < 0 {
                return Err(FrameNotReady)
            }

            let mut vec_output: Vec<u8> = vec![];
            vec_output.reserve((*self.out_packet).size as usize);
            vec_output.set_len((*self.out_packet).size as usize);
            vec_output.copy_from_slice(std::slice::from_raw_parts((*self.out_packet).data, (*self.out_packet).size as usize));
            Ok(vec_output)
        }
    }

    fn flush_encoder(&mut self) {
        unsafe{
            avcodec_send_frame(self.context, std::ptr::null_mut());
        }
    }

    fn destroy(&mut self) {
        todo!()
    }
}