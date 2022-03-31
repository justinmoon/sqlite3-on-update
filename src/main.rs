use neon::prelude::*;

use rusqlite::{hooks::Action, Connection};

pub fn on_update(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let conn = Connection::open("test.db").unwrap();
    let callback = std::sync::Arc::new(cx.argument::<JsFunction>(0)?.root(&mut cx));
    let channel = cx.channel();
    conn.update_hook(Some(
        move |update: Action, db_name: &str, table_name: &str, row_id: i64| {
            let callback = callback.clone();
            let message = format!("{}:{}", table_name, row_id);
            dbg!("rust: ", &message);
            channel
                .send(move |mut cx| {
                    callback
                        .to_inner(&mut cx)
                        .call_with(&cx)
                        .this(cx.null())
                        .arg(cx.string(message));
                    Ok(())
                })
                .join()
                .unwrap();
        },
    ));

    Ok(cx.undefined())
}

fn test_on_update(conn: &Connection) {
    conn.update_hook(Some(
        move |update: Action, db_name: &str, table_name: &str, row_id: i64| {
            let message = format!("{}:{}", table_name, row_id);
            dbg!("rust: ", &message);
        },
    ));
    let ten_millis = time::Duration::from_millis(10);

    thread::sleep(ten_millis);
}

use std::{thread, time};

fn main() {
    let conn = Connection::open("test.db").unwrap();
    let c2 = Connection::open("test.db").unwrap();
    let delay = time::Duration::from_secs(3);
    test_on_update(&conn);

    loop {
        println!("sleeping for 3  sec ");
        thread::sleep(delay);
        conn.execute(
            "INSERT INTO test (text) VALUES ('foo')",
            rusqlite::params![],
        )
        .unwrap();
        c2.execute("INSERT INTO test (text) VALUES ('c2')", rusqlite::params![])
            .unwrap();
    }
}
