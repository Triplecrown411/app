use gio::prelude::*;
use glib::glib_wrapper;
use glib::subclass;
use glib::subclass::prelude::*;
use glib::translate::*;
use wizard_core::Note;

glib_wrapper! {
    pub struct NoteModel(Object<subclass::simple::InstanceStruct<imp::NoteModel>, subclass::simple::ClassStruct<imp::NoteModel>, NoteModelClass>);

    match fn {
        get_type => || imp::NoteModel::get_type().to_glib(),
    }
}

// Constructor for new instances. This simply calls glib::Object::new() with
// initial values for our two properties and then returns the new instance
impl NoteModel {
    pub fn new(title: &str, body: &str) -> NoteModel {
        glib::Object::new(
            Self::static_type(),
            &[
                ("title", &title),
                ("body", &body),
            ],
        )
        .expect("Failed to create")
        .downcast()
        .expect("Created with wrong type")
    }

    pub fn from_note(note: &Note) -> NoteModel {
        // pub id: Option<i32>,
        // pub uuid: Uuid,
        // pub created: Option<DateTime<Utc>>,
        // pub modified: Option<DateTime<Utc>>,
        // pub notify: DateTime<Utc>,
        // pub title: String,
        // pub body: Option<String>,
        // pub is_archived: bool
        let body = match &note.body {
            Some(b) => b.clone(),
            None => String::from("")
        };
        NoteModel::new(&note.title, &body)
    }

    // pub fn cover_url(&self) -> Option<String> {
    //     self.get_property("cover")
    //         .unwrap()
    //         .get::<&str>()
    //         .unwrap()
    //         .map(|s| s.to_string())
    // }
    //
    // pub fn uri(&self) -> Option<String> {
    //     self.get_property("uri")
    //         .unwrap()
    //         .get::<&str>()
    //         .unwrap()
    //         .map(|s| s.to_string())
    // }
}

mod imp {

    use super::*;
    use glib::{glib_object_impl, glib_object_subclass};

    use std::cell::RefCell;

    // Static array for defining the properties of the new type.
    static PROPERTIES: [subclass::Property; 2] = [
        subclass::Property("title", |title| {
            glib::ParamSpec::string(
                title,
                "Title",
                "Title",
                None,
                glib::ParamFlags::READWRITE,
            )
        }),
        subclass::Property("body", |body| {
            glib::ParamSpec::string(body, "Body", "Body", None, glib::ParamFlags::READWRITE)
        })
    ];

    // This is the struct containing all state carried with
    // the new type. Generally this has to make use of
    // interior mutability.
    pub struct NoteModel {
        title: RefCell<Option<String>>,
        body: RefCell<Option<String>>,
    }

    // ObjectSubclass is the trait that defines the new type and
    // contains all information needed by the GObject type system,
    // including the new type's name, parent type, etc.
    impl ObjectSubclass for NoteModel {
        // This type name must be unique per process.
        const NAME: &'static str = "NoteModel";

        // The parent type this one is inheriting from.
        type ParentType = glib::Object;

        // The C/FFI instance and class structs. The simple ones
        // are enough in most cases and more is only needed to
        // expose public instance fields to C APIs or to provide
        // new virtual methods for subclasses of this type.
        type Instance = subclass::simple::InstanceStruct<Self>;
        type Class = subclass::simple::ClassStruct<Self>;

        // This macro defines some boilerplate.
        glib_object_subclass!();

        // Called right before the first time an instance of the new
        // type is created. Here class specific settings can be performed,
        // including installation of properties and registration of signals
        // for the new type.
        fn class_init(klass: &mut Self::Class) {
            klass.install_properties(&PROPERTIES);
        }

        // Called every time a new instance is created. This should return
        // a new instance of our type with its basic values.
        fn new() -> Self {
            Self {
                title: RefCell::new(None),
                body: RefCell::new(None),
            }
        }
    }

    // Trait that is used to override virtual methods of glib::Object.
    impl ObjectImpl for NoteModel {
        // This macro defines some boilerplate.
        glib_object_impl!();

        // Called whenever a property is set on this instance. The id
        // is the same as the index of the property in the PROPERTIES array.
        fn set_property(&self, _obj: &glib::Object, id: usize, value: &glib::Value) {
            let prop = &PROPERTIES[id];

            match *prop {
                subclass::Property("title", ..) => {
                    let title = value
                        .get()
                        .expect("type conformity checked by `Object::set_property`");
                    self.title.replace(title);
                }
                subclass::Property("body", ..) => {
                    let body = value
                        .get()
                        .expect("type conformity checked by `Object::set_property`");
                    self.body.replace(body);
                }
                _ => unimplemented!(),
            }
        }

        // Called whenever a property is retrieved from this instance. The id
        // is the same as the index of the property in the PROPERTIES array.
        fn get_property(&self, _obj: &glib::Object, id: usize) -> Result<glib::Value, ()> {
            let prop = &PROPERTIES[id];

            match *prop {
                subclass::Property("title", ..) => Ok(self.title.borrow().to_value()),
                subclass::Property("body", ..) => Ok(self.body.borrow().to_value()),
                _ => unimplemented!(),
            }
        }
    }
}
