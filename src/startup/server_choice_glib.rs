use crate::startup::type_to_name;
use crate::startup::ServerChoiceWidget as BaseT;
use adw::subclass::prelude::*;
use gettextrs::gettext;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use once_cell::sync::Lazy;
use std::cell::RefCell;

#[derive(Default, gtk::CompositeTemplate)]
#[template(resource = "/xyz/powpingdone/Mampic/ui/server-choice.ui")]
pub struct ServerChoiceWidget {
    #[template_child]
    pub server_name_label: TemplateChild<gtk::Label>,
    #[template_child]
    pub server_type_label: TemplateChild<gtk::Label>,
    #[template_child]
    pub icon: TemplateChild<gtk::Image>,

    pub server_name: RefCell<String>,
    pub server_type: RefCell<String>,
    pub server_type_display: RefCell<String>,
    pub raw_server_string: RefCell<String>,
}

#[glib::object_subclass]
impl ObjectSubclass for ServerChoiceWidget {
    const NAME: &'static str = "ServerChoiceWidget";
    type Type = BaseT;
    type ParentType = adw::Bin;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for ServerChoiceWidget {
    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
            vec![
                glib::ParamSpecString::new(
                    "server-name",
                    "server-name",
                    "name of server",
                    Some(&gettext("No name")),
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecString::new(
                    "server-type",
                    "server-type",
                    "type of server",
                    Some(&"no-server"),
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecString::new(
                    "server-type-display",
                    "server-type-display",
                    "type of server displayed",
                    Some(&"No Server"),
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecString::new(
                    "raw-server-string",
                    "raw-server-string",
                    "self reference to be passed to properties",
                    Some(&""),
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
            "server-name" => s_p!(self.server_name, value),
            "server-type-display" => s_p!(self.server_type_display, value),
            "raw-server-string" => s_p!(self.raw_server_string, value),
            "server-type" => {
                s_p!(self.server_type, value);

                // update display name too
                obj.set_property(
                    "server-type-display",
                    type_to_name!(value.get::<String>().expect("value should be string")),
                );

                if obj.is_realized() {
                    obj.update_icon();
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
            "server-name" => g_p!(self.server_name),
            "server-type" => g_p!(self.server_type),
            "server-type-display" => g_p!(self.server_type_display),
            "raw-server-string" => g_p!(self.raw_server_string),
            _ => unimplemented!(),
        }
    }
}

impl WidgetImpl for ServerChoiceWidget {
    fn realize(&self, widget: &Self::Type) {
        self.parent_realize(widget);

        widget.update_icon();
    }
}

impl BinImpl for ServerChoiceWidget {}
