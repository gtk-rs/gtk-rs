// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::EventController;
use crate::EventControllerScrollFlags;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct EventControllerScroll(Object<ffi::GtkEventControllerScroll, ffi::GtkEventControllerScrollClass>) @extends EventController;

    match fn {
        type_ => || ffi::gtk_event_controller_scroll_get_type(),
    }
}

impl EventControllerScroll {
    #[doc(alias = "gtk_event_controller_scroll_new")]
    pub fn new(
        widget: &impl IsA<Widget>,
        flags: EventControllerScrollFlags,
    ) -> EventControllerScroll {
        skip_assert_initialized!();
        unsafe {
            EventController::from_glib_full(ffi::gtk_event_controller_scroll_new(
                widget.as_ref().to_glib_none().0,
                flags.into_glib(),
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_event_controller_scroll_get_flags")]
    #[doc(alias = "get_flags")]
    pub fn flags(&self) -> EventControllerScrollFlags {
        unsafe {
            from_glib(ffi::gtk_event_controller_scroll_get_flags(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_event_controller_scroll_set_flags")]
    pub fn set_flags(&self, flags: EventControllerScrollFlags) {
        unsafe {
            ffi::gtk_event_controller_scroll_set_flags(self.to_glib_none().0, flags.into_glib());
        }
    }

    #[doc(alias = "decelerate")]
    pub fn connect_decelerate<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn decelerate_trampoline<
            F: Fn(&EventControllerScroll, f64, f64) + 'static,
        >(
            this: *mut ffi::GtkEventControllerScroll,
            vel_x: libc::c_double,
            vel_y: libc::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), vel_x, vel_y)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"decelerate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    decelerate_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "scroll")]
    pub fn connect_scroll<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn scroll_trampoline<
            F: Fn(&EventControllerScroll, f64, f64) + 'static,
        >(
            this: *mut ffi::GtkEventControllerScroll,
            dx: libc::c_double,
            dy: libc::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), dx, dy)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"scroll\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    scroll_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "scroll-begin")]
    pub fn connect_scroll_begin<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn scroll_begin_trampoline<F: Fn(&EventControllerScroll) + 'static>(
            this: *mut ffi::GtkEventControllerScroll,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"scroll-begin\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    scroll_begin_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "scroll-end")]
    pub fn connect_scroll_end<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn scroll_end_trampoline<F: Fn(&EventControllerScroll) + 'static>(
            this: *mut ffi::GtkEventControllerScroll,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"scroll-end\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    scroll_end_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_24")))]
    #[doc(alias = "flags")]
    pub fn connect_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_flags_trampoline<F: Fn(&EventControllerScroll) + 'static>(
            this: *mut ffi::GtkEventControllerScroll,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::flags\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_flags_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for EventControllerScroll {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("EventControllerScroll")
    }
}
