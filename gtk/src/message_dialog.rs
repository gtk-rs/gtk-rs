// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ButtonsType;
use crate::DialogFlags;
use crate::MessageDialog;
use crate::MessageType;
use crate::Widget;
use crate::Window;
use glib::object::{Cast, IsA};
use glib::translate::*;
use libc::c_char;
use std::ptr;

impl MessageDialog {
    /// Creates a new message dialog, which is a simple dialog with some text
    /// the user may want to see. When the user clicks a button a “response”
    /// signal is emitted with response IDs from GtkResponseType. See
    /// [Dialog](crate::Dialog) for more details.
    /// ## `parent`
    /// transient parent, or [`None`] for none
    /// ## `flags`
    /// flags
    /// ## `type_`
    /// type of message
    /// ## `buttons`
    /// set of buttons to use
    /// ## `message_format`
    /// `printf`-style format string, or [`None`]
    ///
    /// # Returns
    ///
    /// a new [MessageDialog](crate::MessageDialog)
    #[doc(alias = "gtk_message_dialog_new")]
    pub fn new<T: IsA<Window>>(
        parent: Option<&T>,
        flags: DialogFlags,
        type_: MessageType,
        buttons: ButtonsType,
        message: &str,
    ) -> MessageDialog {
        assert_initialized_main_thread!();
        unsafe {
            let message: Stash<*const c_char, _> = message.to_glib_none();
            Widget::from_glib_none(ffi::gtk_message_dialog_new(
                parent.map(|p| p.as_ref()).to_glib_none().0,
                flags.into_glib(),
                type_.into_glib(),
                buttons.into_glib(),
                b"%s\0".as_ptr() as *const c_char,
                message.0,
                ptr::null::<c_char>(),
            ))
            .unsafe_cast()
        }
    }
}

pub trait MessageDialogExt: 'static {
    #[doc(alias = "gtk_message_dialog_format_secondary_markup")]
    fn set_secondary_markup(&self, message: Option<&str>);

    #[doc(alias = "gtk_message_dialog_format_secondary_text")]
    fn set_secondary_text(&self, message: Option<&str>);
}

impl<O: IsA<MessageDialog>> MessageDialogExt for O {
    fn set_secondary_markup(&self, message: Option<&str>) {
        match message {
            Some(m) => unsafe {
                let message: Stash<*const c_char, _> = m.to_glib_none();
                ffi::gtk_message_dialog_format_secondary_markup(
                    self.as_ref().to_glib_none().0,
                    b"%s\0".as_ptr() as *const c_char,
                    message.0,
                    ptr::null::<c_char>(),
                )
            },
            None => unsafe {
                ffi::gtk_message_dialog_format_secondary_markup(
                    self.as_ref().to_glib_none().0,
                    ptr::null::<c_char>(),
                )
            },
        }
    }

    fn set_secondary_text(&self, message: Option<&str>) {
        match message {
            Some(m) => unsafe {
                let message: Stash<*const c_char, _> = m.to_glib_none();
                ffi::gtk_message_dialog_format_secondary_text(
                    self.as_ref().to_glib_none().0,
                    b"%s\0".as_ptr() as *const c_char,
                    message.0,
                    ptr::null::<c_char>(),
                )
            },
            None => unsafe {
                ffi::gtk_message_dialog_format_secondary_text(
                    self.as_ref().to_glib_none().0,
                    ptr::null::<c_char>(),
                )
            },
        }
    }
}
