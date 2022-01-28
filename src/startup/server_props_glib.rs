use crate::startup::type_to_name;
use crate::startup::ServerPropertyWindow as BaseT;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;
use gtk::subclass::prelude::*;
use once_cell::sync::Lazy;
use std::cell::RefCell;

#[derive(Default, gtk::CompositeTemplate)]
#[template(resource = "/xyz/powpingdone/Mampic/ui/startup-choose.ui")]
pub struct ServerPropertyWindow {
    #[template_child]
    pub server_url: TemplateChild<gtk::EntryBuffer>,
    #[template_child]
    pub server_name: TemplateChild<gtk::EntryBuffer>,
    #[template_child]
    pub ampache: TemplateChild<gtk::ToggleButton>,
    #[template_child]
    pub subsonic: TemplateChild<gtk::ToggleButton>,
    #[template_child]
    pub mpd: TemplateChild<gtk::ToggleButton>,
    #[template_child]
    pub username: TemplateChild<gtk::EntryBuffer>,
    #[template_child]
    pub password: TemplateChild<gtk::PasswordEntry>,

    pub url: RefCell<String>,
    pub name: RefCell<String>,
    pub server_type: RefCell<String>,
    pub server_disp: RefCell<String>,
    pub uname: RefCell<String>,
    pub passwd: RefCell<String>,
}

#[glib::object_subclass]
impl ObjectSubclass for ServerPropertyWindow {
    const NAME: &'static str = "ServerPropertyWindow";
    type Type = BaseT;
    type ParentType = adw::PreferencesWindow;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for ServerPropertyWindow {
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
                    "server-type",
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
        obj: &Self::Type,
        _id: usize,
        value: &glib::Value,
        pspec: &glib::ParamSpec,
    ) {
        macro_rules! s_p {
            ( $var:expr, $val:expr ) => {
                *($var.borrow_mut()) = $val.get().expect("expected string for property $var")
            };
        }

        match pspec.name() {
            "server-name" => s_p!(self.name, value),
            "server-url" => s_p!(self.url, value),
            "username" => s_p!(self.uname, value),
            "password" => s_p!(self.passwd, value),
            "server-display" => s_p!(self.server_disp, value),

            "server-type" => {
                s_p!(self.server_type, value);

                obj.set_property("server-display", type_to_name!(self.server_type.borrow()));

                if obj.is_realized() {
                    obj.update_buttons();
                }
            }

            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        macro_rules! g_p {
            ( $var:expr ) => {
                $var.borrow().to_value()
            };
        }

        match pspec.name() {
            "server-name" => g_p!(self.name),
            "server-url" => g_p!(self.url),
            "username" => g_p!(self.uname),
            "password" => g_p!(self.passwd),
            "server-type" => g_p!(self.server_type),
            "server-display" => g_p!(self.server_disp),
            _ => unimplemented!(),
        }
    }
}

impl WidgetImpl for ServerPropertyWindow {
    fn realize(&self, widget: &Self::Type) {
        self.parent_realize(widget);

        widget.update_buttons();
    }
}

impl WindowImpl for ServerPropertyWindow {}
impl AdwWindowImpl for ServerPropertyWindow {}
impl PreferencesWindowImpl for ServerPropertyWindow {}
