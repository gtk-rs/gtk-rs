// Copyright 2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gtk_sys;

use glib;
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;

use crate::Application;
use crate::Window;

pub trait GtkApplicationImpl:
    GtkApplicationImplExt + gio::subclass::prelude::ApplicationImpl
{
    fn window_added(&self, application: &Self::Type, window: &Window) {
        self.parent_window_added(application, window)
    }

    fn window_removed(&self, application: &Self::Type, window: &Window) {
        self.parent_window_removed(application, window)
    }
}

pub trait GtkApplicationImplExt: ObjectSubclass {
    fn parent_window_added(&self, application: &Self::Type, window: &Window);
    fn parent_window_removed(&self, application: &Self::Type, window: &Window);
}

impl<T: GtkApplicationImpl> GtkApplicationImplExt for T {
    fn parent_window_added(&self, application: &Self::Type, window: &Window) {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gtk_sys::GtkApplicationClass;
            if let Some(f) = (*parent_class).window_added {
                f(
                    application
                        .unsafe_cast_ref::<Application>()
                        .to_glib_none()
                        .0,
                    window.to_glib_none().0,
                )
            }
        }
    }

    fn parent_window_removed(&self, application: &Self::Type, window: &Window) {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut gtk_sys::GtkApplicationClass;
            if let Some(f) = (*parent_class).window_removed {
                f(
                    application
                        .unsafe_cast_ref::<Application>()
                        .to_glib_none()
                        .0,
                    window.to_glib_none().0,
                )
            }
        }
    }
}

unsafe impl<T: GtkApplicationImpl> IsSubclassable<T> for Application {
    fn override_vfuncs(class: &mut glib::Class<Self>) {
        unsafe extern "C" fn application_window_added<T: GtkApplicationImpl>(
            ptr: *mut gtk_sys::GtkApplication,
            wptr: *mut gtk_sys::GtkWindow,
        ) {
            let instance = &*(ptr as *mut T::Instance);
            let imp = instance.get_impl();
            let wrap: Borrowed<Application> = from_glib_borrow(ptr);

            imp.window_added(wrap.unsafe_cast_ref(), &from_glib_borrow(wptr))
        }
        unsafe extern "C" fn application_window_removed<T: GtkApplicationImpl>(
            ptr: *mut gtk_sys::GtkApplication,
            wptr: *mut gtk_sys::GtkWindow,
        ) {
            let instance = &*(ptr as *mut T::Instance);
            let imp = instance.get_impl();
            let wrap: Borrowed<Application> = from_glib_borrow(ptr);

            imp.window_removed(wrap.unsafe_cast_ref(), &from_glib_borrow(wptr))
        }

        unsafe extern "C" fn application_startup<T: GtkApplicationImpl>(
            ptr: *mut gio_sys::GApplication,
        ) {
            let instance = &*(ptr as *mut T::Instance);
            let imp = instance.get_impl();
            let wrap: Borrowed<gio::Application> = from_glib_borrow(ptr);
            crate::rt::set_initialized();
            imp.startup(wrap.unsafe_cast_ref())
        }

        <gio::Application as IsSubclassable<T>>::override_vfuncs(class);

        let klass = class.as_mut();
        klass.window_added = Some(application_window_added::<T>);
        klass.window_removed = Some(application_window_removed::<T>);
        // Chain our startup handler in here
        let klass = &mut class.as_mut().parent_class;
        klass.startup = Some(application_startup::<T>);
    }
}
