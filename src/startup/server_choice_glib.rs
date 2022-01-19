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
        match pspec.name() {
            "server-name" => {
                let inp: String = value.get().expect("value should be of type String");
                *(self.server_name.borrow_mut()) = inp;
            }
            "server-type" => {
                let inp: String = value.get().expect("value should be of type String");
                *(self.server_type.borrow_mut()) = inp.clone();

                // update display name too
                obj.set_property(
                    "server-type-display",
                    inp.chars().fold("".to_string(), |accum, x| {
                        if accum == "" {
                            accum + &x.to_uppercase().to_string()
                        } else if x == '-' {
                            accum + " "
                        } else {
                            accum + &x.to_string()
                        }
                    }),
                );

                if obj.is_realized() {
                    obj.update_icon();
                }
            }
            "server-type-display" => {
                *(self.server_type_display.borrow_mut()) =
                    value.get().expect("value should be string");
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            "server-name" => self.server_name.borrow().to_value(),
            "server-type" => self.server_type.borrow().to_value(),
            "server-type-display" => self.server_type_display.borrow().to_value(),
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
