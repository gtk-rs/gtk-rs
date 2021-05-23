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

glib::wrapper! {
    pub struct AccelGroup(Object<ffi::GtkAccelGroup, ffi::GtkAccelGroupClass>);

    match fn {
        type_ => || ffi::gtk_accel_group_get_type(),
    }
}

impl AccelGroup {
    #[doc(alias = "gtk_accel_group_new")]
    pub fn new() -> AccelGroup {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_accel_group_new()) }
    }

    #[doc(alias = "gtk_accel_group_from_accel_closure")]
    pub fn from_accel_closure(closure: &glib::Closure) -> Option<AccelGroup> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_accel_group_from_accel_closure(
                closure.to_glib_none().0,
            ))
        }
    }
}

impl Default for AccelGroup {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_ACCEL_GROUP: Option<&AccelGroup> = None;

pub trait AccelGroupExt: 'static {
    #[doc(alias = "gtk_accel_group_activate")]
    fn activate(
        &self,
        accel_quark: glib::Quark,
        acceleratable: &impl IsA<glib::Object>,
        accel_key: u32,
        accel_mods: gdk::ModifierType,
    ) -> bool;

    #[doc(alias = "gtk_accel_group_disconnect")]
    fn disconnect(&self, closure: Option<&glib::Closure>) -> bool;

    #[doc(alias = "gtk_accel_group_disconnect_key")]
    fn disconnect_key(&self, accel_key: u32, accel_mods: gdk::ModifierType) -> bool;

    //#[doc(alias = "gtk_accel_group_find")]
    //fn find(&self, find_func: /*Unimplemented*/FnMut(/*Ignored*/AccelKey, &glib::Closure) -> bool, data: /*Unimplemented*/Option<Fundamental: Pointer>) -> /*Ignored*/Option<AccelKey>;

    #[doc(alias = "gtk_accel_group_get_is_locked")]
    #[doc(alias = "get_is_locked")]
    fn is_locked(&self) -> bool;

    #[doc(alias = "gtk_accel_group_get_modifier_mask")]
    #[doc(alias = "get_modifier_mask")]
    fn modifier_mask(&self) -> gdk::ModifierType;

    #[doc(alias = "gtk_accel_group_lock")]
    fn lock(&self);

    #[doc(alias = "gtk_accel_group_unlock")]
    fn unlock(&self);

    #[doc(alias = "accel-activate")]
    fn connect_accel_activate<
        F: Fn(&Self, &glib::Object, u32, gdk::ModifierType) -> bool + 'static,
    >(
        &self,
        detail: Option<&str>,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "accel-changed")]
    fn connect_accel_changed<F: Fn(&Self, u32, gdk::ModifierType, &glib::Closure) + 'static>(
        &self,
        detail: Option<&str>,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "is-locked")]
    fn connect_is_locked_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "modifier-mask")]
    fn connect_modifier_mask_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<AccelGroup>> AccelGroupExt for O {
    fn activate(
        &self,
        accel_quark: glib::Quark,
        acceleratable: &impl IsA<glib::Object>,
        accel_key: u32,
        accel_mods: gdk::ModifierType,
    ) -> bool {
        unsafe {
            from_glib(ffi::gtk_accel_group_activate(
                self.as_ref().to_glib_none().0,
                accel_quark.into_glib(),
                acceleratable.as_ref().to_glib_none().0,
                accel_key,
                accel_mods.into_glib(),
            ))
        }
    }

    fn disconnect(&self, closure: Option<&glib::Closure>) -> bool {
        unsafe {
            from_glib(ffi::gtk_accel_group_disconnect(
                self.as_ref().to_glib_none().0,
                closure.to_glib_none().0,
            ))
        }
    }

    fn disconnect_key(&self, accel_key: u32, accel_mods: gdk::ModifierType) -> bool {
        unsafe {
            from_glib(ffi::gtk_accel_group_disconnect_key(
                self.as_ref().to_glib_none().0,
                accel_key,
                accel_mods.into_glib(),
            ))
        }
    }

    //fn find(&self, find_func: /*Unimplemented*/FnMut(/*Ignored*/AccelKey, &glib::Closure) -> bool, data: /*Unimplemented*/Option<Fundamental: Pointer>) -> /*Ignored*/Option<AccelKey> {
    //    unsafe { TODO: call ffi:gtk_accel_group_find() }
    //}

    fn is_locked(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_accel_group_get_is_locked(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn modifier_mask(&self) -> gdk::ModifierType {
        unsafe {
            from_glib(ffi::gtk_accel_group_get_modifier_mask(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn lock(&self) {
        unsafe {
            ffi::gtk_accel_group_lock(self.as_ref().to_glib_none().0);
        }
    }

    fn unlock(&self) {
        unsafe {
            ffi::gtk_accel_group_unlock(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "accel-activate")]
    fn connect_accel_activate<
        F: Fn(&Self, &glib::Object, u32, gdk::ModifierType) -> bool + 'static,
    >(
        &self,
        detail: Option<&str>,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn accel_activate_trampoline<
            P: IsA<AccelGroup>,
            F: Fn(&P, &glib::Object, u32, gdk::ModifierType) -> bool + 'static,
        >(
            this: *mut ffi::GtkAccelGroup,
            acceleratable: *mut glib::gobject_ffi::GObject,
            keyval: libc::c_uint,
            modifier: gdk::ffi::GdkModifierType,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                &AccelGroup::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(acceleratable),
                keyval,
                from_glib(modifier),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            let detailed_signal_name = detail.map(|name| format!("accel-activate::{}\0", name));
            let signal_name: &[u8] = detailed_signal_name
                .as_ref()
                .map_or(&b"accel-activate\0"[..], |n| n.as_bytes());
            connect_raw(
                self.as_ptr() as *mut _,
                signal_name.as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    accel_activate_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "accel-changed")]
    fn connect_accel_changed<F: Fn(&Self, u32, gdk::ModifierType, &glib::Closure) + 'static>(
        &self,
        detail: Option<&str>,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn accel_changed_trampoline<
            P: IsA<AccelGroup>,
            F: Fn(&P, u32, gdk::ModifierType, &glib::Closure) + 'static,
        >(
            this: *mut ffi::GtkAccelGroup,
            keyval: libc::c_uint,
            modifier: gdk::ffi::GdkModifierType,
            accel_closure: *mut glib::gobject_ffi::GClosure,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &AccelGroup::from_glib_borrow(this).unsafe_cast_ref(),
                keyval,
                from_glib(modifier),
                &from_glib_borrow(accel_closure),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            let detailed_signal_name = detail.map(|name| format!("accel-changed::{}\0", name));
            let signal_name: &[u8] = detailed_signal_name
                .as_ref()
                .map_or(&b"accel-changed\0"[..], |n| n.as_bytes());
            connect_raw(
                self.as_ptr() as *mut _,
                signal_name.as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    accel_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "is-locked")]
    fn connect_is_locked_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_locked_trampoline<
            P: IsA<AccelGroup>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkAccelGroup,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&AccelGroup::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-locked\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_locked_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "modifier-mask")]
    fn connect_modifier_mask_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_modifier_mask_trampoline<
            P: IsA<AccelGroup>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkAccelGroup,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&AccelGroup::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::modifier-mask\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_modifier_mask_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for AccelGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("AccelGroup")
    }
}
