use video_processing::*;
use video_processing::encoder::device::{EncodeDevice};
use video_processing::decoder::device::DecodeDevice;
use video_processing::frame::{EEncoding, EFrameType, EPixelFormat,EHardwareAcceleration, Frame};

fn main() {

    let settings = encoder::device::EncoderSettings{
        codec: EEncoding::H264,
        pixel_format: EPixelFormat::YUV420,
        frame_type: EFrameType::CPU,
        hardware_acceleration: EHardwareAcceleration::CPU,
        width: 800,
        height: 600,
        fps: 120,
        bitrate: 40000,
    };
    let mut encoder =
        encoder::vk_encoder::VkEncoder::init(settings.clone())
            .expect("Unable to get encoder!");

    //encoder.is_valid().expect("Invalid Encoder!");

    let mut decoder = decoder::sw_decoder::SwDecoder::init(800,600);
    let mut frame= frame::Frame {
        frame_type: frame::EFrameType::CPU,
        pixel_format: frame::EPixelFormat::YUV420,
        codec: frame::EEncoding::H264,
        width: 800,
        height: 600,
        data: vec!(vec![200u8; 800 * 600], vec![200u8; 800 * 600], vec![200u8; 800 * 600]),
        data_ptr: 0,
        index: 0,
    };

    for i in 0..200 {
        frame.index = i;
        frame.data = vec!(vec![200u8; 800 * 600], vec![200u8; 800 * 600], vec![200u8; 800 * 600]);

        for y in 0..600{
            for x in 0..800{
                frame.data[0][y*800 + x] = ((20/(i+1)) +i + 9) as u8;
            }
        }

        for y in 0..300{
            for x in 0..400{
                frame.data[1][y*400 + x] = ((14/(i+1)) +i +49) as u8;
                frame.data[2][y*400 + x] = ((69/(i+1)) +i+27) as u8;
            }
        }

        frame.pixel_format = EPixelFormat::YUV420;

        encoder.encode_frame(&frame)
            .expect("Unable to encode frame");

        frame.pixel_format = EPixelFormat::RGB8;


        let mut encoded_output = Ok(vec![]);
        while encoded_output.is_ok() {
            encoded_output = encoder.get_latest_packet();
            if (encoded_output.is_ok()) {
                frame.data = vec![encoded_output.clone().unwrap()];

                // Decoded output *might* be invalid
                let decoded_output = decoder.decode_frame(&frame);
                if decoded_output.is_ok() {
                    println!("Successfully got a decoded frame: {}", i);
                    println!("Frame Size: {}", decoded_output.unwrap().len());
                } else {
                    println!("Failed to get decoded frame: {}", decoded_output.err().unwrap());
                }
            } else {
                println!("Frame not ready!");
            }
        }
    }

    encoder.flush_encoder();

    for i in 0..50{
        // Lets get the output in RGB!
    }


    println!("Finished!");
}
