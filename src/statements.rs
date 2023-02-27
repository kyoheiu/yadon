pub fn state_create_table() -> String {
    "
     CREATE TABLE IF NOT EXISTS bin (
     id TEXT PRIMARY KEY,
     body TEXT,
     timestamp DATETIME
     )
    "
    .to_owned()
}

pub fn state_upload(ulid: &str, body: &str) -> String {
    format!(
        "
         INSERT INTO bin (id, body, timestamp)
         VALUES (
             '{}',
             '{}',
             datetime('now', 'localtime')
         );
        ",
        ulid, body
    )
}

pub fn state_download(id: &str) -> String {
    format!(
        "
         SELECT *
         FROM bin
         WHERE id = '{}';
        ",
        id
    )
}
