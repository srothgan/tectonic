#![allow(nonstandard_style, missing_docs)]

use fontconfig_sys as fc;

pub use fc::constants::{
    FC_FAMILY, FC_FILE, FC_FONTFORMAT, FC_FULLNAME, FC_INDEX, FC_SLANT, FC_STYLE, FC_WEIGHT,
    FC_WIDTH,
};

pub type FcBool = fc::FcBool;
pub type FcChar8 = fc::FcChar8;

#[allow(non_upper_case_globals)]
pub const FcTrue: FcBool = 1;
#[allow(non_upper_case_globals)]
pub const FcFalse: FcBool = 0;

#[repr(C)]
pub struct FcPattern(());

#[repr(C)]
pub struct FcFontSet {
    pub nfont: libc::c_int,
    sfont: libc::c_int,
    pub fonts: *const *mut FcPattern,
}

#[repr(C)]
pub struct FcObjectSet(());

#[repr(C)]
pub struct FcConfig(());

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FcResult {
    Match,
    NoMatch,
    TypeMismatch,
    ResultNoId,
    OutOfMemory,
}

impl From<fc::FcResult> for FcResult {
    fn from(value: fc::FcResult) -> Self {
        match value {
            fc::FcResultMatch => FcResult::Match,
            fc::FcResultNoMatch => FcResult::NoMatch,
            fc::FcResultTypeMismatch => FcResult::TypeMismatch,
            fc::FcResultNoId => FcResult::ResultNoId,
            fc::FcResultOutOfMemory => FcResult::OutOfMemory,
            _ => FcResult::ResultNoId,
        }
    }
}

impl FcResult {
    pub fn res(self) -> Result<(), crate::FcErr> {
        match crate::FcErr::try_from(self) {
            Ok(err) => Err(err),
            Err(_) => Ok(()),
        }
    }
}

pub unsafe fn FcPatternGetString(
    p: *mut FcPattern,
    object: *const libc::c_char,
    n: libc::c_int,
    s: *mut *const libc::c_char,
) -> FcResult {
    let mut raw = std::ptr::null_mut::<FcChar8>();
    // SAFETY: The caller upholds fontconfig's pointer validity contract.
    let result = unsafe { fc::FcPatternGetString(p.cast::<fc::FcPattern>(), object, n, &mut raw) };
    if result == fc::FcResultMatch {
        // SAFETY: `s` is the caller-provided output pointer used by the
        // previous bridge API. Fontconfig returns NUL-terminated FcChar8 data.
        unsafe { *s = raw.cast::<libc::c_char>() };
    }
    result.into()
}

pub unsafe fn FcPatternGetInteger(
    p: *mut FcPattern,
    object: *const libc::c_char,
    n: libc::c_int,
    i: *mut libc::c_int,
) -> FcResult {
    // SAFETY: The caller upholds fontconfig's pointer validity contract.
    unsafe { fc::FcPatternGetInteger(p.cast::<fc::FcPattern>(), object, n, i) }.into()
}

pub unsafe fn FcInit() -> FcBool {
    // SAFETY: Fontconfig documents FcInit as process-global initialization.
    unsafe { fc::FcInit() }
}

pub unsafe fn FcNameParse(name: *const libc::c_char) -> *mut FcPattern {
    // SAFETY: The caller provides a valid NUL-terminated fontconfig pattern.
    unsafe { fc::FcNameParse(name.cast::<FcChar8>()).cast::<FcPattern>() }
}

pub unsafe fn FcFontList(
    config: *mut FcConfig,
    p: *mut FcPattern,
    os: *mut FcObjectSet,
) -> *mut FcFontSet {
    // SAFETY: The caller upholds fontconfig's pointer validity contract.
    unsafe {
        fc::FcFontList(
            config.cast::<fc::FcConfig>(),
            p.cast::<fc::FcPattern>(),
            os.cast::<fc::FcObjectSet>(),
        )
        .cast::<FcFontSet>()
    }
}

pub unsafe fn FcConfigGetCurrent() -> *mut FcConfig {
    // SAFETY: Fontconfig owns and manages the returned current configuration.
    unsafe { fc::FcConfigGetCurrent().cast::<FcConfig>() }
}

pub unsafe fn FcObjectSetDestroy(os: *mut FcObjectSet) {
    // SAFETY: The caller owns `os` according to fontconfig ownership rules.
    unsafe { fc::FcObjectSetDestroy(os.cast::<fc::FcObjectSet>()) }
}

pub unsafe fn FcPatternReference(pat: *mut FcPattern) {
    // SAFETY: The caller provides a valid fontconfig pattern pointer.
    unsafe { fc::FcPatternReference(pat.cast::<fc::FcPattern>()) }
}

pub unsafe fn FcPatternDestroy(pat: *mut FcPattern) {
    // SAFETY: The caller owns one reference to `pat`.
    unsafe { fc::FcPatternDestroy(pat.cast::<fc::FcPattern>()) }
}

pub unsafe fn FcFontSetDestroy(fs: *mut FcFontSet) {
    // SAFETY: The caller owns `fs` according to fontconfig ownership rules.
    unsafe { fc::FcFontSetDestroy(fs.cast::<fc::FcFontSet>()) }
}

pub unsafe fn FcObjectSetCreate() -> *mut FcObjectSet {
    // SAFETY: This delegates to fontconfig allocation.
    unsafe { fc::FcObjectSetCreate().cast::<FcObjectSet>() }
}

pub unsafe fn FcObjectSetAdd(os: *mut FcObjectSet, object: *const libc::c_char) -> FcBool {
    // SAFETY: The caller provides a valid object-set pointer and C-string.
    unsafe { fc::FcObjectSetAdd(os.cast::<fc::FcObjectSet>(), object) }
}
