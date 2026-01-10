use crate::{bindings::*, modla::ModLicense};

pub struct ModLicenseActivator {
    run_count: u32,
}

impl ModLicense for ModLicenseActivator {
    fn new(
        features: *const *const LV2_Feature,
        license_uri: *const ::std::os::raw::c_char,
    ) -> Self {
        unsafe { mod_license_check(features, license_uri) };
        Self { run_count: 0 }
    }

    fn mod_license_begin_run(&mut self, sample_count: u32) {
        self.run_count = unsafe { mod_license_run_begin(self.run_count, sample_count) };
    }

    fn mod_license_run_silence(&self, buffer: *mut f32, channel: u32, sample_count: u32) {
        unsafe {
            mod_license_run_silence(self.run_count, buffer, sample_count, channel);
        }
    }

    fn mod_license_interface(uri: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_void {
        unsafe { mod_license_interface(uri) }
    }
}
