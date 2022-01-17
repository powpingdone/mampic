use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

use crate::config::{APP_ID, PROFILE};
use crate::startup::SelectServerWindow as BaseT;

#[derive(Debug, gtk::CompositeTemplate)]
#[template(resource = "/xyz/powpingdone/Mampic/ui/startup-choose.ui")]
pub struct SelectServerWindow {
    #[template_child]
    pub viewport: TemplateChild<gtk::Box>,
    #[template_child]
    pub no_servers: TemplateChild<adw::StatusPage>,

    pub settings: gio::Settings,
}

impl Default for SelectServerWindow {
    fn default() -> Self {
        Self {
            viewport: TemplateChild::default(),
            no_servers: TemplateChild::default(),
            settings: gio::Settings::new(APP_ID),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for SelectServerWindow {
    const NAME: &'static str = "SelectServerWindow";
    type Type = BaseT;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for SelectServerWindow {
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

impl WindowImpl for SelectServerWindow {
    // Save window state on delete event
    fn close_request(&self, window: &Self::Type) -> gtk::Inhibit {
        if let Err(err) = window.save_window_size() {
            log::warn!("Failed to save window state, {}", &err);
        }

        // Pass close request on to the parent
        // since this is the select server window
        window
            .application()
            .unwrap()
            .lookup_action("quit")
            .unwrap()
            .activate(None);
        gtk::Inhibit(false)
    }
}

impl WidgetImpl for SelectServerWindow {}
impl ApplicationWindowImpl for SelectServerWindow {}
impl AdwApplicationWindowImpl for SelectServerWindow {}
