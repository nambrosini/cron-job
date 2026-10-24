use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

use chrono::FixedOffset;
use cron_job::{CronJob, Job};

struct NewJob {
    sender: Sender<i32>,
}

impl Job for NewJob {
    fn run(&mut self) {
        self.sender.send(1).unwrap();
    }
}

/// Runs an every-second job for `window` and returns how many times it fired.
fn count_runs(mut cron: CronJob, window: Duration) -> usize {
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    thread::spawn(move || {
        cron.new_job("* * * * * *", NewJob { sender: tx }).unwrap();
        cron.start();
    });

    thread::sleep(window);
    rx.try_iter().count()
}

// A 5.1s window covers 5 or 6 second boundaries depending on start phase.
#[test]
fn test() {
    let count = count_runs(CronJob::default(), Duration::from_millis(5100));
    assert!((5..=6).contains(&count), "ran {count} times");
}

#[test]
fn test_with_offset() {
    let offset = FixedOffset::east_opt(2 * 3600).unwrap();
    let count = count_runs(CronJob::new(Some(offset)), Duration::from_millis(5100));
    assert!((5..=6).contains(&count), "ran {count} times");
}

#[test]
fn slow_job_skips_overrun_ticks() {
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    thread::spawn(move || {
        let mut cron = CronJob::default();
        cron.new_job("* * * * * *", move || {
            tx.send(1).unwrap();
            thread::sleep(Duration::from_millis(1500));
        })
        .unwrap();
        cron.start();
    });

    thread::sleep(Duration::from_millis(4100));
    // Each run takes 1.5s and blocks the next tick, so runs land every ~2s.
    let count = rx.try_iter().count();
    assert!((2..=3).contains(&count), "ran {count} times");
}

#[test]
fn start_returns_when_no_jobs() {
    CronJob::default().start();
}

#[test]
fn new_job_rejects_invalid_expression() {
    let mut cron = CronJob::default();
    assert!(cron.new_job("not a cron", || {}).is_err());
    assert!(cron.new_job("* * * * * *", || {}).is_ok());
    assert_eq!(cron.get_schedules().count(), 1);
}
