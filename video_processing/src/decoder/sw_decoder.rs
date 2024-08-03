use super::device;
use openh264;
use crate::frame::{EEncoding, EFrameType, EPixelFormat, Frame};

pub struct SwDecoder{
    decoder: Option<openh264::decoder::Decoder>
}

impl device::DecodeDevice for SwDecoder{
    fn is_supported() -> bool {
        true
    }

    fn init(width: u32, height: u32) -> Self {
        // The SW Decoder is ALWAYS supported, therefore this will always return a valid object
        (Self{
            decoder: Some(openh264::decoder::Decoder::new()
                .expect("Unable to create OpenH264 decoder!"))
        })
    }

    fn convert_to_nal(frame: &[u8]) -> Vec<Vec<u8>> {
        let mut nal: Vec<Vec<u8>> = vec![];
        for nal_unit in openh264::nal_units(frame){
            nal.push(nal_unit.to_vec())
        }
        nal
    }

    fn decode_frame(&mut self, frame: &Frame) -> Result<Vec<u8>, String> {
        // Encoder MUST have the frame in YUV
        let frame_data: openh264::formats::YUVBuffer;

        match frame.codec{
            EEncoding::H264 => {
                // Only h264 supported, no need to do any preparation
            }
            _ => {
                // If it is not one of the above codecs, assume unsupported!
                return Err(String::from("Codec not supported!"))
            }
        }
        // Check the device the image is on
        match frame.frame_type{
            EFrameType::CPU => {

                // The main thread will stall as the encoder encodes
                // TO-DO: separate the input into NALs so that the output can be sent ASAP
                let bitstream = self.decoder
                    .as_mut()
                    .expect("Unable to get decoder")
                    .decode(&frame.data)
                    .expect("Unable to decode video!");

                // Convert data to designated pixel format
                return match frame.pixel_format{
                    EPixelFormat::RGB8 => {
                        let mut frame_out: Vec<u8> = vec![0; (frame.width * frame.height * 3) as usize];
                        //let mut frame_heap = frame_out.into_boxed_slice();
                        bitstream.expect("Unable to get decoded bitstream")
                            .write_rgb8(frame_out.as_mut());
                        Ok(frame_out)
                    }
                    _ => {
                        Err(String::from("Unsupported Pixel Format"))
                    }
                }
            }
            _ => {
                Err(String::from("Unsupported Frame Type"))
            }
        }
    }

    fn destroy(&mut self) {

    }
}