// This file was generated by gir (d933f9a) from gir-files (469db10)
// DO NOT EDIT

#[cfg(any(feature = "v2_2", feature = "dox"))]
use CredentialPersistence;
use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Credential(Boxed<ffi::WebKitCredential>);

    match fn {
        copy => |ptr| ffi::webkit_credential_copy(mut_override(ptr)),
        free => |ptr| ffi::webkit_credential_free(ptr),
        get_type => || ffi::webkit_credential_get_type(),
    }
}

impl Credential {
    #[cfg(any(feature = "v2_2", feature = "dox"))]
    pub fn new(username: &str, password: &str, persistence: CredentialPersistence) -> Credential {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::webkit_credential_new(username.to_glib_none().0, password.to_glib_none().0, persistence.to_glib()))
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    pub fn get_password(&mut self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_credential_get_password(self.to_glib_none_mut().0))
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    pub fn get_persistence(&mut self) -> CredentialPersistence {
        unsafe {
            from_glib(ffi::webkit_credential_get_persistence(self.to_glib_none_mut().0))
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    pub fn get_username(&mut self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_credential_get_username(self.to_glib_none_mut().0))
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    pub fn has_password(&mut self) -> bool {
        unsafe {
            from_glib(ffi::webkit_credential_has_password(self.to_glib_none_mut().0))
        }
    }
}
