// Copyright 2013-2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

use crate::UnixMountPoint;
use glib::translate::*;
use std::mem;

impl UnixMountPoint {
    #[cfg(any(unix, all(not(doctest), doc)))]
    pub fn get_mount_points() -> (Vec<UnixMountPoint>, u64) {
        unsafe {
            let mut time_read = mem::MaybeUninit::uninit();
            let ret = FromGlibPtrContainer::from_glib_full(ffi::g_unix_mount_points_get(
                time_read.as_mut_ptr(),
            ));
            let time_read = time_read.assume_init();
            (ret, time_read)
        }
    }

    pub fn is_changed_since(time: u64) -> bool {
        unsafe { from_glib(ffi::g_unix_mount_points_changed_since(time)) }
    }
}
