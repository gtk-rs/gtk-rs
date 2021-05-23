// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::EventController;
use crate::Gesture;
use crate::GestureSingle;
use crate::PropagationPhase;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct GestureMultiPress(Object<ffi::GtkGestureMultiPress, ffi::GtkGestureMultiPressClass>) @extends GestureSingle, Gesture, EventController;

    match fn {
        type_ => || ffi::gtk_gesture_multi_press_get_type(),
    }
}

impl GestureMultiPress {
    #[doc(alias = "gtk_gesture_multi_press_new")]
    pub fn new(widget: &impl IsA<Widget>) -> GestureMultiPress {
        skip_assert_initialized!();
        unsafe {
            Gesture::from_glib_full(ffi::gtk_gesture_multi_press_new(
                widget.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-style object to construct a [`GestureMultiPress`]
    /// This method returns an instance of [`GestureMultiPressBuilder`] which can be used to create a [`GestureMultiPress`].
    pub fn builder() -> GestureMultiPressBuilder {
        GestureMultiPressBuilder::default()
    }

    #[doc(alias = "gtk_gesture_multi_press_get_area")]
    #[doc(alias = "get_area")]
    pub fn area(&self) -> Option<gdk::Rectangle> {
        unsafe {
            let mut rect = gdk::Rectangle::uninitialized();
            let ret = from_glib(ffi::gtk_gesture_multi_press_get_area(
                self.to_glib_none().0,
                rect.to_glib_none_mut().0,
            ));
            if ret {
                Some(rect)
            } else {
                None
            }
        }
    }

    #[doc(alias = "gtk_gesture_multi_press_set_area")]
    pub fn set_area(&self, rect: Option<&gdk::Rectangle>) {
        unsafe {
            ffi::gtk_gesture_multi_press_set_area(self.to_glib_none().0, rect.to_glib_none().0);
        }
    }

    #[doc(alias = "pressed")]
    pub fn connect_pressed<F: Fn(&Self, i32, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn pressed_trampoline<
            F: Fn(&GestureMultiPress, i32, f64, f64) + 'static,
        >(
            this: *mut ffi::GtkGestureMultiPress,
            n_press: libc::c_int,
            x: libc::c_double,
            y: libc::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), n_press, x, y)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"pressed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    pressed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "released")]
    pub fn connect_released<F: Fn(&Self, i32, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn released_trampoline<
            F: Fn(&GestureMultiPress, i32, f64, f64) + 'static,
        >(
            this: *mut ffi::GtkGestureMultiPress,
            n_press: libc::c_int,
            x: libc::c_double,
            y: libc::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), n_press, x, y)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"released\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    released_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "stopped")]
    pub fn connect_stopped<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn stopped_trampoline<F: Fn(&GestureMultiPress) + 'static>(
            this: *mut ffi::GtkGestureMultiPress,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"stopped\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    stopped_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`GestureMultiPress`].
pub struct GestureMultiPressBuilder {
    button: Option<u32>,
    exclusive: Option<bool>,
    touch_only: Option<bool>,
    n_points: Option<u32>,
    window: Option<gdk::Window>,
    propagation_phase: Option<PropagationPhase>,
    widget: Option<Widget>,
}

impl GestureMultiPressBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`GestureMultiPressBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`GestureMultiPress`].
    pub fn build(self) -> GestureMultiPress {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref button) = self.button {
            properties.push(("button", button));
        }
        if let Some(ref exclusive) = self.exclusive {
            properties.push(("exclusive", exclusive));
        }
        if let Some(ref touch_only) = self.touch_only {
            properties.push(("touch-only", touch_only));
        }
        if let Some(ref n_points) = self.n_points {
            properties.push(("n-points", n_points));
        }
        if let Some(ref window) = self.window {
            properties.push(("window", window));
        }
        if let Some(ref propagation_phase) = self.propagation_phase {
            properties.push(("propagation-phase", propagation_phase));
        }
        if let Some(ref widget) = self.widget {
            properties.push(("widget", widget));
        }
        glib::Object::new::<GestureMultiPress>(&properties)
            .expect("Failed to create an instance of GestureMultiPress")
    }

    pub fn button(mut self, button: u32) -> Self {
        self.button = Some(button);
        self
    }

    pub fn exclusive(mut self, exclusive: bool) -> Self {
        self.exclusive = Some(exclusive);
        self
    }

    pub fn touch_only(mut self, touch_only: bool) -> Self {
        self.touch_only = Some(touch_only);
        self
    }

    pub fn n_points(mut self, n_points: u32) -> Self {
        self.n_points = Some(n_points);
        self
    }

    pub fn window(mut self, window: &gdk::Window) -> Self {
        self.window = Some(window.clone());
        self
    }

    pub fn propagation_phase(mut self, propagation_phase: PropagationPhase) -> Self {
        self.propagation_phase = Some(propagation_phase);
        self
    }

    pub fn widget(mut self, widget: &impl IsA<Widget>) -> Self {
        self.widget = Some(widget.clone().upcast());
        self
    }
}

impl fmt::Display for GestureMultiPress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("GestureMultiPress")
    }
}
