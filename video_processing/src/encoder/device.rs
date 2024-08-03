use crate::frame;

pub trait EncodeDevice {
    // Check if encoder is supported, if it is not and then init is called,
    // it will *probably* output a broken encoder
    fn is_supported() -> bool;

    // The Init Function for the encoder, sets the width and height of the encoder
    // Outputs the encoder
    fn init(width: u32, height: u32) -> Self;

    // Require all devices to be able to process NALs
    fn convert_to_nal(frame: &[u8]) -> Vec<Vec<u8>>;

    // Encode the passed frame using the currently configured encoder
    fn encode_frame(&mut self, frame: &frame::Frame) -> Result<Vec<u8>, String>;

    fn destroy(&mut self);
}