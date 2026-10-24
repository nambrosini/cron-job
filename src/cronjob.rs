use chrono::{DateTime, FixedOffset, Local, TimeZone, Utc};
use cron::Schedule;

use std::panic::{self, AssertUnwindSafe};
use std::str::FromStr;
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Duration;

use crate::Job;

/// The struct to create and execute all the cronjobs.
pub struct CronJob {
    jobs: Vec<(Schedule, Box<dyn Job>)>,
    offset: Option<FixedOffset>,
    stop_tx: Sender<()>,
    stop_rx: Receiver<()>,
}

/// Stops a running [`CronJob`] from another thread; see [`CronJob::stop_handle`].
#[derive(Clone)]
pub struct StopHandle(Sender<()>);

impl StopHandle {
    /// Makes [`CronJob::start`] return as soon as it isn't running a job.
    /// If `start` isn't running yet, the next call to it returns immediately.
    pub fn stop(&self) {
        // Sending only fails once the `CronJob` is dropped, so there is nothing left to stop.
        let _ = self.0.send(());
    }
}

impl Default for CronJob {
    fn default() -> Self {
        let (stop_tx, stop_rx) = mpsc::channel();
        CronJob {
            jobs: Vec::new(),
            offset: None,
            stop_tx,
            stop_rx,
        }
    }
}

impl CronJob {
    /// Constructs new `CronJob` object.
    /// Cron expressions are evaluated in `offset`, or in the system's local
    /// timezone (DST-aware) when `None`.
    #[must_use]
    pub fn new(offset: Option<FixedOffset>) -> Self {
        CronJob {
            offset,
            ..Self::default()
        }
    }

    /// Sets the offset for the cronjobs.
    pub fn set_offset(&mut self, offset: FixedOffset) {
        self.offset = Some(offset);
    }

    /// Returns the schedules for all the cronjobs, with this you are able to get the next occurrences.
    pub fn get_schedules(&self) -> impl Iterator<Item = &Schedule> {
        self.jobs.iter().map(|(schedule, _)| schedule)
    }

    /// Allows to add a new job to the cronjobs.
    /// # Errors
    /// If `expression` is not a valid cron expression; the job is not added.
    pub fn new_job<J: Job>(&mut self, expression: &str, job: J) -> Result<(), cron::error::Error> {
        let schedule = Schedule::from_str(expression)?;
        self.jobs.push((schedule, Box::new(job)));
        Ok(())
    }

    /// Returns a handle that stops [`CronJob::start`] from another thread.
    #[must_use]
    pub fn stop_handle(&self) -> StopHandle {
        StopHandle(self.stop_tx.clone())
    }

    /// Starts the cronjobs without threading, sleeping until the next job is due.
    ///
    /// Jobs run sequentially; if a job overruns later occurrences, those are
    /// skipped rather than run back to back. A panicking job is reported by the
    /// panic hook (stderr by default) and stays scheduled; the other jobs keep
    /// running. Returns once a [`StopHandle`] is used or no job has any future
    /// occurrence (e.g. when no jobs were added).
    pub fn start(&mut self) {
        match self.offset {
            Some(offset) => self.run_in(&offset),
            None => self.run_in(&Local),
        }
    }

    fn run_in<Z: TimeZone>(&mut self, tz: &Z) {
        let now = || Utc::now().with_timezone(tz);
        let mut next_runs: Vec<Option<DateTime<Z>>> = self
            .jobs
            .iter()
            .map(|(schedule, _)| schedule.after(&now()).next())
            .collect();

        while let Some(next) = next_runs.iter().flatten().min().cloned() {
            // `to_std` fails when `next` is already in the past: no wait needed.
            let wait = next
                .signed_duration_since(now())
                .to_std()
                .unwrap_or(Duration::ZERO);
            // Sleeps until `next`, waking early on stop. `self` holds a sender, so
            // the channel never disconnects and only `Ok` means a stop request.
            if self.stop_rx.recv_timeout(wait).is_ok() {
                return;
            }

            for ((schedule, job), next_run) in self.jobs.iter_mut().zip(&mut next_runs) {
                if next_run.as_ref().is_some_and(|run_at| *run_at <= now()) {
                    // The panic hook has already reported the panic, so the result is ignored.
                    let _ = panic::catch_unwind(AssertUnwindSafe(|| job.run()));
                    *next_run = schedule.after(&now()).next();
                }
            }
        }
    }
}
