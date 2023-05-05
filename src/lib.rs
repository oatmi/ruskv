use sqlite3;
use sqlite3::Value;
use std::env;

pub fn get(key: &String) {
    let connection = sqlite3::open(env::var("SQLITE_DB_FILE").unwrap()).unwrap();

    let mut cursor = connection
        .prepare("SELECT * FROM ruskv WHERE key = ?")
        .unwrap()
        .cursor();
    cursor.bind(&[Value::String(key.to_string())]).unwrap();

    while let Some(row) = cursor.next().unwrap() {
        println!("{}", row[2].as_string().unwrap());
    }
}

pub fn del(key: &String) {
    let connection = sqlite3::open(env::var("SQLITE_DB_FILE").unwrap()).unwrap();
    let sql = format!("DELETE from ruskv WHERE key = '{key}'", key=key);

    connection.execute(sql).unwrap();
}

pub fn set(k: &String, v: &String) {
    let connection = sqlite3::open(env::var("SQLITE_DB_FILE").unwrap()).unwrap();
    let sql = format!("INSERT INTO ruskv (key, val) VALUES ('{key}', '{val}')
                      ON CONFLICT(key) DO UPDATE SET val = '{val}'", key=k, val=v);

    connection.execute(sql).unwrap();
}

pub fn list() {
    let connection = sqlite3::open(env::var("SQLITE_DB_FILE").unwrap()).unwrap();

    let mut cursor = connection
        .prepare("SELECT * FROM ruskv WHERE id >= ?")
        .unwrap()
        .cursor();
    cursor.bind(&[Value::Integer(0)]).unwrap();

    while let Some(row) = cursor.next().unwrap() {
        println!("{}", row[1].as_string().unwrap());
    }
}
