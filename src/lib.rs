use neon::prelude::*;

use rusqlite::{hooks::Action, Connection};

pub fn on_update(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let conn = Connection::open("test.db").unwrap();
    let callback = std::sync::Arc::new(cx.argument::<JsFunction>(0)?.root(&mut cx));
    let channel = cx.channel();
    conn.update_hook(Some(
        |update: Action, db_name: &str, table_name: &str, row_id: i64| {
            let message = format!("{}:{}", table_name, row_id);
            dbg!("rust: ", message);

            channel
                .send(move |mut cx| {
                    callback
                        .into_inner(&mut cx)
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

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("onUpdate", on_update)?;
    Ok(())
}
