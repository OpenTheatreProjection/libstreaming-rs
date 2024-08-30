use crate::frame;

#[derive(Default, Debug, Copy, Clone)]
pub enum EEncoderError{
    #[default]
    NoError = 0,
    NoCodecSupport = 1,
    CouldntCreateContext = 2,
    CouldntCreateEncoder = 3,
    UnsupportedImageFormat = 4,
    UnsupportedImageDevice = 5,
    CouldntReceiveFrame = 6,
    CouldntGetFrameBuffer = 7,
    CouldntEncodeFrame = 8,
    FrameNotReady = 9,
}
pub trait EncodeDevice {
    // Check if encoder is supported, if it is not and then init is called,
    // it will *probably* output a broken encoder
    fn is_supported() -> bool;

    // The Init Function for the encoder, sets the width and height of the encoder
    // Outputs the encoder
    fn init(settings: EncoderSettings) -> Result<Self, EEncoderError> where Self: Sized;

    // Require all devices to be able to process NALs
    fn convert_to_nal(frame: &[u8]) -> Vec<Vec<u8>>;

    // Encode the passed frame using the currently configured encoder
    fn encode_frame(&mut self, frame: &frame::Frame) -> Result<(), EEncoderError>;

    // Get the latest decoded frame
    fn get_latest_packet(&mut self) -> Result<Vec<u8>, EEncoderError>;

    fn flush_encoder(&mut self);

    fn destroy(&mut self);
}

#[derive(Default, Clone, Copy)]
pub struct EncoderSettings{
    pub codec: frame::EEncoding,
    pub pixel_format: frame::EPixelFormat,
    pub frame_type: frame::EFrameType,
    pub hardware_acceleration: frame::EHardwareAcceleration,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub bitrate: i64
}