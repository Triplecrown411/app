use notify_rust::Notification;
use rusqlite::{NO_PARAMS, params, Connection, Result, Row};
use uuid::Uuid;
use chrono::prelude::*;

#[derive(Debug)]
struct Note {
    id: Option<i32>,
    uuid: Uuid,
    created: Option<DateTime<Utc>>,
    modified: Option<DateTime<Utc>>,
    notify: DateTime<Utc>,
    title: String,
    body: Option<String>,
    is_archived: bool
}

impl Note {
    fn new(title: String, body: Option<String>, notify: DateTime<Utc>) -> Note {
        Note {
            id: None,
            uuid: Uuid::new_v4(),
            created: None,
            modified: None,
            notify,
            title,
            body,
            is_archived: false
        }
    }

    fn from_sql(row: &Row) -> Note {
        Note {
            id: Some(row.get_unwrap(0)),
            uuid: row.get_unwrap(1),
            created: row.get_unwrap(2),
            modified: row.get_unwrap(3),
            notify: row.get_unwrap(4),
            title: row.get_unwrap(5),
            body: row.get_unwrap(6),
            is_archived: row.get_unwrap(7)
        }
    }

    fn from_uuid(conn: &Connection, uuid: Uuid) -> Result<Note> {
        conn.query_row("SELECT id, uuid, created, modified, notify, title, body, is_archived FROM notes WHERE uuid = ?1", params![uuid], |row| Ok(Note::from_sql(row)))
    }

    fn from_id(conn: &Connection, id: i32) -> Result<Note> {
        conn.query_row("SELECT id, uuid, created, modified, notify, title, body, is_archived FROM notes WHERE id = ?1", params![id], |row| Ok(Note::from_sql(row)))
    }

    fn create(&mut self, conn: &Connection) -> Result<&mut Note> {
        let id = conn.execute("INSERT INTO notes (uuid, notify, title, body, is_archived) values
                  (?1, ?2, ?3, ?4, ?5)",
                     params![self.uuid,
                     self.notify,
                     self.title,
                     self.body.as_ref(),
                     self.is_archived],
        )?;

        self.id = Some(id as i32);

        Ok(self)
    }
}

fn init_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS notes (
             id INTEGER PRIMARY KEY,
             uuid BLOB NOT NULL,
             created TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
             modified TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
             notify TIMESTAMP,
             title TEXT NOT NULL,
             body TEXT,
             is_archived BOOL NOT NULL DEFAULT FALSE
         )",
        NO_PARAMS,
    )?;

    Ok(())
}

fn main() -> Result<()> {

    let conn = Connection::open("wizard.db")?;

    // TODO make the into a test
    // TODO make a thread that loops, sleeps checks and notifies (this will be service ultimately) until close for now
    // TODO make a separate entrypoint for CLI (for now /.wizard-note-cli 'text', 'body', 'timestamp')
    // TODO CLI with no arguments should list all items in the past that are not archived (i.e. currently alerting)
    // TODO use order by notify
    // TODO search

    init_db(&conn)?;

    let mut note = Note::new(String::from("Test Note"), Some(String::from("Test Note Body")), Utc::now());
    note.create(&conn)?;

    let mut note2 = Note::new(String::from("Test Note"), None, Utc::now());
    note2.create(&conn)?;

    println!("{:#?}", Note::from_id(&conn, note.id.unwrap()));
    println!("{:#?}", Note::from_uuid(&conn, note.uuid));

    let mut stmt = conn.prepare("SELECT id, uuid, created, modified, notify, title, body, is_archived FROM notes")?;

    let notes_iter = stmt.query_map(params![], |row| {
        Ok(Note::from_sql(row))
    })?;

    for note in notes_iter {
        let n = note.unwrap();
        Notification::new()
            .summary(&n.title)
            .body(&n.body.unwrap_or_default())
            .icon("appointment")
            .timeout(10000)
            .show().unwrap();
    }

    conn.execute("DROP TABLE notes", NO_PARAMS)?;

    Ok(())
}
