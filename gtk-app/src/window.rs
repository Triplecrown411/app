use gtk::prelude::*;
use libhandy::prelude::*;
use wizard_core::check;
use glib::{Value};

pub struct Window {
    pub widget: gtk::ApplicationWindow,
}

impl Window {
    pub fn new() -> Self {
        let builder = gtk::Builder::from_resource("/com/sam_morrow/WizardNotes/window.ui");
        let widget: gtk::ApplicationWindow = builder
            .get_object("window")
            .expect("Failed to find the window object");

        // let label: gtk::Label = builder
        //     .get_object("label")
        //     .expect("Failed to find the label object");
        let squeezer: libhandy::Squeezer = builder.get_object("squeezer").expect("Could not find Squeezer");
        let squeezer_clone = squeezer.clone();
        let bottom_switcher: libhandy::ViewSwitcherBar = builder.get_object("bottom_switcher").expect("Could not get Handy Switcher");

        squeezer.connect_local("notify::visible-child", false, move|_| {
            let child = squeezer_clone.get_visible_child().expect("Could not get visible child");
            let name= child.get_property("name").unwrap().get::<String>().unwrap().unwrap();
            let x = name != String::from("headerbar_switcher");

            bottom_switcher.set_reveal(x);
            None
        }).unwrap();

        let note_2 = check().unwrap();
        //label.set_markup(&note_2.title);

        Self { widget }
    }
}
