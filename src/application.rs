use gettextrs::gettext;
use log::{debug, info};

use adw::subclass::prelude::*;
use glib::clone;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gdk, gio, glib};

use crate::config::{APP_ID, PKGDATADIR, PROFILE, VERSION};
use crate::main_win::MainWindow;
use crate::startup::SelectServerWindow;

mod imp {
    use super::*;
    use glib::WeakRef;
    use once_cell::sync::OnceCell;

    #[derive(Debug, Default)]
    pub struct MampicApplication {
        pub window: OnceCell<WeakRef<MainWindow>>,
        pub startup_window: OnceCell<WeakRef<SelectServerWindow>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MampicApplication {
        const NAME: &'static str = "MampicApplication";
        type Type = super::MampicApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for MampicApplication {}

    impl ApplicationImpl for MampicApplication {
        fn activate(&self, app: &Self::Type) {
            debug!("GtkApplication<MampicApplication>::activate");

            if let Some(window) = self.window.get() {
                let window = window.upgrade().unwrap();
                window.show();
                window.present();
                return;
            }

            self.window
                .set(MainWindow::new(app).downgrade())
                .expect("Window already set.");

            self.startup_window
                .set(SelectServerWindow::new(app).downgrade())
                .expect("Server window already set.");

            app.server_window().present();
        }

        fn startup(&self, app: &Self::Type) {
            debug!("GtkApplication<MampicApplication>::startup");
            self.parent_startup(app);

            // Set icons for shell
            gtk::Window::set_default_icon_name(APP_ID);

            app.setup_css();
            app.setup_gactions();
            app.setup_accels();
        }
    }

    impl GtkApplicationImpl for MampicApplication {}
    impl AdwApplicationImpl for MampicApplication {}
}

glib::wrapper! {
    pub struct MampicApplication(ObjectSubclass<imp::MampicApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl MampicApplication {
    pub fn new() -> Self {
        glib::Object::new(&[
            ("application-id", &Some(APP_ID)),
            ("flags", &gio::ApplicationFlags::empty()),
            ("resource-base-path", &Some("/xyz/powpingdone/Mampic/")),
        ])
        .expect("Application initialization failed...")
    }

    fn main_window(&self) -> MainWindow {
        let imp = imp::MampicApplication::from_instance(self);
        imp.window.get().unwrap().upgrade().unwrap()
    }

    fn server_window(&self) -> SelectServerWindow {
        let imp = imp::MampicApplication::from_instance(self);
        imp.startup_window.get().unwrap().upgrade().unwrap()
    }

    fn setup_gactions(&self) {
        // Quit
        let action_quit = gio::SimpleAction::new("quit", None);
        action_quit.connect_activate(clone!(@weak self as app => move |_, _| {
            // This is needed to trigger the delete event and saving the window state
            app.server_window().close();
            app.main_window().close();
            app.quit();
        }));
        self.add_action(&action_quit);

        // About
        let action_about = gio::SimpleAction::new("about", None);
        action_about.connect_activate(clone!(@weak self as app => move |_, _| {
            app.show_about_dialog();
        }));
        self.add_action(&action_about);

        let action_switch = gio::SimpleAction::new("switch", Some(glib::VariantTy::STRING));
        action_switch.connect_activate(clone!(@weak self as app => move |_, inp| {
            let inp: String = inp.unwrap().get().expect("string needs to be passed to MainWindow");
            app.server_window().close();
            app.main_window().setup(inp);
            app.main_window().present();
        }));
        self.add_action(&action_switch);
    }

    // Sets up keyboard shortcuts
    fn setup_accels(&self) {
        self.set_accels_for_action("app.quit", &["<primary>q"]);
    }

    fn setup_css(&self) {
        let provider = gtk::CssProvider::new();
        provider.load_from_resource("/xyz/powpingdone/Mampic/style.css");
        if let Some(display) = gdk::Display::default() {
            gtk::StyleContext::add_provider_for_display(
                &display,
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
    }

    fn show_about_dialog(&self) {
        let dialog = gtk::builders::AboutDialogBuilder::new()
            .logo_icon_name(APP_ID)
            .license_type(gtk::License::Gpl30)
            .website("https://powpingdone.xyz/powpingdone/mampic/")
            .version(VERSION)
            .transient_for(&self.main_window())
            .translator_credits(&gettext("translator-credits"))
            .modal(true)
            .authors(vec!["Aidan Case".into()])
            .artists(vec!["Aidan Case".into()])
            .build();

        dialog.show();
    }

    pub fn run(&self) {
        info!("Mampic Player ({})", APP_ID);
        info!("Version: {} ({})", VERSION, PROFILE);
        info!("Datadir: {}", PKGDATADIR);

        ApplicationExtManual::run(self);
    }
}
