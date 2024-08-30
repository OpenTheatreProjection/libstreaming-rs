#[derive(Default, Clone, Copy)]
pub enum EFrameType{
    #[default]
    CPU,
    DXD11,
    DXD12,
    VULKAN,
    OPENGL,
    UNDEFINED
}
#[derive(Default, Clone, Copy)]
pub enum EPixelFormat{
    RGB8,
    #[deprecated]
    YUV8,
    #[default]
    YUV420,
    UNDEFINED
}
#[derive(Default, Clone, Copy)]
pub enum EEncoding{
    #[default]
    H264,
    H265,
    AV1,
    UNDEFINED
}

#[derive(Default, Clone, Copy)]
pub enum EHardwareAcceleration{
    #[default]
    CPU,
    NVIDIA,
    AMD,
    RASPBERRYPI
}

#[derive(Default, Clone)]
pub struct Frame{
    // Location of the frame in memory
    pub frame_type: EFrameType,
    // For Encoder: Current Pixel Format
    // For Decoder: Target Pixel Format
    pub pixel_format: EPixelFormat,
    // For Encoder: Target encoding
    // For Decoder: Current Encoding
    pub codec: EEncoding,
    pub width: u32,
    pub height: u32,
    // Only Defined for CPU Frames
    pub data: Vec<Vec<u8>>,
    // Pointer to GPU Data
    pub data_ptr: u64,
    // The frame count in the video
    pub index: i32,
}

impl Frame{

}