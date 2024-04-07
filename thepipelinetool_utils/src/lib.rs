use std::{
    env,
    ffi::OsStr,
    fs::File,
    io::{BufRead, BufReader, Error, Read, Write},
    path::{Path, PathBuf},
    process::{self, Command, ExitStatus, Stdio},
    thread,
};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub const UPSTREAM_TASK_ID_KEY: &str = "upstream_task_id";
pub const UPSTREAM_TASK_RESULT_KEY: &str = "key";

pub fn function_name_as_string<T>(_: T) -> String {
    let name = std::any::type_name::<T>();
    let name = &name.replace(['}', '{'], "");

    // Find and cut the rest of the path
    match name[..name.len()].rfind(':') {
        Some(pos) => name[pos + 1..name.len()].into(),
        None => name[..name.len()].into(),
    }
}

pub fn value_from_file<F: for<'a> Deserialize<'a>>(file_path: &Path) -> Result<F, Error> {
    let mut file = File::open(file_path)?;
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)?;
    Ok(serde_json::from_str(&json_data)?)
}

pub fn value_to_file<F: Serialize>(v: &F, file_path: &Path) {
    let json_string = serde_json::to_string_pretty(v).unwrap();
    let mut file =
        File::create(file_path).unwrap_or_else(|e| panic!("couldn't write to file\n {e}"));

    file.write_all(json_string.as_bytes()).unwrap();
}

pub fn execute_function_using_json_files(
    in_file: &Path,
    out_file: &Path,
    task_function: &dyn Fn(Value) -> Value,
) {
    let task_args = value_from_file(in_file).unwrap(); // TODO handle error
    let task_result = (task_function)(task_args);
    value_to_file(&task_result, out_file);
    process::exit(0);
}

pub fn execute_function_using_json_str_args(
    task_args_str: &str,
    task_function: &dyn Fn(Value) -> Value,
) {
    let task_args = serde_json::from_str(task_args_str).unwrap();
    let task_result = (task_function)(task_args);
    println!("{}", serde_json::to_string(&task_result).unwrap());
    process::exit(0);
}

pub fn collector(args: Value) -> Value {
    args
}

pub fn spawn(
    mut cmd: Command,
    handle_stdout_log: Box<dyn Fn(String) + Send>,
    handle_stderr_log: Box<dyn Fn(String) + Send>,
) -> (ExitStatus, bool) {
    let mut child = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to start command");

    let mut stdout = child.stdout.take().expect("failed to take stdout");
    let mut stderr = child.stderr.take().expect("failed to take stderr");

    let stdout_handle = thread::spawn(Box::new(move || {
        let reader = BufReader::new(&mut stdout);
        for line in reader.lines() {
            let line = format!("{}\n", line.expect("failed to read line from stdout"));
            handle_stdout_log(line);
        }
    }));

    let stderr_handle = thread::spawn(Box::new(move || {
        let reader = BufReader::new(&mut stderr);
        for line in reader.lines() {
            let line = format!("{}\n", line.expect("failed to read line from stdout"));
            handle_stderr_log(line);
        }
    }));

    let status = child.wait().expect("failed to wait on child");
    let timed_out = matches!(status.code(), Some(124));
    stdout_handle.join().expect("stdout thread panicked");
    stderr_handle.join().expect("stderr thread panicked");

    (status, timed_out)
}

pub fn run_bash_command(args: &[&str], silent: bool, parse_output_as_json: bool) -> Value {
    let mut res = json!([]);
    for args in args.split(|s| *s == "&&") {
        let output = Command::new(args[0])
            .args(&args[1..])
            .output()
            .unwrap_or_else(|_| panic!("failed to run command:\n{}\n\n", args.join(" ")));
        let result_raw = String::from_utf8_lossy(&output.stdout);
        let err_raw = String::from_utf8_lossy(&output.stderr);

        if !silent {
            print!("{}", result_raw);
        }

        if !output.status.success() {
            eprint!("{}", err_raw);
            panic!("failed to run command:\n{}\n\n", args.join(" "));
        }

        if parse_output_as_json {
            res = serde_json::from_str(result_raw.to_string().trim_end())
                .unwrap_or_else(|_| json!(result_raw.to_string().trim_end()))
        } else {
            res = json!(result_raw.to_string().trim_end())
        }
    }
    res
}

pub fn create_command<P, D>(dag_path: &P, use_timeout: bool, tpt_path: &D) -> Command
where
    P: AsRef<OsStr>,
    D: AsRef<OsStr>,
{
    if use_timeout {
        Command::new("timeout")
    } else {
        let mut command = Command::new(tpt_path);
        command.arg(dag_path);
        command
    }
}

pub fn command_timeout<P, D>(
    command: &mut Command,
    dag_path: &P,
    use_timeout: bool,
    timeout_as_secs: &str,
    tpt_path: &D,
    function: &str,
) where
    P: AsRef<OsStr>,
    D: AsRef<OsStr>,
{
    if use_timeout {
        command.args(["-k", timeout_as_secs, timeout_as_secs]);
        command.arg(tpt_path);
        command.arg(dag_path);
    }

    command.args(["run", "function", function]);
}

pub fn get_dags_dir() -> String {
    env::var("DAGS_DIR")
        .unwrap_or("./bin".to_string())
        .to_string()
}

pub fn _get_dag_path_by_name(dag_name: &str) -> Option<PathBuf> {
    let dags_dir = &get_dags_dir();
    let path: PathBuf = [dags_dir, dag_name].iter().collect();

    if !path.exists() {
        return None;
    }

    Some(path)
}

// pub fn _spawner(f: Box<dyn FnMut() + Send + 'static>) -> JoinHandle<()> {
//     thread::spawn(f)
// }
