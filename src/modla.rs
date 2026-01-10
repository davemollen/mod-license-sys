use crate::bindings::LV2_Feature;
#[cfg_attr(
    all(target_os = "linux", target_arch = "aarch64"),
    path = "modla/supported.rs"
)]
// #[cfg_attr(target_os = "macos", path = "modla/supported.rs")]
mod unsupported;
pub use unsupported::ModLicenseActivator;

pub trait ModLicense {
    fn new(features: *const *const LV2_Feature, license_uri: *const ::std::os::raw::c_char)
        -> Self;
    fn mod_license_begin_run(&mut self, sample_count: u32);
    fn mod_license_run_silence(&self, buffer: *mut f32, channel: u32, sample_count: u32);
    fn mod_license_interface(uri: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_void;
}
