use ash;
use ash::vk;

/*
NOTICE: File contains code adapted from the following repository
https://github.com/clemy/vulkan-video-encode-simple/

THEIR LICENSE:
Copyright (c) 2024 Bernhard C. Schrenk <clemy@clemy.org>
Copyright (c) 2024 Helmut Hlavacs <helmut.hlavacs@univie.ac.at>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
 */
pub const H264_MB_SIZE_ALIGNMENT: usize = 16;
const STD_VIDEO_H264_NO_REFERENCE_PICTURE: u8 = 0xFF;
pub fn align_size(size: usize, alignment: usize) -> usize {
    (size + alignment - 1) & !(alignment - 1)
}

pub fn get_std_video_h264sequence_parameter_set_vui(fps: u32)
    -> vk::native::StdVideoH264SequenceParameterSetVui{
    let mut vui_flags = vk::native::StdVideoH264SpsVuiFlags{
        _bitfield_align_1: [],
        _bitfield_1: Default::default(),
        __bindgen_padding_0: 0,
    };
    vui_flags.set_timing_info_present_flag(1);
    vui_flags.set_fixed_frame_rate_flag(1);

    let mut vui = vk::native::StdVideoH264SequenceParameterSetVui{
        flags: vui_flags,
        aspect_ratio_idc: 0,
        sar_width: 0,
        sar_height: 0,
        video_format: 0,
        colour_primaries: 0,
        transfer_characteristics: 0,
        matrix_coefficients: 0,
        num_units_in_tick: 1, // IMPORTANT
        time_scale: fps * 2, // IMPORTANT
        max_num_reorder_frames: 0,
        max_dec_frame_buffering: 0,
        chroma_sample_loc_type_top_field: 0,
        chroma_sample_loc_type_bottom_field: 0,
        reserved1: 0,
        pHrdParameters: std::ptr::null(),
    };

    vui
}

pub fn get_std_video_h264_sequence_parameter_set(width: u32, height: u32,
                                             p_vui: Option<&mut vk::native::StdVideoH264SequenceParameterSetVui>)
-> vk::native::StdVideoH264SequenceParameterSet{
    let mut sps_flags = vk::native::StdVideoH264SpsFlags{
        _bitfield_align_1: [],
        _bitfield_1: Default::default(),
        __bindgen_padding_0: 0,
    };
    sps_flags.set_direct_8x8_inference_flag(1);
    sps_flags.set_frame_mbs_only_flag(1);
    sps_flags.set_vui_parameters_present_flag(if p_vui.is_some() { 1 } else { 0 });

    let mb_aligned_width = align_size(width as usize, H264_MB_SIZE_ALIGNMENT);
    let mb_aligned_height = align_size(height as usize, H264_MB_SIZE_ALIGNMENT);

    let mut sps = vk::native::StdVideoH264SequenceParameterSet{
        flags: sps_flags,
        profile_idc: vk::native::StdVideoH264ProfileIdc_STD_VIDEO_H264_PROFILE_IDC_MAIN,
        level_idc: vk::native::StdVideoH264LevelIdc_STD_VIDEO_H264_LEVEL_IDC_4_1,
        chroma_format_idc: vk::native::StdVideoH264ChromaFormatIdc_STD_VIDEO_H264_CHROMA_FORMAT_IDC_420,
        seq_parameter_set_id: 0,
        bit_depth_luma_minus8: 0,
        bit_depth_chroma_minus8: 0,
        log2_max_frame_num_minus4: 0,
        pic_order_cnt_type: vk::native::StdVideoH264PocType_STD_VIDEO_H264_POC_TYPE_0,
        offset_for_non_ref_pic: 0,
        offset_for_top_to_bottom_field: 0,
        log2_max_pic_order_cnt_lsb_minus4: 4,
        num_ref_frames_in_pic_order_cnt_cycle: 0,
        max_num_ref_frames: 1,
        reserved1: 0,
        pic_width_in_mbs_minus1: (mb_aligned_width / H264_MB_SIZE_ALIGNMENT - 1) as u32,
        pic_height_in_map_units_minus1: (mb_aligned_height / H264_MB_SIZE_ALIGNMENT - 1) as u32,
        frame_crop_left_offset: 0,
        frame_crop_right_offset: mb_aligned_width as u32 - width,
        frame_crop_top_offset: 0,
        frame_crop_bottom_offset: mb_aligned_height as u32 - height,
        reserved2: 0,
        pOffsetForRefFrame: std::ptr::null(),
        pScalingLists: std::ptr::null(),
        pSequenceParameterSetVui: if p_vui.is_some() { p_vui.unwrap() } else { std::ptr::null() },
    };

    if sps.frame_crop_right_offset != 0 || sps.frame_crop_bottom_offset != 0 {
        sps.flags.set_frame_cropping_flag(1);

        if sps.chroma_format_idc == vk::native::StdVideoH264ChromaFormatIdc_STD_VIDEO_H264_CHROMA_FORMAT_IDC_420{
            sps.frame_crop_right_offset >>= 1;
            sps.frame_crop_bottom_offset >>= 1;
        }
    }

    sps
}

pub fn get_std_video_h264_picture_parameter_set() -> vk::native::StdVideoH264PictureParameterSet{
    let mut pps_flags = vk::native::StdVideoH264PpsFlags{
        _bitfield_align_1: [],
        _bitfield_1: Default::default(),
        __bindgen_padding_0: [0u8;3],
    };

    pps_flags.set_transform_8x8_mode_flag(0);
    pps_flags.set_constrained_intra_pred_flag(0);
    pps_flags.set_deblocking_filter_control_present_flag(1);
    pps_flags.set_entropy_coding_mode_flag(1);


    let mut pps = vk::native::StdVideoH264PictureParameterSet{
        flags: pps_flags,
        seq_parameter_set_id: 0,
        pic_parameter_set_id: 0,
        num_ref_idx_l0_default_active_minus1: 0,
        num_ref_idx_l1_default_active_minus1: 0,
        weighted_bipred_idc: 0,
        pic_init_qp_minus26: 0,
        pic_init_qs_minus26: 0,
        chroma_qp_index_offset: 0,
        second_chroma_qp_index_offset: 0,
        pScalingLists: std::ptr::null(),
    };

    pps
}

pub struct FrameInfo<'a>{
    slice_header_flags: vk::native::StdVideoEncodeH264SliceHeaderFlags,
    slice_header: vk::native::StdVideoEncodeH264SliceHeader,
    slice_info: vk::VideoEncodeH264NaluSliceInfoKHR<'a>,
    picture_info_flags: vk::native::StdVideoEncodeH264PictureInfoFlags,
    std_picture_info: vk::native::StdVideoEncodeH264PictureInfo,
    encode_h264_frame_info: vk::VideoEncodeH264PictureInfoKHR<'a>,
    reference_lists: vk::native::StdVideoEncodeH264ReferenceListsInfo,
}

impl<'a> FrameInfo<'a>{
    pub fn new(frame_count: u32, width: u32, height: u32, sps: vk::native::StdVideoH264SequenceParameterSet,
    pps: vk::native::StdVideoH264PictureParameterSet, gop_frame_count: u32, use_constant_qp: bool)
    -> Self{
        let is_i: bool = gop_frame_count == 0;
        let max_pic_order_cnt_lsb = 1 << (sps.log2_max_pic_order_cnt_lsb_minus4 + 4);

        let mut slice_header_flags = vk::native::StdVideoEncodeH264SliceHeaderFlags{
            _bitfield_align_1: [],
            _bitfield_1: Default::default(),
        };
        slice_header_flags.set_direct_spatial_mv_pred_flag(1);
        slice_header_flags.set_num_ref_idx_active_override_flag(0);

        let mut slice_header = vk::native::StdVideoEncodeH264SliceHeader{
            flags: slice_header_flags.clone(),
            first_mb_in_slice: 0,
            slice_type: if is_i { vk::native::StdVideoH264SliceType_STD_VIDEO_H264_SLICE_TYPE_I}
                        else { vk::native::StdVideoH264SliceType_STD_VIDEO_H264_SLICE_TYPE_P},
            slice_alpha_c0_offset_div2: 0,
            slice_beta_offset_div2: 0,
            slice_qp_delta: 0,
            reserved1: 0,
            cabac_init_idc: 0,
            disable_deblocking_filter_idc: 0,
            pWeightTable: std::ptr::null(),
        };

        let pic_width_in_mbs = sps.pic_width_in_mbs_minus1+1;
        let pic_height_in_mbs = sps.pic_height_in_map_units_minus1+1;
        let i_pic_size_in_mbs = pic_width_in_mbs * pic_height_in_mbs;

        let slice_info = vk::VideoEncodeH264NaluSliceInfoKHR::default()
            .std_slice_header(&slice_header.clone())
            .constant_qp(if use_constant_qp { (pps.pic_init_qp_minus26 + 26) as i32 } else {0});

        let mut picture_info_flags = vk::native::StdVideoEncodeH264PictureInfoFlags{
            _bitfield_align_1: [],
            _bitfield_1: Default::default(),
        };
        picture_info_flags.set_IdrPicFlag(if is_i {1} else {0});
        picture_info_flags.set_is_reference(1);
        picture_info_flags.set_adaptive_ref_pic_marking_mode_flag(0);
        picture_info_flags.set_no_output_of_prior_pics_flag(if is_i {1} else {0});

        let mut std_picture_info = vk::native::StdVideoEncodeH264PictureInfo{
            flags: picture_info_flags.clone(),
            seq_parameter_set_id: 0,
            pic_parameter_set_id: pps.clone().pic_parameter_set_id.clone(),
            idr_pic_id: 0,
            primary_pic_type: if is_i { vk::native::StdVideoH264PictureType_STD_VIDEO_H264_PICTURE_TYPE_IDR}
                            else { vk::native::StdVideoH264PictureType_STD_VIDEO_H264_PICTURE_TYPE_P},
            frame_num: frame_count,
            PicOrderCnt: ((frame_count*2) % max_pic_order_cnt_lsb) as i32,
            temporal_id: 0,
            reserved1: [0;3],
            pRefLists: std::ptr::null(),
        };

        let mut reference_lists = vk::native::StdVideoEncodeH264ReferenceListsInfo{
            flags: vk::native::StdVideoEncodeH264ReferenceListsInfoFlags{
                _bitfield_align_1: [],
                _bitfield_1: Default::default(),
            },
            num_ref_idx_l0_active_minus1: 0,
            num_ref_idx_l1_active_minus1: 0,
            RefPicList0: [0;32],
            RefPicList1: [0;32],
            refList0ModOpCount: 0,
            refList1ModOpCount: 0,
            refPicMarkingOpCount: 0,
            reserved1: [0;7],
            pRefList0ModOperations: std::ptr::null(),
            pRefList1ModOperations: std::ptr::null(),
            pRefPicMarkingOperations: std::ptr::null(),
        };


        // Shouldn't fill the entire array, TO-DO fix
        reference_lists.RefPicList0.fill(STD_VIDEO_H264_NO_REFERENCE_PICTURE);
        reference_lists.RefPicList1.fill(STD_VIDEO_H264_NO_REFERENCE_PICTURE);

        if !is_i{
            reference_lists.RefPicList0[0] = !(gop_frame_count & 1 != 0) as u8;
        }

        std_picture_info.pRefLists = &reference_lists.clone();

        let encode_h264_frame_info = vk::VideoEncodeH264PictureInfoKHR::default()
            .std_picture_info(&std_picture_info.clone())
            .nalu_slice_entries(std::slice::from_ref(&slice_info.clone()).clone());


        Self{
            slice_header_flags: slice_header_flags.clone(),
            slice_header: slice_header.clone(),
            slice_info: slice_info.clone(),
            picture_info_flags: picture_info_flags.clone(),
            std_picture_info: std_picture_info.clone(),
            encode_h264_frame_info: encode_h264_frame_info.clone(),
            reference_lists: reference_lists.clone(),
        }
    }

    pub fn get_encoder_h264_frame_info(&self) -> vk::VideoEncodeH264PictureInfoKHR{
        self.encode_h264_frame_info
    }
}