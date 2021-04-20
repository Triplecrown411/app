use gtk::prelude::*;
use libhandy::prelude::*;
use wizard_core::check;
use glib::{MainContext, PRIORITY_DEFAULT, Sender, Receiver, Continue};
use gio::prelude::*;
use gio::ListModelExt;

use crate::gtypes::NoteModel;

enum Message {
    MobileViewSwitch,
}

pub struct Window {
    pub widget: gtk::ApplicationWindow,
}

impl Window {
    pub fn new() -> Self {

        let (sender, receiver): (Sender<Message>, Receiver<Message>) = MainContext::channel(PRIORITY_DEFAULT);
        let builder = gtk::Builder::from_resource("/com/sam_morrow/WizardNotes/window.ui");
        let widget: gtk::ApplicationWindow = builder
            .get_object("window")
            .expect("Failed to find the window object");

        // let label: gtk::Label = builder
        //     .get_object("label")
        //     .expect("Failed to find the label object");
        let squeezer: libhandy::Squeezer = builder.get_object("squeezer").expect("Could not find Squeezer");

        squeezer.connect("notify::visible-child", false, move|_| {
            let _ = sender.send(Message::MobileViewSwitch);
            None
        }).unwrap();

        let note_2 = check().unwrap();
        //label.set_markup(&note_2.title);

        let squeezer_clone = squeezer.clone();
        let bottom_switcher: libhandy::ViewSwitcherBar = builder.get_object("bottom_switcher").expect("Could not get Handy Switcher");


        // let model = gtk::ListStore::new(&[&str]);
        // model.insert_with_values(0, "title", ["test"]);
        //
        // let model = gio::ListStore::new(GType::static_type());
        // model.insert
        //
        // let tree_view: gtk::TreeView = builder.get_object("todo-list").expect("Could not get todo-list element");

        let _test = NoteModel::from_note(&note_2);

        // tree_view.set_model(Some(&model));
        // let cell_renderer = gtk::CellRendererText::new();
        // let column = gtk::TreeViewColumn::new();
        //
        // column.set_sort_column_id(0);
        //
        // tree_view.append_column(&column);
        //
        // tree_view.show_now();

        receiver.attach(None, move |msg| {
            match msg {
                Message::MobileViewSwitch => {
                    let child = squeezer_clone.get_visible_child().expect("Could not get visible child");
                    let name= child.get_property("name").unwrap().get::<String>().unwrap().unwrap();
                    let x = name != String::from("headerbar_switcher");

                    bottom_switcher.set_reveal(x);
                }
            }
            Continue(true)
        });

        Self { widget }
    }
}
