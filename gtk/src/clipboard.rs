// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Clipboard;
use crate::SelectionData;
use crate::TargetEntry;
use glib::ffi::gpointer;
use glib::translate::*;
use libc::{c_char, c_uint};
use std::boxed::Box as Box_;

impl Clipboard {
    /// Virtually sets the contents of the specified clipboard by providing
    /// a list of supported formats for the clipboard data and a function
    /// to call to get the actual data when it is requested.
    /// ## `targets`
    /// array containing information
    ///  about the available forms for the clipboard data
    /// ## `get_func`
    /// function to call to get the actual clipboard data
    /// ## `clear_func`
    /// when the clipboard contents are set again,
    ///  this function will be called, and `get_func` will not be subsequently
    ///  called.
    ///
    /// # Returns
    ///
    /// [`true`] if setting the clipboard data succeeded.
    ///  If setting the clipboard data failed the provided callback
    ///  functions will be ignored.
    #[doc(alias = "gtk_clipboard_set_with_data")]
    pub fn set_with_data<F: Fn(&Clipboard, &SelectionData, u32) + 'static>(
        &self,
        targets: &[TargetEntry],
        f: F,
    ) -> bool {
        unsafe extern "C" fn trampoline<F: Fn(&Clipboard, &SelectionData, u32) + 'static>(
            clipboard: *mut ffi::GtkClipboard,
            selection_data: *mut ffi::GtkSelectionData,
            info: c_uint,
            user_data: gpointer,
        ) {
            let f: &F = &*(user_data as *const F);
            f(
                &from_glib_borrow(clipboard),
                &from_glib_borrow(selection_data),
                info,
            );
        }
        unsafe extern "C" fn cleanup<F: Fn(&Clipboard, &SelectionData, u32) + 'static>(
            _clipboard: *mut ffi::GtkClipboard,
            user_data: gpointer,
        ) {
            Box_::<F>::from_raw(user_data as *mut _);
        }
        let stashed_targets: Vec<_> = targets.iter().map(|e| e.to_glib_none()).collect();
        let mut t = Vec::with_capacity(stashed_targets.len());
        for stash in &stashed_targets {
            unsafe {
                t.push(ffi::GtkTargetEntry {
                    target: (*stash.0).target,
                    flags: (*stash.0).flags,
                    info: (*stash.0).info,
                });
            }
        }
        let t_ptr: *mut ffi::GtkTargetEntry = t.as_mut_ptr();
        let f: Box_<F> = Box_::new(f);
        let user_data = Box_::into_raw(f) as *mut _;
        let success: bool = unsafe {
            from_glib(ffi::gtk_clipboard_set_with_data(
                self.to_glib_none().0,
                t_ptr,
                t.len() as c_uint,
                Some(trampoline::<F>),
                Some(cleanup::<F>),
                user_data,
            ))
        };
        if !success {
            // Cleanup function is not called in case of a failure.
            unsafe {
                Box_::<F>::from_raw(user_data as *mut _);
            }
        }
        success
    }

    /// Requests the contents of the clipboard as URIs. When the URIs are
    /// later received `callback` will be called.
    ///
    /// The `uris` parameter to `callback` will contain the resulting array of
    /// URIs if the request succeeded, or [`None`] if it failed. This could happen
    /// for various reasons, in particular if the clipboard was empty or if the
    /// contents of the clipboard could not be converted into URI form.
    /// ## `callback`
    /// a function to call when the URIs are received,
    ///  or the retrieval fails. (It will always be called one way or the other.)
    #[doc(alias = "gtk_clipboard_request_uris")]
    pub fn request_uris<P: FnOnce(&Clipboard, &[glib::GString]) + 'static>(&self, callback: P) {
        let callback_data: Box_<P> = Box_::new(callback);
        unsafe extern "C" fn callback_func<P: FnOnce(&Clipboard, &[glib::GString]) + 'static>(
            clipboard: *mut ffi::GtkClipboard,
            uris: *mut *mut c_char,
            data: glib::ffi::gpointer,
        ) {
            let clipboard = from_glib_borrow(clipboard);
            let uris: Vec<glib::GString> = FromGlibPtrContainer::from_glib_none(uris);
            let callback: Box_<P> = Box_::from_raw(data as *mut _);
            (*callback)(&clipboard, &uris);
        }
        let callback = Some(callback_func::<P> as _);
        let super_callback0: Box_<P> = callback_data;
        unsafe {
            ffi::gtk_clipboard_request_uris(
                self.to_glib_none().0,
                callback,
                Box_::into_raw(super_callback0) as *mut _,
            );
        }
    }
}
