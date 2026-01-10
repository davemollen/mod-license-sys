use crate::{LV2_Feature, ModLicense};
use std::ptr;

pub struct ModLicenseActivator;

impl ModLicense for ModLicenseActivator {
    fn new(
        _features: *const *const LV2_Feature,
        _license_uri: *const ::std::os::raw::c_char,
    ) -> Self {
        Self
    }

    fn mod_license_begin_run(&mut self, _sample_count: u32) {
        eprintln!("Can not run \"mod_license_begin_run\" because Mod Licensing is disabled.");
    }

    fn mod_license_run_silence(&self, _buffer: *mut f32, _channel: u32, _sample_count: u32) {
        eprintln!("Can not run \"mod_license_run_silence\" because Mod Licensing is disabled.");
    }

    fn mod_license_interface(_uri: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_void {
        eprintln!("Can not run \"mod_license_interface\" because Mod Licensing is disabled.");
        ptr::null()
    }
}
