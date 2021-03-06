mod models;

use crate::models::parsing::{Job, JobsList, Status, RunsList};
use std::thread;
use crate::models::formatter::{get_jobs_list_string, get_steps_list_string};
use std::time::Duration;

static BASE_URL: &'static str = "https://api.github.com";

fn main() {
    let owner: &str = "sagan1";
    let repo: &str = "venus";

    let runs_list = get_runs_list(owner, repo).expect("Getting the runs list was unsuccessful");
    let run = runs_list.get_most_recent();

    update(owner, repo, run.unwrap().id);
}

// gets the list of runs for this workflow
fn get_runs_list(owner: &str, repo: &str) -> Result<RunsList, serde_json::error::Error> {
    let runs_list_string = ureq::get(format!("{}/repos/{}/{}/actions/runs", BASE_URL, owner, repo).as_str())
        .call().into_string().unwrap();
    println!("{}", runs_list_string);
    let runs: RunsList = serde_json::from_str(runs_list_string.as_str())?;

    Ok(runs)
}

// gets the list of jobs for this run
fn get_jobs_list(owner: &str, repo: &str, run_id: i64) -> Result<JobsList, serde_json::error::Error> {
    let jobs_list_string = ureq::get(format!("{}/repos/{}/{}/actions/runs/{}/jobs", BASE_URL, owner, repo, run_id).as_str())
        .call().into_string().unwrap();

    let jobs_list: JobsList = serde_json::from_str::<JobsList>(jobs_list_string.as_str())?;

    Ok(jobs_list)
}

// repeatedly grabs updated info for the run of this id
fn update(owner: &str, repo: &str, run_id: i64) {
    let jobs_list = get_jobs_list(owner, repo, run_id).expect("Getting the jobs list was unsuccessful");

    let current_job: Option<&Job> = jobs_list.jobs.iter()
        .find(|j| matches!(j.identity.status, Status::InProgress));

    if !jobs_list.jobs.iter().all(|j| matches!(j.identity.status, Status::Completed)) {

        if current_job.is_some() {
            print!("{}{}", get_jobs_list_string(&jobs_list.jobs), get_steps_list_string(&current_job.unwrap().steps));
        } else {
            println!("No current job found");
        }

        thread::sleep(Duration::from_millis(500));
        update(owner, repo, run_id);

    } else {
        println!("Completed all jobs");
    }
}
