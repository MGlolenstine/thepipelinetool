use thepipelinetool_core::{prelude::*, tpt};

fn produce_data(_: ()) -> String {
    assert!(false);
    "world".to_string()
}

fn print_data(arg: String) -> () {
    assert!(false);
    println!("hello {arg}");
}

#[tpt::main]
fn main() {
    // add a task that uses the function 'produce_data'
    let task_ref = add_task(produce_data, (), &TaskOptions::default());

    let mut opts = TaskOptions::default();
    opts.trigger_rule = TriggerRule::AnyFailed;

    // add a task that depends on 'task_ref'
    let _ = add_task_with_ref(print_data, &task_ref, &opts);
}
