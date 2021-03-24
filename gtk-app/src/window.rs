use gtk::prelude::*;
use wizard_core::check;

pub struct Window {
    pub widget: gtk::ApplicationWindow,
}

impl Window {
    pub fn new() -> Self {
        let builder = gtk::Builder::new_from_resource("/com/sam_morrow/WizardNotes/window.ui");
        let widget: gtk::ApplicationWindow = builder
            .get_object("window")
            .expect("Failed to find the window object");

        let label: gtk::Label = builder
            .get_object("label")
            .expect("Failed to find the label object");

        label.set_markup("Hello Rust!");
        check();
        Self { widget }
    }
}
