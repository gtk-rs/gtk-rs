// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
#[cfg(any(feature = "v3_22", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
use crate::Window;
#[cfg(any(feature = "v3_22", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
use cairo;
use glib;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct DrawingContext(Object<ffi::GdkDrawingContext, ffi::GdkDrawingContextClass>);

    match fn {
        get_type => || ffi::gdk_drawing_context_get_type(),
    }
}

impl DrawingContext {
    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    pub fn get_cairo_context(&self) -> Option<cairo::Context> {
        unsafe {
            from_glib_none(ffi::gdk_drawing_context_get_cairo_context(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    pub fn get_clip(&self) -> Option<cairo::Region> {
        unsafe { from_glib_full(ffi::gdk_drawing_context_get_clip(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    pub fn get_window(&self) -> Option<Window> {
        unsafe { from_glib_none(ffi::gdk_drawing_context_get_window(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    pub fn is_valid(&self) -> bool {
        unsafe { from_glib(ffi::gdk_drawing_context_is_valid(self.to_glib_none().0)) }
    }
}

impl fmt::Display for DrawingContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DrawingContext")
    }
}
