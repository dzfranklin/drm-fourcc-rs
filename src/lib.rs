#![allow(non_camel_case_types)]
#![warn(clippy::cargo)]

//! [`DrmFourcc`] is an enum representing every pixel format supported by DRM
//! (as of kernel version 5.10.0).
//!
//! A [fourcc][fourcc_wiki] is four bytes of ascii representing some data format. This enum contains
//! every fourcc representing a pixel format supported by [DRM][drm_wiki], the Linux Direct
//! Rendering Manager. The names of pixel formats generally provide clues as to
//! how they work, for more information you may find
//! [this guide][drm_format_guide] helpful.
//!
//! To get the bytes of the fourcc representing the format, cast to `u32`.
//!
//! ```
//! # use drm_fourcc::DrmFourcc;
//! assert_eq!(DrmFourcc::Xrgb8888 as u32, 875713112);
//! ```
//!
//! To get the string form of the fourcc, use [`DrmFourcc::string_form`].
//!
//! ```
//! # use drm_fourcc::DrmFourcc;
//! assert_eq!(DrmFourcc::Xrgb8888.string_form(), "XR24");
//! ```
//!
//!
//! We also provide a type for representing a fourcc/modifier pair
//!
//! ```
//! # use drm_fourcc::{DrmFormat, DrmFourcc, DrmModifier};
//! let format = DrmFormat {
//!     code: DrmFourcc::Xrgb8888,
//!     modifier: DrmModifier::Linear,
//! };
//! ```
//!
//! The enums are autogenerated from the [canonical list][canonical] in the Linux source code.
//!
//! ## Features
//! - `serde` - Derive Serialize/Deserialize where it makes sense
//! - `build_bindings` - Re-generate autogenerated code. Useful if you need varients added in a
//!     more recent kernel version.
//!
//! [fourcc_wiki]: https://en.wikipedia.org/wiki/FourCC
//! [drm_wiki]: https://en.wikipedia.org/wiki/Direct_Rendering_Managerz
//! [canonical]: https://github.com/torvalds/linux/blame/master/include/uapi/drm/drm_fourcc.h
//! [drm_format_guide]: https://afrantzis.com/pixel-format-guide/drm.html

use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};

pub use as_enum::{DrmFourcc, DrmModifier, DrmVendor};

mod as_enum;
mod consts;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DrmFormat {
    pub code: DrmFourcc,
    pub modifier: DrmModifier,
}

impl DrmFourcc {
    /// Get the string representation of the format's fourcc.
    pub fn string_form(&self) -> String {
        fourcc_string_form(*self as u32).expect("Must be valid fourcc")
    }
}

impl Debug for DrmFourcc {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("DrmFourcc")
            .field(&self.string_form())
            .finish()
    }
}

impl Display for DrmFourcc {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl TryFrom<u32> for DrmFourcc {
    type Error = UnrecognizedFourcc;

    /// Convert from an u32
    ///
    /// ```
    /// # use drm_fourcc::DrmFourcc;
    /// # use std::convert::TryFrom;
    /// assert_eq!(DrmFourcc::try_from(875710274).unwrap(), DrmFourcc::Bgr888);
    ///
    /// assert!(DrmFourcc::try_from(0).is_err());
    ///
    /// // If the u32 is in the valid format to be a fourcc, you can see its string form
    /// assert_eq!(DrmFourcc::try_from(828601953).unwrap_err().string_form(), Some("avc1".to_string()));
    /// ```
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::from_u32(value).ok_or(UnrecognizedFourcc(value))
    }
}

/// Wraps some u32 that isn't a DRM fourcc we recognize
///
/// ```
/// # use drm_fourcc::{DrmFourcc, UnrecognizedFourcc};
/// # use std::convert::TryFrom;
/// // Get the u32
/// assert_eq!(UnrecognizedFourcc(42).0, 42);
///
/// // Get the string form
/// assert_eq!(UnrecognizedFourcc(828601953).string_form(), Some("avc1".to_string()));
/// assert_eq!(UnrecognizedFourcc(0).string_form(), None);
/// ```
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnrecognizedFourcc(pub u32);

impl UnrecognizedFourcc {
    /// If the u32 is in a valid format to be a fourcc, get its string form.
    pub fn string_form(&self) -> Option<String> {
        fourcc_string_form(self.0)
    }
}

impl Debug for UnrecognizedFourcc {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut debug = &mut f.debug_tuple("UnrecognizedFourcc");

        if let Some(string_form) = self.string_form() {
            debug = debug.field(&string_form);
        }

        debug.field(&self.0).finish()
    }
}

impl Display for UnrecognizedFourcc {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self, f)
    }
}

impl Error for UnrecognizedFourcc {}

fn fourcc_string_form(fourcc: u32) -> Option<String> {
    let string = String::from_utf8(fourcc.to_le_bytes().to_vec()).ok()?;

    let mut out = String::new();

    let chars: Vec<char> = string.chars().collect();
    let (start, last_chars) = chars.split_at(3);
    let last = last_chars[0];

    // first three bytes must be characters
    for char in start {
        if char.is_ascii_alphanumeric() {
            out.push(*char);
        } else {
            return None;
        }
    }

    // last byte is allowed to be null
    if last == '\0' {
        out.push(' ');
    } else {
        out.push(last);
    }

    Some(out)
}

impl TryFrom<u8> for DrmVendor {
    type Error = UnrecognizedVendor;

    /// Convert from an u8
    ///
    /// ```
    /// # use drm_fourcc::DrmVendor;
    /// # use std::convert::TryFrom;
    /// assert_eq!(DrmVendor::try_from(2).unwrap(), DrmVendor::Amd);
    ///
    /// assert!(DrmVendor::try_from(0).is_err());
    /// ```
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(UnrecognizedVendor(value))
    }
}

/// Wraps some u8 that isn't a DRM vendor we recognize
///
/// ```
/// # use drm_fourcc::{DrmVendor, UnrecognizedVendor};
/// # use std::convert::TryFrom;
/// // Get the u8
/// assert_eq!(UnrecognizedVendor(42).0, 42);
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnrecognizedVendor(pub u8);

impl Display for UnrecognizedVendor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self, f)
    }
}

impl Error for UnrecognizedVendor {}

impl From<u64> for DrmModifier {
    /// Convert from an u64
    ///
    /// ```
    /// # use drm_fourcc::DrmModifier;
    /// assert_eq!(DrmModifier::from(0), DrmModifier::Linear);
    /// ```
    fn from(value: u64) -> Self {
        Self::from_u64(value)
    }
}

/// Wraps some u64 that isn't a DRM modifier we recognize
///
/// ```
/// # use drm_fourcc::{DrmModifier, UnrecognizedModifier};
/// # use std::convert::TryFrom;
/// // Get the u64
/// assert_eq!(UnrecognizedModifier(42).0, 42);
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnrecognizedModifier(pub u64);

impl Display for UnrecognizedModifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self, f)
    }
}

impl Error for UnrecognizedModifier {}

impl UnrecognizedModifier {
    /// Get the vendor of the unrecognized modifier, if any
    ///
    /// ```
    /// # use drm_fourcc::{DrmModifier, DrmVendor, UnrecognizedModifier, UnrecognizedVendor};
    /// assert_eq!(UnrecognizedModifier(216172782113783827).vendor(), Ok(Some(DrmVendor::Nvidia)));
    /// assert_eq!(UnrecognizedModifier(2).vendor(), Ok(None));
    /// assert_eq!(UnrecognizedModifier(8646911284551352320).vendor(), Err(UnrecognizedVendor(120)));
    /// ```
    pub fn vendor(&self) -> Result<Option<DrmVendor>, UnrecognizedVendor> {
        let vendor = (self.0 >> 56) as u8;
        if vendor == 0 {
            Ok(None)
        } else {
            DrmVendor::try_from(vendor).map(|x| Some(x))
        }
    }
}

impl Into<u64> for DrmModifier {
    /// Convert to an u64
    ///
    /// ```
    /// # use drm_fourcc::DrmModifier;
    /// assert_eq!(0u64, DrmModifier::Linear.into());
    /// ```
    fn into(self) -> u64 {
        self.into_u64()
    }
}

impl PartialEq for DrmModifier {
    fn eq(&self, other: &Self) -> bool {
        self.into_u64() == other.into_u64()
    }
}
impl Eq for DrmModifier {}

impl PartialEq<u64> for DrmModifier {
    fn eq(&self, other: &u64) -> bool {
        &self.into_u64() == other
    }
}

impl Hash for DrmModifier {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.into_u64().hash(state);
    }
}

impl DrmModifier {
    /// Get the vendor of the modifier, if any
    ///
    /// ```
    /// # use drm_fourcc::{DrmModifier, DrmVendor, UnrecognizedVendor};
    /// assert_eq!(DrmModifier::I915_x_tiled.vendor(), Ok(Some(DrmVendor::Intel)));
    /// assert_eq!(DrmModifier::Linear.vendor(), Ok(None));
    /// assert_eq!(DrmModifier::Unrecognized(8646911284551352320).vendor(), Err(UnrecognizedVendor(120)));
    /// ```
    pub fn vendor(&self) -> Result<Option<DrmVendor>, UnrecognizedVendor> {
        let vendor = (self.into_u64() >> 56) as u8;
        if vendor == 0 {
            Ok(None)
        } else {
            DrmVendor::try_from(vendor).map(|x| Some(x))
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn a_specific_var_has_correct_value() {
        assert_eq!(consts::DRM_FOURCC_AYUV, 1448433985);
    }

    #[test]
    fn enum_member_casts_to_const() {
        assert_eq!(
            DrmFourcc::Xrgb8888 as u32,
            consts::DRM_FOURCC_XRGB8888 as u32
        );
    }

    #[test]
    fn enum_member_has_correct_string_format() {
        assert_eq!(DrmFourcc::Xrgb8888.string_form(), "XR24");
    }

    #[test]
    fn fourcc_string_form_handles_valid() {
        assert_eq!(fourcc_string_form(875713112).unwrap(), "XR24");
        assert_eq!(fourcc_string_form(828601953).unwrap(), "avc1");
        assert_eq!(fourcc_string_form(0x316376).unwrap(), "vc1 ");
    }

    #[test]
    fn unrecognized_handles_valid_fourcc() {
        assert_eq!(
            format!("{}", UnrecognizedFourcc(828601953)),
            "UnrecognizedFourcc(\"avc1\", 828601953)"
        );
    }

    #[test]
    fn unrecognized_handles_invalid_fourcc() {
        assert_eq!(
            format!("{}", UnrecognizedFourcc(0)),
            "UnrecognizedFourcc(0)"
        );
    }

    #[test]
    fn can_clone_result() {
        let a = DrmFourcc::try_from(0);
        let b = a;
        assert_eq!(a, b);
    }
}
