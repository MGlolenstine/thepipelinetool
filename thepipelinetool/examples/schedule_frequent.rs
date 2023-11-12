use graph::dag::get_dag;
use thepipelinetool::prelude::*;

fn main() {
    fn hi(args: Value) -> Value {
        println!("{}", args);

        json!({
            "hello": "world"
        })
    }

    let mut dag = get_dag().lock().unwrap();

    dag.options.set_schedule("*/1 * * * *");
    // options
    //     .set_start_date(DateTime::parse_from_rfc3339("1996-12-19T16:39:57-08:00").unwrap());
    // options
    //     .set_end_date(DateTime::parse_from_rfc3339("1997-06-19T16:39:57-08:00").unwrap());
    // options.catchup = true;

    let a = add_task(hi, json!({}), TaskOptions::default());
    let b = add_task(hi, json!({}), TaskOptions::default());
    let _c = add_task(
        hi,
        json!([a.value(), b.get("hello")]),
        TaskOptions {
            timeout: None,
            ..Default::default()
        },
    );

    parse_cli();
}