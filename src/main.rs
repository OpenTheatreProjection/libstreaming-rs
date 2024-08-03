use video_processing::*;
use video_processing::encoder::device::EncodeDevice;
use video_processing::decoder::device::DecodeDevice;

fn main() {
    let mut encoder = encoder::sw_encoder::SwEncoder::init(800,600);

    let mut decoder = decoder::sw_decoder::SwDecoder::init(800,600);

    let mut frame = frame::Frame{
        frame_type: frame::EFrameType::CPU,
        pixel_format: frame::EPixelFormat::RGB8,
        codec: frame::EEncoding::H264,
        width: 800,
        height: 600,
        data: Vec::from([255u8; 800*600*3]),
        data_ptr: 0,
    };

    let encoded_output = encoder.encode_frame(&frame)
        .expect("Unable to encode frame!");

    frame.data = encoded_output;

    let decoded_output = decoder.decode_frame(&frame)
        .expect("Unable to decode frame!");

    println!("Finished!");
}
