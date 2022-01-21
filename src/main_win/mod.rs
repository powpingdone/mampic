mod main_window_glib;

use crate::application::ExampleApplication;
use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib};

glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<main_window_glib::MainWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl MainWindow {
    pub fn new(app: &ExampleApplication) -> Self {
        glib::Object::new(&[("application", app)]).expect("Failed to create MainWindow")
    }

    pub fn setup(&self, server_info: String) {
        // list in order of properties to set
        let props = vec![
            "server-name",
            "server-url",
            "username",
            "password",
            "server-type",
            "server-display",
        ];
        let server_info = server_info
            .strip_suffix("\n")
            // yes this is a serious error, meaning that the configuration got corrupted poss
            .unwrap_or_else(|| {
                panic!("Expected a newline but got none: {}", server_info);
            });

        // check length
        let server_info_len = server_info.split(" :: ").count();
        if server_info_len != props.len() {
            panic!(
                "input does not match with properties to set: expected {}, got {}, input was {}",
                props.len(),
                server_info_len,
                server_info
            );
        }

        // now set properties
        for (prop, value) in props.iter().zip(server_info.split(" :: ")) {
            // it will panic if the property setting goes wrong
            self.set_property(prop, value);
        }
    }

    fn save_window_size(&self) -> Result<(), glib::BoolError> {
        let self_ = main_window_glib::MainWindow::from_instance(self);

        let (width, height) = self.default_size();

        self_.settings.set_int("window-width", width)?;
        self_.settings.set_int("window-height", height)?;

        self_
            .settings
            .set_boolean("is-maximized", self.is_maximized())?;

        Ok(())
    }

    fn load_window_size(&self) {
        let self_ = main_window_glib::MainWindow::from_instance(self);

        let width = self_.settings.int("window-width");
        let height = self_.settings.int("window-height");
        let is_maximized = self_.settings.boolean("is-maximized");

        self.set_default_size(width, height);

        if is_maximized {
            self.maximize();
        }
    }
}
