#[derive(Clone)]

pub enum EFrameType{
    CPU,
    DXD11,
    DXD12,
    VULKAN,
    OPENGL,
    UNDEFINED
}
#[derive(Clone)]

pub enum EPixelFormat{
    RGB8,
    YUV8,
    UNDEFINED
}
#[derive(Clone)]

pub enum EEncoding{
    H264,
    H265,
    AV1,
    UNDEFINED
}

#[derive(Clone)]
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
    pub data: Vec<u8>,
    // Pointer to GPU Data
    pub data_ptr: u64
}

impl Frame{

}