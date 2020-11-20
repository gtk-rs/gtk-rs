// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use crate::Cancellable;
use crate::Drive;
use crate::File;
use crate::Icon;
use crate::MountMountFlags;
use crate::MountOperation;
use crate::MountUnmountFlags;
use crate::Volume;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::pin::Pin;
use std::ptr;

glib::glib_wrapper! {
    pub struct Mount(Interface<ffi::GMount>);

    match fn {
        get_type => || ffi::g_mount_get_type(),
    }
}

pub const NONE_MOUNT: Option<&Mount> = None;

pub trait MountExt: 'static {
    fn can_eject(&self) -> bool;

    fn can_unmount(&self) -> bool;

    fn eject_with_operation<
        P: IsA<MountOperation>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<(), glib::Error>) + Send + 'static,
    >(
        &self,
        flags: MountUnmountFlags,
        mount_operation: Option<&P>,
        cancellable: Option<&Q>,
        callback: R,
    );

    fn eject_with_operation_future<P: IsA<MountOperation> + Clone + 'static>(
        &self,
        flags: MountUnmountFlags,
        mount_operation: Option<&P>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;

    fn get_default_location(&self) -> File;

    fn get_drive(&self) -> Option<Drive>;

    fn get_icon(&self) -> Icon;

    fn get_name(&self) -> glib::GString;

    fn get_root(&self) -> File;

    fn get_sort_key(&self) -> Option<glib::GString>;

    fn get_symbolic_icon(&self) -> Icon;

    fn get_uuid(&self) -> Option<glib::GString>;

    fn get_volume(&self) -> Option<Volume>;

    fn guess_content_type<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<Vec<glib::GString>, glib::Error>) + Send + 'static,
    >(
        &self,
        force_rescan: bool,
        cancellable: Option<&P>,
        callback: Q,
    );

    fn guess_content_type_future(
        &self,
        force_rescan: bool,
    ) -> Pin<
        Box_<dyn std::future::Future<Output = Result<Vec<glib::GString>, glib::Error>> + 'static>,
    >;

    fn guess_content_type_sync<P: IsA<Cancellable>>(
        &self,
        force_rescan: bool,
        cancellable: Option<&P>,
    ) -> Result<Vec<glib::GString>, glib::Error>;

    fn is_shadowed(&self) -> bool;

    fn remount<
        P: IsA<MountOperation>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<(), glib::Error>) + Send + 'static,
    >(
        &self,
        flags: MountMountFlags,
        mount_operation: Option<&P>,
        cancellable: Option<&Q>,
        callback: R,
    );

    fn remount_future<P: IsA<MountOperation> + Clone + 'static>(
        &self,
        flags: MountMountFlags,
        mount_operation: Option<&P>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;

    fn shadow(&self);

    fn unmount_with_operation<
        P: IsA<MountOperation>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<(), glib::Error>) + Send + 'static,
    >(
        &self,
        flags: MountUnmountFlags,
        mount_operation: Option<&P>,
        cancellable: Option<&Q>,
        callback: R,
    );

    fn unmount_with_operation_future<P: IsA<MountOperation> + Clone + 'static>(
        &self,
        flags: MountUnmountFlags,
        mount_operation: Option<&P>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;

    fn unshadow(&self);

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_pre_unmount<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_unmounted<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Mount>> MountExt for O {
    fn can_eject(&self) -> bool {
        unsafe { from_glib(ffi::g_mount_can_eject(self.as_ref().to_glib_none().0)) }
    }

    fn can_unmount(&self) -> bool {
        unsafe { from_glib(ffi::g_mount_can_unmount(self.as_ref().to_glib_none().0)) }
    }

    fn eject_with_operation<
        P: IsA<MountOperation>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<(), glib::Error>) + Send + 'static,
    >(
        &self,
        flags: MountUnmountFlags,
        mount_operation: Option<&P>,
        cancellable: Option<&Q>,
        callback: R,
    ) {
        let user_data: Box_<R> = Box_::new(callback);
        unsafe extern "C" fn eject_with_operation_trampoline<
            R: FnOnce(Result<(), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ =
                ffi::g_mount_eject_with_operation_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<R> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = eject_with_operation_trampoline::<R>;
        unsafe {
            ffi::g_mount_eject_with_operation(
                self.as_ref().to_glib_none().0,
                flags.to_glib(),
                mount_operation.map(|p| p.as_ref()).to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn eject_with_operation_future<P: IsA<MountOperation> + Clone + 'static>(
        &self,
        flags: MountUnmountFlags,
        mount_operation: Option<&P>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        let mount_operation = mount_operation.map(ToOwned::to_owned);
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.eject_with_operation(
                flags,
                mount_operation.as_ref().map(::std::borrow::Borrow::borrow),
                Some(&cancellable),
                move |res| {
                    send.resolve(res);
                },
            );

            cancellable
        }))
    }

    fn get_default_location(&self) -> File {
        unsafe {
            from_glib_full(ffi::g_mount_get_default_location(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_drive(&self) -> Option<Drive> {
        unsafe { from_glib_full(ffi::g_mount_get_drive(self.as_ref().to_glib_none().0)) }
    }

    fn get_icon(&self) -> Icon {
        unsafe { from_glib_full(ffi::g_mount_get_icon(self.as_ref().to_glib_none().0)) }
    }

    fn get_name(&self) -> glib::GString {
        unsafe { from_glib_full(ffi::g_mount_get_name(self.as_ref().to_glib_none().0)) }
    }

    fn get_root(&self) -> File {
        unsafe { from_glib_full(ffi::g_mount_get_root(self.as_ref().to_glib_none().0)) }
    }

    fn get_sort_key(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::g_mount_get_sort_key(self.as_ref().to_glib_none().0)) }
    }

    fn get_symbolic_icon(&self) -> Icon {
        unsafe {
            from_glib_full(ffi::g_mount_get_symbolic_icon(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_uuid(&self) -> Option<glib::GString> {
        unsafe { from_glib_full(ffi::g_mount_get_uuid(self.as_ref().to_glib_none().0)) }
    }

    fn get_volume(&self) -> Option<Volume> {
        unsafe { from_glib_full(ffi::g_mount_get_volume(self.as_ref().to_glib_none().0)) }
    }

    fn guess_content_type<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<Vec<glib::GString>, glib::Error>) + Send + 'static,
    >(
        &self,
        force_rescan: bool,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn guess_content_type_trampoline<
            Q: FnOnce(Result<Vec<glib::GString>, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret =
                ffi::g_mount_guess_content_type_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(FromGlibPtrContainer::from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = guess_content_type_trampoline::<Q>;
        unsafe {
            ffi::g_mount_guess_content_type(
                self.as_ref().to_glib_none().0,
                force_rescan.to_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn guess_content_type_future(
        &self,
        force_rescan: bool,
    ) -> Pin<
        Box_<dyn std::future::Future<Output = Result<Vec<glib::GString>, glib::Error>> + 'static>,
    > {
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.guess_content_type(force_rescan, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    fn guess_content_type_sync<P: IsA<Cancellable>>(
        &self,
        force_rescan: bool,
        cancellable: Option<&P>,
    ) -> Result<Vec<glib::GString>, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_mount_guess_content_type_sync(
                self.as_ref().to_glib_none().0,
                force_rescan.to_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(FromGlibPtrContainer::from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn is_shadowed(&self) -> bool {
        unsafe { from_glib(ffi::g_mount_is_shadowed(self.as_ref().to_glib_none().0)) }
    }

    fn remount<
        P: IsA<MountOperation>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<(), glib::Error>) + Send + 'static,
    >(
        &self,
        flags: MountMountFlags,
        mount_operation: Option<&P>,
        cancellable: Option<&Q>,
        callback: R,
    ) {
        let user_data: Box_<R> = Box_::new(callback);
        unsafe extern "C" fn remount_trampoline<
            R: FnOnce(Result<(), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::g_mount_remount_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<R> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = remount_trampoline::<R>;
        unsafe {
            ffi::g_mount_remount(
                self.as_ref().to_glib_none().0,
                flags.to_glib(),
                mount_operation.map(|p| p.as_ref()).to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn remount_future<P: IsA<MountOperation> + Clone + 'static>(
        &self,
        flags: MountMountFlags,
        mount_operation: Option<&P>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        let mount_operation = mount_operation.map(ToOwned::to_owned);
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.remount(
                flags,
                mount_operation.as_ref().map(::std::borrow::Borrow::borrow),
                Some(&cancellable),
                move |res| {
                    send.resolve(res);
                },
            );

            cancellable
        }))
    }

    fn shadow(&self) {
        unsafe {
            ffi::g_mount_shadow(self.as_ref().to_glib_none().0);
        }
    }

    fn unmount_with_operation<
        P: IsA<MountOperation>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<(), glib::Error>) + Send + 'static,
    >(
        &self,
        flags: MountUnmountFlags,
        mount_operation: Option<&P>,
        cancellable: Option<&Q>,
        callback: R,
    ) {
        let user_data: Box_<R> = Box_::new(callback);
        unsafe extern "C" fn unmount_with_operation_trampoline<
            R: FnOnce(Result<(), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::g_mount_unmount_with_operation_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<R> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = unmount_with_operation_trampoline::<R>;
        unsafe {
            ffi::g_mount_unmount_with_operation(
                self.as_ref().to_glib_none().0,
                flags.to_glib(),
                mount_operation.map(|p| p.as_ref()).to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn unmount_with_operation_future<P: IsA<MountOperation> + Clone + 'static>(
        &self,
        flags: MountUnmountFlags,
        mount_operation: Option<&P>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        let mount_operation = mount_operation.map(ToOwned::to_owned);
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.unmount_with_operation(
                flags,
                mount_operation.as_ref().map(::std::borrow::Borrow::borrow),
                Some(&cancellable),
                move |res| {
                    send.resolve(res);
                },
            );

            cancellable
        }))
    }

    fn unshadow(&self) {
        unsafe {
            ffi::g_mount_unshadow(self.as_ref().to_glib_none().0);
        }
    }

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GMount,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Mount>,
        {
            let f: &F = &*(f as *const F);
            f(&Mount::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_pre_unmount<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn pre_unmount_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GMount,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Mount>,
        {
            let f: &F = &*(f as *const F);
            f(&Mount::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"pre-unmount\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    pre_unmount_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_unmounted<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn unmounted_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GMount,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Mount>,
        {
            let f: &F = &*(f as *const F);
            f(&Mount::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"unmounted\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    unmounted_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Mount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Mount")
    }
}
