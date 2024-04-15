use std::collections::HashSet;

use deadpool::Runtime;
use deadpool_redis::{Config, Pool};
use env::get_redis_url;
use redis_backend::RedisBackend;
use thepipelinetool_core::dev::*;
use thepipelinetool_runner::run::Run;
use thepipelinetool_runner::{
    backend::Backend, blanket_backend::BlanketBackend, pipeline_options::PipelineOptions,
};

// use crate::statics::{_get_default_edges, _get_default_tasks, _get_hash};
use anyhow::Result;

pub mod check_timeout;
pub mod env;
pub mod redis_backend;
pub mod routes;
pub mod scheduler;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum Executor {
    Local,
    Docker,
    Kubernetes,
}

pub fn _get_all_tasks_by_run_id(run_id: usize, pool: Pool) -> Result<Vec<Task>> {
    RedisBackend::dummy(pool).get_all_tasks(run_id)
}

pub fn _get_task_by_id(run_id: usize, task_id: usize, pool: Pool) -> Result<Task> {
    RedisBackend::dummy(pool).get_task_by_id(run_id, task_id)
}

pub async fn _get_all_task_results(
    run_id: usize,
    task_id: usize,
    pool: Pool,
) -> Result<Vec<TaskResult>> {
    RedisBackend::get_all_results(run_id, task_id, pool).await
}

pub fn _get_task_status(run_id: usize, task_id: usize, pool: Pool) -> Result<TaskStatus> {
    RedisBackend::dummy(pool).get_task_status(run_id, task_id)
}

pub fn _get_run_status(run_id: usize, pool: Pool) -> Result<i32> {
    RedisBackend::dummy(pool).get_run_status(run_id)
}

pub fn _get_task_result(run_id: usize, task_id: usize, pool: Pool) -> Result<TaskResult> {
    RedisBackend::dummy(pool).get_task_result(run_id, task_id)
}

// TODO cache response to prevent disk read

// pub fn _get_pipelines() -> Result<Vec<String>> {
//     let paths: Vec<PathBuf> = match fs::read_dir(get_pipelines_dir()) {
//         Err(e) if e.kind() == ErrorKind::NotFound => vec![],
//         Err(e) => panic!("Unexpected Error! {:?}", e),
//         Ok(entries) => entries
//             .filter_map(|entry| {
//                 let path = entry.expect("").path();
//                 if path.is_file() {
//                     Some(path)
//                 } else {
//                     None
//                 }
//             })
//             .collect(),
//     };

//     Ok(paths
//         .iter()
//         .map(|p| {
//             p.file_name()
//                 .and_then(|os_str| os_str.to_str())
//                 .expect("")
//                 .to_string()
//         })
//         .collect())
// }

// pub async fn _trigger_run<T>(
//     run_id: usize,
//     pipeline_name: &str,
//     scheduled_date_for_run: DateTime<Utc>,
//     pool: Pool,
//     trigger_params: Option<Value>,
//     mut backend: T,
// ) where
//     T: BlanketBackend,
// {
//     let hash = _get_hash(pipeline_name);
//     backend.enqueue_run(
//         run_id,
//         pipeline_name,
//         &hash,
//         scheduled_date_for_run,
//         trigger_params,
//     )
// }

//
pub fn get_redis_pool() -> Result<Pool> {
    let cfg = Config::from_url(get_redis_url());
    Ok(cfg.create_pool(Some(Runtime::Tokio1))?)
}

pub fn _get_next_run(options: &PipelineOptions) -> Vec<Value> {
    todo!();
    // if let Some(schedule) = &options.schedule {
    //     match schedule.parse::<Cron>() {
    //         Ok(cron) => {
    //             if !cron.any() {
    //                 info!("Cron will never match any given time!");
    //                 return vec![];
    //             }

    //             if let Some(start_date) = options.start_date {
    //                 info!("Start date: {start_date}");
    //             } else {
    //                 info!("Start date: None");
    //             }

    //             info!("Upcoming:");
    //             let futures = cron.clone().iter_from(
    //                 if let Some(start_date) = options.get_start_date_with_timezone() {
    //                     if options.should_catchup || start_date > Utc::now() {
    //                         start_date
    //                     } else {
    //                         Utc::now()
    //                     }
    //                 } else {
    //                     Utc::now()
    //                 },
    //             );
    //             let mut next_runs = vec![];
    //             for time in futures.take(1) {
    //                 if !cron.contains(time) {
    //                     info!("Failed check! Cron does not contain {}.", time);
    //                     break;
    //                 }
    //                 if let Some(end_date) = options.get_end_date_with_timezone() {
    //                     if time > end_date {
    //                         break;
    //                     }
    //                 }
    //                 next_runs.push(json!({
    //                     "date": format!("{}", time.format("%F %R"))
    //                 }));
    //                 info!("  {}", time.format("%F %R"));
    //             }

    //             return next_runs;
    //         }
    //         Err(err) => info!("{err}: {schedule}"),
    //     }
    // }

    vec![]
}

pub async fn _get_last_run(pipeline_name: &str, pool: Pool) -> Result<Vec<Run>> {
    let r = RedisBackend::get_last_run(pipeline_name, pool).await?;

    Ok(match r {
        Some(run) => vec![run],
        None => vec![],
    })
}

pub async fn _get_recent_runs(pipeline_name: &str, pool: Pool) -> Result<Vec<Run>> {
    RedisBackend::get_recent_runs(pipeline_name, pool).await
}

pub async fn _get_pipelines(pool: Pool) -> Result<HashSet<String>> {
    RedisBackend::get_pipelines(pool).await
}
