use std::{time::Duration, vec};

use thepipelinetool::prelude::*;

fn main() {
    let mut dag = DAG::new();
    let a = dag.add_command(
        json!(["bash", "-c", "sleep 3 && echo hello"]),
        TaskOptions {
            timeout: Some(Duration::new(2, 0)),
            max_attempts: 2,
            ..Default::default()
        },
    );
    let b = dag.add_command(json!(["echo", a.value()]), TaskOptions::default());

    let _c = vec![a, b];

    dag.parse_cli();
}
