use gettextrs::*;
use gio::prelude::*;
use gtk::prelude::*;
use libhandy::prelude::*;

mod gtypes;
use crate::gtypes::NoteModel;
mod config;
mod window;
use crate::window::Window;

fn main() {
    gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));
    libhandy::init();
    setlocale(LocaleCategory::LcAll, "");
    bindtextdomain("wizard-notes", config::LOCALEDIR);
    textdomain("wizard-notes");

    let res = gio::Resource::load(config::PKGDATADIR.to_owned() + "/wizard-notes.gresource")
        .expect("Could not load resources");
    gio::resources_register(&res);

    let app = gtk::Application::new(Some("com.sam_morrow.WizardNotes"), Default::default()).unwrap();
    app.connect_activate(move |app| {
        let window = Window::new();

        window.widget.set_application(Some(app));
        app.add_window(&window.widget);
        window.widget.present();
    });

    let ret = app.run(&std::env::args().collect::<Vec<_>>());
    std::process::exit(ret);
}
