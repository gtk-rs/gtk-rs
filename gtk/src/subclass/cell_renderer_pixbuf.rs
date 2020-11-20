// Copyright 2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib;
use glib::subclass::prelude::*;

use super::cell_renderer::CellRendererImpl;
use crate::CellRenderer;
use crate::CellRendererPixbuf;

pub trait CellRendererPixbufImpl: CellRendererImpl {}

unsafe impl<T: CellRendererPixbufImpl> IsSubclassable<T> for CellRendererPixbuf {
    fn override_vfuncs(class: &mut glib::Class<Self>) {
        <CellRenderer as IsSubclassable<T>>::override_vfuncs(class);
    }
}
