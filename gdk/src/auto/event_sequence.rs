// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

glib::glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct EventSequence(Boxed<ffi::GdkEventSequence>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::gdk_event_sequence_get_type(), ptr as *mut _) as *mut ffi::GdkEventSequence,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::gdk_event_sequence_get_type(), ptr as *mut _),
        get_type => || ffi::gdk_event_sequence_get_type(),
    }
}
