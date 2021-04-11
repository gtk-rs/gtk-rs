// Take a look at the license at the top of the repository in the LICENSE file.

use crate::rt;
use crate::Application;
use gio::ApplicationExt;
use gio::ApplicationFlags;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::{Cast, IsA, ObjectExt, ToValue};

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Default)]
pub struct ApplicationBuilder {
    app_menu: Option<gio::MenuModel>,
    menubar: Option<gio::MenuModel>,
    register_session: Option<bool>,
    action_group: Option<gio::ActionGroup>,
    application_id: Option<String>,
    flags: Option<gio::ApplicationFlags>,
    inactivity_timeout: Option<u32>,
    resource_base_path: Option<String>,
}

impl ApplicationBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Result<Application, glib::BoolError> {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref app_menu) = self.app_menu {
            properties.push(("app-menu", app_menu));
        }
        if let Some(ref menubar) = self.menubar {
            properties.push(("menubar", menubar));
        }
        if let Some(ref register_session) = self.register_session {
            properties.push(("register-session", register_session));
        }
        if let Some(ref action_group) = self.action_group {
            properties.push(("action-group", action_group));
        }
        if let Some(ref application_id) = self.application_id {
            properties.push(("application-id", application_id));
        }
        if let Some(ref flags) = self.flags {
            properties.push(("flags", flags));
        }
        if let Some(ref inactivity_timeout) = self.inactivity_timeout {
            properties.push(("inactivity-timeout", inactivity_timeout));
        }
        if let Some(ref resource_base_path) = self.resource_base_path {
            properties.push(("resource-base-path", resource_base_path));
        }
        let ret = glib::Object::new::<Application>(&properties)?;
        Application::register_startup_hook(&ret);
        Ok(ret)
    }

    pub fn app_menu<P: IsA<gio::MenuModel>>(mut self, app_menu: &P) -> Self {
        self.app_menu = Some(app_menu.clone().upcast());
        self
    }

    pub fn menubar<P: IsA<gio::MenuModel>>(mut self, menubar: &P) -> Self {
        self.menubar = Some(menubar.clone().upcast());
        self
    }

    pub fn register_session(mut self, register_session: bool) -> Self {
        self.register_session = Some(register_session);
        self
    }

    pub fn action_group<P: IsA<gio::ActionGroup>>(mut self, action_group: &P) -> Self {
        self.action_group = Some(action_group.clone().upcast());
        self
    }

    pub fn application_id(mut self, application_id: &str) -> Self {
        self.application_id = Some(application_id.to_string());
        self
    }

    pub fn flags(mut self, flags: gio::ApplicationFlags) -> Self {
        self.flags = Some(flags);
        self
    }

    pub fn inactivity_timeout(mut self, inactivity_timeout: u32) -> Self {
        self.inactivity_timeout = Some(inactivity_timeout);
        self
    }

    pub fn resource_base_path(mut self, resource_base_path: &str) -> Self {
        self.resource_base_path = Some(resource_base_path.to_string());
        self
    }
}

impl Application {
    pub(crate) fn register_startup_hook(app: &Application) {
        skip_assert_initialized!();
        let signalid: Rc<RefCell<Option<SignalHandlerId>>> = Rc::new(RefCell::new(None));
        {
            let signalid_ = signalid.clone();

            let id = app.connect_startup(move |app| {
                app.disconnect(
                    signalid_
                        .borrow_mut()
                        .take()
                        .expect("Signal ID went missing"),
                );
                unsafe { rt::set_initialized() }
            });
            *signalid.borrow_mut() = Some(id);
        }
    }

    #[doc(alias = "gtk_application_new")]
    pub fn new(
        application_id: Option<&str>,
        flags: ApplicationFlags,
    ) -> Result<Application, glib::BoolError> {
        skip_assert_initialized!();
        let app: Application = unsafe {
            Option::from_glib_full(ffi::gtk_application_new(
                application_id.to_glib_none().0,
                flags.to_glib(),
            ))
            .ok_or_else(|| glib::bool_error!("Failed to create application"))?
        };
        Application::register_startup_hook(&app);
        Ok(app)
    }
}
