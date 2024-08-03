use crate::frame;

pub trait DecodeDevice {
    // Check if the decoder is supported, if not, then init is called
    // it will *probably* output a broken decoder
    fn is_supported() -> bool;

    // The Init Function for the decoder, sets the width and height of the decoder
    // Outputs the decoder
    fn init(width: u32, height: u32) -> Self;

    // Decode the passed frame to the selected encoder
    fn decode_frame(&mut self, frame: &frame::Frame) -> Result<Vec<u8>, String>;

    fn destroy(&mut self);
}