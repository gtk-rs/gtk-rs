// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct FilenameCompleter(Object<ffi::GFilenameCompleter, ffi::GFilenameCompleterClass>);

    match fn {
        get_type => || ffi::g_filename_completer_get_type(),
    }
}

impl FilenameCompleter {
    pub fn new() -> FilenameCompleter {
        unsafe { from_glib_full(ffi::g_filename_completer_new()) }
    }
}

impl Default for FilenameCompleter {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_FILENAME_COMPLETER: Option<&FilenameCompleter> = None;

pub trait FilenameCompleterExt: 'static {
    fn get_completion_suffix(&self, initial_text: &str) -> Option<glib::GString>;

    fn get_completions(&self, initial_text: &str) -> Vec<glib::GString>;

    fn set_dirs_only(&self, dirs_only: bool);

    fn connect_got_completion_data<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FilenameCompleter>> FilenameCompleterExt for O {
    fn get_completion_suffix(&self, initial_text: &str) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::g_filename_completer_get_completion_suffix(
                self.as_ref().to_glib_none().0,
                initial_text.to_glib_none().0,
            ))
        }
    }

    fn get_completions(&self, initial_text: &str) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_filename_completer_get_completions(
                self.as_ref().to_glib_none().0,
                initial_text.to_glib_none().0,
            ))
        }
    }

    fn set_dirs_only(&self, dirs_only: bool) {
        unsafe {
            ffi::g_filename_completer_set_dirs_only(
                self.as_ref().to_glib_none().0,
                dirs_only.to_glib(),
            );
        }
    }

    fn connect_got_completion_data<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn got_completion_data_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GFilenameCompleter,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<FilenameCompleter>,
        {
            let f: &F = &*(f as *const F);
            f(&FilenameCompleter::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"got-completion-data\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    got_completion_data_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for FilenameCompleter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FilenameCompleter")
    }
}
