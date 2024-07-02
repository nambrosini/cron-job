use chrono::{DateTime, FixedOffset, Local};
use cron::Schedule;

use std::str::FromStr;
use std::thread;
use std::time::Duration;

use crate::Job;

/// The struct to create and execute all the cronjobs.
pub struct CronJob {
    jobs: Vec<Box<dyn Job>>,
    expressions: Vec<String>,
    offset: Option<FixedOffset>,
    interval: u64,
}

impl CronJob {
    /// Constructs new `CronJob` object.
    #[must_use]
    pub fn new(offset: Option<FixedOffset>, interval: u64) -> Self {
        CronJob {
            jobs: Vec::new(),
            expressions: Vec::new(),
            offset,
            interval,
        }
    }

    /// Sets the interval for the cronjobs.
    pub fn set_interval(&mut self, interval: u64) {
        self.interval = interval;
    }

    /// Sets the offset for the cronjobs.
    pub fn set_offset(&mut self, offset: FixedOffset) {
        self.offset = Some(offset);
    }

    /// Returns the schedules for all the cronjobs, with this you are able to get the next occurrences.
    /// # Errors
    /// If the schedules are not valid.
    pub fn get_schedules(&self) -> Result<Vec<Schedule>, cron::error::Error> {
        self.expressions
            .iter()
            .map(|ex| Schedule::from_str(ex))
            .collect()
    }

    /// Allows to add a new job to the cronjobs.
    pub fn new_job<J: Job>(&mut self, expression: &str, job: J) {
        self.expressions.push(expression.to_string());
        self.jobs.push(Box::new(job));
    }

    /// Starts the cronjobs without threading.
    /// # Errors
    /// If the schedules are not valid.
    pub fn start(&mut self) -> Result<(), cron::error::Error> {
        let schedules = self.get_schedules()?;
        let offset = self
            .offset
            .unwrap_or_else(|| FixedOffset::east_opt(0).unwrap());

        loop {
            let upcomings: Vec<Option<DateTime<FixedOffset>>> = schedules
                .iter()
                .map(|schedule| schedule.upcoming(offset).take(1).next())
                .collect();
            thread::sleep(Duration::from_millis(self.interval));
            let local = &Local::now();

            for (i, upcoming) in upcomings.iter().enumerate() {
                if let Some(datetime) = upcoming {
                    if datetime.timestamp() <= local.timestamp() {
                        self.jobs[i].run();
                    }
                }
            }
        }
    }
}

/// Default implementation for `CronJob`.
impl Default for CronJob {
    fn default() -> Self {
        Self {
            jobs: Vec::new(),
            expressions: Vec::new(),
            offset: None,
            interval: 500,
        }
    }
}
