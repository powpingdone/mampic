use crate::startup::ServerChoiceWidget as BaseT;
#[allow(unused_imports)]
use crate::startup::{AMPACHE, MPD, NO_SERVER, SUBSONIC};
use adw::subclass::prelude::*;
use gettextrs::gettext;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use once_cell::sync::Lazy;
use std::cell::{Cell, RefCell};

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
    pub server_type: Cell<u32>,
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
                glib::ParamSpecUInt::new(
                    "server-type",
                    "server-type",
                    "type of server",
                    NO_SERVER,
                    AMPACHE,
                    NO_SERVER,
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
        match pspec.name() {
            "server-name" => {
                let inp: String = value.get().expect("value should be of type String");
                *(self.server_name.borrow_mut()) = inp;
            }
            "server-type" => {
                self.server_type
                    .set(value.get().expect("value should be of type u32"));
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            "server-name" => self.server_name.borrow().to_value(),
            "server-type" => self.server_type.get().to_value(),
            _ => unimplemented!(),
        }
    }
}

impl WidgetImpl for ServerChoiceWidget {}
impl BinImpl for ServerChoiceWidget {}
