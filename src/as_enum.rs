// Automatically generated by build.rs
use crate::consts;
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u32)]
pub enum DrmFourcc {
    Abgr1555 = consts::DRM_FOURCC_ABGR1555,
    Abgr16161616f = consts::DRM_FOURCC_ABGR16161616F,
    Abgr2101010 = consts::DRM_FOURCC_ABGR2101010,
    Abgr4444 = consts::DRM_FOURCC_ABGR4444,
    Abgr8888 = consts::DRM_FOURCC_ABGR8888,
    Argb1555 = consts::DRM_FOURCC_ARGB1555,
    Argb16161616f = consts::DRM_FOURCC_ARGB16161616F,
    Argb2101010 = consts::DRM_FOURCC_ARGB2101010,
    Argb4444 = consts::DRM_FOURCC_ARGB4444,
    Argb8888 = consts::DRM_FOURCC_ARGB8888,
    Axbxgxrx106106106106 = consts::DRM_FOURCC_AXBXGXRX106106106106,
    Ayuv = consts::DRM_FOURCC_AYUV,
    Bgr233 = consts::DRM_FOURCC_BGR233,
    Bgr565 = consts::DRM_FOURCC_BGR565,
    Bgr565_a8 = consts::DRM_FOURCC_BGR565_A8,
    Bgr888 = consts::DRM_FOURCC_BGR888,
    Bgr888_a8 = consts::DRM_FOURCC_BGR888_A8,
    Bgra1010102 = consts::DRM_FOURCC_BGRA1010102,
    Bgra4444 = consts::DRM_FOURCC_BGRA4444,
    Bgra5551 = consts::DRM_FOURCC_BGRA5551,
    Bgra8888 = consts::DRM_FOURCC_BGRA8888,
    Bgrx1010102 = consts::DRM_FOURCC_BGRX1010102,
    Bgrx4444 = consts::DRM_FOURCC_BGRX4444,
    Bgrx5551 = consts::DRM_FOURCC_BGRX5551,
    Bgrx8888 = consts::DRM_FOURCC_BGRX8888,
    Bgrx8888_a8 = consts::DRM_FOURCC_BGRX8888_A8,
    Big_endian = consts::DRM_FOURCC_BIG_ENDIAN,
    C8 = consts::DRM_FOURCC_C8,
    Gr1616 = consts::DRM_FOURCC_GR1616,
    Gr88 = consts::DRM_FOURCC_GR88,
    Nv12 = consts::DRM_FOURCC_NV12,
    Nv15 = consts::DRM_FOURCC_NV15,
    Nv16 = consts::DRM_FOURCC_NV16,
    Nv21 = consts::DRM_FOURCC_NV21,
    Nv24 = consts::DRM_FOURCC_NV24,
    Nv42 = consts::DRM_FOURCC_NV42,
    Nv61 = consts::DRM_FOURCC_NV61,
    P010 = consts::DRM_FOURCC_P010,
    P012 = consts::DRM_FOURCC_P012,
    P016 = consts::DRM_FOURCC_P016,
    P210 = consts::DRM_FOURCC_P210,
    Q401 = consts::DRM_FOURCC_Q401,
    Q410 = consts::DRM_FOURCC_Q410,
    R16 = consts::DRM_FOURCC_R16,
    R8 = consts::DRM_FOURCC_R8,
    Rg1616 = consts::DRM_FOURCC_RG1616,
    Rg88 = consts::DRM_FOURCC_RG88,
    Rgb332 = consts::DRM_FOURCC_RGB332,
    Rgb565 = consts::DRM_FOURCC_RGB565,
    Rgb565_a8 = consts::DRM_FOURCC_RGB565_A8,
    Rgb888 = consts::DRM_FOURCC_RGB888,
    Rgb888_a8 = consts::DRM_FOURCC_RGB888_A8,
    Rgba1010102 = consts::DRM_FOURCC_RGBA1010102,
    Rgba4444 = consts::DRM_FOURCC_RGBA4444,
    Rgba5551 = consts::DRM_FOURCC_RGBA5551,
    Rgba8888 = consts::DRM_FOURCC_RGBA8888,
    Rgbx1010102 = consts::DRM_FOURCC_RGBX1010102,
    Rgbx4444 = consts::DRM_FOURCC_RGBX4444,
    Rgbx5551 = consts::DRM_FOURCC_RGBX5551,
    Rgbx8888 = consts::DRM_FOURCC_RGBX8888,
    Rgbx8888_a8 = consts::DRM_FOURCC_RGBX8888_A8,
    Uyvy = consts::DRM_FOURCC_UYVY,
    Vuy101010 = consts::DRM_FOURCC_VUY101010,
    Vuy888 = consts::DRM_FOURCC_VUY888,
    Vyuy = consts::DRM_FOURCC_VYUY,
    X0l0 = consts::DRM_FOURCC_X0L0,
    X0l2 = consts::DRM_FOURCC_X0L2,
    Xbgr1555 = consts::DRM_FOURCC_XBGR1555,
    Xbgr16161616f = consts::DRM_FOURCC_XBGR16161616F,
    Xbgr2101010 = consts::DRM_FOURCC_XBGR2101010,
    Xbgr4444 = consts::DRM_FOURCC_XBGR4444,
    Xbgr8888 = consts::DRM_FOURCC_XBGR8888,
    Xbgr8888_a8 = consts::DRM_FOURCC_XBGR8888_A8,
    Xrgb1555 = consts::DRM_FOURCC_XRGB1555,
    Xrgb16161616f = consts::DRM_FOURCC_XRGB16161616F,
    Xrgb2101010 = consts::DRM_FOURCC_XRGB2101010,
    Xrgb4444 = consts::DRM_FOURCC_XRGB4444,
    Xrgb8888 = consts::DRM_FOURCC_XRGB8888,
    Xrgb8888_a8 = consts::DRM_FOURCC_XRGB8888_A8,
    Xvyu12_16161616 = consts::DRM_FOURCC_XVYU12_16161616,
    Xvyu16161616 = consts::DRM_FOURCC_XVYU16161616,
    Xvyu2101010 = consts::DRM_FOURCC_XVYU2101010,
    Xyuv8888 = consts::DRM_FOURCC_XYUV8888,
    Y0l0 = consts::DRM_FOURCC_Y0L0,
    Y0l2 = consts::DRM_FOURCC_Y0L2,
    Y210 = consts::DRM_FOURCC_Y210,
    Y212 = consts::DRM_FOURCC_Y212,
    Y216 = consts::DRM_FOURCC_Y216,
    Y410 = consts::DRM_FOURCC_Y410,
    Y412 = consts::DRM_FOURCC_Y412,
    Y416 = consts::DRM_FOURCC_Y416,
    Yuv410 = consts::DRM_FOURCC_YUV410,
    Yuv411 = consts::DRM_FOURCC_YUV411,
    Yuv420 = consts::DRM_FOURCC_YUV420,
    Yuv420_10bit = consts::DRM_FOURCC_YUV420_10BIT,
    Yuv420_8bit = consts::DRM_FOURCC_YUV420_8BIT,
    Yuv422 = consts::DRM_FOURCC_YUV422,
    Yuv444 = consts::DRM_FOURCC_YUV444,
    Yuyv = consts::DRM_FOURCC_YUYV,
    Yvu410 = consts::DRM_FOURCC_YVU410,
    Yvu411 = consts::DRM_FOURCC_YVU411,
    Yvu420 = consts::DRM_FOURCC_YVU420,
    Yvu422 = consts::DRM_FOURCC_YVU422,
    Yvu444 = consts::DRM_FOURCC_YVU444,
    Yvyu = consts::DRM_FOURCC_YVYU,
}
impl DrmFourcc {
    pub(crate) fn from_u32(n: u32) -> Option<Self> {
        match n {
            consts::DRM_FOURCC_ABGR1555 => Some(Self::Abgr1555),
            consts::DRM_FOURCC_ABGR16161616F => Some(Self::Abgr16161616f),
            consts::DRM_FOURCC_ABGR2101010 => Some(Self::Abgr2101010),
            consts::DRM_FOURCC_ABGR4444 => Some(Self::Abgr4444),
            consts::DRM_FOURCC_ABGR8888 => Some(Self::Abgr8888),
            consts::DRM_FOURCC_ARGB1555 => Some(Self::Argb1555),
            consts::DRM_FOURCC_ARGB16161616F => Some(Self::Argb16161616f),
            consts::DRM_FOURCC_ARGB2101010 => Some(Self::Argb2101010),
            consts::DRM_FOURCC_ARGB4444 => Some(Self::Argb4444),
            consts::DRM_FOURCC_ARGB8888 => Some(Self::Argb8888),
            consts::DRM_FOURCC_AXBXGXRX106106106106 => Some(Self::Axbxgxrx106106106106),
            consts::DRM_FOURCC_AYUV => Some(Self::Ayuv),
            consts::DRM_FOURCC_BGR233 => Some(Self::Bgr233),
            consts::DRM_FOURCC_BGR565 => Some(Self::Bgr565),
            consts::DRM_FOURCC_BGR565_A8 => Some(Self::Bgr565_a8),
            consts::DRM_FOURCC_BGR888 => Some(Self::Bgr888),
            consts::DRM_FOURCC_BGR888_A8 => Some(Self::Bgr888_a8),
            consts::DRM_FOURCC_BGRA1010102 => Some(Self::Bgra1010102),
            consts::DRM_FOURCC_BGRA4444 => Some(Self::Bgra4444),
            consts::DRM_FOURCC_BGRA5551 => Some(Self::Bgra5551),
            consts::DRM_FOURCC_BGRA8888 => Some(Self::Bgra8888),
            consts::DRM_FOURCC_BGRX1010102 => Some(Self::Bgrx1010102),
            consts::DRM_FOURCC_BGRX4444 => Some(Self::Bgrx4444),
            consts::DRM_FOURCC_BGRX5551 => Some(Self::Bgrx5551),
            consts::DRM_FOURCC_BGRX8888 => Some(Self::Bgrx8888),
            consts::DRM_FOURCC_BGRX8888_A8 => Some(Self::Bgrx8888_a8),
            consts::DRM_FOURCC_BIG_ENDIAN => Some(Self::Big_endian),
            consts::DRM_FOURCC_C8 => Some(Self::C8),
            consts::DRM_FOURCC_GR1616 => Some(Self::Gr1616),
            consts::DRM_FOURCC_GR88 => Some(Self::Gr88),
            consts::DRM_FOURCC_NV12 => Some(Self::Nv12),
            consts::DRM_FOURCC_NV15 => Some(Self::Nv15),
            consts::DRM_FOURCC_NV16 => Some(Self::Nv16),
            consts::DRM_FOURCC_NV21 => Some(Self::Nv21),
            consts::DRM_FOURCC_NV24 => Some(Self::Nv24),
            consts::DRM_FOURCC_NV42 => Some(Self::Nv42),
            consts::DRM_FOURCC_NV61 => Some(Self::Nv61),
            consts::DRM_FOURCC_P010 => Some(Self::P010),
            consts::DRM_FOURCC_P012 => Some(Self::P012),
            consts::DRM_FOURCC_P016 => Some(Self::P016),
            consts::DRM_FOURCC_P210 => Some(Self::P210),
            consts::DRM_FOURCC_Q401 => Some(Self::Q401),
            consts::DRM_FOURCC_Q410 => Some(Self::Q410),
            consts::DRM_FOURCC_R16 => Some(Self::R16),
            consts::DRM_FOURCC_R8 => Some(Self::R8),
            consts::DRM_FOURCC_RG1616 => Some(Self::Rg1616),
            consts::DRM_FOURCC_RG88 => Some(Self::Rg88),
            consts::DRM_FOURCC_RGB332 => Some(Self::Rgb332),
            consts::DRM_FOURCC_RGB565 => Some(Self::Rgb565),
            consts::DRM_FOURCC_RGB565_A8 => Some(Self::Rgb565_a8),
            consts::DRM_FOURCC_RGB888 => Some(Self::Rgb888),
            consts::DRM_FOURCC_RGB888_A8 => Some(Self::Rgb888_a8),
            consts::DRM_FOURCC_RGBA1010102 => Some(Self::Rgba1010102),
            consts::DRM_FOURCC_RGBA4444 => Some(Self::Rgba4444),
            consts::DRM_FOURCC_RGBA5551 => Some(Self::Rgba5551),
            consts::DRM_FOURCC_RGBA8888 => Some(Self::Rgba8888),
            consts::DRM_FOURCC_RGBX1010102 => Some(Self::Rgbx1010102),
            consts::DRM_FOURCC_RGBX4444 => Some(Self::Rgbx4444),
            consts::DRM_FOURCC_RGBX5551 => Some(Self::Rgbx5551),
            consts::DRM_FOURCC_RGBX8888 => Some(Self::Rgbx8888),
            consts::DRM_FOURCC_RGBX8888_A8 => Some(Self::Rgbx8888_a8),
            consts::DRM_FOURCC_UYVY => Some(Self::Uyvy),
            consts::DRM_FOURCC_VUY101010 => Some(Self::Vuy101010),
            consts::DRM_FOURCC_VUY888 => Some(Self::Vuy888),
            consts::DRM_FOURCC_VYUY => Some(Self::Vyuy),
            consts::DRM_FOURCC_X0L0 => Some(Self::X0l0),
            consts::DRM_FOURCC_X0L2 => Some(Self::X0l2),
            consts::DRM_FOURCC_XBGR1555 => Some(Self::Xbgr1555),
            consts::DRM_FOURCC_XBGR16161616F => Some(Self::Xbgr16161616f),
            consts::DRM_FOURCC_XBGR2101010 => Some(Self::Xbgr2101010),
            consts::DRM_FOURCC_XBGR4444 => Some(Self::Xbgr4444),
            consts::DRM_FOURCC_XBGR8888 => Some(Self::Xbgr8888),
            consts::DRM_FOURCC_XBGR8888_A8 => Some(Self::Xbgr8888_a8),
            consts::DRM_FOURCC_XRGB1555 => Some(Self::Xrgb1555),
            consts::DRM_FOURCC_XRGB16161616F => Some(Self::Xrgb16161616f),
            consts::DRM_FOURCC_XRGB2101010 => Some(Self::Xrgb2101010),
            consts::DRM_FOURCC_XRGB4444 => Some(Self::Xrgb4444),
            consts::DRM_FOURCC_XRGB8888 => Some(Self::Xrgb8888),
            consts::DRM_FOURCC_XRGB8888_A8 => Some(Self::Xrgb8888_a8),
            consts::DRM_FOURCC_XVYU12_16161616 => Some(Self::Xvyu12_16161616),
            consts::DRM_FOURCC_XVYU16161616 => Some(Self::Xvyu16161616),
            consts::DRM_FOURCC_XVYU2101010 => Some(Self::Xvyu2101010),
            consts::DRM_FOURCC_XYUV8888 => Some(Self::Xyuv8888),
            consts::DRM_FOURCC_Y0L0 => Some(Self::Y0l0),
            consts::DRM_FOURCC_Y0L2 => Some(Self::Y0l2),
            consts::DRM_FOURCC_Y210 => Some(Self::Y210),
            consts::DRM_FOURCC_Y212 => Some(Self::Y212),
            consts::DRM_FOURCC_Y216 => Some(Self::Y216),
            consts::DRM_FOURCC_Y410 => Some(Self::Y410),
            consts::DRM_FOURCC_Y412 => Some(Self::Y412),
            consts::DRM_FOURCC_Y416 => Some(Self::Y416),
            consts::DRM_FOURCC_YUV410 => Some(Self::Yuv410),
            consts::DRM_FOURCC_YUV411 => Some(Self::Yuv411),
            consts::DRM_FOURCC_YUV420 => Some(Self::Yuv420),
            consts::DRM_FOURCC_YUV420_10BIT => Some(Self::Yuv420_10bit),
            consts::DRM_FOURCC_YUV420_8BIT => Some(Self::Yuv420_8bit),
            consts::DRM_FOURCC_YUV422 => Some(Self::Yuv422),
            consts::DRM_FOURCC_YUV444 => Some(Self::Yuv444),
            consts::DRM_FOURCC_YUYV => Some(Self::Yuyv),
            consts::DRM_FOURCC_YVU410 => Some(Self::Yvu410),
            consts::DRM_FOURCC_YVU411 => Some(Self::Yvu411),
            consts::DRM_FOURCC_YVU420 => Some(Self::Yvu420),
            consts::DRM_FOURCC_YVU422 => Some(Self::Yvu422),
            consts::DRM_FOURCC_YVU444 => Some(Self::Yvu444),
            consts::DRM_FOURCC_YVYU => Some(Self::Yvyu),
            _ => None,
        }
    }
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum DrmVendor {
    Allwinner = consts::DRM_FOURCC_ALLWINNER,
    Amd = consts::DRM_FOURCC_AMD,
    Amlogic = consts::DRM_FOURCC_AMLOGIC,
    Arm = consts::DRM_FOURCC_ARM,
    Broadcom = consts::DRM_FOURCC_BROADCOM,
    Intel = consts::DRM_FOURCC_INTEL,
    Nvidia = consts::DRM_FOURCC_NVIDIA,
    Qcom = consts::DRM_FOURCC_QCOM,
    Samsung = consts::DRM_FOURCC_SAMSUNG,
    Vivante = consts::DRM_FOURCC_VIVANTE,
}
impl DrmVendor {
    pub(crate) fn from_u8(n: u8) -> Option<Self> {
        match n {
            consts::DRM_FOURCC_ALLWINNER => Some(Self::Allwinner),
            consts::DRM_FOURCC_AMD => Some(Self::Amd),
            consts::DRM_FOURCC_AMLOGIC => Some(Self::Amlogic),
            consts::DRM_FOURCC_ARM => Some(Self::Arm),
            consts::DRM_FOURCC_BROADCOM => Some(Self::Broadcom),
            consts::DRM_FOURCC_INTEL => Some(Self::Intel),
            consts::DRM_FOURCC_NVIDIA => Some(Self::Nvidia),
            consts::DRM_FOURCC_QCOM => Some(Self::Qcom),
            consts::DRM_FOURCC_SAMSUNG => Some(Self::Samsung),
            consts::DRM_FOURCC_VIVANTE => Some(Self::Vivante),
            _ => None,
        }
    }
}
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DrmModifier {
    Allwinner_tiled,
    Broadcom_sand128,
    Broadcom_sand256,
    Broadcom_sand32,
    Broadcom_sand64,
    Broadcom_uif,
    Broadcom_vc4_t_tiled,
    Generic_16_16_tile,
    Invalid,
    Linear,
    Nvidia_16bx2_block_eight_gob,
    Nvidia_16bx2_block_four_gob,
    Nvidia_16bx2_block_one_gob,
    Nvidia_16bx2_block_sixteen_gob,
    Nvidia_16bx2_block_thirtytwo_gob,
    Nvidia_16bx2_block_two_gob,
    Nvidia_tegra_tiled,
    Qcom_compressed,
    Samsung_16_16_tile,
    Samsung_64_32_tile,
    Vivante_split_super_tiled,
    Vivante_split_tiled,
    Vivante_super_tiled,
    Vivante_tiled,
    I915_x_tiled,
    I915_y_tiled,
    I915_y_tiled_ccs,
    I915_y_tiled_gen12_mc_ccs,
    I915_y_tiled_gen12_rc_ccs,
    Unrecognized(u64),
}
impl DrmModifier {
    pub(crate) fn from_u64(n: u64) -> Self {
        #[allow(unreachable_patterns)]
        match n {
            consts::DRM_FOURCC_ALLWINNER_TILED => Self::Allwinner_tiled,
            consts::DRM_FOURCC_BROADCOM_SAND128 => Self::Broadcom_sand128,
            consts::DRM_FOURCC_BROADCOM_SAND256 => Self::Broadcom_sand256,
            consts::DRM_FOURCC_BROADCOM_SAND32 => Self::Broadcom_sand32,
            consts::DRM_FOURCC_BROADCOM_SAND64 => Self::Broadcom_sand64,
            consts::DRM_FOURCC_BROADCOM_UIF => Self::Broadcom_uif,
            consts::DRM_FOURCC_BROADCOM_VC4_T_TILED => Self::Broadcom_vc4_t_tiled,
            consts::DRM_FOURCC_GENERIC_16_16_TILE => Self::Generic_16_16_tile,
            consts::DRM_FOURCC_INVALID => Self::Invalid,
            consts::DRM_FOURCC_LINEAR => Self::Linear,
            consts::DRM_FOURCC_NVIDIA_16BX2_BLOCK_EIGHT_GOB => Self::Nvidia_16bx2_block_eight_gob,
            consts::DRM_FOURCC_NVIDIA_16BX2_BLOCK_FOUR_GOB => Self::Nvidia_16bx2_block_four_gob,
            consts::DRM_FOURCC_NVIDIA_16BX2_BLOCK_ONE_GOB => Self::Nvidia_16bx2_block_one_gob,
            consts::DRM_FOURCC_NVIDIA_16BX2_BLOCK_SIXTEEN_GOB => {
                Self::Nvidia_16bx2_block_sixteen_gob
            }
            consts::DRM_FOURCC_NVIDIA_16BX2_BLOCK_THIRTYTWO_GOB => {
                Self::Nvidia_16bx2_block_thirtytwo_gob
            }
            consts::DRM_FOURCC_NVIDIA_16BX2_BLOCK_TWO_GOB => Self::Nvidia_16bx2_block_two_gob,
            consts::DRM_FOURCC_NVIDIA_TEGRA_TILED => Self::Nvidia_tegra_tiled,
            consts::DRM_FOURCC_QCOM_COMPRESSED => Self::Qcom_compressed,
            consts::DRM_FOURCC_SAMSUNG_16_16_TILE => Self::Samsung_16_16_tile,
            consts::DRM_FOURCC_SAMSUNG_64_32_TILE => Self::Samsung_64_32_tile,
            consts::DRM_FOURCC_VIVANTE_SPLIT_SUPER_TILED => Self::Vivante_split_super_tiled,
            consts::DRM_FOURCC_VIVANTE_SPLIT_TILED => Self::Vivante_split_tiled,
            consts::DRM_FOURCC_VIVANTE_SUPER_TILED => Self::Vivante_super_tiled,
            consts::DRM_FOURCC_VIVANTE_TILED => Self::Vivante_tiled,
            consts::DRM_FOURCC_I915_X_TILED => Self::I915_x_tiled,
            consts::DRM_FOURCC_I915_Y_TILED => Self::I915_y_tiled,
            consts::DRM_FOURCC_I915_Y_TILED_CCS => Self::I915_y_tiled_ccs,
            consts::DRM_FOURCC_I915_Y_TILED_GEN12_MC_CCS => Self::I915_y_tiled_gen12_mc_ccs,
            consts::DRM_FOURCC_I915_Y_TILED_GEN12_RC_CCS => Self::I915_y_tiled_gen12_rc_ccs,
            x => Self::Unrecognized(x),
        }
    }
    pub(crate) fn into_u64(&self) -> u64 {
        match self {
            Self::Allwinner_tiled => consts::DRM_FOURCC_ALLWINNER_TILED,
            Self::Broadcom_sand128 => consts::DRM_FOURCC_BROADCOM_SAND128,
            Self::Broadcom_sand256 => consts::DRM_FOURCC_BROADCOM_SAND256,
            Self::Broadcom_sand32 => consts::DRM_FOURCC_BROADCOM_SAND32,
            Self::Broadcom_sand64 => consts::DRM_FOURCC_BROADCOM_SAND64,
            Self::Broadcom_uif => consts::DRM_FOURCC_BROADCOM_UIF,
            Self::Broadcom_vc4_t_tiled => consts::DRM_FOURCC_BROADCOM_VC4_T_TILED,
            Self::Generic_16_16_tile => consts::DRM_FOURCC_GENERIC_16_16_TILE,
            Self::Invalid => consts::DRM_FOURCC_INVALID,
            Self::Linear => consts::DRM_FOURCC_LINEAR,
            Self::Nvidia_16bx2_block_eight_gob => consts::DRM_FOURCC_NVIDIA_16BX2_BLOCK_EIGHT_GOB,
            Self::Nvidia_16bx2_block_four_gob => consts::DRM_FOURCC_NVIDIA_16BX2_BLOCK_FOUR_GOB,
            Self::Nvidia_16bx2_block_one_gob => consts::DRM_FOURCC_NVIDIA_16BX2_BLOCK_ONE_GOB,
            Self::Nvidia_16bx2_block_sixteen_gob => {
                consts::DRM_FOURCC_NVIDIA_16BX2_BLOCK_SIXTEEN_GOB
            }
            Self::Nvidia_16bx2_block_thirtytwo_gob => {
                consts::DRM_FOURCC_NVIDIA_16BX2_BLOCK_THIRTYTWO_GOB
            }
            Self::Nvidia_16bx2_block_two_gob => consts::DRM_FOURCC_NVIDIA_16BX2_BLOCK_TWO_GOB,
            Self::Nvidia_tegra_tiled => consts::DRM_FOURCC_NVIDIA_TEGRA_TILED,
            Self::Qcom_compressed => consts::DRM_FOURCC_QCOM_COMPRESSED,
            Self::Samsung_16_16_tile => consts::DRM_FOURCC_SAMSUNG_16_16_TILE,
            Self::Samsung_64_32_tile => consts::DRM_FOURCC_SAMSUNG_64_32_TILE,
            Self::Vivante_split_super_tiled => consts::DRM_FOURCC_VIVANTE_SPLIT_SUPER_TILED,
            Self::Vivante_split_tiled => consts::DRM_FOURCC_VIVANTE_SPLIT_TILED,
            Self::Vivante_super_tiled => consts::DRM_FOURCC_VIVANTE_SUPER_TILED,
            Self::Vivante_tiled => consts::DRM_FOURCC_VIVANTE_TILED,
            Self::I915_x_tiled => consts::DRM_FOURCC_I915_X_TILED,
            Self::I915_y_tiled => consts::DRM_FOURCC_I915_Y_TILED,
            Self::I915_y_tiled_ccs => consts::DRM_FOURCC_I915_Y_TILED_CCS,
            Self::I915_y_tiled_gen12_mc_ccs => consts::DRM_FOURCC_I915_Y_TILED_GEN12_MC_CCS,
            Self::I915_y_tiled_gen12_rc_ccs => consts::DRM_FOURCC_I915_Y_TILED_GEN12_RC_CCS,
            Self::Unrecognized(x) => *x,
        }
    }
}
