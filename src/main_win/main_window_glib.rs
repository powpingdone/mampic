use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, CompositeTemplate};
use once_cell::sync::{Lazy, OnceCell};

use crate::config::{APP_ID, PROFILE};
use crate::main_win::MainWindow as BaseT;

#[derive(Debug, CompositeTemplate)]
#[template(resource = "/xyz/powpingdone/Mampic/ui/window.ui")]
pub struct MainWindow {
    #[template_child]
    pub headerbar: TemplateChild<adw::HeaderBar>,

    pub settings: gio::Settings,
    pub server_name: OnceCell<String>,
    pub server_url: OnceCell<String>,
    pub username: OnceCell<String>,
    pub password: OnceCell<String>,
    pub server_type: OnceCell<String>,
    pub server_display: OnceCell<String>,
}

impl Default for MainWindow {
    fn default() -> Self {
        Self {
            headerbar: TemplateChild::default(),
            settings: gio::Settings::new(APP_ID),
            server_name: OnceCell::new(),
            server_url: OnceCell::new(),
            username: OnceCell::new(),
            password: OnceCell::new(),
            server_type: OnceCell::new(),
            server_display: OnceCell::new(),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for MainWindow {
    const NAME: &'static str = "MainWindow";
    type Type = BaseT;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    // You must call `Widget`'s `init_template()` within `instance_init()`.
    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for MainWindow {
    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
            vec![
                glib::ParamSpecString::new(
                    "server-name",
                    "server-name",
                    "name of server",
                    None,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecString::new(
                    "server-url",
                    "server-url",
                    "what points to the server",
                    None,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecString::new(
                    "username",
                    "username",
                    "the name of the user to access the server",
                    None,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecString::new(
                    "password",
                    "password",
                    "the password of the user to access the server",
                    None,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecString::new(
                    "server-type",
                    "server-display",
                    "type of server",
                    None,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecString::new(
                    "server-display",
                    "server-display",
                    "type of server displayed",
                    None,
                    glib::ParamFlags::READWRITE,
                ),
            ]
        });
        &PROPERTIES
    }

    fn set_property(
        &self,
        _obj: &Self::Type,
        _id: usize,
        value: &glib::Value,
        pspec: &glib::ParamSpec,
    ) {
        macro_rules! s_p {
            ( $var:expr, $val:expr ) => {
                $var.set($val.get().expect("$val was supposted to be of type string"))
                    .expect("$var was already set")
            };
        }

        match pspec.name() {
            "server-name" => s_p!(self.server_name, value),
            "server-url" => s_p!(self.server_url, value),
            "username" => s_p!(self.username, value),
            "password" => s_p!(self.password, value),
            "server-type" => s_p!(self.server_type, value),
            "server-display" => s_p!(self.server_display, value),
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        macro_rules! g_p {
            ( $var:expr ) => {
                $var.get()
                    .unwrap_or(&"NOT CURRENTLY SET".to_string())
                    .to_value()
            };
        }

        match pspec.name() {
            "server-name" => g_p!(self.server_name),
            "server-url" => g_p!(self.server_url),
            "username" => g_p!(self.username),
            "password" => g_p!(self.password),
            "server-type" => g_p!(self.server_type),
            "server-display" => g_p!(self.server_display),
            _ => unimplemented!(),
        }
    }

    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);

        // Devel Profile
        if PROFILE == "Devel" {
            obj.add_css_class("devel");
        }

        // Load latest window state
        obj.load_window_size();
    }
}

impl WindowImpl for MainWindow {
    // Save window state on delete event
    fn close_request(&self, window: &Self::Type) -> gtk::Inhibit {
        if let Err(err) = window.save_window_size() {
            log::warn!("Failed to save window state, {}", &err);
        }

        // Pass close request on to the parent
        self.parent_close_request(window)
    }
}

impl WidgetImpl for MainWindow {}
impl ApplicationWindowImpl for MainWindow {}
impl AdwApplicationWindowImpl for MainWindow {}
