// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Align;
use crate::Buildable;
use crate::Container;
use crate::IconSize;
use crate::ImageType;
use crate::Misc;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::value::SetValueOptional;
use glib::StaticType;
use glib::ToValue;
use glib::Value;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib::glib_wrapper! {
    pub struct Image(Object<ffi::GtkImage, ffi::GtkImageClass>) @extends Misc, Widget, @implements Buildable;

    match fn {
        get_type => || ffi::gtk_image_get_type(),
    }
}

impl Image {
    pub fn new() -> Image {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_image_new()).unsafe_cast() }
    }

    pub fn from_animation<P: IsA<gdk_pixbuf::PixbufAnimation>>(animation: &P) -> Image {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_animation(
                animation.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    pub fn from_file<P: AsRef<std::path::Path>>(filename: P) -> Image {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_file(
                filename.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    pub fn from_gicon<P: IsA<gio::Icon>>(icon: &P, size: IconSize) -> Image {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_gicon(
                icon.as_ref().to_glib_none().0,
                size.to_glib(),
            ))
            .unsafe_cast()
        }
    }

    pub fn from_icon_name(icon_name: Option<&str>, size: IconSize) -> Image {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_icon_name(
                icon_name.to_glib_none().0,
                size.to_glib(),
            ))
            .unsafe_cast()
        }
    }

    pub fn from_pixbuf(pixbuf: Option<&gdk_pixbuf::Pixbuf>) -> Image {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_pixbuf(pixbuf.to_glib_none().0))
                .unsafe_cast()
        }
    }

    pub fn from_resource(resource_path: &str) -> Image {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_resource(
                resource_path.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    pub fn from_surface(surface: Option<&cairo::Surface>) -> Image {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_surface(mut_override(
                surface.to_glib_none().0,
            )))
            .unsafe_cast()
        }
    }
}

impl Default for Image {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct ImageBuilder {
    file: Option<String>,
    gicon: Option<gio::Icon>,
    icon_name: Option<String>,
    icon_size: Option<i32>,
    pixbuf: Option<gdk_pixbuf::Pixbuf>,
    pixbuf_animation: Option<gdk_pixbuf::PixbufAnimation>,
    pixel_size: Option<i32>,
    resource: Option<String>,
    surface: Option<cairo::Surface>,
    use_fallback: Option<bool>,
    app_paintable: Option<bool>,
    can_default: Option<bool>,
    can_focus: Option<bool>,
    events: Option<gdk::EventMask>,
    expand: Option<bool>,
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
    focus_on_click: Option<bool>,
    halign: Option<Align>,
    has_default: Option<bool>,
    has_focus: Option<bool>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    is_focus: Option<bool>,
    margin: Option<i32>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    no_show_all: Option<bool>,
    opacity: Option<f64>,
    parent: Option<Container>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
}

impl ImageBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Image {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref file) = self.file {
            properties.push(("file", file));
        }
        if let Some(ref gicon) = self.gicon {
            properties.push(("gicon", gicon));
        }
        if let Some(ref icon_name) = self.icon_name {
            properties.push(("icon-name", icon_name));
        }
        if let Some(ref icon_size) = self.icon_size {
            properties.push(("icon-size", icon_size));
        }
        if let Some(ref pixbuf) = self.pixbuf {
            properties.push(("pixbuf", pixbuf));
        }
        if let Some(ref pixbuf_animation) = self.pixbuf_animation {
            properties.push(("pixbuf-animation", pixbuf_animation));
        }
        if let Some(ref pixel_size) = self.pixel_size {
            properties.push(("pixel-size", pixel_size));
        }
        if let Some(ref resource) = self.resource {
            properties.push(("resource", resource));
        }
        if let Some(ref surface) = self.surface {
            properties.push(("surface", surface));
        }
        if let Some(ref use_fallback) = self.use_fallback {
            properties.push(("use-fallback", use_fallback));
        }
        if let Some(ref app_paintable) = self.app_paintable {
            properties.push(("app-paintable", app_paintable));
        }
        if let Some(ref can_default) = self.can_default {
            properties.push(("can-default", can_default));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref events) = self.events {
            properties.push(("events", events));
        }
        if let Some(ref expand) = self.expand {
            properties.push(("expand", expand));
        }
        #[cfg(any(feature = "v3_20", feature = "dox"))]
        #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
        {
            if let Some(ref focus_on_click) = self.focus_on_click {
                properties.push(("focus-on-click", focus_on_click));
            }
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_default) = self.has_default {
            properties.push(("has-default", has_default));
        }
        if let Some(ref has_focus) = self.has_focus {
            properties.push(("has-focus", has_focus));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref is_focus) = self.is_focus {
            properties.push(("is-focus", is_focus));
        }
        if let Some(ref margin) = self.margin {
            properties.push(("margin", margin));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref no_show_all) = self.no_show_all {
            properties.push(("no-show-all", no_show_all));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref parent) = self.parent {
            properties.push(("parent", parent));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        let ret = glib::Object::new(Image::static_type(), &properties)
            .expect("object new")
            .downcast::<Image>()
            .expect("downcast");
        ret
    }

    pub fn file(mut self, file: &str) -> Self {
        self.file = Some(file.to_string());
        self
    }

    pub fn gicon<P: IsA<gio::Icon>>(mut self, gicon: &P) -> Self {
        self.gicon = Some(gicon.clone().upcast());
        self
    }

    pub fn icon_name(mut self, icon_name: &str) -> Self {
        self.icon_name = Some(icon_name.to_string());
        self
    }

    pub fn icon_size(mut self, icon_size: i32) -> Self {
        self.icon_size = Some(icon_size);
        self
    }

    pub fn pixbuf(mut self, pixbuf: &gdk_pixbuf::Pixbuf) -> Self {
        self.pixbuf = Some(pixbuf.clone());
        self
    }

    pub fn pixbuf_animation<P: IsA<gdk_pixbuf::PixbufAnimation>>(
        mut self,
        pixbuf_animation: &P,
    ) -> Self {
        self.pixbuf_animation = Some(pixbuf_animation.clone().upcast());
        self
    }

    pub fn pixel_size(mut self, pixel_size: i32) -> Self {
        self.pixel_size = Some(pixel_size);
        self
    }

    pub fn resource(mut self, resource: &str) -> Self {
        self.resource = Some(resource.to_string());
        self
    }

    pub fn surface(mut self, surface: &cairo::Surface) -> Self {
        self.surface = Some(surface.clone());
        self
    }

    pub fn use_fallback(mut self, use_fallback: bool) -> Self {
        self.use_fallback = Some(use_fallback);
        self
    }

    pub fn app_paintable(mut self, app_paintable: bool) -> Self {
        self.app_paintable = Some(app_paintable);
        self
    }

    pub fn can_default(mut self, can_default: bool) -> Self {
        self.can_default = Some(can_default);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn events(mut self, events: gdk::EventMask) -> Self {
        self.events = Some(events);
        self
    }

    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_default(mut self, has_default: bool) -> Self {
        self.has_default = Some(has_default);
        self
    }

    pub fn has_focus(mut self, has_focus: bool) -> Self {
        self.has_focus = Some(has_focus);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn is_focus(mut self, is_focus: bool) -> Self {
        self.is_focus = Some(is_focus);
        self
    }

    pub fn margin(mut self, margin: i32) -> Self {
        self.margin = Some(margin);
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn no_show_all(mut self, no_show_all: bool) -> Self {
        self.no_show_all = Some(no_show_all);
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn parent<P: IsA<Container>>(mut self, parent: &P) -> Self {
        self.parent = Some(parent.clone().upcast());
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }
}

pub const NONE_IMAGE: Option<&Image> = None;

pub trait ImageExt: 'static {
    fn clear(&self);

    fn get_animation(&self) -> Option<gdk_pixbuf::PixbufAnimation>;

    fn get_gicon(&self) -> (gio::Icon, IconSize);

    fn get_pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf>;

    fn get_pixel_size(&self) -> i32;

    fn get_storage_type(&self) -> ImageType;

    fn set_from_animation<P: IsA<gdk_pixbuf::PixbufAnimation>>(&self, animation: &P);

    fn set_from_file<P: AsRef<std::path::Path>>(&self, filename: P);

    fn set_from_gicon<P: IsA<gio::Icon>>(&self, icon: &P, size: IconSize);

    fn set_from_icon_name(&self, icon_name: Option<&str>, size: IconSize);

    fn set_from_pixbuf(&self, pixbuf: Option<&gdk_pixbuf::Pixbuf>);

    fn set_from_resource(&self, resource_path: Option<&str>);

    fn set_from_surface(&self, surface: Option<&cairo::Surface>);

    fn set_pixel_size(&self, pixel_size: i32);

    fn get_property_file(&self) -> Option<glib::GString>;

    fn set_property_file(&self, file: Option<&str>);

    fn set_property_gicon<P: IsA<gio::Icon> + SetValueOptional>(&self, gicon: Option<&P>);

    fn get_property_icon_name(&self) -> Option<glib::GString>;

    fn set_property_icon_name(&self, icon_name: Option<&str>);

    fn get_property_icon_size(&self) -> i32;

    fn set_property_icon_size(&self, icon_size: i32);

    fn set_property_pixbuf(&self, pixbuf: Option<&gdk_pixbuf::Pixbuf>);

    fn get_property_pixbuf_animation(&self) -> Option<gdk_pixbuf::PixbufAnimation>;

    fn set_property_pixbuf_animation<P: IsA<gdk_pixbuf::PixbufAnimation> + SetValueOptional>(
        &self,
        pixbuf_animation: Option<&P>,
    );

    fn get_property_resource(&self) -> Option<glib::GString>;

    fn set_property_resource(&self, resource: Option<&str>);

    fn get_property_surface(&self) -> Option<cairo::Surface>;

    fn set_property_surface(&self, surface: Option<&cairo::Surface>);

    fn get_property_use_fallback(&self) -> bool;

    fn set_property_use_fallback(&self, use_fallback: bool);

    fn connect_property_file_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_icon_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_pixbuf_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_pixbuf_animation_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_pixel_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_resource_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_storage_type_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_surface_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_use_fallback_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;
}

impl<O: IsA<Image>> ImageExt for O {
    fn clear(&self) {
        unsafe {
            ffi::gtk_image_clear(self.as_ref().to_glib_none().0);
        }
    }

    fn get_animation(&self) -> Option<gdk_pixbuf::PixbufAnimation> {
        unsafe { from_glib_none(ffi::gtk_image_get_animation(self.as_ref().to_glib_none().0)) }
    }

    fn get_gicon(&self) -> (gio::Icon, IconSize) {
        unsafe {
            let mut gicon = ptr::null_mut();
            let mut size = mem::MaybeUninit::uninit();
            ffi::gtk_image_get_gicon(
                self.as_ref().to_glib_none().0,
                &mut gicon,
                size.as_mut_ptr(),
            );
            let size = size.assume_init();
            (from_glib_none(gicon), from_glib(size))
        }
    }

    fn get_pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe { from_glib_none(ffi::gtk_image_get_pixbuf(self.as_ref().to_glib_none().0)) }
    }

    fn get_pixel_size(&self) -> i32 {
        unsafe { ffi::gtk_image_get_pixel_size(self.as_ref().to_glib_none().0) }
    }

    fn get_storage_type(&self) -> ImageType {
        unsafe {
            from_glib(ffi::gtk_image_get_storage_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_from_animation<P: IsA<gdk_pixbuf::PixbufAnimation>>(&self, animation: &P) {
        unsafe {
            ffi::gtk_image_set_from_animation(
                self.as_ref().to_glib_none().0,
                animation.as_ref().to_glib_none().0,
            );
        }
    }

    fn set_from_file<P: AsRef<std::path::Path>>(&self, filename: P) {
        unsafe {
            ffi::gtk_image_set_from_file(
                self.as_ref().to_glib_none().0,
                filename.as_ref().to_glib_none().0,
            );
        }
    }

    fn set_from_gicon<P: IsA<gio::Icon>>(&self, icon: &P, size: IconSize) {
        unsafe {
            ffi::gtk_image_set_from_gicon(
                self.as_ref().to_glib_none().0,
                icon.as_ref().to_glib_none().0,
                size.to_glib(),
            );
        }
    }

    fn set_from_icon_name(&self, icon_name: Option<&str>, size: IconSize) {
        unsafe {
            ffi::gtk_image_set_from_icon_name(
                self.as_ref().to_glib_none().0,
                icon_name.to_glib_none().0,
                size.to_glib(),
            );
        }
    }

    fn set_from_pixbuf(&self, pixbuf: Option<&gdk_pixbuf::Pixbuf>) {
        unsafe {
            ffi::gtk_image_set_from_pixbuf(self.as_ref().to_glib_none().0, pixbuf.to_glib_none().0);
        }
    }

    fn set_from_resource(&self, resource_path: Option<&str>) {
        unsafe {
            ffi::gtk_image_set_from_resource(
                self.as_ref().to_glib_none().0,
                resource_path.to_glib_none().0,
            );
        }
    }

    fn set_from_surface(&self, surface: Option<&cairo::Surface>) {
        unsafe {
            ffi::gtk_image_set_from_surface(
                self.as_ref().to_glib_none().0,
                mut_override(surface.to_glib_none().0),
            );
        }
    }

    fn set_pixel_size(&self, pixel_size: i32) {
        unsafe {
            ffi::gtk_image_set_pixel_size(self.as_ref().to_glib_none().0, pixel_size);
        }
    }

    fn get_property_file(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"file\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `file` getter")
        }
    }

    fn set_property_file(&self, file: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"file\0".as_ptr() as *const _,
                Value::from(file).to_glib_none().0,
            );
        }
    }

    fn set_property_gicon<P: IsA<gio::Icon> + SetValueOptional>(&self, gicon: Option<&P>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"gicon\0".as_ptr() as *const _,
                Value::from(gicon).to_glib_none().0,
            );
        }
    }

    fn get_property_icon_name(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"icon-name\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `icon-name` getter")
        }
    }

    fn set_property_icon_name(&self, icon_name: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"icon-name\0".as_ptr() as *const _,
                Value::from(icon_name).to_glib_none().0,
            );
        }
    }

    fn get_property_icon_size(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"icon-size\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `icon-size` getter")
                .unwrap()
        }
    }

    fn set_property_icon_size(&self, icon_size: i32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"icon-size\0".as_ptr() as *const _,
                Value::from(&icon_size).to_glib_none().0,
            );
        }
    }

    fn set_property_pixbuf(&self, pixbuf: Option<&gdk_pixbuf::Pixbuf>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"pixbuf\0".as_ptr() as *const _,
                Value::from(pixbuf).to_glib_none().0,
            );
        }
    }

    fn get_property_pixbuf_animation(&self) -> Option<gdk_pixbuf::PixbufAnimation> {
        unsafe {
            let mut value =
                Value::from_type(<gdk_pixbuf::PixbufAnimation as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"pixbuf-animation\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `pixbuf-animation` getter")
        }
    }

    fn set_property_pixbuf_animation<P: IsA<gdk_pixbuf::PixbufAnimation> + SetValueOptional>(
        &self,
        pixbuf_animation: Option<&P>,
    ) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"pixbuf-animation\0".as_ptr() as *const _,
                Value::from(pixbuf_animation).to_glib_none().0,
            );
        }
    }

    fn get_property_resource(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"resource\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `resource` getter")
        }
    }

    fn set_property_resource(&self, resource: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"resource\0".as_ptr() as *const _,
                Value::from(resource).to_glib_none().0,
            );
        }
    }

    fn get_property_surface(&self) -> Option<cairo::Surface> {
        unsafe {
            let mut value = Value::from_type(<cairo::Surface as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"surface\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `surface` getter")
        }
    }

    fn set_property_surface(&self, surface: Option<&cairo::Surface>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"surface\0".as_ptr() as *const _,
                Value::from(surface).to_glib_none().0,
            );
        }
    }

    fn get_property_use_fallback(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"use-fallback\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `use-fallback` getter")
                .unwrap()
        }
    }

    fn set_property_use_fallback(&self, use_fallback: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"use-fallback\0".as_ptr() as *const _,
                Value::from(&use_fallback).to_glib_none().0,
            );
        }
    }

    fn connect_property_file_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_file_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkImage,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Image>,
        {
            let f: &F = &*(f as *const F);
            f(&Image::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::file\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_file_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_gicon_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkImage,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Image>,
        {
            let f: &F = &*(f as *const F);
            f(&Image::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::gicon\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_gicon_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_name_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkImage,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Image>,
        {
            let f: &F = &*(f as *const F);
            f(&Image::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::icon-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_icon_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_icon_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_size_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkImage,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Image>,
        {
            let f: &F = &*(f as *const F);
            f(&Image::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::icon-size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_icon_size_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_pixbuf_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pixbuf_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkImage,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Image>,
        {
            let f: &F = &*(f as *const F);
            f(&Image::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::pixbuf\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pixbuf_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_pixbuf_animation_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_pixbuf_animation_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkImage,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Image>,
        {
            let f: &F = &*(f as *const F);
            f(&Image::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::pixbuf-animation\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pixbuf_animation_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_pixel_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pixel_size_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkImage,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Image>,
        {
            let f: &F = &*(f as *const F);
            f(&Image::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::pixel-size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pixel_size_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_resource_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_resource_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkImage,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Image>,
        {
            let f: &F = &*(f as *const F);
            f(&Image::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::resource\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_resource_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_storage_type_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_storage_type_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkImage,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Image>,
        {
            let f: &F = &*(f as *const F);
            f(&Image::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::storage-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_storage_type_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_surface_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_surface_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkImage,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Image>,
        {
            let f: &F = &*(f as *const F);
            f(&Image::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::surface\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_surface_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_use_fallback_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_fallback_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkImage,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Image>,
        {
            let f: &F = &*(f as *const F);
            f(&Image::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::use-fallback\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_use_fallback_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Image")
    }
}
