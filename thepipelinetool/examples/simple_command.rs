use std::vec;

use thepipelinetool::prelude::*;

fn main() {
    let mut dag = DAG::new();
    let a = dag.add_command(
        json!(["bash", "-c", "sleep 2 && echo hello"]),
        TaskOptions::default(),
    );
    let b = dag.add_command(json!(["echo", a.value()]), TaskOptions::default());

    let _c = vec![a, b];

    dag.parse_cli();
}
