#[cfg(feature = "enable_swencoder")]
use openh264;
use crate::encoder::device::EncoderSettings;
use crate::frame::{EEncoding, EFrameType, EPixelFormat, Frame};
use super::device;

#[cfg(feature = "enable_swencoder")]
pub struct SwEncoder{
    encoder: Option<openh264::encoder::Encoder>
}


#[cfg(feature = "enable_swencoder")]
impl device::EncodeDevice for SwEncoder{
    fn is_supported() -> bool {
        // The SW encoder will ALWAYS be supported
        true
    }

    fn init(_settings: EncoderSettings) -> Self{
        // Encoded frames can be of any width and height, so these values are unused
        // If a doofus actually tried setting and settings, ignore the
        Self{
            encoder: Some(openh264::encoder::Encoder::new()
                .expect("Unable to build OpenH264 encoder"))
        }
    }

    fn convert_to_nal(frame: &[u8]) -> Vec<Vec<u8>> {
        let mut nal: Vec<Vec<u8>> = vec![];
        for nal_unit in openh264::nal_units(frame){
            nal.push(nal_unit.to_vec())
        }
        nal
    }

    fn encode_frame(&mut self, frame: &Frame) -> Result<Vec<u8>, String>{
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
                // Convert data to designated pixel format
                match frame.pixel_format{
                    EPixelFormat::RGB8 => {
                        let rgb_data =
                            openh264::formats::RgbSliceU8::new(frame.data[0].as_slice(),
                                                               (frame.width as usize,
                                                                frame.height as usize));
                        frame_data = openh264::formats::YUVBuffer::from_rgb_source(rgb_data);
                    }
                    _ => {
                        return Err(String::from("Unsupported Pixel Format"))
                    }
                }

                // The main thread will stall as the encoder encodes
                // TO-DO: separate the input into NALs so that the output can be sent ASAP
                let bitstream = self.encoder
                    .as_mut()
                    .expect("Unable to get encoder")
                    .encode(&frame_data)
                    .expect("Unable to encode video!");


                Ok(bitstream.to_vec())
            }
            _ => {
                Err(String::from("Unsupported Frame Type"))
            }
        }
    }

    fn destroy(&mut self) {
        // All libraries are rust so there's no need to destroy anything :)
    }
}