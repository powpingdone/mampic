mod server_choice_glib;
mod startup_glib;

use crate::application::ExampleApplication;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

glib::wrapper! {
    pub struct SelectServerWindow(ObjectSubclass<startup_glib::SelectServerWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl SelectServerWindow {
    pub fn new(app: &ExampleApplication) -> Self {
        glib::Object::new(&[("application", app)]).expect("Failed to create SelectServerWindow")
    }

    fn save_window_size(&self) -> Result<(), glib::BoolError> {
        let self_ = startup_glib::SelectServerWindow::from_instance(self);

        let (width, height) = self.default_size();

        self_.settings.set_int("server-width", width)?;
        self_.settings.set_int("server-height", height)?;

        self_
            .settings
            .set_boolean("server-maximized", self.is_maximized())?;

        Ok(())
    }

    fn load_window_size(&self) {
        let self_ = startup_glib::SelectServerWindow::from_instance(self);

        let width = self_.settings.int("server-width");
        let height = self_.settings.int("server-height");
        let is_maximized = self_.settings.boolean("server-maximized");

        self.set_default_size(width, height);

        if is_maximized {
            self.maximize();
        }
    }
}

glib::wrapper! {
    pub struct ServerChoiceWidget(ObjectSubclass<server_choice_glib::ServerChoiceWidget>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Buildable;
}

impl ServerChoiceWidget {
    pub fn new(server_name: &String, server_type: &String, display: &String) -> Self {
        glib::Object::new(&[
            ("server-name", server_name),
            ("server-type", server_type),
            ("server-type-display", display),
        ])
        .expect("failed to create a ServerChoiceWidget")
    }

    pub fn update_icon(&self) {
        let true_self = server_choice_glib::ServerChoiceWidget::from_instance(self);
        true_self
            .icon
            .set_icon_name(match true_self.server_type.borrow().as_ref() {
                "mpd" => Some("mpd-logo"),
                "subsonic" => Some("subsonic-logo"),
                "ampache" => Some("ampache-logo"),
                _ => Some("error-symbolic"),
            });
    }
}
