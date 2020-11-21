// Copyright 2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use crate::RadioMenuItem;
use crate::Widget;
use glib::object::Cast;
use glib::translate::*;
use std::ptr;

impl RadioMenuItem {
    pub fn new() -> RadioMenuItem {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_radio_menu_item_new(ptr::null_mut())).unsafe_cast()
        }
    }

    pub fn with_label(label: &str) -> RadioMenuItem {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_radio_menu_item_new_with_label(
                ptr::null_mut(),
                label.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    pub fn with_mnemonic(label: &str) -> RadioMenuItem {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_radio_menu_item_new_with_mnemonic(
                ptr::null_mut(),
                label.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }
}
