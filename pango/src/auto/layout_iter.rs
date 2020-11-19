// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use crate::Layout;
use crate::LayoutLine;
use crate::LayoutRun;
use crate::Rectangle;
use glib;
use glib::translate::*;
use std::mem;

glib::glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct LayoutIter(Boxed<ffi::PangoLayoutIter>);

    match fn {
        copy => |ptr| ffi::pango_layout_iter_copy(mut_override(ptr)),
        free => |ptr| ffi::pango_layout_iter_free(ptr),
        get_type => || ffi::pango_layout_iter_get_type(),
    }
}

impl LayoutIter {
    pub fn at_last_line(&mut self) -> bool {
        unsafe {
            from_glib(ffi::pango_layout_iter_at_last_line(
                self.to_glib_none_mut().0,
            ))
        }
    }

    pub fn get_baseline(&mut self) -> i32 {
        unsafe { ffi::pango_layout_iter_get_baseline(self.to_glib_none_mut().0) }
    }

    pub fn get_char_extents(&mut self) -> Rectangle {
        unsafe {
            let mut logical_rect = Rectangle::uninitialized();
            ffi::pango_layout_iter_get_char_extents(
                self.to_glib_none_mut().0,
                logical_rect.to_glib_none_mut().0,
            );
            logical_rect
        }
    }

    pub fn get_cluster_extents(&mut self) -> (Rectangle, Rectangle) {
        unsafe {
            let mut ink_rect = Rectangle::uninitialized();
            let mut logical_rect = Rectangle::uninitialized();
            ffi::pango_layout_iter_get_cluster_extents(
                self.to_glib_none_mut().0,
                ink_rect.to_glib_none_mut().0,
                logical_rect.to_glib_none_mut().0,
            );
            (ink_rect, logical_rect)
        }
    }

    pub fn get_index(&mut self) -> i32 {
        unsafe { ffi::pango_layout_iter_get_index(self.to_glib_none_mut().0) }
    }

    pub fn get_layout(&mut self) -> Option<Layout> {
        unsafe { from_glib_none(ffi::pango_layout_iter_get_layout(self.to_glib_none_mut().0)) }
    }

    pub fn get_layout_extents(&mut self) -> (Rectangle, Rectangle) {
        unsafe {
            let mut ink_rect = Rectangle::uninitialized();
            let mut logical_rect = Rectangle::uninitialized();
            ffi::pango_layout_iter_get_layout_extents(
                self.to_glib_none_mut().0,
                ink_rect.to_glib_none_mut().0,
                logical_rect.to_glib_none_mut().0,
            );
            (ink_rect, logical_rect)
        }
    }

    pub fn get_line(&mut self) -> Option<LayoutLine> {
        unsafe { from_glib_none(ffi::pango_layout_iter_get_line(self.to_glib_none_mut().0)) }
    }

    pub fn get_line_extents(&mut self) -> (Rectangle, Rectangle) {
        unsafe {
            let mut ink_rect = Rectangle::uninitialized();
            let mut logical_rect = Rectangle::uninitialized();
            ffi::pango_layout_iter_get_line_extents(
                self.to_glib_none_mut().0,
                ink_rect.to_glib_none_mut().0,
                logical_rect.to_glib_none_mut().0,
            );
            (ink_rect, logical_rect)
        }
    }

    pub fn get_line_readonly(&mut self) -> Option<LayoutLine> {
        unsafe {
            from_glib_none(ffi::pango_layout_iter_get_line_readonly(
                self.to_glib_none_mut().0,
            ))
        }
    }

    pub fn get_line_yrange(&mut self) -> (i32, i32) {
        unsafe {
            let mut y0_ = mem::MaybeUninit::uninit();
            let mut y1_ = mem::MaybeUninit::uninit();
            ffi::pango_layout_iter_get_line_yrange(
                self.to_glib_none_mut().0,
                y0_.as_mut_ptr(),
                y1_.as_mut_ptr(),
            );
            let y0_ = y0_.assume_init();
            let y1_ = y1_.assume_init();
            (y0_, y1_)
        }
    }

    pub fn get_run(&mut self) -> Option<LayoutRun> {
        unsafe { from_glib_none(ffi::pango_layout_iter_get_run(self.to_glib_none_mut().0)) }
    }

    pub fn get_run_extents(&mut self) -> (Rectangle, Rectangle) {
        unsafe {
            let mut ink_rect = Rectangle::uninitialized();
            let mut logical_rect = Rectangle::uninitialized();
            ffi::pango_layout_iter_get_run_extents(
                self.to_glib_none_mut().0,
                ink_rect.to_glib_none_mut().0,
                logical_rect.to_glib_none_mut().0,
            );
            (ink_rect, logical_rect)
        }
    }

    pub fn get_run_readonly(&mut self) -> Option<LayoutRun> {
        unsafe {
            from_glib_none(ffi::pango_layout_iter_get_run_readonly(
                self.to_glib_none_mut().0,
            ))
        }
    }

    pub fn next_char(&mut self) -> bool {
        unsafe { from_glib(ffi::pango_layout_iter_next_char(self.to_glib_none_mut().0)) }
    }

    pub fn next_cluster(&mut self) -> bool {
        unsafe {
            from_glib(ffi::pango_layout_iter_next_cluster(
                self.to_glib_none_mut().0,
            ))
        }
    }

    pub fn next_line(&mut self) -> bool {
        unsafe { from_glib(ffi::pango_layout_iter_next_line(self.to_glib_none_mut().0)) }
    }

    pub fn next_run(&mut self) -> bool {
        unsafe { from_glib(ffi::pango_layout_iter_next_run(self.to_glib_none_mut().0)) }
    }
}
