pub mod parsing {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct Run {
        pub id: i64
    }

    #[derive(Serialize, Deserialize)]
    pub struct Identity {
        pub name: String,
        pub status: Status,
        pub conclusion: Option<Conclusion>
    }

    #[derive(Serialize, Deserialize)]
    pub struct TimeStat {
        pub(crate) started_at: String,
        pub(crate) completed_at: Option<String>
    }

    #[derive(Serialize, Deserialize)]
    pub struct Job {
        #[serde(flatten)]
        pub identity: Identity,
        #[serde(flatten)]
        pub time_stat: TimeStat,
        pub steps: Vec<Step>
    }

    #[derive(Serialize, Deserialize)]
    pub struct JobsList {
        pub jobs: Vec<Job>
    }

    #[derive(Serialize, Deserialize)]
    pub struct Step {
        #[serde(flatten)]
        pub identity: Identity,
        #[serde(flatten)]
        pub time_stat: TimeStat
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "snake_case")]
    pub enum Status {
        Queued,
        InProgress,
        Completed
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "snake_case")]
    pub enum Conclusion {
        Failed,
        TimedOut,
        Canceled,
        Neutral,
        Success
    }
}

pub mod formatter {
    use crate::models::parsing::{Identity, Status, Conclusion, Job, TimeStat, Step};
    use colored::Colorize;
    use chrono::{Utc, DateTime};

    // get the colored name for an Identity based on its current status and conclusion
    fn color_identity_name(identity: &Identity) -> String {
        match identity.status {
            Status::Queued => identity.name.as_str().truecolor(130, 130, 130),
            Status::InProgress => identity.name.as_str().yellow(),
            Status::Completed => match identity.conclusion.as_ref().unwrap_or(&Conclusion::Neutral) {
                Conclusion::Success => identity.name.as_str().green(),
                Conclusion::Neutral => identity.name.as_str().yellow(),
                _ => identity.name.as_str().red()
            }
        }.to_string()
    }

    // gets the time elapsed in seconds for this current TimeStat
    pub fn get_seconds_elapsed(time_stat: &TimeStat) -> i64 {
        match &time_stat.completed_at {
            Some(finish) => DateTime::parse_from_rfc3339(finish.as_str()).unwrap()
                .signed_duration_since(DateTime::parse_from_rfc3339(time_stat.started_at.as_str()).unwrap()).num_seconds(),
            None => Utc::now().signed_duration_since(
                DateTime::parse_from_rfc3339(time_stat.started_at.as_str()).unwrap()).num_seconds()
        }
    }

    // converts a vector of jobs to an array-like string with colored names and elapsed time in seconds
    pub fn get_jobs_list_string(jobs: &Vec<Job>) -> String {
        let mut returnable: String = String::from("[");
        for (i, job) in jobs.iter().enumerate() {
            returnable.push_str(color_identity_name(&job.identity).as_str());
            returnable.push_str(format!("({}s)", get_seconds_elapsed(&job.time_stat)).as_str().purple().as_ref());
            returnable.push_str(if i >= jobs.len() - 1 { ", " } else { "]" });
        }
        returnable
    }

    // converts a vector of steps into a list displaying their name, status, and time elapsed
    pub fn get_steps_list_string(steps: &Vec<Step>) -> String {
        let mut returnable: String = String::new();

        for step in steps {
            returnable.push_str("\n    ");
            let status = format!("{:?}", step.identity.status).as_str().italic();
            returnable.push_str(format!("{} {} {} ({}s)", color_identity_name(&step.identity), ">".bold(), status, get_seconds_elapsed(&step.time_stat)).as_str())
        }

        returnable
    }
}
