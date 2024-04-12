use std::{env, path::PathBuf};

use backend::Backend;

pub mod backend;
pub mod blanket_backend;
pub mod in_memory_backend;
pub mod pipeline_options;

use anyhow::Result;

const DEFAULT_TPT_X_COMMAND: &str = "tpt_executor";

pub fn get_tpt_executor_command() -> String {
    env::var("TPT_X_CMD")
        .unwrap_or(DEFAULT_TPT_X_COMMAND.to_string())
        .to_string()
}

// fn pop_priority_queue(backend: &mut dyn Backend) -> Option<OrderedQueuedTask> {
//     backend.pop_priority_queue()
// }
// pub trait Runner<U: Backend + BlanketBackend + Send + Sync + Clone + 'static> {
//     fn run(&mut self, ordered_queued_task: &OrderedQueuedTask);
//     fn pop_priority_queue(&mut self) -> Option<OrderedQueuedTask>;
// }

pub fn get_pipelines_dir() -> String {
    env::var("PIPELINES_DIR")
        .unwrap_or("./bin".to_string())
        .to_string()
}

pub fn get_pipeline_path_buf_by_name(pipeline_name: &str) -> Result<PathBuf> {
    let pipelines_dir = &get_pipelines_dir();
    let path: PathBuf = [pipelines_dir, pipeline_name].iter().collect();

    if !path.exists() {
        return Err(anyhow::Error::msg("missing pipeline"));
    }

    Ok(path)
}
