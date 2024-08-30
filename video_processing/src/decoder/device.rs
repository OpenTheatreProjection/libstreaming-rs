use crate::frame;

pub trait DecodeDevice {
    // Check if the decoder is supported, if not, then init is called
    // it will *probably* output a broken decoder
    fn is_supported() -> bool;

    // The Init Function for the decoder, sets the width and height of the decoder
    // Outputs the decoder
    fn init(width: u32, height: u32) -> Self;

    // Require all devices to be able to process NALs
    fn convert_to_nal(frame: &[u8]) -> Vec<Vec<u8>>;

    // Decode the passed frame to the selected encoder
    fn decode_frame(&mut self, frame: &frame::Frame) -> Result<Vec<u8>, String>;

    fn destroy(&mut self);
}

#[derive(Default, Clone, Copy)]
pub struct DecoderSettings{
    pub codec: frame::EEncoding,
    pub target_pixel_format: frame::EPixelFormat,
    pub hardware_acceleration: frame::EHardwareAcceleration,
    pub fps: u32,
}