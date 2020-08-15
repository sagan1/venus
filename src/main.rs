mod models;

use crate::models::parsing::{Job, Run, JobsList, Status};
use std::process::exit;
use std::thread;
use crate::models::formatter::{get_jobs_list_string, get_steps_list_string};
use std::time::Duration;

static BASE_URL: &'static str = "https://api.github.com";

fn main() {
    let owner: &str = "sagan1";
    let repo: &str = "venus";

    let first_run: Run = match get_first_run(owner, repo) {
        Ok(run) => run,
        _ => exit(0)
    };

    let jobs_list: JobsList = match get_jobs_list(owner, repo, first_run.id) {
        Ok(jobs_list) => jobs_list,
        _ => exit(0)
    };

    let current_job: Option<&Job> = jobs_list.jobs.iter()
        .find(|j| matches!(j.identity.status, Status::InProgress));

    if jobs_list.jobs.iter().all(|j| matches!(j.identity.status, Status::Completed)) {

        if current_job.is_some() {
            println!("{}{}", get_jobs_list_string(&jobs_list.jobs), get_steps_list_string(&current_job.unwrap().steps))
        } else {
            println!("No current job found");
        }

        thread::sleep(Duration::from_secs(1));
        main()

    } else {
        println!("Completed all jobs");
    }
}

// gets the most recent run_id for a repo's workflow
fn get_first_run(owner: &str, repo: &str) -> Result<Run, serde_json::error::Error> {
    let run_id_string = ureq::get(format!("{}/repos/{}/{}/actions/runs", BASE_URL, owner, repo).as_str())
        .call().into_string().unwrap();
    let run: Run = serde_json::from_str(run_id_string.as_str())?;

    Ok(run)
}

// gets the list of jobs for this run
fn get_jobs_list(owner: &str, repo: &str, run_id: i64) -> Result<JobsList, serde_json::error::Error> {
    let jobs_list_string = ureq::get(format!("{}/repos/{}/{}/actions/runs/{}/jobs", BASE_URL, owner, repo, run_id).as_str())
        .call().into_string().unwrap();

    let jobs_list: JobsList = serde_json::from_str(jobs_list_string.as_str())?;

    Ok(jobs_list)
}
